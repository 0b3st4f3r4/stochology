#!/usr/bin/env python3
r"""Deterministic semantic symbol-collision & LaTeX hygiene detector (Layer 0).

Complements .github/checks/structure.py without duplicating its 5-symbol
orthographic check_notation() gate. Focuses on:
  1. Eight semantic symbol-collision families (e/E, R, S, rho, tau, delta, K/k, V)
     when multiple distinct senses coexist inside the same manuscript chapter.
  2. e-value (Ramdas test wealth) vs E-value (VanderWeele-Ding sensitivity) casing.
  3. Non-canonical LaTeX typography (_\text{...} subscript drift or \operatorname
     on canonical operator/subscript tokens) outside SVG figures and code fences.
"""

from __future__ import annotations

import argparse
import glob
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BOOK_SRC = ROOT / "book-src"

FENCE_RE = re.compile(r"^```.*?^```", re.DOTALL | re.MULTILINE)
FIGURE_RE = re.compile(r"<figure\b.*?</figure>", re.DOTALL)
DISPLAY_RE = re.compile(r"\$\$.*?\$\$", re.DOTALL)
INLINE_RE = re.compile(r"(?<!\$)\$(?!\$)([^$\n]+?)(?<!\$)\$(?!\$)")

# Macro drifts beyond structure.py's 5-symbol check (focused on subscripts/operators)
BLOCKING_LATEX_DRIFT = [
    (r"(?:_|\\_)\{?\\text\{(?:epi|vital|gov|Leontief|DEFF|TTS|TTR|CRPS|CORP|PRI|CVaR|EVPI|SEV|PPV)\}",
     "Use \\mathrm{...} instead of \\text{...} in canonical math subscripts"),
    (r"\\operatorname\{(?:DEFF|TTS|TTR|CRPS|CORP|PRI|CVaR|EVPI|SEV)\}",
     "Use \\mathrm{...} instead of \\operatorname{...} for canonical operators"),
]

