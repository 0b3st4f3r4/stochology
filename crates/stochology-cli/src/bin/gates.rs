//! Binary `gates` — Enforces the 8 Epistemological Gates (G1 to G8) over the YAML Cards (`cards/*.yaml`)
//! and their anchored CSV datasets (`data/*.csv`), and executes negative defect proofs (`--defect`)
//! that must exit with code `1`.

mod gates_mod;

use std::env;
use std::path::Path;
use stochology_cli::all_defect_names;
use stochology_lenses::acceptance::{canonical_registry_lines, evaluate_all_syntheses};

use crate::gates_mod::card_audit::run_audit_card;
use crate::gates_mod::defects::run_negative_proof;
use crate::gates_mod::positive::run_positive_gates;

fn print_usage() {
    let defects_csv = all_defect_names().join(", ");
    println!(
        "Usage: gates [--help | --defect <defect_name> | --audit-card <card.yaml> | --print-acceptance-registry]\n\
         Defects: {defects_csv}"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");

    match args.as_slice() {
        [] => {}
        [flag] if flag == "--help" || flag == "-h" => {
            print_usage();
            return;
        }
        [flag] if flag == "--print-acceptance-registry" => {
            match canonical_registry_lines(root, &evaluate_all_syntheses()) {
                Ok(lines) => lines.iter().for_each(|l| println!("{l}")),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            }
            return;
        }
        [flag] if flag == "--audit-card" => {
            eprintln!("Missing card path after '--audit-card'.");
            print_usage();
            std::process::exit(2);
        }
        [flag, card_path] if flag == "--audit-card" => {
            if let Err(msg) = run_audit_card(root, card_path) {
                eprintln!("CARD AUDIT FAILED: {msg}");
                std::process::exit(1);
            }
            return;
        }
        [flag] if flag == "--defect" => {
            eprintln!("Missing defect name after '--defect'.");
            print_usage();
            std::process::exit(2);
        }
        [flag, defect] if flag == "--defect" => {
            run_negative_proof(root, defect);
        }
        other => {
            eprintln!("Unknown arguments: {other:?}");
            print_usage();
            std::process::exit(2);
        }
    }

    match run_positive_gates(root) {
        Ok(()) => {
            println!("=== ALL 8 EPISTEMOLOGICAL GATES APPROVED (EXIT CODE 0) ===");
        }
        Err(msg) => {
            eprintln!("EPISTEMOLOGICAL GATES FAILED: {msg}");
            std::process::exit(1);
        }
    }
}
