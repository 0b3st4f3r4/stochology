#!/usr/bin/env python3
"""Verify Cargo.toml/Cargo.lock consistency and audit all Cargo.lock files via cargo-audit."""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SKIP_DIRS = {".git", "target", "vendor-host", "book", ".venv", "__pycache__"}
TRUTHY_ENV = {"1", "true", "yes", "on"}
DEP_SECTIONS = ("dependencies", "dev-dependencies", "build-dependencies")
SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)(?:[-+].*)?$")
CLAUSE_RE = re.compile(r"^(>=|<=|>|<|=|\^|~)?\s*(.+)$")


def discover_lockfiles(root: Path) -> list[Path]:
    """Discover tracked Cargo.lock files, falling back to a pruned directory walk."""
    res = subprocess.run(
        ["git", "ls-files", "--", "Cargo.lock", "*/Cargo.lock"],
        cwd=root,
        capture_output=True,
        text=True,
    )
    lockfiles: list[Path] = []
    if res.returncode == 0:
        for line in res.stdout.splitlines():
            rel = line.strip()
            if rel:
                path = root / rel
                if path.is_file():
                    lockfiles.append(path)
    else:
        for dirpath, dirnames, filenames in os.walk(root):
            dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
            if "Cargo.lock" in filenames:
                lockfiles.append(Path(dirpath) / "Cargo.lock")

    unique = sorted({p.resolve() for p in lockfiles})
    if not unique:
        raise RuntimeError("no Cargo.lock files discovered in repository")
    return unique


def parse_locked_version(ver: str) -> tuple[int, int, int] | None:
    m = SEMVER_RE.match(ver.strip())
    if not m:
        return None
    return (int(m.group(1)), int(m.group(2)), int(m.group(3)))


def parse_req_parts(ver: str) -> tuple[int, int | None, int | None]:
    core = ver.split("-", 1)[0].split("+", 1)[0].strip()
    raw_parts = [p for p in core.split(".") if p != ""]
    if not raw_parts or len(raw_parts) > 3:
        raise ValueError(f"unsupported semver requirement part: {ver!r}")
    nums: list[int | None] = []
    for part in raw_parts:
        if part in {"*", "x", "X"}:
            nums.append(None)
        else:
            nums.append(int(part))
    while len(nums) < 3:
        nums.append(None)
    major, minor, patch = nums[0], nums[1], nums[2]
    if major is None:
        raise ValueError(f"wildcard major version unsupported in comparator: {ver!r}")
    return major, minor, patch


def bump_for_eq_or_tilde(major: int, minor: int | None, patch: int | None, tilde: bool) -> tuple[int, int, int]:
    if minor is None:
        return (major + 1, 0, 0)
    if patch is None or tilde:
        return (major, minor + 1, 0)
    return (major, minor, patch + 1)


def bump_for_caret(major: int, minor: int | None, patch: int | None) -> tuple[int, int, int]:
    if major > 0:
        return (major + 1, 0, 0)
    if minor is None:
        return (1, 0, 0)
    if minor > 0:
        return (0, minor + 1, 0)
    if patch is None:
        return (0, 1, 0)
    return (0, 0, patch + 1)


def satisfies_clause(locked: tuple[int, int, int], clause: str) -> bool:
    clause = clause.strip()
    if not clause or clause in {"*", "x", "X"}:
        return True
    m = CLAUSE_RE.match(clause)
    if not m:
        raise ValueError(f"invalid semver clause: {clause!r}")
    op, ver_str = m.group(1) or "^", m.group(2).strip()
    if ver_str in {"*", "x", "X"} and op in {"^", "="}:
        return True

    major, minor, patch = parse_req_parts(ver_str)
    lower = (major, minor or 0, patch or 0)

    if op == "^":
        upper = bump_for_caret(major, minor, patch)
        return lower <= locked < upper
    if op == "~":
        upper = bump_for_eq_or_tilde(major, minor, patch, tilde=True)
        return lower <= locked < upper
    if op == "=":
        upper = bump_for_eq_or_tilde(major, minor, patch, tilde=False)
        return lower <= locked < upper
    if op == ">=":
        return locked >= lower
    if op == ">":
        if minor is None or patch is None:
            return locked >= bump_for_eq_or_tilde(major, minor, patch, tilde=False)
        return locked > lower
    if op == "<=":
        if minor is None or patch is None:
            return locked < bump_for_eq_or_tilde(major, minor, patch, tilde=False)
        return locked <= lower
    if op == "<":
        return locked < lower
    raise ValueError(f"unsupported semver operator {op!r} in {clause!r}")