# Semantic collision families: sense_label -> regex inside stripped text/math
COLLISION_FAMILIES: dict[str, dict[str, str]] = {
    "e/E": {
        "Euler exp(x)": r"\be\^\{|\be\^[-+a-zA-Z0-9]",
        "e-value / e-process": r"\*e\*-value|\*e\*-process|\*e\*-BH|e\\?_\{?[it]\}?|E\\?_\{?t\}?|E\\?_\{?\\mathrm\{trip\}\}?",
        "VanderWeele E-value": r"\$E\$-value|\bE-value\b",
        "Evidence clue (E)": r"\bclue\s+\$?E\$?|\bP\(E\s*\\mid|\bW\(E;",
    },
    "R": {
        "Vital reserve (R_t / R_res)": r"R\\?_\{?(?:t|res|\\mathrm\{res\})\}?",
        "Selection indicator (rho_{R,Y} / R=1)": r"\\rho\\?_\{R,Y\}|\bR\s*=\s*1\b",
        "Epidemiological (R_0 / R_t)": r"\bR\\?_0\b|reproduction\s+number",
        "Refusals (R_1-R_3)": r"\$?R\\?_1\$?\s*[–-]\s*\$?R\\?_3\$?|Ethical\s+Refusal",
        "Lakatos deflation (R_Lakatos)": r"R\\?_\{?\\mathrm\{Lakatos\}\}?",
    },
    "S": {
        "Triad axis (S_epi/vital/gov)": r"S\\?_\{?\\mathrm\{(?:epi|vital|gov)\}\}?",
        "Stratum (S1-S7)": r"`?S[1-7]`?",
        "Entropy (S)": r"\bentropy\b[^.\n]{0,40}\$S\$",
        "Susceptible / Stock (S_k)": r"S\\?_k|\bSusceptible\b",
    },
    "rho": {
        "Data-defect (rho_{R,Y})": r"\\rho\\?_\{?R,Y\}?",
        "Queue utilization (rho=lambda/mu)": r"\\rho/(?:1\s*-\s*\\rho)|heavy-traffic|utilization[^.\n]{0,30}\\rho",
        "Intra-cluster (rho_I / rho_cl)": r"\\rho\\?_\{?(?:I|cl|\\mathrm\{cl\})\}?",
        "Cognitive load (rho_cog)": r"\\rho\\?_\{?\\mathrm\{cog\}\}?",
    },
    "tau": {
        "Acceptance thresholds (tau_acc)": r"\\tau\\?_\{?\\mathrm\{acc\}\}?|\\boldsymbol\{\\tau\}",
        "Latencies (tau_detect/delib/fast/cog)": r"\\tau\\?_\{?\\mathrm\{(?:detect|delib|actuate|fast|bureaucratic|cog)\}\}?",
        "Lorenz horizon (tau_L)": r"\\tau\\?_L\b",
        "Delay ceiling (tau^NL-Q)": r"\\bar\{\\tau\}",
        "Basin tolerance (tau_tol)": r"\\tau\\?_\{?\\mathrm\{tol\}\}?",
    },
    "delta": {
        "Status discount (delta_status)": r"\\delta\\?_\{?\\mathrm\{status\}\}?",
        "Frontier band (delta_band)": r"\\delta\\?_\{?\\mathrm\{band\}\}?",
        "Minimum effect (delta_min)": r"\\delta\\?_\{?\\min\}?",
        "DP slack (eps, delta)": r"\\varepsilon,\s*\\delta",
    },
    "K/k": {
        "Contractual cap (X ^ K)": r"X\s*\\wedge\s*K|\(X\s*-\s*K\)",
        "Hill tail cutoff (k)": r"Hill\s+cutoff[^.\n]{0,20}k|Hill\s+estimator",
        "Hypothesis budget (K_max)": r"K\\?_\{?\\max\}?",
        "Carrying capacity / Keynes K": r"K\\sqrt\{1\s*-\s*4H|\bK\s*/\s*\(K\s*\+\s*I",
    },
    "V": {
        "Keynes evidential weight V": r"V\(n\\?_\{?\\mathrm\{eff\}\}?\)|K/\(K\s*\+\s*I\\?_0\)",
        "Kramers barrier (Delta V)": r"\\Delta\s*V",
        "Lebesgue volume (V_min)": r"V\\?_\{?\\min\}?",
        "Vital count in (E,V,G)": r"\(E,\s*V,\s*G\)",
    },
}


def strip_non_prose(text: str) -> str:
    def _keep_newlines(m: re.Match) -> str:
        return "\n" * m.group(0).count("\n")

    cleaned = FIGURE_RE.sub(_keep_newlines, text)
    cleaned = FENCE_RE.sub(_keep_newlines, cleaned)
    return cleaned


def resolve_files(file_arg: str | None) -> list[Path]:
    if not file_arg:
        return sorted(BOOK_SRC.glob("*.md"))
    tok = file_arg.strip()
    if tok.startswith("book-src/"):
        tok = tok[len("book-src/") :]
    p = BOOK_SRC / tok
    if p.exists():
        return [p]
    if re.fullmatch(r"\d{1,2}", tok):
        matches = sorted(BOOK_SRC.glob(f"{int(tok):02d}-*.md"))
        if matches:
            return matches
    matches = sorted(BOOK_SRC.glob(f"*{tok}*.md"))
    if matches:
        return matches
    raise FileNotFoundError(f"Could not resolve target file token: {file_arg}")


def scan_file(path: Path) -> dict:
    raw = path.read_text(encoding="utf-8")
    cleaned = strip_non_prose(raw)
    blocking: list[dict] = []
    collisions: list[dict] = []

    for line_no, line in enumerate(cleaned.splitlines(), 1):
        for pat, msg in BLOCKING_LATEX_DRIFT:
            for m in re.finditer(pat, line):
                blocking.append({
                    "file": path.name,
                    "line": line_no,
                    "token": m.group(0),
                    "reason": msg,
                })

    # Check multi-sense collision families in the same file
    for family, senses in COLLISION_FAMILIES.items():
        present: list[str] = []
        for sense_name, pat in senses.items():
            if re.search(pat, cleaned):
                present.append(sense_name)
        if len(present) >= 2:
            collisions.append({
                "family": family,
                "senses": present,
            })

    # Check bare ambiguous e-value capitalization drift
    e_drift_lines: list[int] = []
    for line_no, line in enumerate(cleaned.splitlines(), 1):
        if re.search(r"(?<![A-Za-z`])E-value(?![s\w])", line) and "VanderWeele" not in line and "confound" not in line.lower():
            if "e-process" in cleaned or "*e*-value" in cleaned:
                e_drift_lines.append(line_no)

    return {
        "file": path.name,
        "blocking": blocking,
        "collisions": collisions,
        "e_casing_advisory_lines": e_drift_lines,
    }


