#!/usr/bin/env bash
set -euo pipefail

echo "========================================================================"
echo "  STOCHOLOGY — FULL WORKSPACE VERIFICATION PIPELINE"
echo "========================================================================"

echo ">>> [1/6] Checking Rust formatting (cargo fmt --all -- --check)..."
cargo fmt --all -- --check

echo ">>> [2/6] Running Clippy lints (cargo clippy --all-targets -- -D warnings)..."
cargo clippy --all-targets -- -D warnings

echo ">>> [3/6] Running unit and integration tests (cargo test)..."
cargo test

echo ">>> [4/6] Verifying test coverage >= 90% (cargo llvm-cov)..."
cargo llvm-cov --workspace --summary-only --fail-under-lines 90

echo ">>> [5/6] Running all 9 progressive chapter laboratories & SHA-256 dataset verification..."
cargo run --release --bin stochology

echo ">>> [6/6] Verifying 1-to-1 textbook structure, sealed datasets, and symbol parity..."
if ! sha256sum -c --quiet data/SHA256SUMS; then
    echo "ERROR: data/SHA256SUMS verification failed!" >&2
    exit 1
fi

for ch in \
    "book-src/00-preface.md" \
    "book-src/01-sequential-loop-naive-agent.md" \
    "book-src/02-inquiry-openness-multiverse.md" \
    "book-src/03-underdetermination-partial-identification.md" \
    "book-src/04-non-identical-asymmetry-heavy-tails.md" \
    "book-src/05-world-to-archive-big-data-paradox.md" \
    "book-src/06-reactive-worlds-performativity-goodhart.md" \
    "book-src/07-irreversible-time-vital-margin.md" \
    "book-src/08-moral-luck-apt-agency-institutions.md" \
    "book-src/09-methodological-families-reflexivity.md" \
    "book-src/appendix-a-mathematics.md" \
    "book-src/appendix-b-architecture-verification.md" \
    "book-src/appendix-c-sources.md" \
    "book-src/lab/index.html"
do
    if [ ! -f "$ch" ]; then
        echo "ERROR: Missing expected textbook or lab file '$ch'!" >&2
        exit 1
    fi
done

if ! grep -q "mathjax.js" "book-src/lab/index.html"; then
    echo "ERROR: book-src/lab/index.html is missing ../mathjax.js inclusion!" >&2
    exit 1
fi

if grep -rnE "file://|/home/" book-src/*.md README.md CONTRIBUTING.md AGENTS.md >/dev/null 2>&1; then
    echo "ERROR: Found non-relative local path or file:// URI in documentation!" >&2
    grep -rnE "file://|/home/" book-src/*.md README.md CONTRIBUTING.md AGENTS.md >&2
    exit 1
fi

for sym in \
    CommunityReserveEnv NaiveOptimizer StochologicalAgent \
    CostLossFraming SeverePowerAnalysis SpecificationCurve \
    EvidenceProcess ManskiBounds LeeAttritionBounds PetersonSurvivalBounds \
    DialecticalPenumbra OrdinalScaleAudit NonlinearSummary HillTailEstimate \
    KeynesEvidentialWeight MengBigDataDefect ClusterDesignEffect KishWeightAudit CaptureRecaptureAudit VintageRevisionAudit \
    CorpPavDecomposition AnchoredDiscrepancy TianPearlCounterfactualBounds PerformativeSystem GoodhartAudit \
    EpistemicKellyAllocation CanguilhemHollingBasin VitalSlackBuffer WaldSurvivorshipAudit InfoGapMaximinSafeguard \
    SosaJudgmentGrade TelicEvaluation DiscursiveCommittee AlgedonicChannel ContractAndLiabilityAudit ChouldechovaTrilemma CognitiveAlertFilter TemporalVault \
    CollapseProposition TriadicIntegrity ProcessTracingInference ReflexiveBrakeAudit
do
    if ! grep -q "$sym" "book-src/appendix-b-architecture-verification.md"; then
        echo "ERROR: Symbol '$sym' missing from book-src/appendix-b-architecture-verification.md!" >&2
        exit 1
    fi
done

echo "========================================================================"
echo "  ALL STOCHOLOGY CHECKS PASSED (0 WARNINGS, 14 SEALED DATASETS, 9 LABS)"
echo "========================================================================"
