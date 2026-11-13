#!/usr/bin/env python3
"""Structural checks for the Stochology manuscript.

Implements the checks of AGENTS.md section V that read the sources: apparatus,
spine, links, figures, table B.1 parity, ledger parity, numeral census, contents
parity, concordance (§V-9) and guard-site cardinality (§V-10). Check §V-8 — the
published surface — reads the HTML mdbook builds, so it lives in surface.py and
runs in the render job.

There is nothing to compile here: the repository is prose plus figures, and every
gate below is a reading of the text. Exit code 0 means no gate failed; the
numeral census only reports.
"""

from __future__ import annotations

import glob
import itertools
import os
import re
import sys
import urllib.parse
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

CHAPTERS = "book-src/[0-9][0-9]-*.md"
APPENDICES = "book-src/appendix-*.md"
ROOT_DOCS = ["README.md", "AGENTS.md", "CONTRIBUTING.md"]

failures: list[str] = []


def rel(path: str) -> str:
    return os.path.relpath(path, ROOT)


def read(path: str) -> str:
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def fail(gate: str, detail: str) -> None:
    failures.append(f"[{gate}] {detail}")


# ---------------------------------------------------------------- 1. apparatus

APPARATUS = [
    (r"\.rs\b", "Rust source extension"),
    (r"\b[a-z_][a-z0-9_]*::[A-Z]\w*", "module::Symbol anchor"),
    (r"\bcargo\s+(?:run|test|build|clippy|fmt|install)\b", "build-tool invocation"),
    (r"/\*\s*===|===\s*CHAPTER", "chapter-flag or frozen-output banner"),
    (r"\.(?:json|csv|parquet|npz|sha256|ipynb)\b", "data-file or archive name"),
    (r"\b(?:SplitMix|DEFAULT_SEED)\w*", "seeded simulator symbol"),
    (r"`\d{2}\.\d+\.?\d*`", "backticked chapter-dot-ordinal claim identifier"),
]

FENCE = re.compile(r"^```.*?^```", re.DOTALL | re.MULTILINE)


def check_apparatus() -> None:
    targets = sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
    targets += [os.path.join(ROOT, name) for name in ROOT_DOCS]
    for path in targets:
        text = read(path)
        # The contract names the forbidden tokens in order to forbid them, so the
        # one document that lists them is exempt from that one pattern.
        patterns = APPARATUS if os.path.basename(path) != "AGENTS.md" else [
            item for item in APPARATUS if "module::Symbol" not in item[1]
        ]
        for line_no, line in enumerate(text.splitlines(), 1):
            for pattern, label in patterns:
                if re.search(pattern, line):
                    fail("apparatus", f"{rel(path)}:{line_no}: {label} -> {line.strip()[:90]}")
        for block in FENCE.findall(text):
            if re.search(r"^\s*\$\s|exit code:?\s*\d|^\s*=== ", block, re.MULTILINE):
                fail("apparatus", f"{rel(path)}: fenced block presenting program output")


# ------------------------------------------------------------------- 2. spine

def numbered_block(text: str, heading: str) -> int:
    start = text.find(heading)
    if start < 0:
        return -1
    count = 0
    seen = False
    for line in text[start + len(heading):].splitlines():
        if re.match(r"\s*\d+\.\s", line):
            count += 1
            seen = True
        elif seen and not line.strip():
            break
    return count


def check_spine() -> None:
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/[0-9][0-9]-*.md"))):
        number = os.path.basename(path)[:2]
        if number == "00":
            continue
        text = read(path)
        bridges = text.count("## Bridge")
        if bridges != 1:
            fail("spine", f"{rel(path)}: {bridges} bridge sections, expected 1")
        outcomes = numbered_block(text, "## By the end of this chapter you can")
        if outcomes != 3:
            fail("spine", f"{rel(path)}: {outcomes} outcomes, expected 3")
        exercises = numbered_block(text, "## Exercises")
        if not 3 <= exercises <= 5:
            fail("spine", f"{rel(path)}: {exercises} exercises, expected 3-5")


