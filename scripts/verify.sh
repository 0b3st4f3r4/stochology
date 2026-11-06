#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo "========================================================================"
echo "  STOCHOLOGY — FULL WORKSPACE VERIFICATION PIPELINE"
echo "========================================================================"

echo ">>> [1/6] Checking Rust formatting (cargo fmt --all -- --check)..."
cargo fmt --all -- --check

echo ">>> [2/6] Running Clippy lints (cargo clippy --all-targets -- -D warnings)..."
cargo clippy --all-targets -- -D warnings

echo ">>> [3/6] Running unit and integration tests (cargo test)..."
cargo test

echo ">>> [4/6] Verifying test coverage >= 90% across lines, regions, and functions (cargo llvm-cov)..."
cargo llvm-cov --workspace --summary-only --fail-under-lines 90 --fail-under-regions 90 --fail-under-functions 90

echo ">>> [5/6] Running all 9 progressive chapter laboratories & SHA-256 dataset verification..."
CLI_OUT="$(cargo run --release --bin stochology)"
if [ -z "$CLI_OUT" ]; then
    echo "ERROR: CLI_OUT is empty!" >&2
    exit 1
fi
printf '%s\n' "$CLI_OUT"

echo ">>> [6/6] Verifying 1-to-1 textbook structure, sealed datasets, lab parity, symbol parity, and bidirectional provenance sync..."
if ! sha256sum -c --quiet data/SHA256SUMS; then
    echo "ERROR: data/SHA256SUMS verification failed!" >&2
    exit 1
fi

while IFS= read -r line; do
    [ -z "$line" ] && continue
    if ! grep -Fq -- "$line" "README.md"; then
        echo "ERROR: Provenance drift in README.md for CLI output line: $line" >&2
        exit 1
    fi
    if ! grep -Fq -- "$line" "book-src/appendix-b-architecture-verification.md"; then
        echo "ERROR: Provenance drift in book-src/appendix-b-architecture-verification.md for CLI output line: $line" >&2
        exit 1
    fi
done <<< "$CLI_OUT"

python3 - "$CLI_OUT" << 'PYBIDIR'
import glob, re, sys

cli_lines = set(l.rstrip() for l in sys.argv[1].splitlines() if l.strip())
assert len(cli_lines) >= 40, f"Unexpectedly few CLI lines: {len(cli_lines)}"

# Reverse guard (doc -> CLI): every line inside any ```text block containing '=== CHAPTER '
# in README.md, appendix-b, and every framework chapter (the nine that declare the banner
# `> Laboratory N of the `stochology` CLI: `--chapter N`.`) must exist verbatim in CLI_OUT.
BANNER = re.compile(r"^> Laboratory (\d+) of the `stochology` CLI: `--chapter \1`\.$", re.M)
framework = [f for f in sorted(glob.glob("book-src/[0-9][0-9]-*.md"))
             if BANNER.search(open(f).read())]
assert len(framework) == 9, f"expected 9 CLI chapters, found {len(framework)}: {framework}"
docs = ["README.md", "book-src/appendix-b-architecture-verification.md"] + framework
bad = 0
for doc in docs:
    content = open(doc).read()
    blocks = re.findall(r"```text\n(.*?)```", content, re.S)
    ch_blocks = [b for b in blocks if "=== CHAPTER " in b]
    assert ch_blocks, f"ERROR: No '=== CHAPTER ' CLI block found in {doc}"
    for b in ch_blocks:
        for raw_line in b.splitlines():
            line = raw_line.rstrip()
            if not line:
                continue
            if line not in cli_lines:
                print(f"ERROR: Reverse provenance drift in {doc} (doc line not emitted by CLI): {line}", file=sys.stderr)
                bad += 1
if bad:
    sys.exit(1)
PYBIDIR

for ch in \
    book-src/[0-9][0-9]-*.md \
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

for html_ch in \
    "book/45-world-to-archive-big-data-paradox.html" \
    "book/46-non-identical-asymmetry-heavy-tails.html" \
    "book/47-irreversible-time-vital-margin.html" \
    "book/48-reactive-worlds-performativity-goodhart.html"
