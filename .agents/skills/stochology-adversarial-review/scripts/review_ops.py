#!/usr/bin/env python3
"""Deterministic operational toolkit for the Stochology Adversarial Review Swarm.

Implements zero-token structural X-ray, 9-shard cartography planning, cross-part
and Johnson-graph J(7,3) relation analysis, wave collision & anti-bloat auditing
(Swarm Reflexive Brake 2), and full CI gate orchestration.

Standard library only; reads the repository without mutating tracked files.
"""

from __future__ import annotations

import argparse
import contextlib
import fcntl
import glob
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.parse
from datetime import datetime, timezone
from pathlib import Path


def _discover_root() -> Path:
    env_root = os.environ.get("STOCHOLOGY_ROOT")
    if env_root and (Path(env_root) / "book-src" / "SUMMARY.md").exists():
        return Path(env_root).resolve()
    cwd = Path.cwd().resolve()
    for candidate in (cwd, *cwd.parents):
        if (candidate / "book-src" / "SUMMARY.md").exists():
            return candidate
    here = Path(__file__).resolve()
    for candidate in here.parents:
        if (candidate / "book-src" / "SUMMARY.md").exists():
            return candidate
    return here.parents[4]


def _discover_main_root(root: Path) -> Path:
    try:
        proc = subprocess.run(
            ["git", "rev-parse", "--git-common-dir"],
            cwd=root,
            capture_output=True,
            text=True,
            check=True,
        )
        common = Path(proc.stdout.strip())
        if not common.is_absolute():
            common = (root / common).resolve()
        else:
            common = common.resolve()
        if common.name == ".git" and (common.parent / "book-src").exists():
            return common.parent
    except (OSError, subprocess.SubprocessError):
        pass
    return root


ROOT = _discover_root()
MAIN_ROOT = _discover_main_root(ROOT)
BOOK_SRC = ROOT / "book-src"
CHECKS_DIR = ROOT / ".github" / "checks"

SVG_FIGURE_RE = re.compile(r'<figure class="stoch-figure[^"]*">.*?</figure>', re.DOTALL)
HEADING_RE = re.compile(r"^(#{1,3})\s+(.+?)\s*$", re.MULTILINE)
MD_LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")
LAYER_BQ_RE = re.compile(
    r"^>\s*Layer\s+(\d+)\s+of\s+Part\s+V\s+·\s+Stratum\s+([IVX]+)\s+·\s+"
    r"Chapter\s+(\d+)\s+·\s+Unique\s+Guard\s+Combination\s+`?\{([^}]+)\}`?\s+·\s+"
    r"Signature\s+`?\(([^)]+)\)`?",
    re.MULTILINE,
)
CITATION_CALL_RE = re.compile(
    r"\b([A-ZÀ-ÖØ-Þ][A-Za-zÀ-ÖØ-öø-ÿ'’-]+(?:\s+(?:&|and|et\s+al\.)\s+"
    r"[A-ZÀ-ÖØ-Þ][A-Za-zÀ-ÖØ-öø-ÿ'’-]+)?)\s*\(?(c\.\s*\d{3,4}\s*BCE|\d{4}[a-z]?)\)?"
)
INLINE_MATH_RE = re.compile(r"(?<!\$)\$(?!\$)([^$\n]+?)(?<!\$)\$(?!\$)")
DISPLAY_MATH_RE = re.compile(r"\$\$.*?\$\$", re.DOTALL)
UNESCAPED_MATH_UNDERSCORE_RE = re.compile(r"(?<!\\)_")

# Exact numeral census predicate matching .github/checks/structure.py (§V-7),
# evaluated per file without the 25-line global print cap.
CENSUS_FENCE_RE = re.compile(r"^```.*?^```", re.DOTALL | re.MULTILINE)
CENSUS_SVG_RE = re.compile(r"<svg\b.*?</svg>", re.DOTALL)
CENSUS_HTML_RE = re.compile(r"<figure\b.*?</figure>", re.DOTALL)
CENSUS_NUMERAL_RE = re.compile(r"\d")
CENSUS_CITATION_RE = re.compile(r"\((?:[^()]*?)\d{4}[a-z]?\)|\d{4}[a-z]?\)")
CENSUS_MATH_RE = re.compile(r"\$[^$]+\$|\$\$.*?\$\$", re.DOTALL)
CENSUS_EDITORIAL_RE = re.compile(r"^\s*(?:\d+\.|[-*])\s|^#{1,6}\s|^\|")


def file_numeral_census(raw_text: str) -> list[dict]:
    """Return all §V-7 numeral census candidate lines for a single file."""
    if "\n" not in raw_text:
        return []

    # Preserve line numbers by replacing stripped blocks with matching newline counts.
    def _blank_newlines(match: re.Match) -> str:
        return "\n" * match.group(0).count("\n")

    lines = raw_text.splitlines()
    masked = "\n".join(lines[1:])
    masked = CENSUS_HTML_RE.sub(_blank_newlines, masked)
    masked = CENSUS_SVG_RE.sub(_blank_newlines, masked)
    masked = CENSUS_FENCE_RE.sub(_blank_newlines, masked)

    candidates: list[dict] = []
    for line_no, line in enumerate(masked.splitlines(), 2):
        if not CENSUS_NUMERAL_RE.search(line):
            continue
        if (
            CENSUS_CITATION_RE.search(line)
            or CENSUS_MATH_RE.search(line)
            or CENSUS_EDITORIAL_RE.search(line)
        ):
            continue
        if re.match(r"^\s*[<>|]|^\s*#", line):
            continue
        candidates.append({"line": line_no, "text": line.strip()[:140]})
    return candidates


ADVISORY_PATTERNS = {
    "orphan_appendix_c": re.compile(r"orphan Appendix C entries.*?:\s*(\d+)"),
    "display_first_symbols": re.compile(
        r"symbols first set inside a display block.*?:\s*(\d+)"
    ),
    "near_duplicate_sentences": re.compile(
        r"near-duplicate sentences across chapters.*?:\s*(\d+)"
    ),
    "weak_cross_references": re.compile(
        r"cross-references sharing no content word.*?:\s*(\d+)"
    ),
    "notation_drift": re.compile(
        r"notation drift that renders identically.*?:\s*(\d+)"
    ),
    "numeral_census_candidates": re.compile(
        r"(\d+)\s+candidate lines without a citation year"
    ),
}


def strip_svg(text: str) -> str:
    return SVG_FIGURE_RE.sub("", text)


def prose_lines(text: str) -> int:
    cleaned = strip_svg(text)
    return sum(1 for line in cleaned.splitlines() if line.strip())


def prose_words(text: str) -> int:
    cleaned = strip_svg(text)
    return len(cleaned.split())


def resolve_chapter_arg(token: str) -> str:
    """Resolve '41', '00', or 'book-src/41-...' to a book-src basename."""
    token = token.strip()
    if token.startswith("book-src/"):
        token = token[len("book-src/") :]
    direct = BOOK_SRC / token
    if direct.exists():
        return direct.name
    if re.fullmatch(r"\d{1,2}", token):
        num = f"{int(token):02d}"
        matches = sorted(BOOK_SRC.glob(f"{num}-*.md"))
        if len(matches) == 1:
            return matches[0].name
    matches = sorted(BOOK_SRC.glob(f"*{token}*.md"))
    if len(matches) == 1:
        return matches[0].name
    raise ValueError(f"Cannot uniquely resolve manuscript file token: {token!r}")


def parse_summary_order() -> list[str]:
    summary_text = (BOOK_SRC / "SUMMARY.md").read_text(encoding="utf-8")
    ordered: list[str] = []
    seen: set[str] = set()
    for _, target in MD_LINK_RE.findall(summary_text):
        decoded = urllib.parse.unquote(target.split("#", 1)[0].strip())
        if decoded.endswith(".md") and (BOOK_SRC / decoded).exists():
            if decoded not in seen:
                ordered.append(decoded)
                seen.add(decoded)
    # Ensure no .md file in book-src/ (except SUMMARY.md) is omitted.
    for path in sorted(BOOK_SRC.glob("*.md")):
        if path.name != "SUMMARY.md" and path.name not in seen:
            ordered.append(path.name)
            seen.add(path.name)
    return ordered


def inspect_file(name: str) -> dict:
    path = BOOK_SRC / name
    raw = path.read_text(encoding="utf-8")
    cleaned = strip_svg(raw)
    headings = [m.group(2) for m in HEADING_RE.finditer(cleaned) if m.group(1) == "#"]
    title = headings[0] if headings else name
    out_links: set[str] = set()
    for _, target in MD_LINK_RE.findall(cleaned):
        base = urllib.parse.unquote(target.split("#", 1)[0].strip())
        if base.endswith(".md") and (BOOK_SRC / base).exists() and base != name:
            out_links.add(base)

    inline_math = INLINE_MATH_RE.findall(cleaned)
    display_math = DISPLAY_MATH_RE.findall(cleaned)
    unescaped_subscripts = 0
    for span in inline_math:
        if span.count("_") >= 2 and UNESCAPED_MATH_UNDERSCORE_RE.search(span):
            # Two unescaped underscores inside inline math risk CommonMark emphasis splitting.
            unescaped_underscores = len(UNESCAPED_MATH_UNDERSCORE_RE.findall(span))
            if unescaped_underscores >= 2:
                unescaped_subscripts += 1

    citations = sorted(
        {f"{m.group(1)} ({m.group(2)})" for m in CITATION_CALL_RE.finditer(cleaned)}
    )

    info: dict = {
        "file": name,
        "title": title,
        "raw_lines": len(raw.splitlines()),
        "prose_lines": prose_lines(raw),
        "prose_words": prose_words(raw),
        "inline_math_count": len(inline_math),
        "display_math_count": len(display_math),
        "unescaped_inline_subscript_risks": unescaped_subscripts,
        "outgoing_links": sorted(out_links),
        "citation_calls_detected": len(citations),
    }

    bq = LAYER_BQ_RE.search(cleaned)
    if bq:
        sites = sorted(set(re.findall(r"T\d", bq.group(4))))
        primary_match = re.search(r"CH\d\d\s*·\s*(T\d)\*", raw)
        info["part_v"] = {
            "layer": f"L{int(bq.group(1)):02d}",
            "stratum_roman": bq.group(2),
            "chapter": f"{int(bq.group(3)):02d}",
            "guard_sites": sites,
            "primary_anchor": primary_match.group(1) if primary_match else None,
            "signature": bq.group(5).replace(" ", ""),
        }
    return info


def build_book_index() -> dict[str, dict]:
    ordered = parse_summary_order()
    index = {name: inspect_file(name) for name in ordered}
    for name in index:
        index[name]["incoming_links"] = []
    for src, meta in index.items():
        for dst in meta["outgoing_links"]:
            if dst in index:
                index[dst]["incoming_links"].append(src)
    for meta in index.values():
        meta["incoming_links"].sort()
    return index


