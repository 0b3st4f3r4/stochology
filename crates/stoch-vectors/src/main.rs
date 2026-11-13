//! `stoch-vectors` — run the harness from a shell.
//!
//! Six subcommands, each the same instrument a test uses, so a divergence can be
//! reproduced outside the test runner:
//!
//! ```text
//! stoch-vectors check [--vectors PATH]   run the fixture, print every mismatch
//! stoch-vectors gen                      print the coverage corpus
//! stoch-vectors census                   run the coverage corpus and its census
//! stoch-vectors topology                 reconcile Table B.1 and print the report
//! stoch-vectors concordance              sweep §V.9
//! stoch-vectors operators                run the three canonical operators
//! ```
//!
//! Exit status is `0` when the instrument agrees and `1` when it does not: a
//! corrupted vector is a failing command, not a warning.

use std::fs;
use std::process::ExitCode;

use stoch_vectors::contract;
use stoch_vectors::fixture;
use stoch_vectors::generator;
use stoch_vectors::operators::{epistemic, governance, vital, UNIT};
use stoch_vectors::schema;
use stoch_vectors::topology;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("check");
    match command {
        "check" => check(&args[1..]),
        "gen" => {
            print!("{}", generator::corpus());
            ExitCode::SUCCESS
        }
        "census" => census(),
        "topology" => topology_report(),
        "concordance" => concordance(),
        "operators" => operators(),
        "help" | "--help" | "-h" => {
            println!(
                "usage: stoch-vectors [check [--vectors PATH] | gen | census | topology | concordance | operators]"
            );
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("unknown subcommand {other:?}; try `stoch-vectors help`");
            ExitCode::from(2)
        }
    }
}

fn check(args: &[String]) -> ExitCode {
    let path = match args {
        [] => stoch_vectors::fixture_path(),
        [flag, value] if flag == "--vectors" => std::path::PathBuf::from(value),
        [flag] if flag == "--vectors" => {
            eprintln!("--vectors needs a path");
            return ExitCode::from(2);
        }
        _ => {
            eprintln!("usage: stoch-vectors check [--vectors PATH]");
            return ExitCode::from(2);
        }
    };
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("cannot read {}: {error}", path.display());
            return ExitCode::from(2);
        }
    };
    let report = match fixture::run(&text) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{path:?}: {error}");
            return ExitCode::FAILURE;
        }
    };
    println!(
        "{} rows: step {}, accept {}, diagnose {}, seal {}",
        report.rows(),
        report.functions[0],
        report.functions[1],
        report.functions[2],
        report.functions[3]
    );
    let mismatches = report.mismatches();
    for mismatch in &mismatches {
        println!("MISMATCH {}", mismatch.id);
        println!("  declared: {}", mismatch.expected);
        println!("  returned: {}", mismatch.actual);
    }
    if mismatches.is_empty() {
        println!("all rows agree with their declaration");
        ExitCode::SUCCESS
    } else {
        println!("{} of {} rows diverge", mismatches.len(), report.rows());
        ExitCode::FAILURE
    }
}

fn census() -> ExitCode {
    let rows = match schema::parse(&generator::corpus()) {
        Ok(rows) => rows,
        Err(error) => {
            eprintln!("the generated corpus does not parse: {error}");
            return ExitCode::FAILURE;
        }
    };
    let report = fixture::run_rows(&rows);
    let mismatches = report.mismatches();
    for mismatch in &mismatches {
        println!("MISMATCH {}", mismatch.id);
        println!("  declared: {}", mismatch.expected);
        println!("  returned: {}", mismatch.actual);
    }
    let census = generator::census(&rows);
    println!(
        "{} generated rows, {} diverging",
        census.rows,
        mismatches.len()
    );
    let missing = census.missing();
    if missing.is_empty() && mismatches.is_empty() {
        println!("the corpus reaches every §6.3 item");
        ExitCode::SUCCESS
    } else {
        for item in missing {
            println!("MISSING {item}");
        }
        ExitCode::FAILURE
    }
}