# ------------------------------------------------------------------- 3. links

LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")


def check_links() -> None:
    targets = sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
    targets += [os.path.join(ROOT, name) for name in ROOT_DOCS]
    for path in targets:
        for match in LINK.finditer(read(path)):
            target = match.group(1).split("#")[0]
            if not target or target.startswith(("http:", "https:", "mailto:")):
                continue
            resolved = os.path.normpath(
                os.path.join(os.path.dirname(path), urllib.parse.unquote(target))
            )
            if not os.path.exists(resolved):
                fail("links", f"{rel(path)}: {target} does not resolve")


# ----------------------------------------------------------------- 4. figures

def check_figures() -> None:
    on_disk = set(
        os.path.basename(p) for p in glob.glob(os.path.join(ROOT, "book-src/figures/*"))
    )
    referenced: set[str] = set()
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        referenced |= set(re.findall(r"figures/([^)\s\"']+)", read(path)))
    for name in sorted(on_disk - referenced):
        fail("figures", f"figures/{name} is referenced by no chapter")
    for name in sorted(referenced - on_disk):
        fail("figures", f"figures/{name} is referenced but absent")


# ------------------------------------------------------------ 5. table B.1

ROMAN = {"I": "S1", "II": "S2", "III": "S3", "IV": "S4", "V": "S5", "VI": "S6", "VII": "S7"}
B1_ROW = re.compile(r"^\| `(L\d\d)` \| (\d+) \| `(S\d)` \|")
BLOCKQUOTE = re.compile(
    r"> Layer (\d+) of Part V · Stratum ([IVX]+) · (?:Chapter (\d+) · )?"
    r"Unique Guard Combination `\{([^}]*)\}` · Signature `\(([^)]*)\)`"
)


def check_b1() -> None:
    table: dict[str, tuple[str, str, set[str], str]] = {}
    for line in read(os.path.join(ROOT, "book-src/appendix-b-architecture-verification.md")).splitlines():
        match = B1_ROW.match(line)
        if match:
            cells = [cell.strip() for cell in line.strip("|").split("|")]
            table[match.group(1)] = (
                match.group(2),
                match.group(3),
                set(re.findall(r"T\d", cells[5])),
                cells[6].strip("`").replace(" ", ""),
            )
    if len(table) != 35:
        fail("b1", f"table has {len(table)} rows, expected 35")

    quoted: dict[str, tuple[str, str, set[str], str]] = {}
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/[5-8][0-9]-*.md"))):
        chapter = os.path.basename(path)[:2]
        for match in BLOCKQUOTE.finditer(read(path)):
            quoted["L" + match.group(1)] = (
                match.group(3) or chapter,
                ROMAN[match.group(2)],
                set(re.findall(r"T\d", match.group(4))),
                "(" + match.group(5).replace(" ", "") + ")",
            )

    for code, (chapter, stratum, guards, signature) in sorted(table.items()):
        if code in quoted:
            q = quoted[code]
            if (chapter, stratum, guards, signature) != (q[0], q[1], q[2], q[3]):
                fail("b1", f"{code}: table {chapter}/{stratum}/{sorted(guards)}/{signature} "
                           f"!= blockquote {q[0]}/{q[1]}/{sorted(q[2])}/{q[3]}")
            continue
        chapter_file = glob.glob(os.path.join(ROOT, f"book-src/{chapter}-*.md"))
        if not chapter_file:
            fail("b1", f"{code}: no chapter file for chapter {chapter}")
            continue
        text = read(chapter_file[0])
        if not [
            line
            for line in text.splitlines()
            if code in line and all(guard in line for guard in guards)
        ]:
            fail("b1", f"{code}: not folded into chapter {chapter}'s order list")


# ----------------------------------------------------------- 6. ledger parity

