#!/usr/bin/env python3
"""C1-C17 of the archived design document `03-cadeia-de-suprimentos.md` §8, run where they
can be run.

The seventeen checks of §8 are not all checks of the same kind. Some read the
tree or the built artifact and can be decided by this machine, offline, with no
account anywhere: those are implemented here and a failure is a failure. The
rest cannot be decided here for one of two reasons, and the script says which
reason for each instead of pretending to pass them:

  * `open-remote`   — needs a remote build platform, a signed tag, or an
    attestation (SLSA) that only the publishing pipeline can produce. Nothing
    local can stand in for it: a locally forged attestation would verify itself.
  * `open-artifact` — needs a published artifact this repository does not build
    yet (the npm package, the TypeScript harness, the published `.crate`). The
    check is written against the artifact the moment it exists; until then the
    gate is declared, not discharged.

An `open-*` row is reported as open with its reason and never as pass. Only a
row that is actually decided here can be green, and a green row is one this
script observed: the module gates measure the artifact built from this tree.

Module-shape gates (C2-C4) and the reproducibility gate (C1) build the artifact
twice, in two different directories, because a build that is repeated in the
same path passes even with the absolute path embedded.

Two measurements are printed side by side and neither is inherited:

  * the *manifest* build, i.e. `cargo build --release --target
    wasm32-unknown-unknown -p stoch-abi` with no release profile declared: this
    is what a reader who follows the crate's README gets today, and it carries
    the `name`, `producers` and `target_features` custom sections;
  * the *shipped-shape* build, i.e. the same command with the one setting that
    03 §5.1 condition 4 requires for the distributed artifact,
    `strip = "symbols"` (`CARGO_PROFILE_RELEASE_STRIP`), which is what removes
    those sections. C2-C4 are judged on this one, because it is the artifact
    §4.1 compares against what a registry serves.

The number in 02 §0 (2 748 bytes) is a measurement of another reduction and is
never used here. Whatever this tree produces is what is printed, and the
divergence is registered next to it.

usage: python3 .github/checks/wasm_gates.py [--root DIR] [--tmpdir DIR] [--keep]
"""

from __future__ import annotations

import argparse
import hashlib
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

LOCAL = "local"
OPEN_REMOTE = "open-remote"
OPEN_ARTIFACT = "open-artifact"

CRATE = "stoch-abi"
TARGET = "wasm32-unknown-unknown"
WASM = Path("target") / TARGET / "release" / "stoch_abi.wasm"
SIZE_BUDGET = 16384  # 03 §8 C4: 16 KiB.

# 03 §5.1 condition 4. The manifest declares no `[profile.release]`, so the
# shipped shape is supplied by environment; if the manifest ever declares it,
# this is redundant and harmless rather than different.
SHIPPED_ENV = {"CARGO_PROFILE_RELEASE_STRIP": "symbols"}

# The section set of the shipped-shape module, by wasm section id:
# Type, Function, Memory, Global, Export, Code, Data. No Custom, no Start, no
# Table, no Element, no DataCount.
EXPECTED_SECTIONS = (1, 3, 5, 6, 7, 10, 11)
EXPECTED_EXPORTS = {"memory", "step", "accept", "diagnose", "vectors_hash", "abi_version"}

SECTION_NAMES = {
    0: "Custom", 1: "Type", 2: "Import", 3: "Function", 4: "Table", 5: "Memory",
    6: "Global", 7: "Export", 8: "Start", 9: "Element", 10: "Code", 11: "Data",
    12: "DataCount",
}


# --------------------------------------------------------------- a wasm reader

def _uleb(data: bytes, offset: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while True:
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7F) << shift
        if byte < 0x80:
            return result, offset
        shift += 7


def _name(data: bytes, offset: int) -> tuple[str, int]:
    length, offset = _uleb(data, offset)
    return data[offset:offset + length].decode("utf-8", "replace"), offset + length


