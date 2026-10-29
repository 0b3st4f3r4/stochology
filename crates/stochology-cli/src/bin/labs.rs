//! Binary `labs` — Executes the 7 methodological and empirical laboratories of the Stochology Protocol
//! over all 13 real datasets (`data/*.csv`) and the 4 canonical Epistemological Cards (`cards/*.yaml`).

mod labs_mod;

use crate::labs_mod::ALL_LABS;
use std::env;
use std::path::Path;

fn print_usage() {
    println!(
        "Usage: labs [--help | --all | --layers | --uncertainty | --lenses | --invariants | --siga | --governance | --real-data]"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        eprintln!("Too many arguments: {args:?}");
        print_usage();
        std::process::exit(2);
    }
    let mode = args.first().map(|s| s.as_str()).unwrap_or("--all");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");

    if mode == "--help" || mode == "-h" {
        print_usage();
        return;
    }
    if mode == "--all" {
        for lab in &ALL_LABS {
            (lab.run)(root);
        }
        return;
    }
    if let Some(lab) = ALL_LABS.iter().find(|l| l.cli_flag == mode) {
        (lab.run)(root);
    } else {
        eprintln!("Unknown option: '{mode}'.");
        print_usage();
        std::process::exit(2);
    }
}