def check_ledger() -> None:
    bibliography = read(os.path.join(ROOT, "book-src/appendix-c-sources.md"))
    ledger = read(os.path.join(ROOT, "book-src/appendix-d-claim-ledger.md"))
    rows = 0
    for line in ledger.splitlines():
        if not line.startswith("| ") or line.startswith("| :") or line.startswith("| Claim"):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if len(cells) < 2:
            continue
        rows += 1
        source = cells[1]
        for name in re.findall(r"([A-Z][A-Za-zÀ-ÿ'\-]+)[^;|]*?\((\d{4})", source):
            surname, year = name
            if surname not in bibliography:
                fail("ledger", f"row {rows}: {surname} ({year}) has no entry in Appendix C")
    if rows == 0:
        fail("ledger", "no claim rows found")


# ------------------------------------------------------------ 7. numeral census

NUMERAL = re.compile(r"\d")
CITATION = re.compile(r"\((?:[^()]*?)\d{4}[a-z]?\)|\d{4}[a-z]?\)")
MATH = re.compile(r"\$[^$]+\$|\$\$.*?\$\$", re.DOTALL)
EDITORIAL = re.compile(r"^\s*(?:\d+\.|[-*])\s|^#{1,6}\s|^\|")


SVG_BLOCK = re.compile(r"<svg\b.*?</svg>", re.DOTALL)
HTML_BLOCK = re.compile(r"<figure\b.*?</figure>", re.DOTALL)


def census() -> int:
    flagged = 0
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        text = read(path)
        body = text.split("\n", 1)[1] if "\n" in text else ""
        body = FENCE.sub("", SVG_BLOCK.sub("", HTML_BLOCK.sub("", body)))
        for line_no, line in enumerate(body.splitlines(), 2):
            if not NUMERAL.search(line):
                continue
            if CITATION.search(line) or MATH.search(line) or EDITORIAL.search(line):
                continue
            if re.match(r"^\s*[<>|]|^\s*#", line):
                continue
            flagged += 1
            if flagged <= 25:
                print(f"  census {rel(path)}: {line.strip()[:100]}")
    return flagged



# -------------------------------------------------------- 8. contents parity

SUMMARY = re.compile(r"\[([^\]]+)\]\(([^)]+\.md)\)")


def check_contents() -> None:
    summary = read(os.path.join(ROOT, "book-src/SUMMARY.md"))
    for text, target in SUMMARY.findall(summary):
        path = os.path.join(ROOT, "book-src", target)
        if not os.path.exists(path):
            fail("contents", f"SUMMARY.md lists {target}, which does not exist")
            continue
        title = next(
            (line[2:].strip() for line in read(path).splitlines() if re.match(r"^# \S", line)),
            None,
        )
        if not title:
            fail("contents", f"{target}: no title line")
            continue
        entry = re.sub(r"^Chapter \d\d \u2014 ", "", re.sub(r"\s+", " ", text.replace("*", ""))).strip()
        heading = re.sub(r"\s+", " ", title.replace("*", "")).strip()
        heading = re.sub(r"^Chapter \d\d \u2014 ", "", heading).strip()
        if not (entry == heading or entry.startswith(heading) or heading.startswith(entry)):
            fail("contents", f"SUMMARY.md '{entry[:50]}' does not agree with '{heading[:50]}'")


# ------------------------------------------------------------ 9. concordance

# §V-9: every row of the contract's table is copied here verbatim. Changing a
# string is a change to AGENTS.md and takes the same human confirmation as a
# retitling; the gate must not be repaired by rewording the manuscript.

