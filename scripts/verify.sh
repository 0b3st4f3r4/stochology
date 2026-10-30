#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo ">>> [1/6] Checking formatting (cargo fmt)..."
cargo fmt --all --check

echo ">>> [2/6] Checking strict lints (cargo clippy -D warnings) and rustdoc..."
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

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
for card in cards/card_0*.yaml; do
    cargo run --release --bin gates -- --audit-card "$card" >/dev/null
    echo "  [OK] Portable external card audit (--audit-card $card) passed"
done

for defect in cromwell phi5 moulton performativity sniedovich clock vault concept_drift chouldechova machine analog_wall instrumental_reason stochary_cybernetics peeking brier_dominance var_underreport lee_point_estimate replace_collapse triadic_technium strategic_accommodation coupled_society; do
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

echo ">>> [6/6] Verifying doc-to-code symbol parity, CAPIBA Protocol contract, and release tag hygiene..."
for sym in SplitMix64 DpiOperationalStage MengAudit data_defect_correlation_ddc data_defect_index_ddi law_of_large_populations_z ManskiBounds lee_trimming_bounds lee_tipping_point_delta PerformativeSystem perfgd_trajectory ChouldechovaAudit disparate_impact_fpr_ratio minmax_penalty_disparity CorpPavDecomposition compute_logarithmic MurphyDiagram CorpRocMurphyTriptych SaviConfidenceSequence gro_betting VarEsEBacktest GerstgrasserAccumulateAudit SyntheticDataPolicy AlertFatigueAudit PredeterminedChangeControlPlan ProfitLossAttributionTest EvaluatedCompetenceCriterion ComplexFractalToroidalTechniumSynthesis EmpiricalTriadicTechniumAudit RubinMultipleImputationAudit ShapleyCausalAudit SaatyAhpRankReversalAudit LlmJudgePpiRectifier RobustKalmanControlAudit RegulatoryComplianceAudit EpistemicPkmProvenanceAudit StrategicAiAccommodationSynthesis EmpiricalStrategicAccommodationAudit prospective_ppi_gold_sample_size ppi_effective_sample_size_multiplier SimpsonCausalParadoxAudit LongContextDpiDilutionAudit MultimodalCorrelatedFusionAudit PreMortemCounterfactualRootCauseAudit CloudLocalLoraArchitectureAudit CoupledPublicPrivateSocietySynthesis EmpiricalCoupledSocietyAudit SfcFlowMatrix H1aRegulatorCouplingAblation H1bCreditScreeningAblation H2VcgGreenLaffontCommonsAblation; do
    if ! grep -q "$sym" "docs/PROTOCOL.md"; then
        echo "ERROR: docs/PROTOCOL.md omits canonical symbol '$sym'!" >&2
        exit 1
    fi
    if ! grep -rq --include='*.rs' "\b$sym\b" crates/; then
        echo "ERROR: Crate codebase omits symbol '$sym' cited in docs/PROTOCOL.md!" >&2
        exit 1
    fi
done
echo "  [OK] docs/PROTOCOL.md <-> Rust codebase symbol parity verified (51 canonical symbols)"
for ch in $(grep -oE '\]\([a-z0-9-]+\.md\)' "book-src/SUMMARY.md" | sed -E 's/^\]\((.*)\)$/\1/' | sort -u); do
    if [ ! -f "book-src/$ch" ]; then
        echo "ERROR: book-src/SUMMARY.md cites non-existent book-src chapter '$ch'!" >&2
        exit 1
    fi
done
echo "  [OK] book-src/SUMMARY.md chapter and appendix references verified against book-src/*.md"
# Three-tempo chapter template (docs/SYLLABUS.md): every numbered chapter opens with Detachment, Abstraction, Return, and its negative proof.
for ch in book-src/[0-9][0-9]-*.md; do
    case "$ch" in book-src/00-preface.md) continue ;; esac
    if ! sed -n '1,5p' "$ch" | grep -q '^> \*\*Three tempos\.\*\* \*\*Detachment:\*\*.*\*\*Abstraction:\*\*.*\*\*Return:\*\*.*\*\*Negative proof:\*\*'; then
        echo "ERROR: '$ch' does not open with the three-tempo block (Detachment, Abstraction, Return, Negative proof)!" >&2
        exit 1
    fi