do
    if ! grep -q "$html_ch" ".github/workflows/pages.yml"; then
        echo "ERROR: .github/workflows/pages.yml is missing check for '$html_ch'!" >&2
        exit 1
    fi
done

if ! grep -q "mathjax.js" "book-src/lab/index.html"; then
    echo "ERROR: book-src/lab/index.html is missing ../mathjax.js inclusion!" >&2
    exit 1
fi

LAB_JSON="$(cargo run --quiet --release --bin stochology -- --export-lab-json)"
python3 - "$LAB_JSON" << 'PYLAB'
import json, re, sys
expected = json.loads(sys.argv[1])
html = open("book-src/lab/index.html").read()
m = re.search(r"const LAB_TRAJECTORIES = (\{.*?\});\s*\n", html, re.S)
assert m, "ERROR: Missing const LAB_TRAJECTORIES in book-src/lab/index.html"
actual = json.loads(m.group(1))
assert actual == expected, "ERROR: book-src/lab/index.html LAB_TRAJECTORIES drifted from cargo run --bin stochology -- --export-lab-json"
assert actual["111"]["terminalReserve"] > actual["011"]["terminalReserve"], "FullTriad (111) must exceed NoEpistemicMargin (011) terminalReserve"
assert actual["111"]["meanHarm"] < actual["000"]["meanHarm"], "FullTriad (111) must reduce meanHarm vs Naive (000)"
assert actual["111"]["ruinStep"] is None, "FullTriad (111) must not suffer ruin"
for axis_key in ("sEpi", "sVital", "sGov"):
    vals = {round(actual[k][axis_key], 3) for k in actual}
    assert len(vals) == 8, f"ERROR: {axis_key} in LAB_TRAJECTORIES must have 8 distinct trajectory-driven values, got {len(vals)}: {vals}"

# MULTILAYER_NODES must agree with multilayer.rs on primary site, guard triple, signature, stratum and chapter.
ml = open("crates/stochology/src/multilayer.rs").read()
def ml_section(name):
    i = ml.index("pub const fn " + name + "(")
    return ml[i:ml.index("\n    }\n", i)]
prim = {m.group(1): m.group(2) + m.group(3)
        for m in re.finditer(r"Self::(L\d\d)\w*\s*=>\s*GuardSite::(T\d)(\w+),", ml_section("primary_guard_site"))}
triple = {}
for blk in re.split(r"(?=Self::L\d\d)", ml_section("guard_sites")):
    nm = re.match(r"Self::(L\d\d)", blk)
    sites = re.findall(r"GuardSite::(T\d)", blk)
    if nm and sites:
        triple[nm.group(1)] = sites[:3]
axis = {**{f"T{i}": "E" for i in (1, 2, 3)}, **{f"T{i}": "V" for i in (4, 5)}, **{f"T{i}": "G" for i in (6, 7)}}
nodes = re.findall(
    r'\{id:"(L\d\d)".*?ch:(\d+).*?stratum:"(S\d)".*?primary:"(T\d\w+) \(([\d,]+)\)", triple:"([^"]+)"',
    html,
)
assert len(nodes) == 35, f"ERROR: expected 35 MULTILAYER_NODES, found {len(nodes)}"
for cid, ch, stratum, primary, sig, trip in nodes:
    order_sites = [t.strip()[:2] for t in trip.split(",")]
    if primary != prim[cid]:
        raise SystemExit(f"ERROR: book-src/lab/index.html {cid} primary={primary} but multilayer.rs says {prim[cid]}")
    if order_sites != triple[cid]:
        raise SystemExit(f"ERROR: book-src/lab/index.html {cid} triple={order_sites} but multilayer.rs says {triple[cid]}")
    want_sig = [sum(1 for s in order_sites if axis[s] == k) for k in "EVG"]
    if [int(x) for x in sig.split(",")] != want_sig:
        raise SystemExit(f"ERROR: book-src/lab/index.html {cid} signature={sig} but triple implies {want_sig}")
    if int(ch) != 50 + int(cid[1:]):
        raise SystemExit(f"ERROR: book-src/lab/index.html {cid} chapter={ch}, expected {50 + int(cid[1:])}")