def sections(data: bytes) -> list[dict]:
    """Every section of a module, header included, in file order."""
    if data[:4] != b"\0asm":
        raise ValueError("not a wasm module: missing \\0asm magic")
    version = int.from_bytes(data[4:8], "little")
    found = []
    offset = 8
    while offset < len(data):
        section_id = data[offset]
        offset += 1
        size, offset = _uleb(data, offset)
        payload = data[offset:offset + size]
        offset += size
        custom = None
        if section_id == 0:
            custom, _ = _name(payload, 0)
        found.append({
            "id": section_id,
            "name": SECTION_NAMES.get(section_id, f"Unknown({section_id})"),
            "custom": custom,
            "payload": payload,
            "size": size,
        })
    return version, found


def exports_of(section: dict) -> dict[str, int]:
    count, offset = _uleb(section["payload"], 0)
    result = {}
    for _ in range(count):
        name, offset = _name(section["payload"], offset)
        kind = section["payload"][offset]
        index, offset = _uleb(section["payload"], offset + 1)
        result[name] = kind
    return result


def count_of(section: dict) -> int:
    count, _ = _uleb(section["payload"], 0)
    return count


def describe(module: list[dict]) -> dict:
    by_id = {section["id"]: section for section in module}
    return {
        "ids": [section["id"] for section in module],
        "custom": [section["custom"] for section in module if section["id"] == 0],
        "imports": count_of(by_id[2]) if 2 in by_id else 0,
        "exports": exports_of(by_id[7]) if 7 in by_id else {},
        "globals": count_of(by_id[6]) if 6 in by_id else 0,
        "memories": count_of(by_id[5]) if 5 in by_id else 0,
    }


# ------------------------------------------------------------------- the build

def build_env(tmpdir: Path, shipped: bool) -> dict:
    env = os.environ.copy()
    env["TMPDIR"] = str(tmpdir)
    env.pop("CARGO_TARGET_DIR", None)
    if shipped:
        env.update(SHIPPED_ENV)
    else:
        for key in SHIPPED_ENV:
            env.pop(key, None)
    return env