def satisfies_req(locked: tuple[int, int, int], req: str) -> bool:
    clauses = [c.strip() for c in req.split(",") if c.strip()]
    if not clauses:
        return True
    return all(satisfies_clause(locked, c) for c in clauses)


def collect_manifest_paths(lockfile: Path) -> list[Path]:
    root_manifest = lockfile.parent / "Cargo.toml"
    if not root_manifest.is_file():
        raise RuntimeError(f"missing sibling Cargo.toml for {lockfile.relative_to(ROOT)}")

    manifests = [root_manifest]
    data = tomllib.loads(root_manifest.read_text(encoding="utf-8"))
    members = data.get("workspace", {}).get("members", [])
    for pattern in members:
        matched = sorted(lockfile.parent.glob(pattern))
        for candidate in matched:
            member_toml = candidate / "Cargo.toml"
            if member_toml.is_file() and member_toml not in manifests:
                manifests.append(member_toml)
    return manifests


def iter_dep_tables(manifest_data: dict) -> list[dict]:
    tables: list[dict] = []
    for section in DEP_SECTIONS:
        tbl = manifest_data.get(section)
        if isinstance(tbl, dict):
            tables.append(tbl)
    target_tbl = manifest_data.get("target")
    if isinstance(target_tbl, dict):
        for target_cfg in target_tbl.values():
            if isinstance(target_cfg, dict):
                for section in DEP_SECTIONS:
                    tbl = target_cfg.get(section)
                    if isinstance(tbl, dict):
                        tables.append(tbl)
    return tables


def check_manifest_lock_sync(lockfile: Path) -> int:
    """Verify workspace member dependency edges and versioned direct deps match Cargo.lock."""
    lock_data = tomllib.loads(lockfile.read_text(encoding="utf-8"))
    locked_versions: dict[str, list[str]] = {}
    locked_member_deps: dict[str, set[str]] = {}
    for pkg in lock_data.get("package", []):
        name = pkg.get("name")
        version = pkg.get("version")
        if isinstance(name, str) and isinstance(version, str):
            locked_versions.setdefault(name, []).append(version)
        if isinstance(name, str) and "source" not in pkg:
            deps = {
                entry.split()[0]
                for entry in pkg.get("dependencies", [])
                if isinstance(entry, str) and entry.strip()
            }
            locked_member_deps[name] = deps

    manifests = collect_manifest_paths(lockfile)
    root_ws_deps = tomllib.loads(manifests[0].read_text(encoding="utf-8")).get("workspace", {}).get("dependencies", {})

    checked = 0
    errors: list[str] = []
    for manifest in manifests:
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        declared_dep_names: set[str] = set()
        for table in iter_dep_tables(data):
            for dep_key, spec in table.items():
                if isinstance(spec, str):
                    crate_name = dep_key
                    req = spec
                elif isinstance(spec, dict):
                    crate_name = spec.get("package", dep_key)
                    if spec.get("workspace") is True and isinstance(root_ws_deps.get(dep_key), (str, dict)):
                        ws_spec = root_ws_deps[dep_key]
                        if isinstance(ws_spec, str):
                            req = ws_spec
                        else:
                            crate_name = ws_spec.get("package", crate_name)
                            req = ws_spec.get("version")
                    else:
                        req = spec.get("version")
                else:
                    continue

                declared_dep_names.add(crate_name)
                if not isinstance(req, str):
                    continue

                checked += 1
                candidates = locked_versions.get(crate_name, [])
                if not candidates:
                    errors.append(
                        f"{manifest.relative_to(ROOT)}: dependency {crate_name!r} ({req}) is missing from {lockfile.relative_to(ROOT)}"
                    )
                    continue

                matched = False
                for ver in candidates:
                    parsed = parse_locked_version(ver)
                    if parsed is not None and satisfies_req(parsed, req):
                        matched = True
                        break
                if not matched:
                    errors.append(
                        f"{manifest.relative_to(ROOT)}: dependency {crate_name!r} requires {req!r}, "
                        f"but {lockfile.relative_to(ROOT)} locks {candidates}"
                    )

        pkg_name = data.get("package", {}).get("name")
        if isinstance(pkg_name, str):
            if pkg_name not in locked_member_deps:
                errors.append(
                    f"{manifest.relative_to(ROOT)}: package {pkg_name!r} is missing from {lockfile.relative_to(ROOT)}"
                )
            else:
                locked_deps = locked_member_deps[pkg_name]
                missing_in_lock = sorted(declared_dep_names - locked_deps)
                extra_in_lock = sorted(locked_deps - declared_dep_names)
                if missing_in_lock or extra_in_lock:
                    parts: list[str] = []
                    if missing_in_lock:
                        parts.append(f"missing in lockfile: {missing_in_lock}")
                    if extra_in_lock:
                        parts.append(f"extra in lockfile: {extra_in_lock}")
                    errors.append(
                        f"{manifest.relative_to(ROOT)} ({pkg_name}): dependency edges out of sync with "
                        f"{lockfile.relative_to(ROOT)} ({'; '.join(parts)})"
                    )

    if errors:
        raise RuntimeError("\n".join(errors))
    return checked