PYLAB

echo ">>> [6b] Verifying the cross-domain audit report against book-src/appendix-b-architecture-verification.md..."
SA_REPORT="$(cargo run --quiet --release --bin stochology -- --audit-report)"
python3 - "$SA_REPORT" << 'PYSA'
import re, sys

report = [line.rstrip() for line in sys.argv[1].splitlines() if line.strip()]
assert report, "ERROR: --audit-report produced no output"
doc = open("book-src/appendix-b-architecture-verification.md").read()
match = re.search(r"```text\n(=== CROSS-DOMAIN AUDIT REPORT ===.*?)```", doc, re.S)
assert match, "ERROR: appendix B is missing the frozen cross-domain audit report block"
documented = [line.rstrip() for line in match.group(1).splitlines() if line.strip()]
missing = [line for line in documented if line not in report]
extra = [line for line in report if line not in documented]
assert not missing, f"ERROR: appendix B documents report lines the CLI no longer emits: {missing[:3]}"
assert not extra, f"ERROR: the CLI emits report lines appendix B does not document: {extra[:3]}"
PYSA

if ! grep -q "notes/The Ethics of Aim" ".agents/hooks.json"; then
    echo "ERROR: .agents/hooks.json is missing protection for notes/The Ethics of Aim.md!" >&2
    exit 1
fi

while IFS= read -r sum_line; do
    [ -z "$sum_line" ] && continue
    csv_path="${sum_line##* }"
    csv_file="$(basename "$csv_path")"
    if ! grep -Fq "$csv_file" "crates/stochology-cli/src/loaders.rs"; then
        echo "ERROR: Sealed dataset '$csv_file' is not loaded in crates/stochology-cli/src/loaders.rs!" >&2
        exit 1
    fi
done < "data/SHA256SUMS"

python3 - << 'PYSYM'
import glob, re, sys
rust_src = "\n".join(open(p).read() for p in sorted(glob.glob("crates/stochology/src/*.rs")) + sorted(glob.glob("crates/stochology-cli/src/*.rs")))
pub_types = set(re.findall(r"\bpub\s+(?:struct|enum|trait)\s+([A-Za-z0-9_]+)\b", rust_src))
pub_fns = set(re.findall(r"\bpub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z0-9_]+)\b", rust_src))
trait_fns = set(re.findall(r"^\s{4}fn\s+([A-Za-z0-9_]+)\b", rust_src, re.M))
pub_consts = set(re.findall(r"\bpub\s+const\s+([A-Za-z0-9_]+)\b", rust_src))
pub_fields = set(re.findall(r"\bpub\s+([a-z0-9_]+)\s*:", rust_src))
enum_variants = set(re.findall(r"^\s{4}([A-Z][A-Za-z0-9_]*)\b", rust_src, re.M))
valid_members = pub_fns | trait_fns | pub_consts | pub_fields | enum_variants | {"default", "clone", "eq", "fmt"}

bad = 0
docs = sorted(glob.glob("book-src/*.md")) + ["README.md", "AGENTS.md", "notes/The Ethics of Aim.md", "notes/The Grammar of Aim.md", "notes/What Must Be Known When We Do Not Know.md"]
for doc in docs:
    text = open(doc).read()
    for m in re.finditer(r"`([A-Z][A-Za-z0-9_]*)::([A-Za-z0-9_]+)(?:\([^`]*)?`", text):
        ty, member = m.group(1), m.group(2)
        if ty in pub_types and member not in valid_members:
            print(f"ERROR: Unresolved or phantom symbol `{ty}::{member}` in {doc}", file=sys.stderr)
            bad += 1
if bad:
    sys.exit(1)
PYSYM