def build(tree: Path, tmpdir: Path, shipped: bool) -> Path:
    """Build the module from `tree` and return its path.

    `--locked` is not decoration. `Cargo.lock` is a tracked file: a build that is
    allowed to re-resolve can produce an artifact from dependencies the lock does
    not name, and C1 would then certify the reproducibility of a tree nobody
    committed. If the lock stops satisfying the workspace, this fails loudly here
    instead of quietly rewriting the lock and the artifact along with it.
    """
    result = subprocess.run(
        ["cargo", "build", "--release", "--target", TARGET, "-p", CRATE, "--offline", "--locked"],
        cwd=tree,
        env=build_env(tmpdir, shipped),
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError(
            "cargo build failed in " + str(tree) + "\n" + result.stdout[-2000:] + result.stderr[-4000:]
        )
    module = tree / WASM
    if not module.is_file():
        raise RuntimeError(f"{module} was not produced")
    return module


def clone_tree(root: Path, destination: Path) -> Path:
    """A build tree with the manifests and the crates, and nothing built yet.

    `AGENTS.md` travels with it because C13's sweep reads the contract at test
    time: a tree without the contract fails that test for the wrong reason, and
    a probe is only readable when the one intended difference is the difference
    it reports.
    """
    destination.mkdir(parents=True)
    for name in ("Cargo.toml", "Cargo.lock", "AGENTS.md"):
        source = root / name
        if source.is_file():
            shutil.copy2(source, destination / name)
    shutil.copytree(root / "crates", destination / "crates", ignore=shutil.ignore_patterns("target"))
    if (root / "book-src").is_dir():
        shutil.copytree(root / "book-src", destination / "book-src")
    return destination


# --------------------------------------------------------------- the gate table

class Report:
    def __init__(self) -> None:
        self.rows: list[tuple[str, str, str]] = []
        self.failed = 0

    def record(self, gate: str, status: str, detail: str) -> None:
        self.rows.append((gate, status, detail))
        if status != LOCAL and not status.startswith("pass") and not status.startswith("open"):
            self.failed += 1

    def local(self, gate: str, ok: bool, detail: str) -> bool:
        self.rows.append((gate, "pass" if ok else "FAIL", detail))
        if not ok:
            self.failed += 1
        return ok

    def open(self, gate: str, kind: str, reason: str) -> None:
        self.rows.append((gate, kind, reason))


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def rel(root: Path, path: Path) -> str:
    try:
        return str(path.relative_to(root))
    except ValueError:
        return str(path)


def gate_c1(report: Report, root: Path, a: Path, b: Path) -> Path:
    left = build(a, a.parent / "tmp", shipped=True)
    right = build(b, b.parent / "tmp", shipped=True)
    one = left.read_bytes()
    two = right.read_bytes()
    digest = hashlib.sha256(one).hexdigest()
    ok = one == two
    report.local(
        "C1",
        ok,
        f"two directories, {len(one)} bytes each, sha256 {digest[:16]}… "
        + ("identical" if ok else f"DIFFER (second: {hashlib.sha256(two).hexdigest()[:16]}…)"),
    )
    return left


def gate_c2(report: Report, module: list[dict], size: int, plain_size: int, plain_customs: list[str]) -> None:
    ids = [section["id"] for section in module]
    customs = [section["custom"] for section in module if section["id"] == 0]
    ok = tuple(ids) == EXPECTED_SECTIONS and not customs
    detail = (
        f"sections {[SECTION_NAMES.get(i, i) for i in ids]}, "
        f"{len(customs)} Custom"
        + (f" ({', '.join(customs)})" if customs else "")
    )
    if not ok:
        detail += f"; expected {[SECTION_NAMES[i] for i in EXPECTED_SECTIONS]} and zero Custom"
    report.local("C2", ok, detail + f" [shipped-shape build, {size} bytes; manifest build "
                                   f"{plain_size} bytes with {len(plain_customs)} Custom: "
                                   f"{', '.join(plain_customs) or 'none'} — no [profile.release] "
                                   f"declares strip]")


def gate_c3(report: Report, module: list[dict]) -> None:
    shape = describe(module)
    names = set(shape["exports"])
    ok = (
        shape["imports"] == 0
        and names == EXPECTED_EXPORTS
        and shape["globals"] <= 2
        and shape["memories"] == 1
        and shape["exports"].get("memory") is not None
    )
    detail = (
        f"exports {sorted(names)}, imports {shape['imports']}, "
        f"globals {shape['globals']}, memories {shape['memories']}"
    )
    if not ok:
        detail += f"; expected exports {sorted(EXPECTED_EXPORTS)}, imports 0, globals <= 2"
    report.local("C3", ok, detail)


def gate_c4(report: Report, size: int, plain_size: int) -> None:
    ok = size <= SIZE_BUDGET
    report.local(
        "C4",
        ok,
        f"measured {size} bytes <= {SIZE_BUDGET} (shipped-shape build; the manifest build "
        f"measures {plain_size}; 02 §0 carries 2 748 for another reduction and is not used)",
    )


def gate_c5(report: Report, root: Path, tmpdir: Path) -> None:
    manifest = root / "crates/stoch-core/Cargo.toml"
    text = read(manifest)
    declares = re.search(r"^\[dependencies\]", text, re.MULTILINE) is not None
    env = os.environ.copy()
    env["TMPDIR"] = str(tmpdir)
    result = subprocess.run(
        ["cargo", "tree", "-e", "normal", "--offline", "--locked", "-p", "stoch-core", "--prefix", "none"],
        cwd=root, env=env, capture_output=True, text=True,
    )
    lines = [line for line in result.stdout.splitlines() if line.strip()]
    ok = not declares and result.returncode == 0 and len(lines) == 1
    report.local(
        "C5",
        ok,
        f"crates/stoch-core/Cargo.toml "
        + ("declares [dependencies]" if declares else "declares no [dependencies]")
        + f"; `cargo tree -e normal -p stoch-core` = {len(lines)} line(s), expected 1"
        + ("" if result.returncode == 0 else f"; tree exit {result.returncode}"),
    )


def gate_c5b(report: Report, root: Path) -> None:
    manifests = [root / "Cargo.toml"] + sorted((root / "crates").glob("*/Cargo.toml"))
    offenders = []
    for manifest in manifests:
        text = read(manifest)
        for match in re.finditer(r"^\[(patch|replace|lib\.links)\]", text, re.MULTILINE):
            offenders.append(f"{rel(root, manifest)}: [{match.group(1)}]")
    build_scripts = sorted(
        path for path in root.rglob("build.rs")
        if "target" not in path.parts and ".git" not in path.parts
    )
    ok = not offenders and not build_scripts
    report.local(
        "C5b",
        ok,
        f"{len(manifests)} manifest(s), no [patch]/[replace]/[lib.links]; "
        f"{len(build_scripts)} build.rs"
        + (f"; offenders: {offenders + [rel(root, p) for p in build_scripts]}" if not ok else ""),
    )


# `unsafe` as a keyword, not as the word. A doc comment that says "no `unsafe`
# block" is the opposite of a violation, and `#![deny(unsafe_code)]` is the
# enforcement of this very gate; matching the bare word would fail on both.
UNSAFE_CODE = re.compile(r"\bunsafe\s*(?:\{|fn\b|impl\b|trait\b|extern\b)")


def unsafe_regions(text: str) -> int:
    """How many `unsafe` blocks, fns, impls, traits or externs a source has."""
    stripped = "\n".join(re.sub(r"//.*$", "", line) for line in text.splitlines())
    return len(UNSAFE_CODE.findall(stripped))


CRATE_ROOT = Path("crates/stoch-abi/src/lib.rs")
BOUNDARY = Path("crates/stoch-abi/src/abi.rs")
DENY = "#![deny(unsafe_code)]"
ALLOW = "#![allow(unsafe_code)]"


def gate_c6(report: Report, root: Path) -> None:
    """C6 in the literal form of 03 §8: deny at the crate, one allow at the module.

    Four things are required at once, and each of them can fail on its own: the
    crate root denies the raw-pointer token, the crate root does not also lift
    that denial, exactly one `#![allow(unsafe_code)]` exists in the whole
    workspace, and that one sits in `crates/stoch-abi/src/abi.rs` — the single
    module where every `unsafe` region lives. A second module that lifts the
    denial, or a stray `unsafe` outside the boundary, is the failure.
    """
    crate_root = root / CRATE_ROOT
    boundary = root / BOUNDARY
    crate_text = read(crate_root) if crate_root.is_file() else ""
    boundary_text = read(boundary) if boundary.is_file() else ""

    denies = DENY in crate_text
    crate_lifts = ALLOW in crate_text
    boundary_lifts = ALLOW in boundary_text

    allows = []
    regions = {}
    for crate in ("stoch-core", "stoch-vectors", "stoch-abi"):
        source = root / "crates" / crate / "src"
        if not source.is_dir():
            continue
        for path in sorted(source.rglob("*.rs")):
            text = read(path)
            if ALLOW in text:
                allows.append(path)
            count = unsafe_regions(text)
            if count and path.resolve() != boundary.resolve():
                regions[path] = count

    inside = unsafe_regions(boundary_text)
    only_allow = [path for path in allows if path.resolve() == boundary.resolve()]
    ok = (
        denies
        and not crate_lifts
        and boundary_lifts
        and len(allows) == 1
        and len(only_allow) == 1
        and inside > 0
        and not regions
    )
    report.local(
        "C6",
        ok,
        f"{CRATE_ROOT} denies the token: {denies}, and lifts it: {crate_lifts}; "
        f"one allow in {len(allows)} file(s): {[rel(root, p) for p in allows]}; "
        f"{BOUNDARY} carries it: {boundary_lifts}, {inside} unsafe region(s) there"
        + (f"; unsafe outside the boundary: { {rel(root, p): n for p, n in regions.items()} }" if regions else ""),
    )


def gate_c12(report: Report, root: Path) -> None:
    fixture = root / "crates/stoch-core/vectors.kv"
    lib = read(root / "crates/stoch-core/src/lib.rs")
    match = re.search(r'VECTORS_SHA256_HEX:\s*&str\s*=\s*"([0-9a-f]{64})"', lib)
    if not fixture.is_file() or match is None:
        report.local("C12", False, f"fixture {fixture.is_file()} / golden constant {match is not None}")
        return
    measured = hashlib.sha256(fixture.read_bytes()).hexdigest()
    golden = match.group(1)
    report.local(
        "C12",
        measured == golden,
        f"sha256(vectors.kv) {measured[:16]}… vs VECTORS_SHA256_HEX {golden[:16]}…",
    )


def gate_c13(report: Report, root: Path, tmpdir: Path) -> None:
    env = os.environ.copy()
    env["TMPDIR"] = str(tmpdir)
    result = subprocess.run(
        ["cargo", "test", "--offline", "--locked", "-p", "stoch-vectors", "--test", "concordance"],
        cwd=root, env=env, capture_output=True, text=True,
    )
    passed = re.search(r"test result: ok\. (\d+) passed", result.stdout)
    report.local(
        "C13",
        result.returncode == 0 and passed is not None,
        "the §V-9 sweep of 01 §7.1 reads AGENTS.md at test time and sweeps book-src/ and "
        f"crates/**/*.rs: exit {result.returncode}"
        + (f", {passed.group(1)} test(s) passed" if passed else "; no test summary"),
    )


def gate_c14(report: Report, root: Path, tmpdir: Path) -> None:
    env = os.environ.copy()
    env["TMPDIR"] = str(tmpdir)
    result = subprocess.run(
        ["cargo", "package", "-p", "stoch-core", "--list", "--offline", "--locked", "--allow-dirty"],
        cwd=root, env=env, capture_output=True, text=True,
    )
    listing = [line.strip() for line in result.stdout.splitlines() if line.strip()]
    native = [name for name in listing if name.endswith((".wasm", ".node", ".so", ".dylib", ".a", ".exe"))]
    scripts = [name for name in listing if name.endswith("build.rs")]
    required = {"Cargo.toml", "src/lib.rs", "vectors.kv", "README.md"}
    ok = result.returncode == 0 and not native and not scripts and required.issubset(set(listing))
    report.local(
        "C14",
        ok,
        f"cargo package --list -p stoch-core: {len(listing)} file(s), "
        f"{len(native)} native, {len(scripts)} build script"
        + (f"; missing {sorted(required - set(listing))}" if not required.issubset(set(listing)) else "")
        + " [local half only: `npm publish --dry-run` has no package to run on]",
    )


# ------------------------------------------------------------------------ main

def run(root: Path, tmpdir: Path, keep: bool) -> int:
    report = Report()

    scratch = tmpdir / "wasm-gates"
    if scratch.exists():
        shutil.rmtree(scratch)
    scratch.mkdir(parents=True)

    tree_a = clone_tree(root, scratch / "a")
    tree_b = clone_tree(root, scratch / "b")

    # The manifest build, measured first so that the divergence is a number and
    # not a memory: no [profile.release] is declared, so this is what a reader
    # following the crate README gets.
    plain = build(tree_a, scratch / "tmp", shipped=False)
    plain_size = plain.stat().st_size
    _, plain_module = sections(plain.read_bytes())
    plain_customs = [s["custom"] for s in plain_module if s["id"] == 0]

    shipped = gate_c1(report, root, tree_a, tree_b)
    version, module = sections(shipped.read_bytes())
    size = shipped.stat().st_size

    gate_c2(report, module, size, plain_size, plain_customs)
    gate_c3(report, module)
    gate_c4(report, size, plain_size)
    gate_c5(report, root, scratch / "tmp")
    gate_c5b(report, root)
    gate_c6(report, root)

    report.open(
        "C7", OPEN_ARTIFACT,
        "no npm package is built by this repository yet: there is no `package.json`, no "
        "`dist/` and no tarball to list. The allowlist check runs on the tarball the moment "
        "`npm pack` produces one",
    )
    report.open(
        "C8", OPEN_ARTIFACT,
        "same missing artifact: no published `package.json` to require the absence of "
        "`scripts`, `dependencies`, `optionalDependencies`, `bin` and `binary`",
    )
    report.open(
        "C9", OPEN_ARTIFACT,
        "no `dist/index.js` glue exists, so the no-I/O sweep has no subject",
    )
    report.open(
        "C10", OPEN_ARTIFACT,
        "no TypeScript sources and no `tsconfig`: `tsc --strict --noEmit` has nothing to "
        "type-check and `tsc` is not installed here",
    )
    report.open(
        "C11", OPEN_ARTIFACT,
        "parity needs the TypeScript harness (`ts/harness.mjs` -> `out-ts.txt`) to diff against "
        "the Rust side; it does not exist. `cargo test -p stoch-abi` alone is not parity, and "
        "the node smoke of `crates/stoch-abi/tests/wasm-smoke.mjs` exercises one host, not two",
    )

    gate_c12(report, root)
    gate_c13(report, root, scratch / "tmp")
    gate_c14(report, root, scratch / "tmp")

    report.open(
        "C15", OPEN_REMOTE,
        "post-publication: downloads from crates.io and npm and compares against the build job's "
        "artifact. Needs the registries and a publish, neither of which a local run has",
    )
    report.open(
        "C16", OPEN_REMOTE,
        "cross-source: diffs `src/` of the extracted published `.crate` against the checkout. "
        "Needs a published crate; the local half (what `cargo package` would put in it) is C14",
    )
    report.open(
        "C17", OPEN_REMOTE,
        "signed tag plus the SLSA attestation's `gitCommit`: `git verify-tag` needs the "
        "maintainer's key and the attestation is produced by the publishing platform. A tag "
        "made and verified on this machine would attest to nothing",
    )

    width = max(len(gate) for gate, _, _ in report.rows)
    print()
    for gate, status, detail in report.rows:
        print(f"{gate:<{width}}  {status:<12}  {detail}")

    decided = sum(1 for _, status, _ in report.rows if status == "pass")
    failed = sum(1 for _, status, _ in report.rows if status == "FAIL")
    opened = len(report.rows) - decided - failed
    print()
    print(
        f"{len(report.rows)} gates: {decided} decided here, {failed} failed, {opened} declared open "
        f"(none of them marked pass)"
    )
    print(f"fixture: {root}")
    print(f"scratch: {scratch}" + ("" if keep else " (removed)"))

    if failed:
        print()
        print("local gate(s) violated: " + ", ".join(g for g, status, _ in report.rows if status == "FAIL"))
    if not keep:
        shutil.rmtree(scratch, ignore_errors=True)
    return 1 if failed else 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=str(ROOT), help="workspace to gate (default: this checkout)")
    parser.add_argument(
        "--tmpdir",
        default=os.environ.get("TMPDIR") or str(Path.home() / ".cache/stoch-wasm-gates"),
        help="directory for cargo's temporary files; must not be a tmpfs (03 F7)",
    )
    parser.add_argument("--keep", action="store_true", help="keep the scratch trees")
    arguments = parser.parse_args(argv)

    root = Path(arguments.root).resolve()
    tmpdir = Path(arguments.tmpdir).resolve()
    tmpdir.mkdir(parents=True, exist_ok=True)

    try:
        return run(root, tmpdir, arguments.keep)
    except Exception as error:  # noqa: BLE001 - the failure has to be readable in CI
        print(f"wasm gates could not run: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
