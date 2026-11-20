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
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut padded = Vec::with_capacity((data.len() + 9).div_ceil(64) * 64);
    padded.extend_from_slice(data);
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut w = [0_u32; 64];
        for (idx, word) in chunk.chunks_exact(4).enumerate() {
            w[idx] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, delta) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(delta);
        }
    }

    let mut hex = String::with_capacity(64);
    for word in h {
        use core::fmt::Write as _;
        let _ = write!(&mut hex, "{word:08x}");
    }
    hex
}