if grep -rnE '\[Identity\]|\[Analytical ' book-src/*.md README.md AGENTS.md "notes/The Ethics of Aim.md" "notes/The Grammar of Aim.md" "notes/What Must Be Known When We Do Not Know.md" >/dev/null 2>&1; then
    echo "ERROR: Found non-canonical provenance tag ([Identity] or [Analytical ...]); only the 4 canonical tags ([Measured], [Simulated], [Assumed], [Literature]) are permitted!" >&2
    grep -rnE '\[Identity\]|\[Analytical ' book-src/*.md README.md AGENTS.md "notes/The Ethics of Aim.md" "notes/The Grammar of Aim.md" "notes/What Must Be Known When We Do Not Know.md" >&2
    exit 1
fi

for ch_prov in book-src/00-preface.md $(grep -lE '^> Laboratory [0-9]+ of the `stochology` CLI: `--chapter [0-9]+`\.$' book-src/[0-9][0-9]-*.md); do
    tag_count="$({ grep -oE '\[(Measured|Simulated|Assumed|Literature)' "$ch_prov" || true; } | wc -l)"
    if [ "$tag_count" -lt 10 ]; then
        echo "ERROR: Chapter '$ch_prov' has only $tag_count provenance tags (minimum 10 required)!" >&2
        exit 1
    fi
done

for ch_prov in book-src/appendix-[a-c]-*.md README.md; do
    if ! grep -qE '\[(Measured|Simulated|Assumed|Literature)' "$ch_prov"; then
        echo "ERROR: File '$ch_prov' is missing bracketed provenance tags ([Measured], [Simulated], [Assumed], [Literature])!" >&2
        exit 1
    fi
done

python3 -c '
import glob, re, sys
bad = 0
decl_pat = lambda s: re.compile(r"(?:\bpub(?:\([^)]*\))?\s+(?:const\s+)?(?:fn|struct|enum|trait|const|type)\s+|^\s{4}fn\s+)" + re.escape(s) + r"\b", re.M)
for md in sorted(glob.glob("book-src/*.md")) + ["README.md", "notes/The Ethics of Aim.md", "notes/The Grammar of Aim.md", "notes/What Must Be Known When We Do Not Know.md"]:
    text = open(md).read()
    for m in re.finditer(r"\[`([^`]+)`\]\((?:\.\./)?(crates/[^)#]+)#L(\d+)(?:-L(\d+))?\)", text):
        sym, path, l1, l2 = m.group(1), m.group(2), int(m.group(3)), int(m.group(4) or m.group(3))
        lines = open(path).read().splitlines()
        short = sym.split("::")[-1].split("(")[0].strip()
        window = "\n".join(lines[max(0, l1-3):min(len(lines), l2+2)])
        if not decl_pat(short).search(window):
            print(f"ERROR: Stale #L declaration anchor in {md}: [{sym}]({path}#L{l1}-L{l2})", file=sys.stderr)
            bad += 1
if bad:
    sys.exit(1)
'

python3 - "$CLI_OUT" << 'PYEOF'
import re, sys

out = sys.argv[1]

# 1. Verify LaLonde RCT ATE lies strictly inside Lee bounds and Manski bounds
m_ate = re.search(r"Experimental RCT ATE \[Measured, zero survey attrition\] = \$(\d+)", out)
m_lee_emp = re.search(r"Lee \(2009\) Positive-Wage Selection Bounds \[Measured, re78>0: 140/185 vs 168/260\]: \[\$(-?\d+), \$(-?\d+)\]", out)
m_manski = re.search(r"Manski Worst-Case Two-Arm ATE Bounds \[Assumed: 15/200 & 30/290 attrition\]: \[\$(-?\d+), \$(-?\d+)\]", out)
m_lee = re.search(r"Lee Monotone Attrition Two-Arm ATE Bounds \[Assumed: 15/200 & 30/290 attrition\]: \[\$(-?\d+), \$(-?\d+)\]", out)
assert m_ate and m_lee_emp and m_manski and m_lee, "Missing LaLonde bounds in CLI output"
ate = float(m_ate.group(1))
le_l, le_u = float(m_lee_emp.group(1)), float(m_lee_emp.group(2))
m_l, m_u = float(m_manski.group(1)), float(m_manski.group(2))
l_l, l_u = float(m_lee.group(1)), float(m_lee.group(2))
assert le_l < ate < le_u, f"Empirical Lee positive-wage bounds failed to bracket ATE: {le_l} < {ate} < {le_u}"
assert m_l < l_l < ate < l_u < m_u, f"LaLonde inclusion hierarchy violated: {m_l} < {l_l} < {ate} < {l_u} < {m_u}"

# 2. Verify CORP-PAV +6h storm forecast Brier identity BS = REL - RES + UNC and RES > REL
m_brier = re.search(r"Total Brier = ([0-9.]+) = REL \(([0-9.]+)\) - RES \(([0-9.]+)\) \+ UNC \(([0-9.]+)\) \[closure_err = ([0-9.e+-]+)\]", out)
assert m_brier, "Missing CORP-PAV Brier line"
bs, rel, res, unc, cerr = map(float, m_brier.groups())
assert abs(bs - (rel - res + unc)) < 1e-3 and cerr < 1e-12 and res > rel, "CORP-PAV Brier identity failed"

# 3. Verify DJIA non-ergodic Itô drain and Baker-McHale shrinkage < 1.0
m_djia = re.search(r"Arithmetic Ensemble Drift = \+([0-9.]+)/day vs Time-Average Growth = \+([0-9.]+)/day", out)
m_bm = re.search(r"Plug-in Kelly = ([0-9.]+) vs Baker-McHale Epistemic Kelly = ([0-9.]+) \(Shrinkage = ([0-9.]+)\)", out)
assert m_djia and m_bm, "Missing DJIA drift or Kelly shrinkage line"
mu_ens, g_time = map(float, m_djia.groups())
f_plug, f_bm, k_bm = map(float, m_bm.groups())
assert g_time < mu_ens and f_bm < f_plug and 0.40 < k_bm < 0.98, "DJIA Itô drain or Baker-McHale shrinkage failed"

# 4. Verify 2x2x2 factorial dominance of FullTriad and net reserve erosion under Prop 1
harms = re.findall(r"\[E=([01]) V=([01]) G=([01])\].*?harm=\s*([0-9.]+) \(MC100=\s*([0-9.]+).*?R_24=\s*([0-9.]+)", out)
assert len(harms) == 8, f"Expected 8 factorial rows, found {len(harms)}"
full_h, full_mc, full_r24 = next((float(h), float(mc), float(r24)) for e, v, g, h, mc, r24 in harms if (e, v, g) == ("1", "1", "1"))
for e, v, g, h, mc, r24 in harms:
    if (e, v, g) != ("1", "1", "1"):
        assert float(h) > full_h and float(mc) > full_mc and float(r24) < full_r24, f"FullTriad failed to dominate ({e},{v},{g})"
    if (e, v, g) == ("0", "1", "1"):
        assert float(r24) < 45.0, f"NoEpistemicMargin (011) must suffer net reserve erosion below R_0=45.0, got {r24}"

# 5. Verify Fixed-Budget Mechanism Isolation Factorial, 3^5=243 structural DGP invariance, Wide-Band Stress Grid, and 4 Falsifiers by relational properties
m_fb = re.search(r"Fixed-Budget Mechanism Isolation Factorial.*?delta_harm_Epi=([+-]?[0-9.]+), delta_harm_Vital=([+-]?[0-9.]+), delta_harm_Gov=([+-]?[0-9.]+)", out)
assert m_fb, "Missing Fixed-Budget Mechanism Isolation Factorial"
dh_e, dh_v, dh_g = map(float, m_fb.groups())
assert dh_e > 0.0 and dh_v > 0.0 and dh_g > 0.0, f"Removing each triad axis at fixed budget must strictly increase harm: ({dh_e}, {dh_v}, {dh_g})"

m_inv = re.search(r"3\^5=243 Threshold Invariance=([0-9.]+)% \| 3\^5=243 Local Structural DGP Invariance=([0-9.]+)% \| Wide-Band Stress Grid: dom=([0-9.]+)%, intact=([0-9.]+)%", out)
assert m_inv, "Missing sensitivity / stress grid line"
thr_inv, loc_inv, w_dom, w_intact = [float(x) / 100.0 for x in m_inv.groups()]
assert 0.80 <= thr_inv <= 1.0 and 0.80 <= loc_inv <= 1.0, f"Local invariance out of bounds: {thr_inv}, {loc_inv}"
assert 0.50 < w_dom < 1.0 and 0.50 < w_intact < 1.0, f"Wide-band stress grid must show genuine empirical boundary (<100%): dom={w_dom}, intact={w_intact}"

# 5a. Relational coherence (refuted <=> d_obs <= d_crit) and design non-saturation on d_obs/d_crit (never dictating whether empirical outcomes are all refuted, all corroborated, or mixed, nor banning SEV == 1.000)
fals = re.findall(r"F([1-4])\(n_eff=(\d+), d_crit=([0-9.]+), d_obs=([0-9.]+), SEV=([0-9.]+), refuted=(true|false)\)", out)
assert len(fals) == 4, f"Expected 4 evaluated falsifiers F1..F4, found {len(fals)}"
for fid, neff, d_crit, d_obs, sev, ref_str in fals:
    is_refuted = (ref_str == "true")
    ratio = float(d_obs) / float(d_crit)
    assert int(neff) >= 15 and 0.95 <= float(sev) <= 1.000 and is_refuted == (float(d_obs) <= float(d_crit)), (
        f"Falsifier F{fid} failed relational coherence check: n_eff={neff}, d_crit={d_crit}, d_obs={d_obs}, SEV={sev}, refuted={ref_str}"
    )
    assert 0.0 < ratio <= 10.0, (
        f"Falsifier F{fid} design saturation check failed: d_obs/d_crit={ratio:.2f} exceeds 10x order-of-magnitude bound"
    )

# 6. Verify notes/The Ethics of Aim.md numerical parity with CLI output (Storms + 2x2x2 Table 1)
ethics = open("notes/The Ethics of Aim.md").read()
assert "19{,}025" in ethics or "19,025" in ethics, "notes/The Ethics of Aim.md missing n_transitions = 19,025"
assert f"{bs:.4f} = {rel:.4f} - {res:.4f} + {unc:.4f}" in ethics, "notes/The Ethics of Aim.md CORP-PAV Brier decomposition drifted from CLI"
for e, v, g, h, mc, r24 in harms:
    assert f"${float(h):.2f}$ / ${float(mc):.2f}$" in ethics, (
        f"notes/The Ethics of Aim.md Table 1 drifted from CLI for cell ({e},{v},{g}): expected ${float(h):.2f}$ / ${float(mc):.2f}$"
    )
for fid, neff, d_crit, d_obs, sev, ref_str in fals:
    assert d_obs in ethics and sev in ethics, (
        f"notes/The Ethics of Aim.md F{fid} drifted from CLI: expected d_obs={d_obs} and SEV={sev}"
    )
PYEOF

if grep -rnE "file://|/home/" book-src/*.md README.md CONTRIBUTING.md AGENTS.md "notes/The Ethics of Aim.md" "notes/The Grammar of Aim.md" >/dev/null 2>&1; then
    echo "ERROR: Found non-relative local path or file:// URI in documentation!" >&2
    grep -rnE "file://|/home/" book-src/*.md README.md CONTRIBUTING.md AGENTS.md "notes/The Ethics of Aim.md" "notes/The Grammar of Aim.md" >&2
    exit 1
fi

for sym in \
    SplitMix64 sha256_hex imbens_manski_critical_value CommunityReserveEnv AgentPolicy NaiveOptimizer CompensatoryBufferAgent StochologicalAgent MultiSeedSummary \
    EpistemicMarginState CognitiveFramingRegime pauker_kassirer_threshold BitemporalRecord \
    VitalMarginState TailMomentRegime KellySizingRegime HollingTailRuptureAudit WaldSurvivorshipAudit SniedovichInfoGapAudit \
    GovernanceState CausalEnvironment IcpIdentificationStatus MacKenzieRegime GoodhartVariant TelicGrade SosaJudgmentGrade AlgedonicChannel BatesonDoubleBindAudit CognitiveAlertFilter ContractAndLiabilityAudit TemporalVault KairosSynchronousPanel ZhengMeisterQueueAudit \
    CollapseProposition TriadicIntegrity StructuralCollapseProofAudit DiagnosticClue ProcessTracingTestKind ProcessTracingInference ReflexiveBrakeAudit LeontiefVsMultiplicativeAudit PreregisteredFalsifier \
    LatentModelExploitationAudit LatentWorldModelConfig FrictionEnvelopeControlAudit ControlRegime TopologyCentralityAudit SocialNetworkSpec \
    ModelDriftFallbackAudit DriftMonitorConfig CellularAutomatonAudit ElementaryRule AutomatonRegime
do
    if ! grep -q "$sym" "book-src/appendix-b-architecture-verification.md"; then
        echo "ERROR: Symbol '$sym' missing from book-src/appendix-b-architecture-verification.md!" >&2
        exit 1
    fi
done

python3 - << 'PYKATEX'
import glob, re, sys

md_files = (
    sorted(glob.glob("book-src/*.md"))
    + ["README.md", "AGENTS.md", "CONTRIBUTING.md", "data/SOURCES.md"]
    + sorted(glob.glob("notes/*.md"))
)
bad = 0
for path in md_files:
    raw = open(path).read()
    masked = re.sub(r"^```[\s\S]*?^```", lambda m: "\n" * m.group(0).count("\n"), raw, flags=re.M)
    masked = re.sub(r"`[^`\n]+`", lambda m: " " * len(m.group(0)), masked)
    for m in re.finditer(r"(?<!\\)\$\$([\s\S]*?)(?<!\\)\$\$|(?<!\\)\$(?:\\\$|[^\$\n])+?(?<!\\)\$", masked):
        is_display = m.group(1) is not None
        expr = m.group(1) if is_display else m.group(0)[1:-1]
        lineno = masked[:m.start()].count("\n") + 1
        without_text = re.sub(r"\\text(?:tt|rm|bf|it)?\{[^{}]*\}", "", expr)
        if r"\_" in without_text or r"\*" in expr or re.search(r"\\\\[,;!%{}]", expr):
            print(f"ERROR: Invalid KaTeX escape sequence in {path}:L{lineno}: {m.group(0)[:80]}", file=sys.stderr)
            bad += 1
        if not is_display:
            if expr.startswith((" ", "\t")) or expr.endswith((" ", "\t")):
                print(f"ERROR: Whitespace adjacent to inline $ delimiter in {path}:L{lineno}: {m.group(0)}", file=sys.stderr)
                bad += 1
            after_idx = m.end()
            if after_idx < len(masked) and re.match(r"[\w\d]", masked[after_idx]):
                print(f"ERROR: Word/digit character immediately after closing $ in {path}:L{lineno}: {m.group(0)}{masked[after_idx]}", file=sys.stderr)
                bad += 1

book_md_all = "\n".join(open(p).read() for p in sorted(glob.glob("book-src/*.md")))
for svg_path in sorted(glob.glob("book-src/figures/*.svg")):
    svg_text = open(svg_path).read()
    no_style = re.sub(r"<style>[\s\S]*?</style>", "", svg_text)
    hex_outside = re.findall(r"#[0-9a-fA-F]{3,8}\b", no_style)
    if "<style>" not in svg_text or "var(--font-sans" not in svg_text or "var(--mono-font" not in svg_text or hex_outside:
        print(f"ERROR: {svg_path} does not follow theme/book.css tokens (hardcoded hex outside style: {hex_outside})", file=sys.stderr)
        bad += 1
    rel_svg = "figures/" + svg_path.split("/")[-1]
    if rel_svg not in book_md_all:
        print(f"ERROR: {svg_path} is not referenced in book-src/*.md", file=sys.stderr)
        bad += 1

if bad:
    sys.exit(1)
PYKATEX

echo "========================================================================"
echo "  ALL STOCHOLOGY CHECKS PASSED (0 WARNINGS, 14 SEALED DATASETS, 9 LABS)"
echo "========================================================================"