def build_shards(index: dict[str, dict]) -> list[dict]:
    """Partition all manuscript files into 9 federated shards with full coverage."""
    buckets: dict[str, dict] = {
        "S1-preface-part1": {
            "id": "S1-preface-part1",
            "guild_lead": "gov-ethicist + epi-methodologist",
            "scope": "Prologue (Aim Among Mirrors), Part I (Thinking Under Uncertainty, Ch. 00–08) & Appendix F (Manuscript Audit Map)",
            "files": [],
        },
        "S2-part2-math-phil": {
            "id": "S2-part2-math-phil",
            "guild_lead": "epi-methodologist + gov-ethicist",
            "scope": "Part II Bases: Mathematics & Philosophy (Ch. 09–20)",
            "files": [],
        },
        "S3-part2-sci-part3-circuit": {
            "id": "S3-part2-sci-part3-circuit",
            "guild_lead": "vital-cyberneticist + philology-archivist",
            "scope": "Part II Natural Sciences & Part III Circuit Opening (Ch. 21–31)",
            "files": [],
        },
        "S4-part3-judgment-triad": {
            "id": "S4-part3-judgment-triad",
            "guild_lead": "epi-methodologist + gov-ethicist",
            "scope": "Part III Foundations of Judgment & Triad Synthesis (Ch. 32–41)",
            "files": [],
        },
        "S5-part4-framework-core-appendices": {
            "id": "S5-part4-framework-core-appendices",
            "guild_lead": "vital-cyberneticist + epi-methodologist + gov-ethicist",
            "scope": "Part IV Sequential Framework (Ch. 42–50) & Appendices A, B, E, G",
            "files": [],
        },
        "S6-part5-strata-1-to-4": {
            "id": "S6-part5-strata-1-to-4",
            "guild_lead": "vital-cyberneticist + epi-methodologist",
            "scope": "Part V Strata S1–S4 (Physical, Formal, Living, Clinical: Ch. 51–70)",
            "files": [],
        },
        "S7-part5-strata-5-to-7-ledgers": {
            "id": "S7-part5-strata-5-to-7-ledgers",
            "guild_lead": "philology-archivist + gov-ethicist + commons-steward",
            "scope": "Part V Strata S5–S7 (Archival, Strategic, Constitutional: Ch. 71–85) & Appendices C, D",
            "files": [],
        },
        "S8-part6-trajectory": {
            "id": "S8-part6-trajectory",
            "guild_lead": "epi-methodologist + vital-cyberneticist + gov-ethicist",
            "scope": "Part VI The Trajectory of Aim (Laminar Transits T1–T7: Ch. 86–92)",
            "files": [],
        },
        "S9-part7-superposition": {
            "id": "S9-part7-superposition",
            "guild_lead": "counter-inductive-critic + epi-methodologist + vital-cyberneticist + gov-ethicist",
            "scope": "Part VII The Superposed Aim (Simultaneous Stress & Reopening: Ch. 93–99)",
            "files": [],
        },
    }

    for name in index:
        m = re.match(r"^(\d{2})-", name)
        num = int(m.group(1)) if m else None
        if (
            name == "00-preface.md"
            or name.startswith("part-i-")
            or (num is not None and 1 <= num <= 8)
            or name == "appendix-f-manuscript-audit.md"
        ):
            buckets["S1-preface-part1"]["files"].append(name)
        elif name == "part-ii-introduction.md" or (num is not None and 9 <= num <= 20):
            buckets["S2-part2-math-phil"]["files"].append(name)
        elif (num is not None and 21 <= num <= 31) or name in {
            "part-ii-conclusion.md",
            "part-iii-introduction.md",
        }:
            buckets["S3-part2-sci-part3-circuit"]["files"].append(name)
        elif (num is not None and 32 <= num <= 41) or name == "part-iii-conclusion.md":
            buckets["S4-part3-judgment-triad"]["files"].append(name)
        elif (
            name.startswith("part-iv-")
            or (num is not None and 42 <= num <= 50)
            or name
            in {
                "appendix-a-mathematics.md",
                "appendix-b-architecture-verification.md",
                "appendix-e-lexicon.md",
                "appendix-g-glossary.md",
            }
        ):
            buckets["S5-part4-framework-core-appendices"]["files"].append(name)
        elif (
            name == "part-v-introduction.md"
            or name
            in {
                "stratum-i-opening.md",
                "stratum-ii-opening.md",
                "stratum-iii-opening.md",
                "stratum-iv-opening.md",
            }
            or (num is not None and 51 <= num <= 70)
        ):
            buckets["S6-part5-strata-1-to-4"]["files"].append(name)
        elif name.startswith("part-vi-") or (num is not None and 86 <= num <= 92):
            buckets["S8-part6-trajectory"]["files"].append(name)
        elif name.startswith("part-vii-") or (num is not None and 93 <= num <= 99):
            buckets["S9-part7-superposition"]["files"].append(name)
        else:
            buckets["S7-part5-strata-5-to-7-ledgers"]["files"].append(name)

    shards: list[dict] = []
    assigned_total = 0
    for shard in buckets.values():
        files = shard["files"]
        assigned_total += len(files)
        shard["file_count"] = len(files)
        shard["prose_lines"] = sum(index[f]["prose_lines"] for f in files)
        shard["prose_words"] = sum(index[f]["prose_words"] for f in files)
        shards.append(shard)

    if assigned_total != len(index):
        raise RuntimeError(
            f"Shard coverage mismatch: assigned {assigned_total} vs {len(index)} indexed files"
        )
    return shards


def parse_appendix_c_layers() -> list[dict]:
    path = BOOK_SRC / "appendix-c-sources.md"
    text = strip_svg(path.read_text(encoding="utf-8"))
    layers: list[dict] = []
    in_table = False
    for line in text.splitlines():
        if line.startswith("| Layer of the argument |"):
            in_table = True
            continue
        if in_table:
            if not line.startswith("|"):
                break
            if ":---" in line:
                continue
            cells = [c.strip() for c in line.strip("|").split("|")]
            if len(cells) >= 3:
                ch_nums: set[int] = set()
                for rng in re.finditer(r"`(\d{2})`–`(\d{2})`", cells[2]):
                    for n in range(int(rng.group(1)), int(rng.group(2)) + 1):
                        ch_nums.add(n)
                for single in re.finditer(r"`(\d{2})`", cells[2]):
                    ch_nums.add(int(single.group(1)))
                layers.append(
                    {
                        "layer": cells[0],
                        "role": cells[1],
                        "chapters": [f"{n:02d}" for n in sorted(ch_nums)],
                    }
                )
    return layers


def johnson_graph_part_v(index: dict[str, dict]) -> dict:
    part_v = {name: meta for name, meta in index.items() if "part_v" in meta}
    edges: list[dict] = []
    unlinked_siblings: list[dict] = []
    names = sorted(part_v.keys())
    for i, a in enumerate(names):
        sa = set(part_v[a]["part_v"]["guard_sites"])
        for b in names[i + 1 :]:
            sb = set(part_v[b]["part_v"]["guard_sites"])
            shared = sorted(sa & sb)
            if len(shared) == 2:
                linked = (b in part_v[a]["outgoing_links"]) or (
                    a in part_v[b]["outgoing_links"]
                )
                edge = {
                    "a": a,
                    "b": b,
                    "order_a": part_v[a]["part_v"]["layer"],
                    "order_b": part_v[b]["part_v"]["layer"],
                    "shared_guards": shared,
                    "cross_linked_in_prose": linked,
                }
                edges.append(edge)
                if not linked:
                    unlinked_siblings.append(edge)
    return {
        "order_count": len(part_v),
        "johnson_j7_3_edge_count": len(edges),
        "linked_edges": sum(1 for e in edges if e["cross_linked_in_prose"]),
        "unlinked_edges": len(unlinked_siblings),
        "edges": edges,
        "unlinked_sample": unlinked_siblings[:25],
    }