CONCORDANCE = [
    (
        "the triadic verdict is a minimum, never an aggregate that trades one axis against another",
        "The minimum is not a weighted sum",
        [
            "weighted average of the three",
            "weighted sum of the three scores",
            "average of the three axes",
        ],
    ),
    (
        "design effects compose additively, the product being only an upper bound",
        "the exact composition is additive",
        ["composed multiplicatively", "product of the design effects"],
    ),
    (
        "the governance coordinate fixes the sign of the differential and is not a third reading",
        "not one reading among three",
        [
            "governance coordinate is one of the three",
            "enters the minimum as a third reading",
        ],
    ),
    (
        "the stratum anchors are recorded designations, not consequences of the table",
        "designation recorded rather than derived",
        [
            "is derived from the table of orders",
            "follows from the triples",
            "strict 1-to-1 bijection",
        ],
    ),
]


def collapse(text: str) -> str:
    return re.sub(r"\s+", " ", text)


def check_concordance() -> None:
    texts = {
        path: collapse(read(path))
        for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
    }
    for proposition, canonical, variants in CONCORDANCE:
        if not any(canonical in body for body in texts.values()):
            fail("concordance", f"{proposition}: canonical string occurs in no book-src/ file -> '{canonical}'")
        for path, body in texts.items():
            for variant in variants:
                if variant in body:
                    fail("concordance", f"{rel(path)}: contradicting variant of {proposition} -> '{variant}'")


# ------------------------------------------------------- 10. guard-site count

SITES = tuple(f"T{index}" for index in range(1, 8))
TABLE_E1_HEADER = "| Guard site |"
FORBIDDEN_FOOTING = ("the two independent counts agree", "cheapest available test")


def parse_b1_rows() -> dict[str, dict]:
    rows: dict[str, dict] = {}
    path = os.path.join(ROOT, "book-src/appendix-b-architecture-verification.md")
    for line in read(path).splitlines():
        match = B1_ROW.match(line)
        if not match:
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        primary = re.search(r"T\d", cells[4])
        rows[match.group(1)] = {
            "chapter": match.group(2),
            "stratum": match.group(3),
            "primary": primary.group(0) if primary else None,
            "triple": frozenset(re.findall(r"T\d", cells[5])),
            "signature": cells[6].strip("`").replace(" ", ""),
        }
    return rows


def table_e1_sites() -> list[str] | None:
    lines = read(os.path.join(ROOT, "book-src/appendix-e-lexicon.md")).splitlines()
    start = next((index for index, line in enumerate(lines) if line.startswith(TABLE_E1_HEADER)), None)
    if start is None:
        return None
    named: list[str] = []
    for line in lines[start + 1:]:
        if not line.startswith("|"):
            break
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if not cells or cells[0].startswith(":"):
            continue
        named.append(cells[0].strip("`"))
    return named


