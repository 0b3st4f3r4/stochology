//! Unified command-line entry point (`stochology`) for running the progressive
//! Chapter 1–9 laboratories, verifying dataset integrity (`data/SHA256SUMS`),
//! and exporting deterministic trajectory JSON (`--export-lab-json`) for `book-src/lab/index.html`.

use std::process::ExitCode;
use stochology_cli::{
    audit_report, export_lab_trajectories_json, resolve_workspace_root,
    run_all_or_chapter_with_brake,
};

fn execute_args(args: &[String]) -> Result<(), String> {
    let mut chapter: Option<usize> = None;
    let mut brake_override: Option<bool> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--all" | "--verify" => {
                chapter = None;
                i += 1;
            }
            "--export-lab-json" => {
                println!("{}", export_lab_trajectories_json());
                return Ok(());
            }
            "--audit-report" => {
                print!("{}", audit_report());
                return Ok(());
            }
            "--chapter" | "-c" => {
                if i + 1 >= args.len() {
                    return Err("Error: --chapter requires a chapter number (1..9)".to_string());
                }
                match args[i + 1].parse::<usize>() {
                    Ok(c) => chapter = Some(c),
                    Err(_) => {
                        return Err(format!("Error: invalid chapter number '{}'", args[i + 1]));
                    }
                }
                i += 2;
            }
            "--brake" => {
                if i + 1 >= args.len() {
                    return Err("Error: --brake requires 'on' or 'off'".to_string());
                }
                match args[i + 1].as_str() {
                    "on" => brake_override = Some(true),
                    "off" => brake_override = Some(false),
                    other => {
                        return Err(format!(
                            "Error: invalid --brake mode '{other}' (expected 'on' or 'off')"
                        ));
                    }
                }
                i += 2;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: stochology [--all | --verify | --chapter <1..9> [--brake on|off] | --export-lab-json | --audit-report]"
                );
                return Ok(());
            }
            other => {
                return Err(format!(
                    "Unknown argument '{other}'. Usage: stochology [--all | --verify | --chapter <1..9> [--brake on|off] | --export-lab-json | --audit-report]"
                ));
            }
        }
    }

    let root = resolve_workspace_root();
    run_all_or_chapter_with_brake(&root, chapter, brake_override)
}

fn run_with_args(args: &[String]) -> ExitCode {
    match execute_args(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[FAILED] {err}");
            ExitCode::from(1)
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    run_with_args(&args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_argument_parsing_covers_flags_and_errors() {
        assert_eq!(run_with_args(&["--help".to_string()]), ExitCode::SUCCESS);
        assert_eq!(
            run_with_args(&["--export-lab-json".to_string()]),
            ExitCode::SUCCESS
        );
        assert_eq!(
            run_with_args(&["--audit-report".to_string()]),
            ExitCode::SUCCESS
        );
        assert!(
            execute_args(&[
                "--all".to_string(),
                "--chapter".to_string(),
                "1".to_string()
            ])
            .is_ok()
        );
        assert!(
            execute_args(&[
                "--chapter".to_string(),
                "8".to_string(),
                "--brake".to_string(),
                "on".to_string()
            ])
            .is_ok()
        );
        assert!(
            execute_args(&[
                "--chapter".to_string(),
                "8".to_string(),
                "--brake".to_string(),
                "off".to_string()
            ])
            .is_ok()
        );
        assert!(execute_args(&["--brake".to_string()]).is_err());
        assert!(execute_args(&["--brake".to_string(), "maybe".to_string()]).is_err());
        assert_eq!(run_with_args(&["--chapter".to_string()]), ExitCode::from(1));
        assert!(execute_args(&["--chapter".to_string(), "abc".to_string()]).is_err());
        assert!(execute_args(&["--chapter".to_string(), "99".to_string()]).is_err());
        assert_eq!(run_with_args(&["--unknown".to_string()]), ExitCode::from(1));
    }
}
