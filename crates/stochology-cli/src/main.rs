//! Unified command-line entry point (`stochology`) for running the progressive
//! Chapter 1–9 laboratories and verifying dataset integrity (`data/SHA256SUMS`).

use std::process::ExitCode;
use stochology_cli::{resolve_workspace_root, run_all_or_chapter};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut chapter: Option<usize> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--all" | "--verify" => {
                chapter = None;
                i += 1;
            }
            "--chapter" | "-c" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --chapter requires a chapter number (1..9)");
                    return ExitCode::from(1);
                }
                match args[i + 1].parse::<usize>() {
                    Ok(c) => chapter = Some(c),
                    Err(_) => {
                        eprintln!("Error: invalid chapter number '{}'", args[i + 1]);
                        return ExitCode::from(1);
                    }
                }
                i += 2;
            }
            "--help" | "-h" => {
                println!("Usage: stochology [--all | --verify | --chapter <1..9>]");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!(
                    "Unknown argument '{other}'. Usage: stochology [--all | --verify | --chapter <1..9>]"
                );
                return ExitCode::from(1);
            }
        }
    }

    let root = resolve_workspace_root();
    match run_all_or_chapter(&root, chapter) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[FAILED] {err}");
            ExitCode::from(1)
        }
    }
}