def run_structure_check() -> dict:
    proc = subprocess.run(
        [sys.executable, str(CHECKS_DIR / "structure.py")],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    out = proc.stdout + ("\n" + proc.stderr if proc.stderr else "")
    advisories: dict[str, int] = {}
    for key, pat in ADVISORY_PATTERNS.items():
        m = pat.search(out)
        if m:
            advisories[key] = int(m.group(1))
    return {
        "exit_code": proc.returncode,
        "passed": proc.returncode == 0,
        "advisories": advisories,
        "raw_output": out,
    }


def count_appendix_entries() -> dict[str, int]:
    c_text = strip_svg((BOOK_SRC / "appendix-c-sources.md").read_text(encoding="utf-8"))
    d_text = strip_svg(
        (BOOK_SRC / "appendix-d-claim-ledger.md").read_text(encoding="utf-8")
    )
    bib_entries = sum(1 for line in c_text.splitlines() if line.startswith("- **"))
    ledger_rows = sum(
        1
        for line in d_text.splitlines()
        if line.startswith("| ")
        and not line.startswith("| Claim |")
        and not line.startswith("| :---")
    )
    return {"appendix_c_entries": bib_entries, "appendix_d_rows": ledger_rows}


def cmd_baseline(args: argparse.Namespace) -> int:
    index = build_book_index()
    struct = run_structure_check()
    counts = count_appendix_entries()
    file_metrics = {
        name: {
            "prose_lines": meta["prose_lines"],
            "prose_words": meta["prose_words"],
            "unescaped_inline_subscript_risks": meta[
                "unescaped_inline_subscript_risks"
            ],
        }
        for name, meta in index.items()
    }
    payload = {
        "manuscript_files": len(index),
        "total_prose_lines": sum(m["prose_lines"] for m in index.values()),
        "total_prose_words": sum(m["prose_words"] for m in index.values()),
        **counts,
        "structure_passed": struct["passed"],
        "advisories": struct["advisories"],
        "files": file_metrics,
    }
    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(
            json.dumps(payload, indent=2, ensure_ascii=False), encoding="utf-8"
        )
    summary = {k: v for k, v in payload.items() if k != "files"}
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    if not struct["passed"]:
        print(
            "\n[FAIL] structure.py reported failures:\n" + struct["raw_output"],
            file=sys.stderr,
        )
        return 1
    return 0


def cmd_shards(args: argparse.Namespace) -> int:
    index = build_book_index()
    shards = build_shards(index)
    if args.format == "json":
        text = json.dumps(shards, indent=2, ensure_ascii=False)
    else:
        lines = [
            "# Stochology 9-Shard Federated Cartography Plan",
            "",
            "| Shard ID | Lead Guilds | Files | Prose Lines | Prose Words | Scope |",
            "| :--- | :--- | ---: | ---: | ---: | :--- |",
        ]
        for s in shards:
            lines.append(
                f"| `{s['id']}` | {s['guild_lead']} | {s['file_count']} | "
                f"{s['prose_lines']} | {s['prose_words']} | {s['scope']} |"
            )
        lines.append("")
        for s in shards:
            lines.append(f"## {s['id']} — {s['scope']}")
            lines.append(f"- **Lead Guilds:** {s['guild_lead']}")
            lines.append(
                f"- **Load:** {s['file_count']} files · {s['prose_lines']} prose lines · {s['prose_words']} words"
            )
            lines.append("- **Files:** " + ", ".join(f"`{f}`" for f in s["files"]))
            lines.append("")
        text = "\n".join(lines)
    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(text, encoding="utf-8")
    print(text)
    return 0


def cmd_graph(args: argparse.Namespace) -> int:
    index = build_book_index()
    jg = johnson_graph_part_v(index)
    layers = parse_appendix_c_layers()
    if args.chapter:
        target = resolve_chapter_arg(args.chapter)
        meta = index[target]
        ch_prefix = target[:2] if re.match(r"^\d{2}-", target) else None
        shared_layers = [
            l["layer"] for l in layers if ch_prefix and ch_prefix in l["chapters"]
        ]
        j_neighbors = [e for e in jg["edges"] if e["a"] == target or e["b"] == target]
        result = {
            "file": target,
            "title": meta["title"],
            "prose_lines": meta["prose_lines"],
            "prose_words": meta["prose_words"],
            "part_v": meta.get("part_v"),
            "outgoing_links": meta["outgoing_links"],
            "incoming_links": meta["incoming_links"],
            "appendix_c1_layers": shared_layers,
            "johnson_j7_3_neighbors": j_neighbors,
        }
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0

    summary = {
        "total_files": len(index),
        "total_directed_markdown_links": sum(
            len(m["outgoing_links"]) for m in index.values()
        ),
        "part_v_johnson_j7_3": {k: v for k, v in jg.items() if k != "edges"},
        "appendix_c1_layers": layers,
    }
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    return 0


DEFAULT_STATE_FILE = MAIN_ROOT / "research" / "runs" / "state.json"
VALID_WAVE_STATUSES = ("pending", "active", "clean", "edited", "escalated", "halted")
DEFAULT_ADVERSARIAL_MODEL = "gemini-4-hydrogen-high"
DEFAULT_PROPOSER_MODEL = None  # Omit `model` in swarm{action:"add"} to inherit active session model (Carbon)


def _chapter_number_from_file(filename: str) -> int | None:
    m = re.match(r"^(\d{2})-", filename)
    return int(m.group(1)) if m else None


def _body_link_counts(src_file: str) -> dict[str, int]:
    """Count markdown links inside src_file excluding the closing ## Bridge section."""
    raw_clean = strip_svg((BOOK_SRC / src_file).read_text(encoding="utf-8"))
    bridge_idx = raw_clean.find("\n## Bridge")
    body_no_bridge = raw_clean[:bridge_idx] if bridge_idx >= 0 else raw_clean
    counts: dict[str, int] = {}
    for _, tgt in MD_LINK_RE.findall(body_no_bridge):
        base = urllib.parse.unquote(tgt.split("#", 1)[0].strip())
        if base.endswith(".md") and (BOOK_SRC / base).exists() and base != src_file:
            counts[base] = counts.get(base, 0) + 1
    return counts


def plan_wave_footprint(
    index: dict[str, dict],
    jg: dict,
    edits: list[str],
    explicit_partners: set[str] | None = None,
    max_partners: int = 6,
) -> dict:
    """Compute ranked full-read core relation partners and peripheral consumers for an edit set."""
    explicit_partners = explicit_partners or set()
    scored_candidates: dict[str, tuple[int, str]] = {}
    downstream_consumers: set[str] = set()
    edits_has_appendix = any(e.startswith("appendix-") for e in edits)
    skip_prefixes = (
        ()
        if edits_has_appendix
        else ("appendix-c-", "appendix-d-", "appendix-f-", "appendix-g-")
    )
    body_cache: dict[str, dict[str, int]] = {}
    all_body_links: set[str] = set()

    def _link_mentions(src_file: str, dst_file: str) -> int:
        if src_file not in body_cache:
            body_cache[src_file] = _body_link_counts(src_file)
        return body_cache[src_file].get(dst_file, 0)

    for f in edits:
        if f not in body_cache:
            body_cache[f] = _body_link_counts(f)
        all_body_links |= {
            dst
            for dst in body_cache[f]
            if dst not in edits and not dst.startswith(skip_prefixes)
        }
        meta = index[f]
        outgoing = {
            x for x in meta["outgoing_links"] if not x.startswith(skip_prefixes)
        }
        incoming = {
            x for x in meta["incoming_links"] if not x.startswith(skip_prefixes)
        }

        for cand in outgoing | incoming:
            if cand in edits:
                continue
            out_cnt = _link_mentions(f, cand) if cand in outgoing else 0
            in_cnt = _link_mentions(cand, f) if cand in incoming else 0
            is_reciprocal = out_cnt > 0 and in_cnt > 0
            if is_reciprocal:
                score = 100 + 10 * (out_cnt + in_cnt)
                reason = "reciprocal_dialogue"
            elif out_cnt > 0:
                score = 40 + 10 * out_cnt
                reason = "outgoing_dependency"
            elif (
                len(incoming - outgoing) <= 6 or not outgoing or edits_has_appendix
            ) and in_cnt > 0:
                score = 20 + 5 * in_cnt
                reason = "direct_consumer"
            else:
                downstream_consumers.add(cand)
                continue
            if edits_has_appendix and cand.startswith("appendix-"):
                score += 15
            prev = scored_candidates.get(cand, (0, ""))
            if score > prev[0]:
                scored_candidates[cand] = (score, reason)

        if "part_v" in meta:
            anchor = meta["part_v"].get("primary_anchor")
            for edge in jg["edges"]:
                other = (
                    edge["b"]
                    if edge["a"] == f
                    else (edge["a"] if edge["b"] == f else None)
                )
                if other and other not in edits:
                    bonus = (
                        55
                        if edge["cross_linked_in_prose"]
                        else (35 if anchor and anchor in edge["shared_guards"] else 0)
                    )
                    if bonus > 0:
                        prev = scored_candidates.get(other, (0, ""))
                        if prev[0] + bonus > prev[0]:
                            scored_candidates[other] = (
                                prev[0] + bonus,
                                prev[1] or "johnson_j7_3_sibling",
                            )

    ranked = [
        cand
        for cand, _ in sorted(
            scored_candidates.items(),
            key=lambda kv: (-kv[1][0], -len(index[kv[0]]["incoming_links"]), kv[0]),
        )
    ]
    core_auto = ranked[:max_partners]
    overflow = ranked[max_partners:]

    partners = sorted((explicit_partners | set(core_auto)) - set(edits))
    consumers = sorted(
        (downstream_consumers | set(overflow)) - set(edits) - set(partners)
    )
    return {
        "edit_set": edits,
        "core_relation_partners_full_read": partners,
        "body_links": sorted(all_body_links),
        "secondary_interface_consumers_card_check": consumers,
        "core_wave_footprint_count": len(edits) + len(partners),
        "total_edit_prose_words": sum(index[f]["prose_words"] for f in edits),
        "total_partner_prose_words": sum(index[f]["prose_words"] for f in partners),
    }


def cmd_wave_plan(args: argparse.Namespace) -> int:
    index = build_book_index()
    jg = johnson_graph_part_v(index)
    edits = sorted(
        {resolve_chapter_arg(tok) for tok in args.edit.split(",") if tok.strip()}
    )
    if not (1 <= len(edits) <= 5):
        print(
            f"[WARN] Wave edit set has {len(edits)} file(s); standard wave size is 1–5 files.",
            file=sys.stderr,
        )

    explicit_partners = (
        {resolve_chapter_arg(tok) for tok in args.partners.split(",") if tok.strip()}
        if args.partners
        else set()
    )
    payload = plan_wave_footprint(
        index, jg, edits, explicit_partners=explicit_partners, max_partners=args.max_partners
    )

    collision_report = None
    if args.active_wave:
        other_footprint = {
            resolve_chapter_arg(tok)
            for tok in args.active_wave.split(",")
            if tok.strip()
        }
        our_footprint = set(edits) | set(payload["core_relation_partners_full_read"])
        overlap = sorted(our_footprint & other_footprint)
        collision_report = {
            "disjoint": len(overlap) == 0,
            "overlapping_files": overlap,
        }

    payload["concurrent_wave_check"] = collision_report
    print(json.dumps(payload, indent=2, ensure_ascii=False))
    if collision_report and not collision_report["disjoint"]:
        return 2
    return 0


def cmd_wave_audit(args: argparse.Namespace) -> int:
    targets = sorted(
        {resolve_chapter_arg(tok) for tok in args.files.split(",") if tok.strip()}
    )
    baseline_data = None
    if args.baseline:
        bpath = Path(args.baseline)
        if not bpath.exists():
            print(
                f"[FAIL] Baseline file not found: {bpath}. Pass the absolute path from the main repository (<MAIN_ROOT>/research/runs/baseline.json).",
                file=sys.stderr,
            )
            return 1
        baseline_data = json.loads(bpath.read_text(encoding="utf-8"))

    struct = run_structure_check()
    deltas: list[dict] = []
    warnings: list[str] = []
    for t in targets:
        raw_t = (BOOK_SRC / t).read_text(encoding="utf-8")
        census_items = file_numeral_census(raw_t)
        meta = inspect_file(t)
        entry: dict = {
            "file": t,
            "prose_lines": meta["prose_lines"],
            "prose_words": meta["prose_words"],
            "unescaped_inline_subscript_risks": meta[
                "unescaped_inline_subscript_risks"
            ],
            "numeral_census_count": len(census_items),
            "numeral_census_lines": census_items,
        }
        if meta["unescaped_inline_subscript_risks"] > 0:
            warnings.append(
                f"{t}: {meta['unescaped_inline_subscript_risks']} inline math span(s) carry unescaped '_' pairs (escape as '\\_' for CommonMark safety)"
            )
        if baseline_data and t in baseline_data.get("files", {}):
            base_words = baseline_data["files"][t]["prose_words"]
            diff_words = meta["prose_words"] - base_words
            pct = (100.0 * diff_words / base_words) if base_words else 0.0
            entry["word_delta"] = diff_words
            entry["word_growth_pct"] = round(pct, 2)
            if pct > args.max_growth_pct:
                warnings.append(
                    f"[SWARM BRAKE 2 — ANTI-BLOAT] {t} grew by +{diff_words} words (+{pct:.1f}% > {args.max_growth_pct:.1f}%). Verify subtraction-first discipline!"
                )
        deltas.append(entry)

    advisory_regressions: list[str] = []
    if baseline_data:
        base_adv = baseline_data.get("advisories", {})
        for k, cur_val in struct["advisories"].items():
            if k == "numeral_census_candidates":
                continue
            prev_val = base_adv.get(k, 0)
            if cur_val > prev_val:
                advisory_regressions.append(f"{k}: {prev_val} -> {cur_val}")

    report = {
        "structure_passed": struct["passed"],
        "advisory_regressions": advisory_regressions,
        "warnings": warnings,
        "audited_files": deltas,
    }
    print(json.dumps(report, indent=2, ensure_ascii=False))
    if not struct["passed"] or advisory_regressions:
        if not struct["passed"]:
            print("\n" + struct["raw_output"], file=sys.stderr)
        return 1
    return 0


def run_step(
    label: str, cmd: list[str], env: dict[str, str] | None = None
) -> tuple[bool, str]:
    proc = subprocess.run(cmd, cwd=ROOT, env=env, capture_output=True, text=True)
    out = (proc.stdout or "") + ("\n" + proc.stderr if proc.stderr else "")
    return (
        proc.returncode == 0,
        f"=== {label} (exit {proc.returncode}) ===\n{out.strip()}",
    )


def cmd_gates(args: argparse.Namespace) -> int:
    results: list[dict] = []
    all_ok = True

    ok, log = run_step(
        "structure.py", [sys.executable, str(CHECKS_DIR / "structure.py")]
    )
    results.append({"gate": "structure.py", "passed": ok})
    all_ok = all_ok and ok
    if not ok:
        print(log, file=sys.stderr)

    if args.full_render:
        ok_mb, log_mb = run_step("mdbook build", ["mdbook", "build"])
        results.append({"gate": "mdbook build", "passed": ok_mb})
        all_ok = all_ok and ok_mb
        if not ok_mb:
            print(log_mb, file=sys.stderr)
        else:
            ok_sf, log_sf = run_step(
                "surface.py (§V-8)",
                [sys.executable, str(CHECKS_DIR / "surface.py"), "book"],
            )
            results.append({"gate": "surface.py", "passed": ok_sf})
            all_ok = all_ok and ok_sf
            if not ok_sf:
                print(log_sf, file=sys.stderr)

            if args.with_pdf:
                ok_pdf, log_pdf = run_step(
                    "build_pdf.py",
                    [
                        sys.executable,
                        str(CHECKS_DIR / "build_pdf.py"),
                        "book",
                        "book/stochology.pdf",
                    ],
                )
                results.append({"gate": "build_pdf.py", "passed": ok_pdf})
                all_ok = all_ok and ok_pdf
                if not ok_pdf:
                    print(log_pdf, file=sys.stderr)

            summary_entries = sum(
                1
                for line in (BOOK_SRC / "SUMMARY.md")
                .read_text(encoding="utf-8")
                .splitlines()
                if line.startswith("- [")
            )
            html_pages = len(list((ROOT / "book").rglob("*.html")))
            ok_pages = html_pages >= summary_entries
            results.append(
                {
                    "gate": "summary_page_parity",
                    "passed": ok_pages,
                    "summary_entries": summary_entries,
                    "html_pages": html_pages,
                }
            )
            all_ok = all_ok and ok_pages

    if args.with_core:
        disk_tmp_root = ROOT / "target" / ".rs-tmp"
        disk_tmp_root.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=disk_tmp_root) as tmpdir:
            env = os.environ.copy()
            env["TMPDIR"] = tmpdir
            for label, cmd in [
                ("cargo test --offline", ["cargo", "test", "--offline"]),
                (
                    "cargo test --doc --offline -p stoch-core",
                    ["cargo", "test", "--doc", "--offline", "-p", "stoch-core"],
                ),
                (
                    "wasm_gates.py",
                    [
                        sys.executable,
                        str(CHECKS_DIR / "wasm_gates.py"),
                        "--tmpdir",
                        tmpdir,
                    ],
                ),
            ]:
                ok_c, log_c = run_step(label, cmd, env=env)
                results.append({"gate": label, "passed": ok_c})
                all_ok = all_ok and ok_c
                if not ok_c:
                    print(log_c, file=sys.stderr)

    print(json.dumps({"all_passed": all_ok, "gates": results}, indent=2))
    return 0 if all_ok else 1


STAGE_NAMES: dict[int, str] = {
    1: "Estágio 1 — Fundações, Hubs & Framework (Partes III–IV: Caps. 27–50)",
    2: "Estágio 2 — Bases Disciplinares (Parte II: Caps. 09–26)",
    3: "Estágio 3 — Prólogo & Entrada Intuitiva (Prólogo & Parte I: Caps. 00–08)",
    4: "Estágio 4 — Arquitetura Multicamadas (Parte V — 35 Ordens: Caps. 51–85)",
    5: "Estágio 5 — Trajetória Laminar & Alvo Sobreposto (Partes VI–VII: Caps. 86–99)",
    6: "Estágio 6 — Molduras, Apêndices & Fechamento (W100–W111)",
}