fn topology_report() -> ExitCode {
    let table = match fs::read_to_string(stoch_vectors::appendix_b_path()) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("cannot read Appendix B: {error}");
            return ExitCode::FAILURE;
        }
    };
    let preface = match fs::read_to_string(stoch_vectors::preface_path()) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("cannot read the preface: {error}");
            return ExitCode::FAILURE;
        }
    };
    match topology::reconcile(&table, &preface) {
        Ok(parity) => {
            print!("{}", topology::render(&parity));
            ExitCode::SUCCESS
        }
        Err(errors) => {
            for error in errors {
                println!("DIVERGENCE {error}");
            }
            ExitCode::FAILURE
        }
    }
}

fn concordance() -> ExitCode {
    let root = stoch_vectors::workspace_root();
    match contract::sweep(&root) {
        Ok(sweep) => {
            println!(
                "{} propositions, {} files under book-src/",
                sweep.rows.len(),
                sweep.book_files
            );
            for (proposition, sites) in &sweep.canonical_sites {
                println!("  {proposition}: {} site(s)", sites.len());
            }
            for error in &sweep.errors {
                println!("DIVERGENCE {error}");
            }
            if sweep.passed() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            println!("DIVERGENCE {error}");
            ExitCode::FAILURE
        }
    }
}

/// The three operators on one declared example each. Nothing here feeds a step:
/// the outputs are margins and durations, printed for inspection.
fn operators() -> ExitCode {
    let design = [
        epistemic::compose_design_effects(&[2 * UNIT, 3 * UNIT])
            .expect("composes"),
        epistemic::design_effect_product_bound(&[2 * UNIT, 3 * UNIT])
            .expect("bounded"),
    ];
    println!(
        "I  design effects: additive={} product-bound={}",
        design[0], design[1]
    );
    let parameter = epistemic::Nonresponse::new(UNIT / 2, 1000, UNIT / 2)
        .expect("declared parameter");
    println!(
        "I  non-response: defect={} bias={} corrected={}",
        epistemic::data_defect_index(parameter).expect("defect"),
        epistemic::meng_bias(parameter).expect("bias"),
        epistemic::corrected_level(700_000, parameter).expect("corrected")
    );

    let dynamics = vital::ReserveDynamics::new(120, 100, 0).expect("declared dynamics");
    let crossing = vital::recovery_time(dynamics, 0, 100, 10).expect("searched");
    println!(
        "II drag={} time-growth={} ensemble-growth={} crossing={:?}",
        vital::volatility_drag(2 * UNIT).expect("drag"),
        vital::time_growth(500_000, 2 * UNIT).expect("time"),
        vital::ensemble_growth(500_000),
        crossing
    );
    println!(
        "II outcome={:?} survives={}",
        vital::survival_outcome(5, crossing.map(i64::from)),
        vital::survival_condition(5, i64::from(crossing.expect("crossing")))
    );

    let readings = governance::Bitemporal::new(900_000, 400_000, 700_000).expect("readings");
    let integrity = governance::Integrity::new(10, 8).expect("gate");
    let below = governance::differential_sign(UNIT / 2 - 1, UNIT / 2).expect("sign");
    let tie = governance::differential_sign(UNIT / 2, UNIT / 2).expect("sign");
    let above = governance::differential_sign(UNIT / 2 + 1, UNIT / 2).expect("sign");
    println!(
        "III responsibility={} ratio={} gate={} sign(below)={:?} factor={} sign(tie)={:?} factor={} sign(above)={:?} factor={}",
        readings.responsibility(),
        integrity.ratio().expect("ratio"),
        integrity.gate(800_000).expect("gate"),
        below,
        below.factor(),
        tie,
        tie.factor(),
        above,
        above.factor()
    );
    ExitCode::SUCCESS
}