def check_guard_sites() -> None:
    rows = parse_b1_rows()

    # (a) Table E.1 enumerates T1-T7 once each and names no eighth site.
    named = table_e1_sites()
    if named is None:
        fail("guard-sites", "appendix E: Table E.1 header '| Guard site |' not found")
    else:
        counts = Counter(named)
        for site in SITES:
            if counts[site] != 1:
                fail("guard-sites", f"appendix E: Table E.1 names {site} {counts[site]} times, expected 1")
        for site, count in sorted(counts.items()):
            if site not in SITES:
                fail("guard-sites", f"appendix E: Table E.1 names an eighth site -> '{site}' ({count})")

    # (b) every triple is a three-element subset of the seven, all 35 distinct,
    #     and each carried by exactly one order.
    if len(rows) != 35:
        fail("guard-sites", f"table B.1 has {len(rows)} rows, expected 35")
    claimed: dict[frozenset[str], list[str]] = {}
    for code, row in rows.items():
        triple = row["triple"]
        if len(triple) != 3:
            fail("guard-sites", f"{code}: triple {{{', '.join(sorted(triple))}}} has {len(triple)} elements, expected 3")
            continue
        if not triple <= set(SITES):
            fail("guard-sites", f"{code}: triple {{{', '.join(sorted(triple))}}} names a site outside T1-T7")
            continue
        claimed.setdefault(triple, []).append(code)
    for triple, codes in sorted(claimed.items(), key=lambda item: sorted(item[0])):
        if len(codes) != 1:
            fail("guard-sites", f"triple {{{', '.join(sorted(triple))}}} carried by {len(codes)} orders: {', '.join(sorted(codes))}")
    expected = {frozenset(combo) for combo in itertools.combinations(SITES, 3)}
    for triple in sorted(expected - set(claimed), key=lambda item: sorted(item)):
        fail("guard-sites", f"triple {{{', '.join(sorted(triple))}}} is carried by no order")

    # (c) each stratum carries five distinct primary sites, each site primary in
    #     exactly five strata.
    by_stratum: dict[str, list[tuple[str, str | None]]] = {}
    for code, row in rows.items():
        by_stratum.setdefault(row["stratum"], []).append((code, row["primary"]))
    site_strata: dict[str, set[str]] = {site: set() for site in SITES}
    site_orders: Counter = Counter()
    for stratum, items in sorted(by_stratum.items()):
        primaries = [primary for _, primary in items]
        if len(items) != 5:
            fail("guard-sites", f"stratum {stratum}: {len(items)} orders, expected 5")
        if len(set(primaries)) != len(items):
            fail("guard-sites", f"stratum {stratum}: primary sites are not pairwise distinct -> {sorted(primaries)}")
        for code, primary in items:
            if primary is None:
                fail("guard-sites", f"{code}: no primary site in the table row")
                continue
            site_orders[primary] += 1
            site_strata.setdefault(primary, set()).add(stratum)
    for site in SITES:
        if site_orders[site] != 5:
            fail("guard-sites", f"{site} is primary in {site_orders[site]} orders, expected 5")
        if len(site_strata.get(site, set())) != 5:
            fail("guard-sites", f"{site} is primary in {len(site_strata.get(site, set()))} strata, expected 5")

    # (d) each Part V chapter's blockquote prints the triple its row carries.
    for code, row in sorted(rows.items()):
        files = sorted(glob.glob(os.path.join(ROOT, f"book-src/{row['chapter']}-*.md")))
        if not files:
            fail("guard-sites", f"{code}: no chapter file for chapter {row['chapter']}")
            continue
        text = read(files[0])
        lines = [line for line in text.splitlines() if line.startswith("> Layer ")]
        if len(lines) != 1:
            fail("guard-sites", f"{rel(files[0])}: {len(lines)} '> Layer' blockquotes, expected 1")
            continue
        match = BLOCKQUOTE.search(lines[0])
        if not match:
            fail("guard-sites", f"{rel(files[0])}: blockquote does not parse as a Layer line")
            continue
        if "L" + match.group(1) != code:
            fail("guard-sites", f"{rel(files[0])}: blockquote declares {'L' + match.group(1)}, row {code} points here")
        quoted = frozenset(re.findall(r"T\d", match.group(4)))
        if quoted != row["triple"]:
            fail("guard-sites", f"{code}: chapter {row['chapter']} prints {{{', '.join(sorted(quoted))}}}, "
                                f"row carries {{{', '.join(sorted(row['triple']))}}}")

    # (e) no file derives the count from a second source: the footing is that the
    #     binomial ratio is injective, not that two counts meet.
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        body = collapse(read(path))
        for phrase in FORBIDDEN_FOOTING:
            if phrase in body:
                fail("guard-sites", f"{rel(path)}: forbidden footing phrase -> '{phrase}'")


# --------------------------------------------------------------------- driver

def main() -> int:
    check_apparatus()
    check_spine()
    check_links()
    check_figures()
    check_b1()
    check_ledger()
    check_contents()
    check_concordance()
    check_guard_sites()

    print("numeral census (advisory, not a gate):")
    flagged = census()
    print(f"  {flagged} candidate lines without a citation year, math span or editorial purpose")

    if failures:
        print(f"\n{len(failures)} structural failure(s):")
        for item in failures:
            print("  " + item)
        return 1
    print("\nall structural gates pass")
    return 0


if __name__ == "__main__":
    sys.exit(main())