# Tuple schema: (wave_id, stage, stage_order, files, pair1_domain, pair2_rigor, pair3_arch)
WAVE_CATALOG_SPECS: list[tuple[str, int, int, list[str], str, str, str]] = [
    # --- Stage 1: Architectural Hubs & Foundations (Part III & IV: 27–50) ---
    ("W029", 1, 1, ["29-aim-and-the-three-planes.md"],
     "Filosofia da Ação, Intencionalidade & Arquitetura dos Três Planos",
     "Teoria dos Circuitos de Mediação & Mecanismos de Dados Faltantes (MNAR/MAR)",
     "Hub Central da Obra (Circuito Mundo–Arquivo–Instituição & 7 Sítios)"),
    ("W041", 1, 2, ["41-the-triad-and-the-minimum.md"],
     "Escolha Social, Complementaridade de Leontief & Regimes de Ação",
     "Invariância Ordinal de Caixa, Geometria Jacobiana & Terminação Reflexiva",
     "Hub da Tríade Nuclear, 3 Proposições de Colapso & Não-Compensação"),
    ("W050", 1, 3, ["50-methodological-families-reflexivity.md"],
     "Síntese Metodológica, Process Tracing & Cibernética Reflexiva",
     "Evidência Keynesiana (V), Semiótica Galênica & 5 Falsificadores Pré-Registrados",
     "Hub dos 4 Freios Reflexivos, Anti-Inchaço & Autoaplicação da Tríade"),
    ("W027", 1, 4, ["27-what-judging-is.md"],
     "Filosofia Antiga do Stókhos, Métron & Ética do Julgamento",
     "Filologia Clássica (Platão/Filebo, Galeno, Hipócrates, Antípatro) & Taxonomia da Incerteza",
     "Recusas Fundamentais (Cap. 27/40) & Ponte para Crítica à Máquina"),
    ("W028", 1, 5, ["28-the-machine-and-the-deception.md"],
     "Epistemologia da IA, Teoria Crítica & Crítica ao Hype Computacional",
     "Computabilidade (Minsky–Papert), Positivismo Lógico (Carnap) & Ruína Atuarial (Cramér–Lundberg)",
     "Filtro das 3 Perguntas, Desmistificação AGI/ASI & Voz Ensaística"),
    ("W030", 1, 6, ["30-probability-as-language.md"],
     "Fundamentos da Probabilidade, Epistemologia Bayesiana/Imprecisa & Semântica",
     "Axiomas de Kolmogorov/Cox, e-Processos & Validade Anytime (Ville)",
     "Escalas de Medida, Penumbra Dialética & Eixo Epistêmico T1–T2"),
    ("W031", 1, 7, ["31-distribution-and-moment.md"],
     "Teoria das Distribuições, Estimação de Caudas & Riscos Competitivos",
     "Estimador de Hill, Pickands–Balkema–de Haan, CRPS & Desigualdade de Jensen",
     "Subdeterminação de Momentos & Transição Epistêmico–Vital"),
    ("W032", 1, 8, ["32-the-question-before-the-data.md"],
     "Metodologia Científica, Enquadramento de Problemas & Regra de Cromwell",
     "Deslocamentos de Lakatos, Discrepância Kennedy–O'Hagan & Análise Multiverso",
     "Sítio T1 (Pre-Draw Framing), Erro Tipo III & Pré-Registro"),
    ("W033", 1, 9, ["33-measuring-and-erring.md"],
     "Metrologia Científica, Teoria Representacional da Medida & Erro Instrumental",
     "Rastreabilidade GUM, Escalas de Stevens & Testes de Invariância Não-Paramétrica",
     "Sítio T1/T3 (Silêncio != Zero) & Lastro Empírico em Medição"),
    ("W034", 1, 10, ["34-sample-population-archive.md"],
     "Amostragem Complexa, Arquivística Quantitativa & Inferência Populacional",
     r"Paradoxo do Big Data de Meng (`\rho_{R,Y}`), DEFF de Kish–Moulton & Arquivos Sombrios",
     "Sítio T3 (Mundo -> Arquivo) & Colapso 1 (Blind Undersizing)"),
    ("W035", 1, 11, ["35-inference-and-evidence.md"],
     "Inferência Estatística, Epigrafia Forense & Padrões Probatórios",
     "Severidade Estatística de Mayo, Limites de Manski & Largura Intervalar",
     "Sítio T2 (Severidade vs. Significância) & Subdeterminação Evidencial"),
    ("W036", 1, 12, ["36-decision-loss-threshold.md"],
     "Teoria da Decisão Estatística, Economia Médica & Limiares Normativos",
     "Limiar de Pauker–Kassirer, Minimax Regret, EVPI & Gargalo Informacional",
     "Partição em 3 Regimes de Ação, Fadiga de Alerta & Ponto Fixo"),
    ("W037", 1, 13, ["37-model-fit-and-error.md"],
     "Aprendizado Estatístico, Validação Tabular & Modelagem Preditiva",
     "Erro de Aproximação vs. Estimação, Vazamento de Dados & Calibração Isotônica",
     "Ponte para Performatividade, Discrepância Estrutural & Anti-Overfitting"),
    ("W038", 1, 14, ["38-time-and-dependence.md"],
     "Física Estatística Aplicada, Dinâmica Não-Ergódica & Teoria de Filas",
     "Cálculo de Itô (Dreno de Variância), Ruína Absorvente & Hipérbole de Kingman",
     "Normatividade Vital (Canguilhem), Bacias de Holling & TTS >= TTR"),
    ("W039", 1, 15, ["39-cause-intervention-counterfactual.md"],
     "Inferência Causal, Epistemologia das Virtudes & Justiça Algorítmica",
     "Hierarquia de Pearl, ICP, Limites de Tian–Pearl & Teorema de Chouldechova",
     "Sosa AAA, Injustiça Epistêmica (Fricker) & Atribuição vs. Intervenção"),
    ("W040", 1, 16, ["40-institution-incentive-metric.md"],
     "Economia Institucional, Desenho de Mecanismos & Sociologia das Métricas",
     "Variantes de Goodhart, Process Tracing Bayesiano & Paradoxos Deliberativos",
     "Recusas Éticas (Cap. 40), Canal Algedônico S3* & Cofre Temporal"),
    ("W042", 1, 17, ["42-sequential-loop-naive-agent.md"],
     "Controle Sequencial, Governança de Comuns & Crítica à Otimização Ingênua",
     "Formulação do Caso 'The Community and the Reserve' & Dinâmica de Barreira",
     "Abertura do Caso da Parte IV & Contraste Agente Ingênuo vs. Triádico"),
    ("W043", 1, 18, ["43-inquiry-openness-multiverse.md"],
     "Epistemologia da Investigação, Objetividade Forte & Risco Indutivo",
     "Procedimento e-BH, Curva de Especificação (Multiverso) & Erro Tipo III de Kimball",
     "Filtro I (T1 — Abertura Antidogmática) & Precificação Custo–Perda"),
    ("W044", 1, 19, ["44-underdetermination-partial-identification.md"],
     "Econometria de Identificação Parcial & Avaliação de Políticas (LaLonde)",
     "e-Processos, Razão de Lakatos & Limites de Manski, Lee, Imbens–Manski, Peterson",
     "Filtro I (T2 — Severidade & Largura nas Duas Pontas)"),
    ("W045", 1, 20, ["45-world-to-archive-big-data-paradox.md"],
     "Estatística Oficial, Amostragem de Redes & Arquivística Bitemporal",
     "Prova da Identidade de Meng (2018), DEFF Aditivo, Horvitz–Thompson & Cota de Chao (1984)",
     "Filtro I (T3 — Safras Bitemporais `t_ev` vs. `t_disp` & Fração Amostral)"),
    ("W046", 1, 21, ["46-non-identical-asymmetry-heavy-tails.md"],
     "Teoria do Risco Extremo, Aviação/Confiabilidade & Medição Não-Idêntica",
     "Escalas de Stevens, Gap de Jensen, Decomposição CRPS & Índice de Hill (1975)",
     "Filtro II (T4 — Penumbra Dialética & Regimes de Momentos)"),
    ("W047", 1, 22, ["47-irreversible-time-vital-margin.md"],
     "Fisiologia Alostática, Dinâmica de Redes/Mercados & Pesquisa Operacional",
     "Shrinkage Kelly (Baker–McHale), Solver de Wald (1943), Sniedovich & Filas de Kingman",
     "Filtro II (T5 — Canguilhem/Holling/McEwen & Desigualdade TTS vs. TTR)"),
    ("W048", 1, 23, ["48-reactive-worlds-performativity-goodhart.md"],
     "Sociologia Econômica (Performatividade), Previsão Reativa & Ciência Atmosférica",
     "Decomposição CORP-PAV, ICP, Kennedy–O'Hagan, Tian–Pearl & Pontos Fixos `θ_PS` vs. `θ_PO`",
     "Filtro III (T6 — Goodhart em Espaço Latente & Fechamento Reativo)"),
    ("W049", 1, 24, ["49-moral-luck-apt-agency-institutions.md"],
     "Filosofia Moral, Teoria Organizacional & Neurociência da Atenção",
     "Sosa (Aptidão), List–Pettit, Innes–Holmström–Milgrom, Chouldechova & 10 bits/s (Zheng–Meister)",
     "Filtro III (T7 — Sorte Moral, Canal Algedônico S3* & Desconto Testemunhal)"),

    # --- Stage 2: Part II Disciplinary Bases (09–26) ---
    ("W009", 2, 1, ["09-sets-relations-functions.md"],
     "Teoria dos Conjuntos & Topologia Mensurável",
     "Sistemas Axiomáticos (Kolmogorov/Cox) & Pipelines Funcionais",
     "Base Epistêmica T1/T3 & Ligação Parte III"),
    ("W010", 2, 2, ["10-numbers-order-bounds.md"],
     "Análise Real, Teoria da Ordem & Completude",
     "Aritmética Intervalar, Identificação Parcial & Índice de Cauda",
     "Intervalos como Resposta Honesta (T2/T4)"),
    ("W011", 2, 3, ["11-sequences-limits-continuity.md"],
     "Análise Real, Convergência & Sistemas Dinâmicos",
     "Continuidade Uniforme vs. Robustez & Dreno de Variância de Itô",
     "Base Matemática da Não-Ergodicidade (T4/T5)"),
    ("W012", 2, 4, ["12-derivatives-and-convexity.md"],
     "Cálculo Diferencial, Otimização & Análise Convexa",
     "Expansão de Taylor da Lacuna de Jensen & Curvatura de Filas",
     "Base da Assimetria Vital (T4/T5)"),
    ("W013", 2, 5, ["13-integrals-and-accumulation.md"],
     "Teoria da Medida & Integração de Lebesgue",
     "Troca Limite–Integral sob Caudas Pesadas & ES/CVaR Coerente",
     "Base de Acumulação, Ruína & Pior Caso"),
    ("W014", 2, 6, ["14-linear-algebra.md"],
     "Álgebra Linear, Geometria de Alta Dimensão & GF(2)",
     "Rank–Nullity, Matriz de Gram, Projeção & Não-Afinidade de Paridade",
     "Base de Projeção, Concentração & Limite Perceptron"),
    ("W015", 2, 7, ["15-probability-as-a-measure.md"],
     "Teoria da Probabilidade & Martingales",
     "Espaços `(Ω, F, P)`, Desigualdade de Ville & Classificação Estratégica",
     "Base Formal T2/T6"),
    ("W016", 2, 8, ["16-what-knowledge-is.md"],
     "Epistemologia Analítica Contemporânea",
     "Análise Tripartida, Casos Gettier & Epistemologia da Virtude",
     "Base Filosófica de Agência Apta & Erro"),
    ("W017", 2, 9, ["17-justification-and-induction.md"],
     "Lógica da Ciência, Indução & Agnotologia",
     "Hume, Enigma de Goodman, Abdução de Peirce & Dúvida Estratégica",
     "Base Filosófica da Margem Epistêmica (T1/T2)"),
    ("W018", 2, 10, ["18-philosophy-of-science.md"],
     "Filosofia da Ciência & Sociologia do Conhecimento",
     "Popper, Duhem–Quine, Lakatos, Mayo & Injustiça Testemunhal",
     "Base da Severidade & Demarcação"),
    ("W019", 2, 11, ["19-ethics-and-the-imposition-of-risk.md"],
     "Ética Normativa, Risco & Contratualismo",
     "Risco Não-Recíproco, Scanlon, Sorte Moral & Performatividade (MacKenzie)",
     "Base Ética de T6/T7 & Bacia de Recepção"),
    ("W020", 2, 12, ["20-legitimacy-and-institutions.md"],
     "Filosofia Política, Direito Constitucional & Cibernética",
     "Contestabilidade (Pettit), Algedônico (Beer) & Precaução (Jonas)",
     "Base Política T7 & Recusas Institucionais"),
    ("W021", 2, 13, ["21-mechanics-determinism-chaos.md"],
     "Mecânica Clássica, Caos Determinístico & Termodinâmica da Informação",
     "Expoente de Lyapunov, Princípio de Landauer & Instabilidade por Atraso",
     "Base Física contra Certeza Pontual e Fatalismo"),
    ("W022", 2, 14, ["22-thermodynamics-and-entropy.md"],
     "Termodinâmica Clássica & de Não-Equilíbrio",
     "Leis de Clausius/Conservação & Estruturas Dissipativas de Prigogine",
     "Base Física da Dissipação Administrativa & Margem Vital"),
    ("W023", 2, 15, ["23-statistical-physics-and-time.md"],
     "Física Estatística & Ecologia de Sistemas",
     "Quebra da Hipótese Ergódica, Escala & Bacias de Holling",
     "Base Física–Ecológica de Resiliência & Governança"),
    ("W024", 2, 16, ["24-chemical-rates-equilibrium-buffers.md"],
     "Cinética Química, Bioquímica & Tampões Fisiológicos",
     "Revisão Cinética, Le Chatelier, Tampão de Van Slyke & Saturação",
     "Isomorfismo Tampão–Fila & Normatividade de Canguilhem"),
    ("W025", 2, 17, ["25-evolution-variation-selection.md"],
     "Biologia Evolutiva & Cibernética de Sistemas",
     "Deriva Neutra (Kimura), Lei de Ashby & Seleção Artificial como Goodhart",
     "Base Evolutiva de Variedade Requisita & Conjectura Aberta"),
    ("W026", 2, 18, ["26-regulation-and-homeostasis.md"],
     "Fisiologia da Regulação, Controle & Teoria de Filas",
     "Alostase, Canalização, Aproximação de Kingman & Desigualdade TTS/TTR",
     "Base Biológica–Cibernética da Margem Vital (T5)"),

    # --- Stage 3: Prologue & Part I (00–08) ---
    ("W000", 3, 1, ["00-preface.md"],
     "Filosofia Política Crítica, Ética do Alvo & Crítica Cultural",
     "Filologia & Textos Primários (Cícero/Estobeu/Sófocles, Marx, Adorno, Espinosa, Canguilhem)",
     "Prólogo Programático (Espelhos, Ar Viciado, Elo Mais Fraco & Voz)"),
    ("W001", 3, 2, ["01-deciding-without-knowing.md"],
     "Tomada de Decisão Sob Incerteza (Nível Introdutório)",
     "Estrutura do Problema do Guarda-Chuvas & Ação Pré-Desfecho",
     "Entrada Intuitiva & Ponte para Média vs. Cauda"),
    ("W002", 3, 3, ["02-the-average-is-not-the-story.md"],
     "Estatística Descritiva & Risco Sistêmico Intuitivo",
     "Contraexemplo 99×1+1001, Mediana & Desigualdade de Jensen (1906)",
     "Ponte para Caudas Pesadas & Big Data"),
    ("W003", 3, 4, ["03-the-biggest-number-in-the-room.md"],
     "Gestão de Reservas & Eventos Extremos",
     "Desigualdade de Sobrevivência & Falácia do 'Nunca Aconteceu'",
     "Motivação Intuitiva da Margem Vital"),
    ("W004", 3, 5, ["04-it-worked-so-i-was-right.md"],
     "Psicologia do Julgamento & Avaliação Institucional",
     "Viés de Resultado, Viés de Sobrevivência & Agência Apta",
     "Distinção Tiro vs. Acerto & Ponte Epistemológica"),
    ("W005", 3, 6, ["05-what-a-number-doesnt-say.md"],
     "Comunicação de Risco & Numeramento Médico",
     "Frequências Naturais (Gigerenzer), Precisão vs. Exatidão & Fadiga de Alerta",
     "Semente de T1/T3 & Intervalos de Confiança"),
    ("W006", 3, 7, ["06-who-is-missing-from-the-table.md"],
     "Vieses Amostrais & Memória Institucional",
     "Blindagem Faltante de Wald, Paradoxo de Meng & 3 Formas de Vazio",
     "Introdução do Circuito de Mediação e dos 7 Sítios"),
    ("W007", 3, 8, ["07-when-the-measure-becomes-the-goal.md"],
     "Sociologia das Métricas & Economia das Metas",
     "Lei de Goodhart, Previsão Performativa & Colapso 1",
     "Semente de T6 & Crítica à Supressão Ingênua de Métricas"),
    ("W008", 3, 9, ["08-three-things-you-cant-trade.md"],
     "Teoria Integrada da Tríade Nuclear (Nível I)",
     "Mínimo de Leontief (Não-Substituibilidade) & 3 Colapsos",
     "Síntese da Parte I & Regra de Reflexividade"),

    # --- Stage 4: Part V — 35 Orders in Strata I–VII (51–85) ---
    ("W051", 4, 1, ["51-layer-micro-kinetic-sensorimotor-order.md"],
     "Controle Motor, Biomecânica & Ergonomia Sensorimotora",
     "Equações de Controle em Malha, Dois Relógios & Carga Superficial",
     "`L01 · S:I · {T1,T5,T7} · (1,1,1)` & Irmãos `J(7,3)`"),
    ("W052", 4, 2, ["52-layer-meso-buffered-queue-order.md"],
     "Engenharia de Tráfego, Filas & Logística de Buffers",
     "Estocástica de Chegadas Performativas & Dimensionamento de Buffer",
     "`L02 · S:I · {T4,T6,T7} · (0,1,2)` & Exposição Epistêmica"),
    ("W053", 4, 3, ["53-layer-macro-coupled-topology-order.md"],
     "Redes Complexas, Energia Elétrica & Topologia Acoplada",
     "Dinâmica de Cascata em Grafos & Corrida Propagação vs. Resposta",
     "`L03 · S:I · {T4,T5,T6} · (0,2,1)` & Exposição Epistêmica"),
    ("W054", 4, 4, ["54-layer-geophysical-chaos-data-assimilation-order.md"],
     "Dinâmica de Fluidos Geofísicos & Assimilação de Dados",
     "Caos de Lorenz, Fechamento Subgrade & Correção Logarítmica",
     "`L04 · S:I · {T3,T4,T5} · (1,2,0)` & Exposição de Governança"),
    ("W055", 4, 5, ["55-layer-continuum-fracture-structural-order.md"],
     "Mecânica dos Sólidos, Fratura & Engenharia Estrutural",
     "Limiar de Trinca, Dobra Populacional & Margem Assumida por Terceiros",
     "`L05 · S:I · {T2,T4,T7} · (1,1,1)` & Pedra Angular `S:I`"),
    ("W056", 4, 6, ["56-layer-formal-computable-type-order.md"],
     "Lógica Linear, Teoria de Tipos & Verificação Formal",
     "Sistemas de Tipos Afins, Cobertura Sintática & Expiração de Provas",
     "`L06 · S:II · {T1,T4,T5} · (1,2,0)` & Eixo de Governança Vazio"),
    ("W057", 4, 7, ["57-layer-probability-credence-measurement-order.md"],
     "Teoria da Decisão, Credências Imprecisas & Metrologia",
     "Conjuntos Credais, Distância Evidencial & Compromisso de Escala",
     "`L07 · S:II · {T1,T2,T4} · (2,1,0)` & Exposição de Governança"),
    ("W058", 4, 8, ["58-layer-metrological-information-geometric-order.md"],
     "Metrologia Física & Geometria da Informação",
     "Incerteza Tipo A/B (GUM), Variedade Fisher–Rao & Projeções de Bregman",
     "`L08 · S:II · {T3,T4,T6} · (1,1,1)` & Limites da Soma"),
    ("W059", 4, 9, ["59-layer-experimental-sampling-causal-order.md"],
     "Desenho Experimental, Amostragem Complexa & Causalidade",
     "Amostragem de Pesquisas, Causalidade Experimental & Duas Populações",
     "`L09 · S:II · {T1,T2,T3} · (3,0,0)` & Ruína do Metodólogo Puro"),
    ("W060", 4, 10, ["60-layer-cryptographic-zero-knowledge-adversarial-order.md"],
     "Criptografia Aplicada, Provas Zero-Knowledge & Protocolos",
     "Aritmética Modular, Solidez como e-Processo & Selo de Compromisso",
     "`L10 · S:II · {T2,T6,T7} · (1,0,2)` & Exposição Vital"),
    ("W061", 4, 11, ["61-layer-biochemical-kinetic-molecular-order.md"],
     "Cinética Bioquímica, Enzimologia & Química Analítica",
     "Taxas Irreversíveis, Revisão Cinética & Leitura Performativa",
     "`L11 · S:III · {T2,T5,T6} · (1,1,1)` & Substrato Vivo"),
    ("W062", 4, 12, ["62-layer-morphogenetic-developmental-order.md"],
     "Biologia do Desenvolvimento & Morfogênese",
     "Gradientes Morfogenéticos, Limiares de Tecido & Memória Mantida",
     "`L12 · S:III · {T1,T5,T6} · (1,1,1)` & Forma e Barreira"),
    ("W063", 4, 13, ["63-layer-immunological-self-governance-order.md"],
     "Imunologia Celular/Sistêmica & Autoimunidade",
     "Reconhecimento de Dois Sinais, Amostragem Somática & Tempo de Resposta",
     "`L13 · S:III · {T2,T5,T7} · (1,1,1)` & Auto-Governança sem 2º Canal"),
    ("W064", 4, 14, ["64-layer-taxonomy-systematics-ontology-order.md"],
     "Taxonomia Biológica, Sistemática Filogenética & Ontologia",
     "Tipificação por Holótipo, Caracteres & Deriva Nomenclatural",
     "`L14 · S:III · {T1,T3,T4} · (2,1,0)` & Exposição de Governança"),
    ("W065", 4, 15, ["65-layer-stratigraphy-paleobiology-deep-time-order.md"],
     "Estratigrafia, Geocronologia & Paleobiologia",
     "Preservação Taphonômica, Datação com Erro & Viés de Intervalo",
     "`L15 · S:III · {T1,T3,T5} · (2,1,0)` & Ofício Ausente"),
    ("W066", 4, 16, ["66-layer-pharmacological-clinical-diagnostic-order.md"],
     "Farmacologia Clínica, Ensaios & Medicina Diagnóstica",
     "Curvas Dose–Resposta, Severidade em Nulos & Janela Terapêutica",
     "`L16 · S:IV · {T2,T4,T5} · (1,2,0)` & Exposição de Governança"),
    ("W067", 4, 17, ["67-layer-epidemiology-outbreak-surveillance-order.md"],
     "Epidemiologia de Doenças Infecciosas & Filodinâmica",
     "Razão de Relógios, Subnotificação & Definição de Caso",
     "`L17 · S:IV · {T2,T3,T5} · (2,1,0)` & Fronteira do Arquivo"),
    ("W068", 4, 18, ["68-layer-pharmacovigilance-regulatory-toxicology-order.md"],
     "Farmacovigilância, Toxicologia Regulatória & Pós-Mercado",
     "Regra dos Três, Validade Anytime & Sinais Adversos Pós-Aprovação",
     "`L18 · S:IV · {T2,T3,T7} · (2,0,1)` & Veto Antes do Sinal Forte"),
    ("W069", 4, 19, ["69-layer-metascience-scientometrics-replication-order.md"],
     "Metaciência, Cienciometria & Meta-Análise",
     "Viés de Publicação, Supercorreção & Controle sob Dependência/Peeking",
     "`L19 · S:IV · {T2,T3,T6} · (2,0,1)` & Paradoxo da Replicação"),
    ("W070", 4, 20, ["70-layer-acoustic-aesthetic-tragic-pedagogical-order.md"],
     "Acústica/Teoria Musical, Tragédia Ática & Pedagogia/Psicometria",
     "Coma Pitagórica, Sófocles/Aristóteles & Validade Educacional (Chevallard, Messick)",
     "`L20 · S:IV · {T1,T4,T6} · (1,1,1)` & Formação e Resíduo Não-Abolível"),
    ("W071", 4, 21, ["71-layer-linguistic-evidential-pragmatic-order.md"],
     "Linguística Formal, Evidencialidade & Pragmática",
     "Marcadores Evidenciais Gramaticais, Atos de Fala & Responsabilidade",
     "`L21 · S:V · {T3,T6,T7} · (1,0,2)` & Circuito no Enunciado"),
    ("W072", 4, 22, ["72-layer-philological-epigraphic-textual-order.md"],
     "Filologia Clássica, Epigrafia & Crítica Textual",
     "Estemática (Recensio), Testemunhos & Escala de Severidade Filológica",
     "`L22 · S:V · {T2,T3,T4} · (2,1,0)` & Sítio de Governança Ausente"),
    ("W073", 4, 23, ["73-layer-taphonomic-historiographical-order.md"],
     "Historiografia, Tafonomia Documental & História Quantitativa",
     "Viés de Sobrevivência Documental, Taxas por Janela & Resíduo",
     "`L23 · S:V · {T3,T5,T7} · (1,1,1)` & Responsabilidade Historiográfica"),
    ("W074", 4, 24, ["74-layer-critical-archival-transitional-justice-order.md"],
     "Arquivística Crítica, Justiça de Transição & Testemunho",
     "Enquadramento de Dano, Arquivos Subalternos & Direito de Contestar",
     "`L24 · S:V · {T1,T3,T7} · (2,0,1)` & Margem Vital Ausente"),
    ("W075", 4, 25, ["75-layer-media-ecology-information-retrieval-order.md"],
     "Ecologia da Mídia, Recuperação da Informação & Rankings",
     "Métricas de Ranking Performativas, Retenção de Corpus & Seleção",
     "`L25 · S:V · {T1,T3,T6} · (2,0,1)` & Circuito da Busca"),
    ("W076", 4, 26, ["76-layer-sequential-decision-bandit-real-options-order.md"],
     "Teoria da Decisão Sequencial, Bandits & Opções Reais",
     "Parada Ótima, Cotas de Bandits & Não-Permutabilidade de Arquivos",
     "`L26 · S:VI · {T1,T2,T5} · (2,1,0)` & Limite da Assinatura"),
    ("W077", 4, 27, ["77-layer-game-theory-strategic-adversarial-order.md"],
     "Teoria dos Jogos, Classificação Estratégica & ML Adversarial",
     "Equilíbrios de Stackelberg & Superfície de Manipulação",
     "`L27 · S:VI · {T1,T2,T6} · (2,0,1)` & Adversário Interno"),
    ("W078", 4, 28, ["78-layer-macro-finance-econometrics-actuarial-order.md"],
     "Macro-Finanças, Econometria & Teoria Atuarial da Ruína",
     "Cramér–Lundberg, Medidas Coerentes & Risco Endógeno de Caudas",
     "`L28 · S:VI · {T2,T4,T6} · (1,1,1)` & Pedra Angular `S:VI`"),
    ("W079", 4, 29, ["79-layer-official-statistics-forensic-accounting-order.md"],
     "Estatísticas Oficiais, Contabilidade Forense & Contas Nacionais",
     "Lei de Benford, Partidas Dobradas, Arquivos Sombrios & Índices de Bem-Estar",
     "`L29 · S:VI · {T3,T4,T7} · (1,1,1)` & Conservação Contábil"),
    ("W080", 4, 30, ["80-layer-corporate-governance-principal-agent-order.md"],
     "Governança Corporativa, Teoria Principal–Agente & Mecanismos",
     "Contratos Multitarefa, Responsabilidade Limitada & Restrições de Relato",
     "`L30 · S:VI · {T1,T6,T7} · (1,0,2)` & Canibalização de Incentivos"),
    ("W081", 4, 31, ["81-layer-criminal-proof-forensic-epistemology-order.md"],
     "Processo Penal, Padrões de Prova & Epistemologia Forense",
     "Razões de Perda, Falácia do Promotor & Arquivos de Condenação",
     "`L31 · S:VII · {T1,T2,T7} · (2,0,1)` & Decisão Irreversível Penal"),
    ("W082", 4, 32, ["82-layer-evidentiary-tort-constitutional-legal-order.md"],
     "Direito Civil Probatório, Responsabilidade Civil & Constitucional",
     "Três Padrões Probatórios, Não-Agregação de Classes & Disjuntores Legais",
     "`L32 · S:VII · {T1,T4,T7} · (1,1,1)` & Dois Circuitos da Culpa"),
    ("W083", 4, 33, ["83-layer-distributed-consensus-sre-order.md"],
     "Sistemas Distribuídos, Consenso Tolerante a Falhas & SRE",
     "Protocolos de Quórum, Amplificação de Cauda & Orçamentos de Erro em Série",
     "`L33 · S:VII · {T5,T6,T7} · (0,1,2)` & Disjuntores Operacionais"),
    ("W084", 4, 34, ["84-layer-infrastructure-nuclear-hydraulic-safety-order.md"],
     "Engenharia de Alta Periculosidade, Segurança Nuclear & Hidráulica",
     "Persistência, Borda Livre (Freeboard), Defesa em Profundidade & Causa Comum",
     "`L34 · S:VII · {T4,T5,T7} · (0,2,1)` & Aritmética da Tempestade"),
    ("W085", 4, 35, ["85-layer-polycentric-commons-urban-biospheric-order.md"],
     "Governança de Comuns (Ostrom), Filosofia Política (Scott/Kropotkin) & Metabolismo Urbano",
     "Colapso do Bacalhau (1992), Semilattice & Fechamento Bacia/Raiz",
     "`L35 · S:VII · {T3,T5,T6} · (1,1,1)` & Fechamento das 35 Ordens"),

    # --- Stage 5: Part VI Laminar Trajectory & Part VII Superposed Aim (86–99) ---
    ("W086", 5, 1, ["86-posture-before-bowstring-draws.md"],
     "Epistemologia do Enquadramento & Sociologia de Painéis",
     "Pisos Interiores, Erro Tipo III & Ruído Inter-Julgadores",
     "Parte VI · `T1` Laminar (Postura Antes do Disparo)"),
    ("W087", 5, 2, ["87-aiming-against-counterfactual-winds.md"],
     "Filosofia da Ciência, Inferência & Robustez Distribucional",
     "Martingales de Teste, Penalidade de Lakatos, Manski & Falha de DRO",
     "Parte VI · `T2` Laminar (Contra Ventos Contrafactuais)"),
    ("W088", 5, 3, ["88-what-the-wind-leaves-on-stone.md"],
     "Amostragem, Demografia de Arquivos & Identificação MNAR",
     "Meng, DEFF Aditivo, Peso Keynesiano & Limites Sharp MNAR `CVaR_p`",
     "Parte VI · `T3` Laminar (O Que o Vento Deixa na Pedra)"),
    ("W089", 5, 4, ["89-curvature-of-the-flight-path.md"],
     "Geometria Diferencial, Scoring Próprio & Caudas Pesadas",
     "Jacobiano Condicional ao Mapa, Invariância de Caixa Viável & Identidade do Gargalo",
     "Parte VI · `T4` Laminar (Curvatura da Trajetória)"),
    ("W090", 5, 5, ["90-crosswind-clock-and-vessel.md"],
     "Física Estatística, Filas & Fisiologia da Sobrevivência",
     "Quatro Relógios Irreversíveis, Regimes Lexicográficos & Preempção Vital",
     "Parte VI · `T5` Laminar (Relógio de Vento Cruzado)"),
    ("W091", 5, 6, ["91-when-the-target-reads-the-archer.md"],
     "Teoria de Controle Reativo, Economia & Performatividade",
     "CORP-PAV, ICP, Não-Contração Performativa & Falha de DRO Offline",
     "Parte VI · `T6` Laminar (Quando o Alvo Lê o Arqueiro)"),
    ("W092", 5, 7, ["92-sealed-bow-and-uncommuted-verdict.md"],
     "Escolha Social, Criptografia de Selo & Estatística IUT",
     "Predicado de 5 Estágios, Não-Comutação, Viés Côncavo do min & Teste IUT (Berger)",
     "Parte VI · `T7` Laminar & Dobra Co-Diagnóstica `D_diag`"),
    ("W093", 5, 8, ["93-fracturing-the-frame-before-the-seal.md"],
     "Lógica de Contestação Pré-Selo & Resgate Multi-Lista",
     "Indecidibilidade de Premise-Rigging, Standing de Testemunha & Contraindutividade",
     "Parte VII · Sobreposição Pré-Selo (`F16/F17`)"),
    ("W094", 5, 9, ["94-yesterdays-crosswind-in-the-string.md"],
     "Teoria de Trajetórias Multi-Passo & Epistemologia Anti-Gettier",
     "Invariância Sintática vs. Pushforward em `t > 1` & Censura de Suporte",
     "Parte VII · Skopós Fantasma & Trajetória (`F16`)"),
    ("W095", 5, 10, ["95-collision-at-ten-bits-per-second.md"],
     "Neurociência da Atenção, Regulação Financeira/Clínica & Redes",
     "Teto 10 bits/s (Zheng–Meister), Haldane–Madouros, Fronteira Chanfrada & Compliance DoS",
     "Parte VII · Colisão em 10 bits/s & Histerese (`F15`)"),
    ("W096", 5, 11, ["96-arrows-that-bend-the-strata.md"],
     "Sistemas Dinâmicos Acoplados & Engenharia de Ilhamento",
     "Mapas Recorrentes, Transferência de Variância & Disjuntores Sacrificiais",
     "Parte VII · Causalidade Descendente Cíclica (`F19/F23`)"),
    ("W097", 5, 12, ["97-the-hijacked-kairos-and-exception.md"],
     "Direito de Exceção, Agnotologia Regulatória & Saúde do Cuidado",
     "Pré-Compromisso de Elster (1979), Toxicologia/Agnotologia (Schäffer, Oreskes) & Captura",
     "Parte VII · Kairós Sequestrado & Freio 3 (`F17`)"),
    ("W098", 5, 13, ["98-when-the-target-draws-its-bow.md"],
     "Ética da Alteridade, Econometria Judicial & Economia do Cuidado/Desastres",
     "Externalização entre Dois Arqueiros, Fiança (Dobbie/Kleinberg) & Risco de Enchentes",
     "Parte VII · Bacia de Recepção `Θ_basin` & 5º Falsificador (`F18/F22`)"),
    ("W099", 5, 14, ["99-unsealing-the-vault-after-impact.md"],
     "Epistemologia Trágica, Segurança Industrial & Enfermagem/Epidemiologia",
     "Macondo (LR=1), Identificação Parcial (Manski/Stoye), Meta-Análise Clínica & Matriz `Λ_ab`",
     "Parte VII · Anagnórisis, Canguilhem & Feyerabend (`F20/F22/F23`)"),

    # --- Stage 6: Framing Waves (W100–W107) & Appendices (W108–W111) ---
    ("W100", 6, 1, ["index.md", "part-i-introduction.md", "part-i-conclusion.md"],
     "Arquitetura Editorial & Retórica de Entrada do Livro",
     "Links HTML/SVG do Portal (`index.md`), Contratos de Abertura & Citações Iniciais",
     "Moldura Inicial (`index.md` + Parte I Intro/Conclusão)"),
    ("W101", 6, 2, ["part-ii-introduction.md", "part-ii-conclusion.md"],
     "Pedagogia Interdisciplinar (Matemática–Filosofia–Ciências)",
     "Precisão Simbólica & Transições Entre os Três Blocos de Base",
     "Moldura da Parte II (Intro/Conclusão)"),
    ("W102", 6, 3, ["part-iii-introduction.md", "part-iii-conclusion.md"],
     "Arquitetura dos Fundamentos do Julgamento",
     "Coerência Circuito–Guardas–Mínimo na Síntese da Parte III",
     "Moldura da Parte III (Intro/Conclusão)"),
    ("W103", 6, 4, ["part-iv-introduction.md", "part-iv-conclusion.md"],
     "Arquitetura do Framework & Composição dos Três Filtros",
     "Composição Formal dos Filtros I–III & Encadeamento para Parte V",
     "Moldura da Parte IV (Intro/Conclusão)"),
    ("W104", 6, 5, ["part-v-introduction.md", "stratum-i-opening.md", "stratum-ii-opening.md", "stratum-iii-opening.md", "stratum-iv-opening.md"],
     "Arquitetura Multicamadas (Estratos I–IV)",
     "Paridade `L01`–`L20`, `C(7,3)` & Fontes Empíricas dos Estratos I–IV (`stratum-iv-opening`)",
     "Moldura da Parte V — Metade I (Intro + Estratos I–IV)"),
    ("W105", 6, 6, ["stratum-v-opening.md", "stratum-vi-opening.md", "stratum-vii-opening.md", "part-v-conclusion.md"],
     "Arquitetura Multicamadas (Estratos V–VII & Fechamento)",
     "Paridade Estratos V–VII com `L21`–`L35` & Fechamento Não-Substitutivo",
     "Moldura da Parte V — Metade II (Estratos V–VII + Conclusão)"),
    ("W106", 6, 7, ["part-vi-introduction.md", "part-vi-conclusion.md"],
     "Narrativa Formal da Trajetória Laminar (Parte VI)",
     "Pushforward Irreversível, Duas Fitas & Quatro Limites Formais",
     "Moldura da Parte VI (Intro/Conclusão)"),
    ("W107", 6, 8, ["part-vii-introduction.md", "part-vii-conclusion.md"],
     "Arquitetura da Sobreposição Constitucional (Parte VII)",
     "Escopo Constitucional da Contraindução & Espelho VI–VII",
     "Moldura da Parte VII (Intro/Conclusão)"),
    ("W108", 6, 9, ["appendix-a-mathematics.md"],
     "Matemática Consolidada, Grafos & Gramática `LL(1)` (Apêndice A)",
     "Verificação Simbólica Integral & Consistência de Notação (`A.1`–`A.7`)",
     "Paridade Apêndice A <-> Capítulos & Regra §II"),
    ("W109", 6, 10, ["appendix-b-architecture-verification.md"],
     "Gramática Formal do Alvo & Grafo Multicamadas (Apêndice B)",
     "Produções B.1, Não-Comutação, `Table B.1` (35 Linhas), §V-5 (`SUMMARY.md`) & §V-10(b)/(c)",
     "Paridade `Table B.1` <-> `51`–`85` & `SUMMARY.md`; `Livro -> Núcleo` (não citável)"),
    ("W110", 6, 11, ["appendix-e-lexicon.md", "appendix-g-glossary.md"],
     "Lexicografia Clássica, Política & Estatística/Econometria (`E.2`, Apêndices E & G)",
     "Tabela E.1 (`T1`–`T7`), Índices de Símbolos & Definições Cruzadas",
     "Paridade Apêndices E/G <-> Capítulos & Regra §V-10(a)"),
    ("W111", 6, 12, ["appendix-c-sources.md", "appendix-d-claim-ledger.md", "appendix-f-manuscript-audit.md"],
     "Custódia Bibliográfica, Ledger Empírico & Auditoria Integral (Apêndices C, D & F)",
     "Paridade Exata §V-6 (Ledger D <-> Bibliografia C <-> Textos Primários)",
     "Fechamento Global §II/§V, Papel Não-Normativo de `F` & Auditoria Final"),
]