def analyze(file_arg: str | None = None) -> dict:
    paths = [p for p in resolve_files(file_arg) if p.name != "SUMMARY.md"]
    results = [scan_file(p) for p in paths]
    total_blocking = sum(len(r["blocking"]) for r in results)
    hotspot_files = [r for r in results if r["collisions"] or r["e_casing_advisory_lines"]]
    family_counts: dict[str, int] = defaultdict(int)
    for r in results:
        for c in r["collisions"]:
            family_counts[c["family"]] += 1

    return {
        "summary": {
            "files_scanned": len(results),
            "blocking_count": total_blocking,
            "collision_hotspot_files": len(hotspot_files),
            "collision_family_counts": dict(sorted(family_counts.items(), key=lambda kv: (-kv[1], kv[0]))),
        },
        "blocking": [b for r in results for b in r["blocking"]],
        "hotspots": hotspot_files,
    }


def format_markdown(report: dict, max_rows: int = 35) -> str:
    s = report["summary"]
    status = "PASS" if s["blocking_count"] == 0 else "FAIL"
    lines = ["# Semantic Notation & Symbol-Collision Audit", ""]
    lines.append(
        f"- **Status:** `{status}` · Scanned `{s['files_scanned']}` files · "
        f"`{s['blocking_count']}` blocking macro drifts · `{s['collision_hotspot_files']}` collision-hotspot files"
    )
    if s["collision_family_counts"]:
        lines.append(
            "- **Multi-Sense Family Counts (Advisory):** "
            + ", ".join(f"`{k}`={v}" for k, v in s["collision_family_counts"].items())
        )

    if report["blocking"]:
        lines.extend([
            "",
            "## Blocking LaTeX Typography Drifts",
            "",
            "| File | Line | Token | Repair Direction |",
            "| :--- | :---: | :--- | :--- |",
        ])
        for b in report["blocking"][:50]:
            lines.append(f"| `{b['file']}` | {b['line']} | `{b['token']}` | {b['reason']} |")

    if report["hotspots"]:
        lines.extend([
            "",
            "## Top Semantic Collision Hotspots (Advisory — Require Local Disambiguation)",
            "",
            "| File | Co-Occurring Symbol Families & Senses |",
            "| :--- | :--- |",
        ])
        ranked = sorted(report["hotspots"], key=lambda r: (-len(r["collisions"]), r["file"]))
        for r in ranked[:max_rows]:
            parts = [f"**`{c['family']}`** ({' vs '.join(c['senses'])})" for c in r["collisions"]]
            if r["e_casing_advisory_lines"]:
                parts.append(f"**`E-value casing`** (lines {r['e_casing_advisory_lines'][:5]})")
            lines.append(f"| `{r['file']}` | {'; '.join(parts)} |")
        if len(ranked) > max_rows:
            lines.append(f"\n*Truncated {len(ranked) - max_rows} lower-density hotspot files to preserve cognitive budget.*")

    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Check Stochology semantic symbol collisions and LaTeX macro hygiene.")
    parser.add_argument("--file", help="Target chapter number or filename (e.g. 50 or 50-methodological-families-reflexivity.md)")
    parser.add_argument("--json", action="store_true", help="Emit JSON output")
    parser.add_argument("--markdown", action="store_true", help="Emit Markdown summary")
    parser.add_argument("--out", help="Optional file path to write output")
    args = parser.parse_args()

    report = analyze(args.file)
    output = json.dumps(report, indent=2) + "\n" if args.json else format_markdown(report)
    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(output, encoding="utf-8")
    else:
        sys.stdout.write(output)
    return 0 if report["summary"]["blocking_count"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
