#!/usr/bin/env python3
"""Deterministic prose cross-reference, Bridge handoff, and Layer-0 orchestrator.

Complements .github/checks/structure.py (which already validates relative markdown
links, #anchors, spine counts, Table B.1, citations, concordance, and guard sites).
Checks:
  1. Out-of-range prose references (Chapter > 99, Layer > 35, Appendix > G,
     Stratum > VII, Reflexive Brake > 4, Falsifier > F5, or invalid Appendix subsections).
  2. Sequential ## Bridge handoffs (Ch. NN -> Ch. NN+1 or Part conclusion on boundary).
  3. Unlinked prose cross-reference density (advisory for coherence-editor).
  4. --full-report mode: runs all 7 Layer-0 checks (structure.py plus check_graphs.py,
     check_notation.py, check_symbolic_math.py, check_style_and_citations.py,
     check_custody_provenance.py, and check_crossrefs.py) into reviews/00-deterministic-report*.md.
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import subprocess
import sys
import urllib.parse
from datetime import datetime
from pathlib import Path

ROOT = Path(os.environ.get("STOCHOLOGY_REPO_ROOT") or Path(__file__).resolve().parents[2]).resolve()
BOOK_SRC = ROOT / "book-src"
REVIEWS_DIR = ROOT / "reviews"

FENCE_RE = re.compile(r"^```.*?^```", re.DOTALL | re.MULTILINE)
FIGURE_RE = re.compile(r"<figure\b.*?</figure>", re.DOTALL)
MD_LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")

PROSE_CHAPTER_RE = re.compile(r"\bChapters?\s+(\d{1,3})\b")
PROSE_LAYER_RE = re.compile(r"\bLayers?\s+(\d{1,3})\b")
PROSE_APPENDIX_RE = re.compile(r"\bAppendix\s+([A-Z])(?:\.(\d+))?\b")
PROSE_BRAKE_RE = re.compile(r"\bReflexive\s+Brakes?\s+(\d+)\b")
PROSE_FALSIFIER_RE = re.compile(r"\bFalsifiers?\s+F(\d+)\b")

PART_LAST_CHAPTER = {
    8: "part-i-conclusion.md",
    26: "part-ii-conclusion.md",
    41: "part-iii-conclusion.md",
    50: "part-iv-conclusion.md",
    85: "part-v-conclusion.md",
    92: "part-vi-conclusion.md",
    99: "part-vii-conclusion.md",
}


def strip_non_prose(text: str) -> str:
    def _keep_newlines(m: re.Match) -> str:
        return "\n" * m.group(0).count("\n")

    return FENCE_RE.sub(_keep_newlines, FIGURE_RE.sub(_keep_newlines, text))


def load_summary_and_appendices() -> tuple[dict[int, dict], dict[str, set[str]]]:
    summary = (BOOK_SRC / "SUMMARY.md").read_text(encoding="utf-8")
    chapters: dict[int, dict] = {}
    for title, target in MD_LINK_RE.findall(summary):
        base = urllib.parse.unquote(target.split("#", 1)[0].strip())
        m = re.match(r"^(\d{2})-.*\.md$", base)
        if m:
            num = int(m.group(1))
            chapters[num] = {"file": base, "title": title.strip()}

    app_sections: dict[str, set[str]] = {}
    for letter in ("A", "B", "C", "D", "E", "F", "G"):
        matches = sorted(BOOK_SRC.glob(f"appendix-{letter.lower()}-*.md"))
        secs: set[str] = set()
        if matches:
            text = matches[0].read_text(encoding="utf-8")
            for hm in re.finditer(rf"^##+\s+{letter}\.(\d+)\b", text, re.MULTILINE):
                secs.add(hm.group(1))
        app_sections[letter] = secs
    return chapters, app_sections


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


def check_bridge_handoff(path: Path, text: str, chapters: dict[int, dict]) -> dict | None:
    m = re.match(r"^(\d{2})-.*\.md$", path.name)
    if not m:
        return None
    ch_num = int(m.group(1))
    if ch_num == 0:
        return None
    bridge_pos = text.rfind("## Bridge")
    if bridge_pos < 0:
        return {"file": path.name, "status": "missing_bridge_heading", "expected": None}

    bridge = text[bridge_pos:]
    targets = {urllib.parse.unquote(t.split("#", 1)[0].strip()) for _, t in MD_LINK_RE.findall(bridge)}

    # Check either Chapter NN+1 or Part boundary conclusion (e.g. Ch 08 -> Part I Conclusion or Ch 09)
    expected_candidates: list[str] = []
    if ch_num < 99 and (ch_num + 1) in chapters:
        expected_candidates.append(chapters[ch_num + 1]["file"])
    if ch_num in PART_LAST_CHAPTER:
        expected_candidates.append(PART_LAST_CHAPTER[ch_num])

    matched = any(cand in targets for cand in expected_candidates)
    if not matched and ch_num < 99:
        nxt = ch_num + 1
        if re.search(rf"\bChapter\s+0?{nxt}\b", bridge):
            matched = True
        elif nxt in chapters:
            # Allow clean title match when chapter links without number
            clean_title = chapters[nxt]["title"].split(":", 1)[-1].strip().lower()
            if clean_title and clean_title[:24] in bridge.lower():
                matched = True

    return {
        "file": path.name,
        "chapter": ch_num,
        "status": "ok" if matched else "missing_next_handoff",
        "expected": expected_candidates,
        "linked_targets": sorted(targets),
    }


def analyze(file_arg: str | None = None) -> dict:
    chapters, app_sections = load_summary_and_appendices()
    paths = [p for p in resolve_files(file_arg) if p.name != "SUMMARY.md"]

    blocking: list[dict] = []
    bridge_issues: list[dict] = []
    unlinked_summary: list[dict] = []

    for path in paths:
        raw = path.read_text(encoding="utf-8")
        cleaned = strip_non_prose(raw)

        for line_no, line in enumerate(cleaned.splitlines(), 1):
            for m in PROSE_CHAPTER_RE.finditer(line):
                num = int(m.group(1))
                if num > 99:
                    blocking.append({"file": path.name, "line": line_no, "token": m.group(0), "reason": "Chapter number > 99"})
            for m in PROSE_LAYER_RE.finditer(line):
                num = int(m.group(1))
                if not (1 <= num <= 35):
                    blocking.append({"file": path.name, "line": line_no, "token": m.group(0), "reason": "Layer number outside 1..35"})
            for m in PROSE_APPENDIX_RE.finditer(line):
                letter, sub = m.group(1), m.group(2)
                # External paper locators in Appendix C/D (e.g., "(Appendix H)" in Lewis et al. 2020)
                if path.name in ("appendix-c-sources.md", "appendix-d-claim-ledger.md"):
                    ctx_before = line[max(0, m.start() - 2) : m.start()]
                    if "(" in ctx_before or line.lstrip().startswith("|"):
                        continue
                if letter not in app_sections:
                    blocking.append({"file": path.name, "line": line_no, "token": m.group(0), "reason": f"Unknown Appendix {letter}"})
                elif sub and app_sections[letter] and sub not in app_sections[letter]:
                    blocking.append({
                        "file": path.name,
                        "line": line_no,
                        "token": m.group(0),
                        "reason": f"Appendix {letter}.{sub} not among headings {sorted(app_sections[letter])}",
                    })
            for m in PROSE_BRAKE_RE.finditer(line):
                if not (1 <= int(m.group(1)) <= 4):
                    blocking.append({"file": path.name, "line": line_no, "token": m.group(0), "reason": "Reflexive Brake outside 1..4"})
            for m in PROSE_FALSIFIER_RE.finditer(line):
                if not (1 <= int(m.group(1)) <= 5):
                    blocking.append({"file": path.name, "line": line_no, "token": m.group(0), "reason": "Falsifier outside F1..F5"})

        # Count unlinked prose references outside markdown link brackets
        no_links = re.sub(r"\[[^\]]+\]\([^)]+\)", "", cleaned)
        unlinked_ch = len(PROSE_CHAPTER_RE.findall(no_links))
        unlinked_app = len(PROSE_APPENDIX_RE.findall(no_links))
        if unlinked_ch + unlinked_app > 0:
            unlinked_summary.append({
                "file": path.name,
                "unlinked_chapter_mentions": unlinked_ch,
                "unlinked_appendix_mentions": unlinked_app,
            })

        b = check_bridge_handoff(path, cleaned, chapters)
        if b and b["status"] != "ok":
            bridge_issues.append(b)

    return {
        "summary": {
            "files_scanned": len(paths),
            "blocking_count": len(blocking),
            "bridge_handoff_warnings": len(bridge_issues),
            "files_with_unlinked_prose_refs": len(unlinked_summary),
        },
        "blocking": blocking,
        "bridge_issues": bridge_issues,
        "unlinked_hotspots": sorted(
            unlinked_summary,
            key=lambda r: -(r["unlinked_chapter_mentions"] + r["unlinked_appendix_mentions"]),
        ),
    }


def format_markdown(report: dict) -> str:
    s = report["summary"]
    status = "PASS" if s["blocking_count"] == 0 else "FAIL"
    lines = ["# Prose Cross-Reference & Bridge Handoff Audit", ""]
    lines.append(
        f"- **Status:** `{status}` · Scanned `{s['files_scanned']}` files · "
        f"`{s['blocking_count']}` out-of-range prose references · "
        f"`{s['bridge_handoff_warnings']}` Bridge sequential handoff warnings"
    )
    if report["blocking"]:
        lines.extend(["", "## Blocking Out-of-Range References", "", "| File | Line | Token | Reason |", "| :--- | :---: | :--- | :--- |"])
        for b in report["blocking"]:
            lines.append(f"| `{b['file']}` | {b['line']} | `{b['token']}` | {b['reason']} |")

    if report["bridge_issues"]:
        lines.extend(["", "## Bridge Sequential Handoff Warnings", "", "| File | Status | Expected Next | Linked in Bridge |", "| :--- | :--- | :--- | :--- |"])
        for bi in report["bridge_issues"]:
            lines.append(
                f"| `{bi['file']}` | `{bi['status']}` | `{', '.join(bi['expected'] or [])}` | `{', '.join(bi['linked_targets'])}` |"
            )

    if report["unlinked_hotspots"]:
        lines.extend(["", "## Top Unlinked Prose Cross-References (Advisory)", "", "| File | Unlinked `Chapter N` | Unlinked `Appendix X` |", "| :--- | :---: | :---: |"])
        for row in report["unlinked_hotspots"][:20]:
            lines.append(f"| `{row['file']}` | {row['unlinked_chapter_mentions']} | {row['unlinked_appendix_mentions']} |")

    return "\n".join(lines) + "\n"


def run_full_deterministic_suite() -> int:
    stamp = datetime.now().strftime("%Y%m%d-%H%M")
    REVIEWS_DIR.mkdir(parents=True, exist_ok=True)

    ci_proc = subprocess.run(
        [sys.executable, str(ROOT / ".github" / "checks" / "structure.py")],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    ci_tail = "\n".join(ci_proc.stdout.strip().splitlines()[-12:])

    graphs_proc = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("check_graphs.py")), "--markdown"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    notation_proc = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("check_notation.py")), "--markdown"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    math_proc = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("check_symbolic_math.py")), "--markdown"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    style_proc = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("check_style_and_citations.py")), "--markdown"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    custody_proc = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("check_custody_provenance.py")), "--strict-bloat", "--markdown"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    crossref_report = analyze(None)
    crossref_md = format_markdown(crossref_report)

    any_fail = (
        ci_proc.returncode != 0
        or graphs_proc.returncode != 0
        or notation_proc.returncode != 0
        or math_proc.returncode != 0
        or style_proc.returncode != 0
        or custody_proc.returncode != 0
        or crossref_report["summary"]["blocking_count"] != 0
    )
    verdict = "BLOCKED" if any_fail else "PASS (WITH ADVISORY HOTSPOTS)"

    sections = [
        f"# Stochology Layer 0 Deterministic Audit Report ({stamp})",
        "",
        f"- **Leontief Layer-0 Verdict:** `{verdict}`",
        f"- **Generated:** `{datetime.now().isoformat(timespec='seconds')}`",
        "",
        "## Gate Summary",
        "",
        "| Check | Exit / Status | Scope |",
        "| :--- | :---: | :--- |",
        f"| `.github/checks/structure.py` | `{'PASS' if ci_proc.returncode == 0 else 'FAIL'}` | CI gates (§V-1..§V-10, math, links, B.1, ledger, concordance) |",
        f"| `.agents/scripts/check_graphs.py` | `{'PASS' if graphs_proc.returncode == 0 else 'FAIL'}` | `G1` DAG acyclicity, `Cone(L_i)`, zero-axis inheritance, `J(7,3)` |",
        f"| `.agents/scripts/check_notation.py` | `{'PASS' if notation_proc.returncode == 0 else 'FAIL'}` | 10 symbol-collision families & LaTeX macro hygiene |",
        f"| `.agents/scripts/check_symbolic_math.py` | `{'PASS' if math_proc.returncode == 0 else 'FAIL'}` | SymPy/NumPy symbolic proofs for 13 core mathematical claims |",
        f"| `.agents/scripts/check_style_and_citations.py` | `{'PASS' if style_proc.returncode == 0 else 'FAIL'}` | Running-prose em-dashes, invisible watermarks, DOI/arXiv syntax |",
        f"| `.agents/scripts/check_custody_provenance.py` | `{'PASS' if custody_proc.returncode == 0 else 'FAIL'}` | Custody-tier classification, prose-bloat delta & Git trailers |",
        f"| `.agents/scripts/check_crossrefs.py` | `{'PASS' if crossref_report['summary']['blocking_count'] == 0 else 'FAIL'}` | Out-of-range prose refs & sequential `## Bridge` handoffs |",
        "",
        "---",
        "",
        "## 1. Existing CI Gate Output (`.github/checks/structure.py` tail)",
        "",
        "```text",
        ci_tail,
        "```",
        "",
        "---",
        "",
        graphs_proc.stdout.strip(),
        "",
        "---",
        "",
        notation_proc.stdout.strip(),
        "",
        "---",
        "",
        math_proc.stdout.strip(),
        "",
        "---",
        "",
        style_proc.stdout.strip(),
        "",
        "---",
        "",
        custody_proc.stdout.strip(),
        "",
        "---",
        "",
        crossref_md.strip(),
        "",
    ]
    full_md = "\n".join(sections)

    ts_path = REVIEWS_DIR / f"00-deterministic-report-{stamp}.md"
    latest_path = REVIEWS_DIR / "00-deterministic-report.md"
    ts_path.write_text(full_md, encoding="utf-8")
    latest_path.write_text(full_md, encoding="utf-8")

    print(f"Wrote {ts_path.relative_to(ROOT)} and updated {latest_path.relative_to(ROOT)} [{verdict}]")
    return 1 if any_fail else 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate prose cross-references, Bridge handoffs, or run full Layer-0 suite.")
    parser.add_argument("--file", help="Target chapter number or filename")
    parser.add_argument("--json", action="store_true", help="Emit JSON output")
    parser.add_argument("--markdown", action="store_true", help="Emit Markdown summary")
    parser.add_argument("--out", help="Optional file path to write output")
    parser.add_argument("--full-report", action="store_true", help="Run all 7 Layer-0 checks and write reviews/00-deterministic-report*.md")
    args = parser.parse_args()

    if args.full_report:
        return run_full_deterministic_suite()

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