def build_wave_catalog(index: dict[str, dict]) -> list[dict]:
    """Validate and materialize all 112 waves with exact 129-file bijection."""
    if len(WAVE_CATALOG_SPECS) != 112:
        raise RuntimeError(
            f"Expected 112 waves in catalog, got {len(WAVE_CATALOG_SPECS)}"
        )

    seen_ids: set[str] = set()
    seen_files: set[str] = set()
    catalog: list[dict] = []

    for wave_id, stage, stage_order, files, p1, p2, p3 in WAVE_CATALOG_SPECS:
        if wave_id in seen_ids:
            raise RuntimeError(f"Duplicate wave_id in catalog: {wave_id}")
        seen_ids.add(wave_id)
        for f in files:
            if f not in index:
                raise RuntimeError(f"Wave {wave_id} references unknown file: {f}")
            if f in seen_files:
                raise RuntimeError(f"File {f} assigned to multiple waves ({wave_id})")
            seen_files.add(f)

        guard_sites: list[str] = []
        part_v_meta = None
        if len(files) == 1 and "part_v" in index[files[0]]:
            part_v_meta = index[files[0]]["part_v"]
            guard_sites = part_v_meta["guard_sites"]

        catalog.append(
            {
                "wave_id": wave_id,
                "stage": stage,
                "stage_name": STAGE_NAMES[stage],
                "stage_order": stage_order,
                "files": list(files),
                "titles": [index[f]["title"] for f in files],
                "prose_words": sum(index[f]["prose_words"] for f in files),
                "part_v": part_v_meta,
                "guard_sites": guard_sites,
                "pair1_domain": p1,
                "pair2_rigor": p2,
                "pair3_arch": p3,
                "board_tag": f"[{wave_id}]",
                "workers": {
                    "s1_dom": {"roster": f"{wave_id.lower()}-s1", "model": DEFAULT_PROPOSER_MODEL, "workspace": "share"},
                    "a1_dom": {"roster": f"{wave_id.lower()}-a1", "model": DEFAULT_ADVERSARIAL_MODEL, "workspace": "inherit"},
                    "s2_rigor": {"roster": f"{wave_id.lower()}-s2", "model": DEFAULT_PROPOSER_MODEL, "workspace": "inherit"},
                    "a2_rigor": {"roster": f"{wave_id.lower()}-a2", "model": DEFAULT_ADVERSARIAL_MODEL, "workspace": "inherit"},
                    "s3_arch": {"roster": f"{wave_id.lower()}-s3", "model": DEFAULT_PROPOSER_MODEL, "workspace": "inherit"},
                    "a3_arch": {"roster": f"{wave_id.lower()}-a3", "model": DEFAULT_ADVERSARIAL_MODEL, "workspace": "inherit"},
                },
            }
        )

    missing = set(index.keys()) - seen_files
    if missing:
        raise RuntimeError(
            f"Catalog does not cover {len(missing)} manuscript file(s): {sorted(missing)}"
        )
    return catalog