done
for inc in $(grep -ohE '\{\{#include [^}:]+' book-src/*.md | sed -E 's/^\{\{#include //'); do
    if [ ! -f "book-src/$inc" ]; then
        echo "ERROR: mdBook include target 'book-src/$inc' does not exist!" >&2
        exit 1
    fi
done
echo "  [OK] Three-tempo chapter template present in every numbered chapter; mdBook includes resolve"

# Overclaim phrase blacklist (AGENTS.md II.C): audited overclaims must not return to derived texts.
# AGENTS.md and .agents/ are excluded on purpose: they quote the rules. Patterns tolerate LaTeX ($20\text{ W}$, 100\times$).
BLACKLIST_DOCS=(book-src/*.md docs/PROTOCOL.md docs/SYLLABUS.md docs/syntheses/*.md README.md crates/*/README.md)
BLACKLIST_PATTERNS=(
    'retinas?[[:space:]]+transduc'
    '20[- ]*(\\text\{[[:space:]]*)?W\}?\$?[- ]+human[[:space:]]+retina'
    '(^|[^[:alpha:]])real[- ]+interventions?'
    'toxic[[:space:]]+noise'
    'only[[:space:]]+antidote'
    '100[[:space:]]*(\\times|×|x)\$?[[:space:]]*(vs\.?|versus)(\\[[:space:]]|~|[[:space:]])*(an?[[:space:]]+)?(Nvidia[[:space:]]+)?H100'
)
blacklist_failed=0
for pat in "${BLACKLIST_PATTERNS[@]}"; do
    if hits="$(grep -niE -- "$pat" "${BLACKLIST_DOCS[@]}")"; then
        echo "ERROR: blacklisted overclaim phrase /$pat/ found:" >&2
        echo "$hits" | cut -c1-240 >&2
        blacklist_failed=1
    fi
done
if [ "$blacklist_failed" -ne 0 ]; then
    exit 1
fi
echo "  [OK] Overclaim phrase blacklist clean across book-src/*.md, docs/PROTOCOL.md, docs/SYLLABUS.md, docs/syntheses/*.md, and README.md (${#BLACKLIST_PATTERNS[@]} patterns)"

# Cross-reference integrity: SUMMARY.md order, "Chapter N" bounds, and relative chapter links.
summary_prefixes="$(grep -oE '\]\([0-9]{2}-[a-z0-9-]+\.md\)' book-src/SUMMARY.md | sed -E 's/^\]\(([0-9]{2}).*/\1/')"
prev_prefix=-1
for prefix in $summary_prefixes; do
    if [ "$((10#$prefix))" -le "$prev_prefix" ]; then
        echo "ERROR: book-src/SUMMARY.md chapter prefixes are not strictly increasing ($prev_prefix then $prefix)!" >&2
        exit 1
    fi
    prev_prefix="$((10#$prefix))"
done
max_chapter="$prev_prefix"
xref_failed=0
while IFS= read -r hit; do
    for n in $(printf '%s\n' "${hit#*:*:}" | grep -oE '[0-9]+'); do
        if [ "$((10#$n))" -gt "$max_chapter" ]; then
            echo "ERROR: '$hit' cites chapter $n, beyond the last chapter ($max_chapter) in book-src/SUMMARY.md!" >&2
            xref_failed=1
        fi
    done