def resolve_cargo_audit_cmd() -> list[str]:
    direct = shutil.which("cargo-audit")
    if direct:
        return [direct, "audit"]
    cargo = shutil.which("cargo")
    if cargo:
        probe = subprocess.run([cargo, "audit", "--version"], capture_output=True, text=True)
        if probe.returncode == 0:
            return [cargo, "audit"]
    raise RuntimeError(
        "cargo-audit is not installed or not on PATH. "
        "Install it with: cargo install cargo-audit --locked"
    )


def local_advisory_db_dir() -> Path:
    cargo_home = Path(os.environ.get("CARGO_HOME", str(Path.home() / ".cargo")))
    return cargo_home / "advisory-db"


def looks_like_fetch_error(output: str) -> bool:
    lower = output.lower()
    if "vulnerabilities found" in lower or "yanked crate" in lower or "crate:" in lower:
        return False
    fetch_markers = (
        "error fetching",
        "failed to fetch",
        "advisory-db",
        "couldn't resolve",
        "could not resolve",
        "network",
        "timed out",
        "temporary failure in name resolution",
        "git fetch",
    )
    return any(marker in lower for marker in fetch_markers)


def run_cargo_audit(base_cmd: list[str], lockfile: Path, *, fetch: bool, stale: bool) -> subprocess.CompletedProcess[str]:
    cmd = [*base_cmd, "--deny", "yanked", "--file", str(lockfile)]
    if not fetch:
        cmd.append("--no-fetch")
    if stale:
        cmd.append("--stale")
    return subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)


def emit_process_output(res: subprocess.CompletedProcess[str]) -> None:
    if res.stdout:
        sys.stdout.write(res.stdout)
        if not res.stdout.endswith("\n"):
            sys.stdout.write("\n")
    if res.stderr:
        sys.stderr.write(res.stderr)
        if not res.stderr.endswith("\n"):
            sys.stderr.write("\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Disallow offline --no-fetch --stale fallback if advisory DB refresh fails (auto-enabled when CI is set).",
    )
    args = parser.parse_args()
    strict = args.strict or os.environ.get("CI", "").strip().lower() in TRUTHY_ENV

    try:
        lockfiles = discover_lockfiles(ROOT)
        base_cmd = resolve_cargo_audit_cmd()
    except RuntimeError as exc:
        print(f"security-audit error: {exc}", file=sys.stderr)
        return 1

    print(f"security-audit: discovered {len(lockfiles)} lockfile(s) (strict={strict})")
    for lf in lockfiles:
        try:
            checked = check_manifest_lock_sync(lf)
        except (RuntimeError, ValueError, tomllib.TOMLDecodeError) as exc:
            print(f"security-audit FAIL [manifest-sync] {lf.relative_to(ROOT)}:\n{exc}", file=sys.stderr)
            return 1
        print(f"  manifest-sync ok  {lf.relative_to(ROOT)} ({checked} versioned direct dep(s))")

    use_stale_fallback = False
    for idx, lf in enumerate(lockfiles):
        rel = lf.relative_to(ROOT)
        should_fetch = idx == 0 and not use_stale_fallback
        res = run_cargo_audit(base_cmd, lf, fetch=should_fetch, stale=use_stale_fallback)

        if res.returncode != 0 and should_fetch and not strict:
            combined = f"{res.stdout}\n{res.stderr}"
            db_dir = local_advisory_db_dir()
            if looks_like_fetch_error(combined) and db_dir.is_dir():
                print(
                    f"security-audit warning: failed to refresh RustSec advisory DB ({db_dir}); "
                    "falling back to --no-fetch --stale in non-strict local mode",
                    file=sys.stderr,
                )
                use_stale_fallback = True
                res = run_cargo_audit(base_cmd, lf, fetch=False, stale=True)

        if res.returncode != 0:
            emit_process_output(res)
            print(f"security-audit FAIL [cargo-audit] {rel}", file=sys.stderr)
            return res.returncode or 1

        pkg_count = len(tomllib.loads(lf.read_text(encoding="utf-8")).get("package", []))
        mode = "offline-stale" if use_stale_fallback else ("fetched" if should_fetch else "cached-db")
        print(f"  cargo-audit   ok  {rel} ({pkg_count} locked package(s), {mode})")

    print("all Rust dependency security checks pass")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
