#!/usr/bin/env python3
"""Bit-for-bit AOT `.cwasm` reproducibility and static Regime III-B independence check (P2-C1)."""

from __future__ import annotations

import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(Path(__file__).resolve().parent))
import wasm_gates  # noqa: E402

TARGETS = ("x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu")
FORBIDDEN_EMERGENCY_TOKENS = (
    "frost",
    "Frost",
    "SingleUseNonceStore",
    "HyParView",
    "Mutex",
    "RwLock",
    "async fn",
    "quorum",
)


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_kv(stdout: str) -> dict[str, str]:
    fields: dict[str, str] = {}
    for token in stdout.strip().split():
        if "=" in token:
            k, v = token.split("=", 1)
            fields[k] = v
    return fields


def check_static_regime_iiib_independence() -> None:
    emergency_rs = ROOT / "crates" / "host-workspace" / "stoch-host" / "src" / "emergency.rs"
    metrics_rs = ROOT / "crates" / "host-workspace" / "stoch-host" / "src" / "metrics.rs"
    for path in (emergency_rs, metrics_rs):
        if not path.is_file():
            raise RuntimeError(f"missing required source for static check: {path}")

    text = emergency_rs.read_text(encoding="utf-8")
    if "step(" not in text:
        raise RuntimeError("emergency.rs must call stoch_core::step directly")

    for src in (emergency_rs, metrics_rs):
        body = src.read_text(encoding="utf-8")
        for token in FORBIDDEN_EMERGENCY_TOKENS:
            if re.search(re.escape(token), body):
                raise RuntimeError(
                    f"static Regime III-B rule violated: {src.relative_to(ROOT)} contains {token!r}"
                )
    print("static-iiib  pass  emergency.rs calls stoch_core::step with zero consensus/lock/async dependencies")


def build_aot_repro_binary(tmpdir: Path) -> Path:
    env = os.environ.copy()
    env["TMPDIR"] = str(tmpdir)
    host_ws = ROOT / "crates" / "host-workspace"
    res = subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "--offline",
            "--locked",
            "-p",
            "stoch-aot",
            "--bin",
            "stoch_aot_repro",
        ],
        cwd=host_ws,
        env=env,
        capture_output=True,
        text=True,
    )
    if res.returncode != 0:
        raise RuntimeError(
            "building stoch_aot_repro failed:\n" + res.stdout[-2000:] + res.stderr[-4000:]
        )
    bin_path = ROOT / "target" / "host-workspace" / "release" / "stoch_aot_repro"
    if not bin_path.is_file():
        raise RuntimeError(f"expected binary not found at {bin_path}")
    return bin_path


def run_aot_precompile(
    aot_bin: Path,
    wasm_path: Path,
    out_path: Path,
    target: str,
    env_overrides: dict[str, str],
) -> dict[str, str]:
    env = os.environ.copy()
    env.update(env_overrides)
    res = subprocess.run(
        [
            str(aot_bin),
            "--wasm",
            str(wasm_path),
            "--out",
            str(out_path),
            "--target",
            target,
        ],
        env=env,
        capture_output=True,
        text=True,
    )
    if res.returncode != 0:
        raise RuntimeError(
            f"stoch_aot_repro failed for {target}:\n{res.stdout}\n{res.stderr}"
        )
    return parse_kv(res.stdout)


def main() -> int:
    check_static_regime_iiib_independence()

    tmp_base = Path(os.environ.get("TMPDIR", tempfile.gettempdir()))
    tmp_base.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=tmp_base, prefix="stoch-repro-aot-") as scratch:
        work = Path(scratch)
        aot_bin = build_aot_repro_binary(work)

        tree_a = wasm_gates.clone_tree(ROOT, work / "tree-a")
        tree_b = wasm_gates.clone_tree(ROOT, work / "tree-b")

        env_a = {"TZ": "UTC", "LC_ALL": "C", "SOURCE_DATE_EPOCH": "0"}
        env_b = {"TZ": "America/Sao_Paulo", "LC_ALL": "POSIX", "SOURCE_DATE_EPOCH": "0"}

        wasm_a = wasm_gates.build(tree_a, work / "tmp-a", shipped=True, extra_env=env_a)
        wasm_b = wasm_gates.build(tree_b, work / "tmp-b", shipped=True, extra_env=env_b)
        cmp_wasm = subprocess.run(["cmp", "-s", str(wasm_a), str(wasm_b)])
        if cmp_wasm.returncode != 0 or wasm_a.read_bytes() != wasm_b.read_bytes():
            raise RuntimeError("stoch_abi.wasm differs between clean perturbed builds")
        print(
            f"wasm-repro   pass  bytes={wasm_a.stat().st_size} sha256={sha256_file(wasm_a)[:16]}…"
        )

        b3sum_bin = shutil.which("b3sum")
        for target in TARGETS:
            cwasm_a = work / f"stoch_abi.{target}.a.cwasm"
            cwasm_b = work / f"stoch_abi.{target}.b.cwasm"
            info_a = run_aot_precompile(aot_bin, wasm_a, cwasm_a, target, env_a)
            info_b = run_aot_precompile(aot_bin, wasm_b, cwasm_b, target, env_b)

            cmp_cwasm = subprocess.run(["cmp", "-s", str(cwasm_a), str(cwasm_b)])
            sha_a = sha256_file(cwasm_a)
            sha_b = sha256_file(cwasm_b)
            if (
                cmp_cwasm.returncode != 0
                or sha_a != sha_b
                or info_a.get("cwasm_blake3") != info_b.get("cwasm_blake3")
            ):
                raise RuntimeError(
                    f"AOT .cwasm diverged for {target}: a={sha_a} b={sha_b}"
                )
            if b3sum_bin:
                b3_a = subprocess.check_output([b3sum_bin, str(cwasm_a)], text=True).split()[0]
                b3_b = subprocess.check_output([b3sum_bin, str(cwasm_b)], text=True).split()[0]
                if b3_a != b3_b or b3_a != info_a.get("cwasm_blake3"):
                    raise RuntimeError(f"b3sum mismatch for {target}: {b3_a} vs {b3_b}")
            print(
                f"cwasm-repro  pass  target={target} bytes={cwasm_a.stat().st_size} "
                f"sha256={sha_a[:16]}… blake3={info_a.get('cwasm_blake3', '')[:16]}…"
            )

    print("all AOT .cwasm reproducibility and static Regime III-B checks pass")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
