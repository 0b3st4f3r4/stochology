//! Deterministic offline Wasmtime AOT `.cwasm` precompiler (`parallel_compilation(false)`).
//!
//! Outputs `sha256` and `blake3` hexadecimal digests of both the input `.wasm` artifact and the
//! precompiled `.cwasm` image across target triples (`x86_64-unknown-linux-gnu`,
//! `aarch64-unknown-linux-gnu`, etc.) for bit-for-bit reproducibility gates (`P2-C1`).

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use sha2::{Digest, Sha256};
use wasmtime::{Config, Engine};

fn main() {
    if let Err(msg) = run() {
        eprintln!("stoch_aot_repro error: {msg}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut wasm_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut target_triple: Option<String> = None;

    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--wasm" => wasm_path = Some(PathBuf::from(next_arg(&mut args, "--wasm")?)),
            "--out" => out_path = Some(PathBuf::from(next_arg(&mut args, "--out")?)),
            "--target" => target_triple = Some(next_arg(&mut args, "--target")?),
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let wasm_path = wasm_path.ok_or_else(|| "missing --wasm <path>".to_string())?;
    let out_path = out_path.ok_or_else(|| "missing --out <path>".to_string())?;

    let wasm_bytes = fs::read(&wasm_path)
        .map_err(|e| format!("failed reading {}: {e}", wasm_path.display()))?;

    let mut config = Config::new();
    config.parallel_compilation(false);
    if let Some(ref triple) = target_triple {
        config
            .target(triple)
            .map_err(|e| format!("unsupported target {triple}: {e}"))?;
    }

    let engine = Engine::new(&config).map_err(|e| format!("engine init failed: {e}"))?;
    let cwasm_bytes = engine
        .precompile_module(&wasm_bytes)
        .map_err(|e| format!("precompile_module failed: {e}"))?;

    if let Some(parent) = out_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("create_dir_all {} failed: {e}", parent.display()))?;
        }
    }
    fs::write(&out_path, &cwasm_bytes)
        .map_err(|e| format!("writing {} failed: {e}", out_path.display()))?;

    let target_label = target_triple.as_deref().unwrap_or("native");
    println!(
        "target={target_label} wasm_bytes={} wasm_sha256={} wasm_blake3={} cwasm_bytes={} cwasm_sha256={} cwasm_blake3={}",
        wasm_bytes.len(),
        sha256_hex(&wasm_bytes),
        blake3::hash(&wasm_bytes).to_hex(),
        cwasm_bytes.len(),
        sha256_hex(&cwasm_bytes),
        blake3::hash(&cwasm_bytes).to_hex(),
    );
    Ok(())
}

fn next_arg(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))
}

fn sha256_hex(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    let mut hex = String::with_capacity(64);
    for byte in digest {
        use core::fmt::Write as _;
        let _ = write!(&mut hex, "{byte:02x}");
    }
    hex
}