@contextlib.contextmanager
def _state_lock(state_path: Path):
    state_path.parent.mkdir(parents=True, exist_ok=True)
    lock_path = state_path.with_suffix(state_path.suffix + ".lock")
    with open(lock_path, "a+", encoding="utf-8") as lock_fh:
        fcntl.flock(lock_fh.fileno(), fcntl.LOCK_EX)
        try:
            yield
        finally:
            fcntl.flock(lock_fh.fileno(), fcntl.LOCK_UN)


def _atomic_write_json(target: Path, payload: dict) -> None:
    target.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=f".{target.name}.", dir=target.parent)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as fh:
            fh.write(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")
            fh.flush()
            os.fsync(fh.fileno())
        os.replace(tmp_name, target)
    except Exception:
        with contextlib.suppress(OSError):
            os.unlink(tmp_name)
        raise


def _load_or_init_state(state_path: Path, catalog: list[dict], create_if_missing: bool = False) -> dict:
    if state_path.exists():
        return json.loads(state_path.read_text(encoding="utf-8"))
    now = datetime.now(timezone.utc).isoformat()
    state = {
        "schema_version": 1,
        "updated_at": now,
        "waves": {
            w["wave_id"]: {
                "status": "pending",
                "stage": w["stage"],
                "files": w["files"],
                "commit": None,
                "notes": None,
                "updated_at": now,
            }
            for w in catalog
        },
    }
    if create_if_missing:
        _atomic_write_json(state_path, state)
    return state


def _elastic_reservoir_limit(
    quota_primary: float | None,
    quota_secondary: float | None = None,
    allow_single_pool: bool = False,
) -> dict:
    quotas = [q for q in (quota_primary, quota_secondary) if q is not None]
    if not quotas:
        return {
            "k_max": 1,
            "band": "unspecified (cota omitida; teto conservador K=1)",
            "min_quota": None,
            "action": "conservative_k1_measure_quota",
            "cross_family_degraded": False,
        }
    if len(quotas) == 1:
        q_val = quotas[0]
        if q_val > 60.0:
            return {
                "k_max": 4,
                "band": "> 60% (K=4, 24 revisores)",
                "min_quota": q_val,
                "action": "normal",
                "cross_family_degraded": False,
            }
        if q_val >= 30.0:
            return {
                "k_max": 2,
                "band": "[30%, 60%] (K=2, 12 revisores)",
                "min_quota": q_val,
                "action": "throttle_k2",
                "cross_family_degraded": False,
            }
        if q_val >= 15.0:
            return {
                "k_max": 1,
                "band": "[15%, 30%) (K=1, 6 revisores)",
                "min_quota": q_val,
                "action": "throttle_k1",
                "cross_family_degraded": False,
            }
        return {
            "k_max": 0,
            "band": "< 15% no reservatório ativo — checkpoint e pausa até quota_resets",
            "min_quota": q_val,
            "action": "checkpoint_and_pause",
            "cross_family_degraded": False,
        }

    assert quota_primary is not None and quota_secondary is not None
    min_q = min(quota_primary, quota_secondary)
    max_q = max(quota_primary, quota_secondary)
    if min_q > 60.0:
        return {
            "k_max": 4,
            "band": "> 60% (K=4, 24 revisores)",
            "min_quota": min_q,
            "action": "normal",
            "cross_family_degraded": False,
        }
    if min_q >= 30.0:
        return {
            "k_max": 2,
            "band": "[30%, 60%] (K=2, 12 revisores)",
            "min_quota": min_q,
            "action": "throttle_k2",
            "cross_family_degraded": False,
        }
    if min_q >= 15.0:
        return {
            "k_max": 1,
            "band": "[15%, 30%) (K=1, 6 revisores)",
            "min_quota": min_q,
            "action": "throttle_k1",
            "cross_family_degraded": False,
        }
    if max_q >= 15.0:
        exhausted = "primary" if quota_primary < 15.0 else "secondary"
        healthy = "secondary" if exhausted == "primary" else "primary"
        if allow_single_pool:
            return {
                "k_max": 1,
                "band": f"< 15% no pool {exhausted}; fallback explícito K=1 no pool {healthy} ({max_q:.1f}%)",
                "min_quota": min_q,
                "action": f"single_pool_fallback_to_{healthy}_k1",
                "healthy_pool": healthy,
                "cross_family_degraded": True,
            }
        return {
            "k_max": 0,
            "band": f"< 15% no pool {exhausted}; pool {healthy} em {max_q:.1f}% (aguarde ou use --allow-single-pool)",
            "min_quota": min_q,
            "action": f"hold_new_waves_finish_inflight_or_allow_single_pool_{healthy}",
            "healthy_pool": healthy,
            "cross_family_degraded": False,
        }
    return {
        "k_max": 0,
        "band": "< 15% em ambos os reservatórios",
        "min_quota": min_q,
        "action": "checkpoint_and_pause",
        "cross_family_degraded": False,
    }


def _waves_compatible(
    cand: dict,
    cand_footprint: dict,
    other: dict,
    other_footprint: dict,
) -> tuple[bool, str | None]:
    edits_a = set(cand["files"])
    edits_b = set(other["files"])
    if edits_a & edits_b:
        return False, "edit_overlap"

    partners_a = set(cand_footprint["core_relation_partners_full_read"]) | set(
        cand_footprint.get("body_links", ())
    )
    partners_b = set(other_footprint["core_relation_partners_full_read"]) | set(
        other_footprint.get("body_links", ())
    )
    if (edits_a & partners_b) or (edits_b & partners_a):
        return False, "partner_read_write_collision"

    nums_a = [n for f in cand["files"] if (n := _chapter_number_from_file(f)) is not None]
    nums_b = [n for f in other["files"] if (n := _chapter_number_from_file(f)) is not None]
    for na in nums_a:
        for nb in nums_b:
            if abs(na - nb) < 2:
                return False, "adjacent_chapter_bridge_race"

    guards_a = set(cand["guard_sites"])
    guards_b = set(other["guard_sites"])
    if guards_a and guards_b and len(guards_a & guards_b) >= 2:
        return False, "johnson_j7_3_sibling_edge"

    return True, None


def cmd_waves(args: argparse.Namespace) -> int:
    index = build_book_index()
    jg = johnson_graph_part_v(index)
    catalog = build_wave_catalog(index)

    if args.wave:
        target = args.wave.strip().upper()
        matches = [w for w in catalog if w["wave_id"] == target]
        if not matches:
            print(f"Unknown wave id: {target}", file=sys.stderr)
            return 1
        w = dict(matches[0])
        w["footprint"] = plan_wave_footprint(index, jg, w["files"])
        text = json.dumps(w, indent=2, ensure_ascii=False)
        if args.out:
            out_path = Path(args.out)
            out_path.parent.mkdir(parents=True, exist_ok=True)
            out_path.write_text(text + "\n", encoding="utf-8")
        print(text)
        return 0

    filtered = [w for w in catalog if args.stage is None or w["stage"] == args.stage]
    filtered.sort(key=lambda w: (w["stage"], w["stage_order"]))

    if args.format == "json":
        text = json.dumps(filtered, indent=2, ensure_ascii=False)
    else:
        lines: list[str] = []
        current_stage = None
        for w in filtered:
            if w["stage"] != current_stage:
                current_stage = w["stage"]
                if lines:
                    lines.append("")
                lines.append(f"### {w['stage_name']}")
                lines.append("")
                lines.append(
                    "| Onda | Arquivos Alvo | Par 1 (`S1 ↔ A1`: Domínio) | Par 2 (`S2 ↔ A2`: Rigor & Fontes) | Par 3 (`S3 ↔ A3`: Transversalidade & Voz) |"
                )
                lines.append("| :--- | :--- | :--- | :--- | :--- |")
            files_col = ", ".join(f"`{f}`" for f in w["files"])
            lines.append(
                f"| `{w['wave_id']}` | {files_col} | {w['pair1_domain']} | {w['pair2_rigor']} | {w['pair3_arch']} |"
            )
        text = "\n".join(lines)

    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(text + "\n", encoding="utf-8")
    print(text)
    return 0


def cmd_state(args: argparse.Namespace) -> int:
    index = build_book_index()
    catalog = build_wave_catalog(index)
    state_path = Path(args.state_file)

    if args.init:
        with _state_lock(state_path):
            existed = state_path.exists()
            state = _load_or_init_state(state_path, catalog, create_if_missing=True)
        print(
            json.dumps(
                {
                    "action": "already_exists" if existed else "initialized",
                    "state_file": str(state_path),
                    "wave_count": len(state["waves"]),
                },
                indent=2,
                ensure_ascii=False,
            )
        )
        return 0

    with _state_lock(state_path):
        if not state_path.exists():
            print(
                f"State file does not exist: {state_path}. Run with --init first.",
                file=sys.stderr,
            )
            return 1

        state = _load_or_init_state(state_path, catalog, create_if_missing=False)
        if args.set:
            for spec in args.set:
                if ":" not in spec:
                    print(f"Invalid --set format {spec!r}; expected WAVE_ID:STATUS", file=sys.stderr)
                    return 1
                wid, status = spec.split(":", 1)
                wid = wid.strip().upper()
                status = status.strip().lower()
                if wid not in state["waves"]:
                    print(f"Unknown wave_id {wid!r}", file=sys.stderr)
                    return 1
                if status not in VALID_WAVE_STATUSES:
                    print(
                        f"Invalid status {status!r}; expected one of {VALID_WAVE_STATUSES}",
                        file=sys.stderr,
                    )
                    return 1
                now = datetime.now(timezone.utc).isoformat()
                state["waves"][wid]["status"] = status
                if args.commit is not None:
                    state["waves"][wid]["commit"] = args.commit
                if args.notes is not None:
                    state["waves"][wid]["notes"] = args.notes
                state["waves"][wid]["updated_at"] = now
                state["updated_at"] = now
            _atomic_write_json(state_path, state)

    by_status: dict[str, int] = {s: 0 for s in VALID_WAVE_STATUSES}
    by_stage: dict[str, dict[str, int]] = {}
    for w in catalog:
        wid = w["wave_id"]
        st = state["waves"].get(wid, {}).get("status", "pending")
        by_status[st] = by_status.get(st, 0) + 1
        s_key = f"stage_{w['stage']}"
        by_stage.setdefault(s_key, {s: 0 for s in VALID_WAVE_STATUSES})
        by_stage[s_key][st] += 1

    summary = {
        "state_file": str(state_path),
        "updated_at": state.get("updated_at"),
        "totals": by_status,
        "by_stage": by_stage,
    }
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    return 0


def cmd_next_batch(args: argparse.Namespace) -> int:
    adv_model = getattr(args, "adversarial_model", DEFAULT_ADVERSARIAL_MODEL)
    fb_model = getattr(args, "fallback_model", None)
    for m_opt in (adv_model, fb_model):
        if m_opt and "opus" in m_opt.lower():
            print(
                f"Prohibited model {m_opt!r}: Claude Opus is disabled; use {DEFAULT_ADVERSARIAL_MODEL!r}.",
                file=sys.stderr,
            )
            return 2

    index = build_book_index()
    jg = johnson_graph_part_v(index)
    catalog = build_wave_catalog(index)
    state_path = Path(args.state_file)
    state = _load_or_init_state(state_path, catalog, create_if_missing=False)

    quota_primary = args.quota if args.quota is not None else args.quota_gemini
    quota_secondary = (
        args.quota_adversarial
        if getattr(args, "quota_adversarial", None) is not None
        else args.quota_claude
    )
    reservoir = _elastic_reservoir_limit(
        quota_primary,
        quota_secondary,
        allow_single_pool=getattr(args, "allow_single_pool", False),
    )
    effective_k = min(args.k, reservoir["k_max"])

    target_stage = args.stage
    if target_stage is None:
        for stg in sorted(STAGE_NAMES):
            has_open = any(
                w["stage"] == stg
                and state["waves"].get(w["wave_id"], {}).get("status", "pending")
                in ("pending", "active")
                for w in catalog
            )
            if has_open:
                target_stage = stg
                break

    if target_stage is None:
        print(
            json.dumps(
                {
                    "target_stage": None,
                    "reservoir": reservoir,
                    "effective_k": effective_k,
                    "slots_available": 0,
                    "stage_gate_waiting_active": False,
                    "active_wave_ids": [],
                    "selected_count": 0,
                    "selected_waves": [],
                    "skipped_in_stage": {},
                    "message": "Todas as 112 ondas já saíram de pending/active.",
                },
                indent=2,
                ensure_ascii=False,
            )
        )
        return 0

    active_waves = [
        w
        for w in catalog
        if state["waves"].get(w["wave_id"], {}).get("status") == "active"
    ]
    slots_available = max(0, effective_k - len(active_waves))
    footprints: dict[str, dict] = {}
    for w in active_waves:
        footprints[w["wave_id"]] = plan_wave_footprint(index, jg, w["files"])

    stage_pending = sorted(
        (
            w
            for w in catalog
            if w["stage"] == target_stage
            and state["waves"].get(w["wave_id"], {}).get("status", "pending") == "pending"
        ),
        key=lambda w: w["stage_order"],
    )
    stage_active = [w for w in active_waves if w["stage"] == target_stage]
    stage_gate_waiting_active = bool(stage_active and not stage_pending)

    other_unfinished_exists = any(
        state["waves"].get(w["wave_id"], {}).get("status", "pending")
        not in ("clean", "edited")
        and w["wave_id"] != "W111"
        for w in catalog
    )

    selected: list[dict] = []
    skipped_reasons: dict[str, str] = {}

    if slots_available > 0:
        for cand in stage_pending:
            if len(selected) >= slots_available:
                break
            if cand["wave_id"] == "W111" and (
                other_unfinished_exists or active_waves or selected
            ):
                skipped_reasons[cand["wave_id"]] = (
                    "closure_wave_waits_for_W000_to_W110_completion"
                )
                continue
            cand_fp = plan_wave_footprint(index, jg, cand["files"])
            compatible = True
            block_reason = None
            for peer in active_waves + selected:
                ok, reason = _waves_compatible(
                    cand, cand_fp, peer, footprints[peer["wave_id"]]
                )
                if not ok:
                    compatible = False
                    block_reason = f"{reason} (with {peer['wave_id']})"
                    break
            if compatible:
                footprints[cand["wave_id"]] = cand_fp
                selected.append(cand)
            elif block_reason:
                skipped_reasons[cand["wave_id"]] = block_reason

    def _effective_workers(w: dict) -> dict:
        workers = {k: dict(v) for k, v in w["workers"].items()}
        for seat in ("a1_dom", "a2_rigor", "a3_arch"):
            workers[seat]["model"] = adv_model
        if reservoir.get("cross_family_degraded"):
            healthy = reservoir.get("healthy_pool")
            if healthy == "secondary":
                repl = fb_model or adv_model
                for seat in ("s1_dom", "s2_rigor", "s3_arch"):
                    workers[seat]["model"] = repl
            elif healthy == "primary":
                repl = fb_model if fb_model is not None else DEFAULT_PROPOSER_MODEL
                for seat in ("a1_dom", "a2_rigor", "a3_arch"):
                    workers[seat]["model"] = repl
            elif fb_model is not None:
                for seat in workers:
                    workers[seat]["model"] = fb_model
        return workers

    selected_payload = [
        {
            "wave_id": w["wave_id"],
            "stage": w["stage"],
            "stage_order": w["stage_order"],
            "files": w["files"],
            "titles": w["titles"],
            "prose_words": w["prose_words"],
            "guard_sites": w["guard_sites"],
            "core_relation_partners_full_read": footprints[w["wave_id"]][
                "core_relation_partners_full_read"
            ],
            "board_tag": w["board_tag"],
            "workers": _effective_workers(w),
            "specialties": {
                "pair1_domain": w["pair1_domain"],
                "pair2_rigor": w["pair2_rigor"],
                "pair3_arch": w["pair3_arch"],
            },
        }
        for w in selected
    ]

    blocking_unfinished = [
        {
            "wave_id": w["wave_id"],
            "status": state["waves"].get(w["wave_id"], {}).get("status", "pending"),
        }
        for w in catalog
        if state["waves"].get(w["wave_id"], {}).get("status", "pending")
        not in ("clean", "edited", "pending")
        and w["wave_id"] != "W111"
    ]

    output: dict = {
        "target_stage": target_stage,
        "target_stage_name": STAGE_NAMES[target_stage],
        "state_file_exists": state_path.exists(),
        "reservoir": reservoir,
        "effective_k": effective_k,
        "slots_available": slots_available,
        "stage_gate_waiting_active": stage_gate_waiting_active,
        "active_wave_ids": [w["wave_id"] for w in active_waves],
        "selected_count": len(selected_payload),
        "selected_waves": selected_payload,
        "skipped_in_stage": skipped_reasons,
    }
    if stage_gate_waiting_active:
        output["message"] = (
            "Portão de estágio: aguarde as ondas ativas deste estágio concluírem e rode os portões de fechamento antes do próximo estágio."
        )
    elif not selected_payload and "W111" in skipped_reasons and blocking_unfinished:
        output["blocking_unfinished_waves"] = blocking_unfinished
        summary_blockers = ", ".join(f"{b['wave_id']}:{b['status']}" for b in blocking_unfinished)
        output["message"] = (
            f"W111 aguarda conclusão de W000–W110; ondas em aberto ({summary_blockers}) devem ser resolvidas ou reabertas via `state --set Wnnn:pending`."
        )
    if reservoir.get("cross_family_degraded"):
        output["epistemic_warning"] = (
            "CROSS-FAMILY DEGRADED: se não houver 3ª família distinta no --fallback-model, registre CROSS-FAMILY: degraded no verdict.md para reauditoria."
        )
    print(json.dumps(output, indent=2, ensure_ascii=False))
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Deterministic Stochology Adversarial Review operations toolkit."
    )
    sub = parser.add_subparsers(dest="command", required=True)

    p_base = sub.add_parser(
        "baseline", help="Capture structural baseline & per-file metrics."
    )
    p_base.add_argument(
        "--out", help="Optional JSON path (in <artifactDir>/) to save baseline."
    )
    p_base.set_defaults(func=cmd_baseline)

    p_shards = sub.add_parser(
        "shards", help="Emit balanced 9-shard federated cartography manifest."
    )
    p_shards.add_argument("--format", choices=("md", "json"), default="md")
    p_shards.add_argument("--out", help="Optional output path (in <artifactDir>/).")
    p_shards.set_defaults(func=cmd_shards)

    p_graph = sub.add_parser(
        "graph",
        help="Inspect cross-part markdown links and Part V Johnson graph J(7,3).",
    )
    p_graph.add_argument(
        "--chapter", help="Inspect a specific chapter/file (e.g. '41' or '51')."
    )
    p_graph.set_defaults(func=cmd_graph)

    p_wp = sub.add_parser(
        "wave-plan",
        help="Compute recommended full-read relation partners and check concurrent wave disjointness.",
    )
    p_wp.add_argument(
        "--edit", required=True, help="Comma-separated chapter numbers or filenames."
    )
    p_wp.add_argument(
        "--partners", help="Optional extra comma-separated partner files."
    )
    p_wp.add_argument(
        "--max-partners",
        type=int,
        default=6,
        help="Max auto-selected full-read core relation partners (default: 6; overflow goes to card/section check).",
    )
    p_wp.add_argument(
        "--active-wave",
        help="Optional comma-separated footprint (edits + partners) of a concurrent wave to test disjointness.",
    )
    p_wp.set_defaults(func=cmd_wave_plan)

    p_wa = sub.add_parser(
        "wave-audit",
        help="Audit modified wave files for structural gates, math subscripts, census, and prose bloat.",
    )
    p_wa.add_argument(
        "--files", required=True, help="Comma-separated chapter numbers or filenames."
    )
    p_wa.add_argument("--baseline", help="Path to baseline.json captured before edits.")
    p_wa.add_argument(
        "--max-growth-pct",
        type=float,
        default=15.0,
        help="Swarm Brake 2 prose-word growth threshold percentage (default: 15%%).",
    )
    p_wa.set_defaults(func=cmd_wave_audit)

    p_gates = sub.add_parser(
        "gates", help="Run manuscript, surface/render, and optional core/wasm gates."
    )
    p_gates.add_argument(
        "--full-render", action="store_true", help="Run mdbook build + surface.py."
    )
    p_gates.add_argument(
        "--with-pdf", action="store_true", help="Also build offline PDF edition."
    )
    p_gates.add_argument(
        "--with-core",
        action="store_true",
        help="Also run offline Rust tests + wasm_gates.py.",
    )
    p_gates.set_defaults(func=cmd_gates)

    p_waves = sub.add_parser(
        "waves",
        help="List or inspect the 112-wave (W000–W111) bespoke 6-agent catalog covering all 129 files.",
    )
    p_waves.add_argument("--stage", type=int, choices=(1, 2, 3, 4, 5, 6))
    p_waves.add_argument("--wave", help="Inspect a single wave (e.g. W041).")
    p_waves.add_argument("--format", choices=("md", "json"), default="md")
    p_waves.add_argument("--out", help="Optional output file path.")
    p_waves.set_defaults(func=cmd_waves)

    p_state = sub.add_parser(
        "state",
        help="Initialize, update, or summarize persistent 112-wave state in research/runs/state.json.",
    )
    p_state.add_argument(
        "--state-file",
        default=str(DEFAULT_STATE_FILE),
        help="Path to state JSON (default: research/runs/state.json).",
    )
    p_state.add_argument(
        "--init", action="store_true", help="Initialize state.json with 112 pending waves if absent."
    )
    p_state.add_argument(
        "--set",
        action="append",
        help="Update wave status as WAVE_ID:STATUS (pending|active|clean|edited|escalated|halted).",
    )
    p_state.add_argument("--commit", help="Optional local commit hash recorded with --set.")
    p_state.add_argument("--notes", help="Optional short note recorded with --set.")
    p_state.add_argument("--summary", action="store_true", help="Print status summary.")
    p_state.set_defaults(func=cmd_state)

    p_nb = sub.add_parser(
        "next-batch",
        help="Select up to K mutually disjoint pending waves respecting elastic reservoir watermarks.",
    )
    p_nb.add_argument("--k", type=int, default=4, help="Requested max concurrent waves (default: 4).")
    p_nb.add_argument("--quota", type=float, help="Remaining quota % for the active Gemini reservoir (Carbon + Hydrogen).")
    p_nb.add_argument("--quota-gemini", type=float, help="Remaining quota % for the Gemini pool (Carbon + Hydrogen).")
    p_nb.add_argument("--quota-adversarial", type=float, help="Optional separate quota % if adversarial seats use an independent reservoir.")
    p_nb.add_argument("--quota-claude", type=float, help="Deprecated alias for --quota-adversarial.")
    p_nb.add_argument(
        "--adversarial-model",
        default=DEFAULT_ADVERSARIAL_MODEL,
        help=f"Model ID for adversarial twins a1..a3 (default: {DEFAULT_ADVERSARIAL_MODEL}; Opus prohibited).",
    )
    p_nb.add_argument("--stage", type=int, choices=(1, 2, 3, 4, 5, 6), help="Restrict to a specific Macro-Stage.")
    p_nb.add_argument(
        "--allow-single-pool",
        action="store_true",
        help="Explicitly unlock K=1 when one reservoir <15%% and the other >=15%% (requires 3rd distinct family or CROSS-FAMILY: degraded tag).",
    )
    p_nb.add_argument(
        "--fallback-model",
        default=None,
        help="Optional explicit model ID for exhausted pool seats under --allow-single-pool.",
    )
    p_nb.add_argument(
        "--state-file",
        default=str(DEFAULT_STATE_FILE),
        help="Path to state JSON (default: <MAIN_ROOT>/research/runs/state.json).",
    )
    p_nb.set_defaults(func=cmd_next_batch)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