done < <(grep -noE 'Chapters? [0-9]+((–|-|, | and | & |, and )[0-9]+)*' book-src/*.md)
while IFS= read -r doc; do
    doc_dir="$(dirname "$doc")"
    while IFS= read -r target; do
        case "$target" in *://*) continue ;; esac
        if [ ! -f "$doc_dir/$target" ]; then
            echo "ERROR: '$doc' links to non-existent chapter '$target'!" >&2
            xref_failed=1
        fi
    done < <(grep -oE '\]\([^)#[:space:]]*[0-9]{2}-[A-Za-z0-9_-]+\.md' "$doc" | sed -E 's/^\]\(//')
    while IFS= read -r rel_target; do
        if [ ! -e "$doc_dir/$rel_target" ]; then
            echo "ERROR: '$doc' links to non-existent workspace path '$rel_target'!" >&2
            xref_failed=1
        fi
    done < <(grep -oE '\]\(\.\./(crates|data|cards|docs)/[^)#[:space:]]+' "$doc" | sed -E 's/^\]\(//')
done < <(find book-src docs -name '*.md' | sort; echo README.md)
if [ "$xref_failed" -ne 0 ]; then
    exit 1
fi
echo "  [OK] Cross-references verified (SUMMARY.md prefixes strictly increasing, 'Chapter N' <= $max_chapter, chapter & workspace links resolve)"

# Epistemic ledger parity: book-src/appendix-c-sources.md must match data/SHA256SUMS and data/preregistration/acceptance_registry.txt
reg_prefix="$(awk '$2 == "data/preregistration/acceptance_registry.txt" {print substr($1, 1, 12)}' data/SHA256SUMS)"
soc_prefix="$(awk '$2 == "data/preregistration/coupled_society_preregistration.yaml" {print substr($1, 1, 12)}' data/SHA256SUMS)"
reg_k="$(sed -nE 's/.*Acceptance Registry entry K = ([0-9]+).*/\1/p' data/preregistration/coupled_society_preregistration.yaml)"
reg_sources="$(grep -c '^source ' data/preregistration/acceptance_registry.txt)"
reg_gated="$(grep -c '^gated ' data/preregistration/acceptance_registry.txt)"
reg_discarded="$(grep -c '^discarded ' data/preregistration/acceptance_registry.txt)"
for token in "${reg_prefix}..." "${soc_prefix}..." "K = ${reg_k}" "${reg_sources} claim-defining" "${reg_gated} approved" "${reg_discarded} synthetic"; do
    if ! grep -Fq "$token" "book-src/appendix-c-sources.md"; then
        echo "ERROR: 'book-src/appendix-c-sources.md' is out of sync with data/preregistration/acceptance_registry.txt (missing '$token')!" >&2
        exit 1
    fi
done
echo "  [OK] book-src/appendix-c-sources.md synchronized with acceptance_registry.txt (K=$reg_k, sources=$reg_sources, gated=$reg_gated, discarded=$reg_discarded)"

# CAPIBA Protocol Contract Gate (6 mnemonic pillars -> gates & 21 disjoint negative defect proofs)
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
CAPIBA_A2_DEFECTS="chouldechova analog_wall instrumental_reason stochary_cybernetics triadic_technium strategic_accommodation coupled_society"
CAPIBA_ALL_DEFECTS="$CAPIBA_C_DEFECTS $CAPIBA_A1_DEFECTS $CAPIBA_P_DEFECTS $CAPIBA_I_DEFECTS $CAPIBA_B_DEFECTS $CAPIBA_A2_DEFECTS"
capiba_count="$(echo "$CAPIBA_ALL_DEFECTS" | tr ' ' '\n' | sort -u | wc -l | tr -d ' ')"
if [ "$capiba_count" -ne 21 ]; then
    echo "ERROR: CAPIBA pillar defect partition must cover exactly 21 unique defects, found $capiba_count!" >&2
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
echo "  [OK] CAPIBA Protocol 6-pillar contract verified (gates pinned + 3+4+2+3+2+7 = 21 negative proofs + preface dedication hygiene)"

# Phi3 seal integrity: every manifest entry matches its file, and dataset hashes are unchanged since HEAD
if ! sha256sum -c --quiet data/SHA256SUMS; then
    echo "ERROR: data/SHA256SUMS does not match the files it seals!" >&2
    exit 1
fi
if git rev-parse --verify -q HEAD >/dev/null; then
    head_csv="$(git show HEAD:data/SHA256SUMS | grep '\.csv$' || true)"
    work_csv="$(grep '\.csv$' data/SHA256SUMS || true)"
    if [ "$head_csv" != "$work_csv" ]; then
        echo "ERROR: dataset hashes in data/SHA256SUMS differ from HEAD; datasets are immutable!" >&2
        exit 1
    fi
    head_reg="$(git show HEAD:data/SHA256SUMS | grep 'acceptance_registry' || true)"
    work_reg="$(grep 'acceptance_registry' data/SHA256SUMS || true)"
    if [ "$head_reg" != "$work_reg" ]; then
        echo "  [NOTICE] acceptance registry re-sealed since HEAD: requires human inline-diff review before commit (AGENTS.md V.3)"
    fi
fi
echo "  [OK] data/SHA256SUMS seals every listed file and dataset hashes match HEAD"

workspace_ver="$(sed -nE 's/^version = "([^"]+)"/\1/p' Cargo.toml | head -n 1)"
while IFS= read -r dep_ver; do
    [ -z "$dep_ver" ] && continue
    if [ "$dep_ver" != "$workspace_ver" ]; then
        echo "ERROR: [workspace.dependencies] stochology-* version '$dep_ver' does not match workspace version '$workspace_ver'!" >&2
        exit 1
    fi
done < <(sed -nE 's/^stochology-[a-z-]+ = \{ version = "([^"]+)".*/\1/p' Cargo.toml)
while IFS= read -r tag; do
    [ -z "$tag" ] && continue
    if ! printf '%s\n' "$tag" | grep -qE '^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$'; then
        echo "ERROR: non-SemVer git tag '$tag' found (expected vX.Y.Z)!" >&2
        exit 1
    fi
done < <(git tag --list)
if [ "${GITHUB_REF_TYPE:-}" = "tag" ] && [ "${GITHUB_REF_NAME:-}" != "v${workspace_ver}" ]; then
    echo "ERROR: tag '${GITHUB_REF_NAME:-}' does not match Cargo.toml workspace version 'v${workspace_ver}'!" >&2
    exit 1
fi
echo "  [OK] Release tag hygiene verified (workspace version v${workspace_ver})"

echo ">>> FULL VERIFICATION APPROVED (STOCHOLOGY PROTOCOL) <<<"
