#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo ">>> [1/6] Checking formatting (cargo fmt)..."
cargo fmt --all --check

echo ">>> [2/6] Checking strict lints (cargo clippy -D warnings)..."
cargo clippy --workspace --all-targets -- -D warnings

echo ">>> [3/6] Running unit and integration tests (cargo test)..."
cargo test --workspace

echo ">>> [4/6] Running methodological and empirical laboratories (labs)..."
cargo run --release --bin labs -- --all
cargo run --release --bin labs -- --layers
cargo run --release --bin labs -- --uncertainty
cargo run --release --bin labs -- --lenses
cargo run --release --bin labs -- --invariants
cargo run --release --bin labs -- --siga
cargo run --release --bin labs -- --governance
cargo run --release --bin labs -- --real-data

echo ">>> [5/6] Running epistemological gates (gates) and negative proofs..."
cargo run --release --bin gates

for defect in cromwell phi5 moulton performativity sniedovich clock vault concept_drift chouldechova machine analog_wall instrumental_reason stochary_cybernetics peeking brier_dominance var_underreport lee_point_estimate replace_collapse; do
    set +e
    cargo run --release --bin gates -- --defect "$defect" >/dev/null 2>&1
    rc=$?
    set -e
    if [ "$rc" -ne 1 ]; then
        echo "ERROR: negative proof '$defect' should have exited with code 1, but exited with $rc!" >&2
        exit 1
    else
        echo "  [OK] Negative proof '$defect' blocked with exit code 1"
    fi
done

echo ">>> [6/6] Verifying doc-to-code symbol parity, CAPIBA Protocol contract, and absence of git tags..."
for sym in SplitMix64 DpiOperationalStage MengAudit data_defect_correlation_ddc data_defect_index_ddi law_of_large_populations_z ManskiBounds lee_trimming_bounds lee_tipping_point_delta PerformativeSystem perfgd_trajectory ChouldechovaAudit disparate_impact_fpr_ratio minmax_penalty_disparity CorpPavDecomposition compute_logarithmic MurphyDiagram CorpRocMurphyTriptych SaviConfidenceSequence gro_betting VarEsEBacktest GerstgrasserAccumulateAudit SyntheticDataPolicy AlertFatigueAudit PredeterminedChangeControlPlan ProfitLossAttributionTest EvaluatedCompetenceCriterion; do
    if ! grep -q "$sym" "docs/PROTOCOL.md"; then
        echo "ERROR: docs/PROTOCOL.md omits canonical symbol '$sym'!" >&2
        exit 1
    fi
    if ! grep -rq "$sym" crates/; then
        echo "ERROR: Crate codebase omits symbol '$sym' cited in docs/PROTOCOL.md!" >&2
        exit 1
    fi
done
echo "  [OK] docs/PROTOCOL.md <-> Rust codebase symbol parity verified (27 canonical symbols)"
for ch in $(grep -oE '[0-9]{2}-[a-z0-9-]+\.md' "book-src/SUMMARY.md" | sort -u); do
    if [ ! -f "book-src/$ch" ]; then
        echo "ERROR: book-src/SUMMARY.md cites non-existent book-src chapter '$ch'!" >&2
        exit 1
    fi
done
echo "  [OK] book-src/SUMMARY.md chapter references verified against book-src/*.md"

# CAPIBA Protocol Contract Gate (6 mnemonic pillars -> gates & 18 disjoint negative defect proofs)
CAPIBA_EN="Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol"
for doc in "README.md" "AGENTS.md" "docs/PROTOCOL.md" "book-src/00-preface.md"; do
    if ! grep -q "$CAPIBA_EN" "$doc"; then
        echo "ERROR: '$doc' omits canonical CAPIBA Protocol expansion ('$CAPIBA_EN')!" >&2
        exit 1
    fi
done
if ! grep -q "Lourenço da Fonseca Barbosa" "book-src/00-preface.md"; then
    echo "ERROR: 'book-src/00-preface.md' omits the dedication to Lourenço da Fonseca Barbosa (Capiba)!" >&2
    exit 1
fi
for norm_doc in "AGENTS.md" "docs/PROTOCOL.md"; do
    if grep -qiE 'sombrinha|fervura' "$norm_doc"; then
        echo "ERROR: Normative document '$norm_doc' contains untested metaphorical vocabulary (belongs only in book-src/00-preface.md)!" >&2
        exit 1
    fi
done
CAPIBA_C_DEFECTS="cromwell brier_dominance concept_drift"
CAPIBA_A1_DEFECTS="moulton peeking var_underreport phi5"
CAPIBA_P_DEFECTS="performativity replace_collapse"
CAPIBA_I_DEFECTS="lee_point_estimate sniedovich machine"
CAPIBA_B_DEFECTS="clock vault"
CAPIBA_A2_DEFECTS="chouldechova analog_wall instrumental_reason stochary_cybernetics"
CAPIBA_ALL_DEFECTS="$CAPIBA_C_DEFECTS $CAPIBA_A1_DEFECTS $CAPIBA_P_DEFECTS $CAPIBA_I_DEFECTS $CAPIBA_B_DEFECTS $CAPIBA_A2_DEFECTS"
capiba_count="$(echo "$CAPIBA_ALL_DEFECTS" | tr ' ' '\n' | sort -u | wc -l | tr -d ' ')"
if [ "$capiba_count" -ne 18 ]; then
    echo "ERROR: CAPIBA pillar defect partition must cover exactly 18 unique defects, found $capiba_count!" >&2
    exit 1
fi
for pillar_gate in "G6" "G2/G3/G6" "G6/G8" "G3/G6/G8" "G7" "G4/G8"; do
    for doc in "README.md" "AGENTS.md" "docs/PROTOCOL.md"; do
        if ! grep -q "\`$pillar_gate\`" "$doc"; then
            echo "ERROR: '$doc' omits CAPIBA pillar gate mapping '$pillar_gate'!" >&2
            exit 1
        fi
    done
done
echo "  [OK] CAPIBA Protocol 6-pillar contract verified (gates pinned + 3+4+2+3+2+4 = 18 negative proofs + preface dedication hygiene)"

if [ -n "$(git tag --list)" ]; then
    echo "ERROR: git tags found; this repository does not use git tags." >&2
    exit 1
fi

echo ">>> FULL VERIFICATION APPROVED (STOCHOLOGY PROTOCOL) <<<"
