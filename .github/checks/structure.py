#!/usr/bin/env python3
"""Structural checks for the Stochology manuscript.

Implements the checks of AGENTS.md section V that read the sources: apparatus,
spine, links (files and #anchors), figures, table B.1 parity, ledger parity and
citation parity (surname and year on one Appendix C entry), numeral census,
contents parity, concordance (§V-9), guard-site cardinality (§V-10), and seven
readings §V does not name but the text admits: math integrity (a display or inline
span that never closes, a subscript that markdown would read as emphasis), notation
(one symbol spelled one way), attribution (an author-year call that cannot be
resolved to one Appendix C entry), falsifier parity (each collapse proposition
carries the condition under which it fails), concordance polarity (§V-9 matches a
substring, so a sentence that negates the canonical string would pass it),
collapse refutation (a stated proposition that says nothing about what would
refute it) and guard footing (seven sites derived from the wrong inventory of
transits, against §V-10(e)). Check §V-8 — the published surface — reads the HTML
mdbook builds, so it lives in surface.py and runs in the render job. Five further
readings are advisory and print rather than fail: orphan bibliography entries,
symbols first set inside a display block, near-duplicate sentences across chapters,
cross-references that name no title they could mean, and notation drift that
renders identically.

A gate may catch a defect the manuscript has not repaired yet. DECLARED_OPEN names
each such site by the key the gate would have used, so the gate prints it and fails
only on what is not declared; a declaration covers one site (a second occurrence
fails) and an entry whose defect is gone is reported as stale.

There is nothing to compile here: the repository is prose plus figures, and every
gate below is a reading of the text. Exit code 0 means no gate failed; the
numeral census only reports.
"""

from __future__ import annotations

import glob
import html
import itertools
import os
import re
import sys
import unicodedata
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
HEADING = re.compile(r"^(#{1,6})\s+(.*?)\s*#*\s*$", re.MULTILINE)
HTML_ID = re.compile(r"""\bid=["']([^"']+)["']""")


def heading_slug(heading: str) -> str:
    """The id mdbook gives a heading: its text lowercased, whitespace to '-',
    everything but alphanumerics, '_' and '-' dropped. Checked against the
    rendered book, where it reproduces every heading id."""
    text = re.sub(r"!?\[([^\]]*)\]\([^)]*\)", r"\1", heading)
    text = re.sub(r"<[^>]+>", "", text)
    text = html.unescape(text.replace("`", "").replace("*", ""))
    slug = []
    for char in text.strip().lower():
        if char.isalnum() or char in "_-":
            slug.append(char)
        elif char.isspace():
            slug.append("-")
    return "".join(slug)


_anchor_cache: dict[str, set[str]] = {}


def anchors(path: str) -> set[str]:
    """Every fragment a link may target in a file: heading ids, with mdbook's
    -1, -2 suffixes on repeats, plus explicit HTML id attributes."""
    if path not in _anchor_cache:
        text = FENCE.sub("", read(path))
        found: set[str] = set(HTML_ID.findall(text))
        repeats: Counter = Counter()
        for match in HEADING.finditer(text):
            slug = heading_slug(match.group(2))
            found.add(slug if repeats[slug] == 0 else f"{slug}-{repeats[slug]}")
            repeats[slug] += 1
        _anchor_cache[path] = found
    return _anchor_cache[path]


def check_links() -> None:
    targets = sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
    targets += [os.path.join(ROOT, name) for name in ROOT_DOCS]
    for path in targets:
        for match in LINK.finditer(read(path)):
            target, _, fragment = match.group(1).partition("#")
            if target.startswith(("http:", "https:", "mailto:")):
                continue
            if target:
                resolved = os.path.normpath(
                    os.path.join(os.path.dirname(path), urllib.parse.unquote(target))
                )
                if not os.path.exists(resolved):
                    fail("links", f"{rel(path)}: {target} does not resolve")
                    continue
            else:
                resolved = path
            if not fragment or not resolved.endswith(".md"):
                continue
            if urllib.parse.unquote(fragment) not in anchors(resolved):
                fail("links", f"{rel(path)}: #{fragment} names no heading or id in {rel(resolved)}")


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

# An entry is a bold author key followed by the first parenthesis that carries a
# year: "- **Surname, X., & Other, Y.** (2004)." or, for edited volumes,
# "- **Surname, X.** (Eds.). (2008).". Entries dated only by era (c. 400 BCE) or
# by a law report have no four-digit year and cannot be cited by one.
BIB_ENTRY = re.compile(
    r"^- \*\*(.+?)\*\*(?: \(Eds?\.\)\.)? \(([^)]*\d{4}[^)]*)\)", re.MULTILINE
)


def fold(text: str) -> str:
    """Accent-insensitive form, so 'Holmström' in prose meets 'Holmström' in a key
    whichever normalisation either file was saved in."""
    return unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode()


_bibliography: list[tuple[str, set[str]]] = []


def bibliography() -> list[tuple[str, set[str]]]:
    if not _bibliography:
        text = read(os.path.join(ROOT, "book-src/appendix-c-sources.md"))
        for key, years in BIB_ENTRY.findall(text):
            _bibliography.append((fold(key), set(re.findall(r"\d{4}", years))))
    return _bibliography


def has_entry(surname: str, year: str) -> bool:
    """True when one entry carries both the surname, as a whole word of its
    author key, and the year. A surname elsewhere in the file and a year on a
    different entry do not add up to a source."""
    name = fold(re.sub(r"['’]s$", "", surname))
    word = re.compile(r"(?<![A-Za-z])" + re.escape(name) + r"(?![a-z])")
    return any(year in years and word.search(key) for key, years in bibliography())


def check_ledger() -> None:
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
        for surname, year in re.findall(r"([A-Z][A-Za-zÀ-ÿ'\-]+)[^;|]*?\((\d{4})", source):
            if not has_entry(surname, year):
                fail("ledger", f"row {rows}: {surname} ({year}) has no entry in Appendix C with that surname and year")
    if rows == 0:
        fail("ledger", "no claim rows found")


# ------------------------------------------------------- 6b. citation parity

# Every author-year citation in the prose must name an Appendix C entry that
# carries the same surname and the same year. Two forms are read:
#   narrative      Surname (2004), Surname's (2004), Surname et al. (2004),
#                  Surname and Other (2004), A, B and C (2004)
#   parenthetical  (Surname 2004), (Surname & Other 2004; Third et al. 1999)
# A narrative citation is satisfied when any capitalised word of its author run,
# or the word just before it, matches: "the Lighthill Report (1973)" names
# Lighthill, and "Kish weighting and Chao's (1984)" names Chao.

CITE_NAME = r"(?:Mc|Mac|De|Di|La|Le)?[A-ZÀ-Ý][a-zà-ÿ'’]+(?:-[A-ZÀ-Ý]?[a-zà-ÿ'’]+)*"
CITE_NARRATIVE = re.compile(
    r"(?:(" + CITE_NAME + r")\s+)?((?:" + CITE_NAME + r")(?:(?:,\s+" + CITE_NAME + r")*,?\s+(?:and|&)\s+"
    + CITE_NAME + r"|\s+et al\.)?)(?:'s|’s)?\s+\((\d{4})[a-z]?(?:[,;][^)]*)?\)"
)
CITE_GROUP = re.compile(r"\(([^()]*\d{4}[^()]*)\)")
CITE_ITEM = re.compile(r"^\s*(?:see |e\.g\.,? |cf\. )?(" + CITE_NAME + r")[^;]*?\s(\d{4})[a-z]?\s*$")
NOT_AUTHORS = set(
    "Chapter Chapters Layer Layers Appendix Part Table Figure Section Stratum The A In On Of For See By At "
    "And Or If As January February March April May June July August September October November December".split()
)
CITATION_EXEMPT = ("appendix-c-sources.md", "appendix-d-claim-ledger.md", "SUMMARY.md")


def blank_block(match: re.Match) -> str:
    return "\n" * match.group(0).count("\n")


def citation_text(text: str) -> str:
    """Prose only, with line numbers preserved: figures, fences and math removed,
    link targets dropped so a file name never reads as an author."""
    for pattern, flags in (
        (r"<svg\b.*?</svg>", re.DOTALL),
        (r"<figure\b.*?</figure>", re.DOTALL),
        (r"^```.*?^```", re.DOTALL | re.MULTILINE),
        (r"\$\$.*?\$\$", re.DOTALL),
    ):
        text = re.sub(pattern, blank_block, text, flags=flags)
    text = re.sub(r"\$[^$\n]+\$", "", text)
    return re.sub(r"\]\([^)]*\)", "]", text)


def check_citations() -> None:
    targets = sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
    targets += [os.path.join(ROOT, name) for name in ("README.md", "CONTRIBUTING.md")]
    for path in targets:
        if path.endswith(CITATION_EXEMPT):
            continue
        for line_no, line in enumerate(citation_text(read(path)).splitlines(), 1):
            for match in CITE_NARRATIVE.finditer(line):
                before, run, year = match.group(1), match.group(2), match.group(3)
                names = [name for name in re.findall(CITE_NAME, run) if name not in NOT_AUTHORS]
                if not names:
                    continue
                if before and before not in NOT_AUTHORS:
                    names.append(before)
                if not any(has_entry(name, year) for name in names):
                    fail("citations", f"{rel(path)}:{line_no}: {run} ({year}) matches no Appendix C entry by surname and year")
            for group in CITE_GROUP.finditer(line):
                for part in group.group(1).split(";"):
                    item = CITE_ITEM.match(part)
                    if not item or item.group(1) in NOT_AUTHORS:
                        continue
                    if not has_entry(item.group(1), item.group(2)):
                        fail("citations", f"{rel(path)}:{line_no}: ({part.strip()}) matches no Appendix C entry by surname and year")


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


# ----------------------------------------------- 9b. declared open defects

# A gate can catch a defect the manuscript has not repaired yet. Shipping it as
# a failure would leave the tree red for everyone working beside it; omitting it
# would leave the defect unread. A declared entry names the defect by the key the
# gate would have used, so the gate prints it and only fails on what is not
# declared. The list cannot outlive its reason: an entry whose defect is gone is
# reported as stale and removed by the next pass over this file.
DECLARED_OPEN: dict[str, set[tuple[str, str]]] = {
    # A declaration is a debt against the manuscript, not a standing exemption.
    # The four year sites this dict carried were paid in Rodada 4: the year either
    # acquired its source in the same paragraph ("Rumelhart, Hinton & Williams
    # 1986", "Minsky & Papert 1969", "Neustadt & Fineberg 1983") or lost the year
    # as decor ("the 1987 market crash" -> "rules that sell as the market falls").
    # The gate now fails on any year in prose whose paragraph names no source.
    #
    #
    # Audit A1 (audit/audit-partV-algebra.md §7) declared the option hurdle in
    # chapter 76 here while nothing fixed $r > \alpha$. Rodada 8 paid it in the
    # text: the sentence now states the yield and the condition under which the
    # hurdle is defined, so the site is no longer declared and the ratio-limit
    # gate reads every site in book-src/ with no exemption.
}
declared_seen: set[tuple[str, str, str]] = set()


def declare(gate: str, path: str, key: str) -> bool:
    """One declaration covers one site: a second occurrence of the same key is a
    new defect and fails, so a declared defect cannot become a licence."""
    entry = (rel(path), key)
    if entry in DECLARED_OPEN.get(gate, set()) and (gate, entry[0], entry[1]) not in declared_seen:
        declared_seen.add((gate, entry[0], entry[1]))
        print(f"  declared open [{gate}] {rel(path)}: {key}")
        return True
    return False


def report_stale_declarations() -> None:
    stale = [
        (gate, entry)
        for gate, entries in DECLARED_OPEN.items()
        for entry in entries
        if (gate, entry[0], entry[1]) not in declared_seen
    ]
    if stale:
        print(f"\ndeclared defects no longer present (remove the entry): {len(stale)}")
        for gate, (path, key) in stale:
            print(f"  [{gate}] {path}: {key}")


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


# ------------------------------------------------------- 11. math integrity

# §V says the repository is prose and every gate reads it; math that does not
# close is the one defect that silently turns a displayed identity into prose.
# Three readings, each mechanical: '$$' inside a file is even, '$' inside a
# paragraph is even (inline math may wrap a line, so the paragraph is the unit),
# and no '_' or '^' survives outside math, where markdown reads '_' as emphasis.
DISPLAY_BLOCK = re.compile(r"\$\$.*?\$\$", re.DOTALL)
INLINE_SPAN = re.compile(r"\$[^$]+\$", re.DOTALL)
ESCAPED_DOLLAR = "\u00a4"


def blank_lines(match: re.Match) -> str:
    """Remove a match but keep its newlines, so line numbers survive."""
    return "\n" * match.group(0).count("\n")


def without_math(text: str) -> str:
    """A line-preserving copy of what markdown reads as prose: escaped dollars
    are neutralised first, so a literal '$5' never opens a span."""
    text = text.replace("\\$", ESCAPED_DOLLAR)
    text = DISPLAY_BLOCK.sub(blank_lines, text)
    text = INLINE_SPAN.sub(blank_lines, text)
    text = re.sub(r"`[^`]*`", blank_lines, text)
    return SVG_BLOCK.sub(blank_lines, text)


def check_math() -> None:
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        text = read(path)
        delimiters = text.count("$$")
        if delimiters % 2:
            fail("math", f"{rel(path)}: {delimiters} '$$' delimiters -> a display block never closes")
        body = DISPLAY_BLOCK.sub(blank_lines, text).replace("\\$", ESCAPED_DOLLAR)
        paragraph: list[str] = []
        opened = 0
        for line_no, line in enumerate(body.splitlines() + [""], 1):
            if line.strip():
                if not paragraph:
                    opened = line_no
                paragraph.append(line)
                continue
            if paragraph:
                if "\n".join(paragraph).count("$") % 2:
                    fail("math", f"{rel(path)}:{opened}: paragraph carries an odd number of '$' -> inline math unclosed")
                paragraph = []
        for line_no, line in enumerate(without_math(text).splitlines(), 1):
            stray = re.search(r"(?<!\\)[_^]", line)
            if stray:
                fail("math", f"{rel(path)}:{line_no}: unescaped '{stray.group(0)}' outside math -> {line.strip()[:80]}")

    # A display block is math, not a fence: an odd '$' inside one is a second
    # block opened and never closed.
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        for match in DISPLAY_BLOCK.finditer(read(path)):
            inner = match.group(0)[2:-2]
            if inner.count("$") % 2:
                line_no = read(path).count("\n", 0, match.start()) + 1
                fail("math", f"{rel(path)}:{line_no}: display block carries an odd number of '$'")


# ------------------------------------------------------------ 12. notation

# One symbol, one spelling. The canonical forms are the ones the manuscript
# already uses; each variant below changes the symbol's identity (an unescaped
# '_' reaches markdown as emphasis, '\text' and the bare subscript set the name
# in a different face, a swapped argument order names a different correlation).
# Whitespace inside a brace group is not listed: it renders identically, so it
# is reported and not failed.
NOTATION = [
    ("the epistemic margin coordinate", [
        r"S_(?=\{\\mathrm\{epi)",
        r"S\\_\{\\text\{epi",
        r"S\\_\{epi\}",
        r"S\\_\{\\mathrm\{Ep",
    ]),
    ("the vital margin coordinate", [
        r"S_(?=\{\\mathrm\{vital)",
        r"S\\_\{\\text\{vital",
        r"S\\_\{vital\}",
    ]),
    ("the governance coordinate", [
        r"S_(?=\{\\mathrm\{gov)",
        r"S\\_\{\\text\{gov",
        r"S\\_\{gov\}",
    ]),
    ("the archive correlation", [
        r"\\rho_(?=\{R,Y\})",
        r"\\rho\\_\{\\text\{R,Y",
        r"\\rho\\_\{Y,\s*R\}",
    ]),
    ("the Leontief minimum", [
        r"M_(?=\{\\mathrm\{Leontief)",
        r"M\\_\{\\text\{Leontief",
        r"M\\_\{Leontief\}",
        r"M\\_\{\\mathrm\{leontief",
    ]),
]
NOTATION_IN_MATH = ("DEFF", "TTS", "TTR")


def math_blocks(text: str) -> list[tuple[int, str]]:
    """(line number, body) for every display and inline math span."""
    found = [(match.start(), match.group(0)) for match in DISPLAY_BLOCK.finditer(text)]
    body = DISPLAY_BLOCK.sub(blank_lines, text)
    found += [(match.start(), match.group(0)) for match in INLINE_SPAN.finditer(body)]
    return [(text.count("\n", 0, start) + 1, span) for start, span in sorted(found)]


def check_notation() -> None:
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        text = read(path)
        for symbol, variants in NOTATION:
            for variant in variants:
                for match in re.finditer(variant, text):
                    line_no = text.count("\n", 0, match.start()) + 1
                    fail("notation", f"{rel(path)}:{line_no}: {symbol} written as '{match.group(0)}'")
        for line_no, span in math_blocks(text):
            bare = re.sub(r"\\mathrm\{[^{}]*\}", "", span)
            for token in NOTATION_IN_MATH:
                if re.search(r"(?<![A-Za-z])" + token + r"(?![a-z])", bare):
                    fail("notation", f"{rel(path)}:{line_no}: {token} set without \\mathrm inside math")


# -------------------------------------------------------- 13. attribution

# Two entries under one surname and one year are indistinguishable to a reader
# who is given only 'Surname (year)'. The citation resolves them by naming a
# co-author that belongs to exactly one of the two, and that is mechanical.

def bibliography_keys() -> list[tuple[str, str, set[str]]]:
    """(first-author surname, year, co-author surnames) per Appendix C entry."""
    text = read(os.path.join(ROOT, "book-src/appendix-c-sources.md"))
    entries: list[tuple[str, str, set[str]]] = []
    for key, years in BIB_ENTRY.findall(text):
        folded = fold(key)
        first = re.split(r",| and | & ", folded)[0].strip()
        authors = [name for name in re.findall(CITE_NAME, folded) if name != first]
        for year in re.findall(r"\d{4}", years):
            entries.append((first, year, set(authors)))
    return entries


def ambiguous_entries() -> dict[tuple[str, str], list[set[str]]]:
    table: dict[tuple[str, str], list[set[str]]] = {}
    for surname, year, coauthors in bibliography_keys():
        table.setdefault((surname, year), []).append(coauthors)
    return {pair: entries for pair, entries in table.items() if len(entries) > 1}


def citation_calls(text: str) -> list[tuple[int, list[str], str, str]]:
    """(line number, author surnames, year, suffix) for every author-year call,
    narrative and parenthetical, on prose with math and figures removed."""
    calls: list[tuple[int, list[str], str, str]] = []
    for line_no, line in enumerate(citation_text(text).splitlines(), 1):
        for match in CITE_NARRATIVE.finditer(line):
            names = [name for name in re.findall(CITE_NAME, match.group(2)) if name not in NOT_AUTHORS]
            if match.group(1) and match.group(1) not in NOT_AUTHORS:
                names.append(match.group(1))
            calls.append((line_no, names, match.group(3), ""))
        for group in CITE_GROUP.finditer(line):
            for part in group.group(1).split(";"):
                item = CITE_ITEM.match(part)
                if not item or item.group(1) in NOT_AUTHORS:
                    continue
                names = [name for name in re.findall(CITE_NAME, part) if name not in NOT_AUTHORS]
                suffix = re.search(r"\b" + item.group(2) + r"([a-z])\b", part)
                calls.append((line_no, names, item.group(2), suffix.group(1) if suffix else ""))
    return calls


def check_attribution() -> None:
    ambiguous = ambiguous_entries()
    if not ambiguous:
        return
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        if path.endswith(CITATION_EXEMPT):
            continue
        for line_no, names, year, suffix in citation_calls(read(path)):
            for name in names:
                entries = ambiguous.get((fold(name), year))
                if not entries or suffix:
                    continue
                named = [entry for entry in entries if entry & {fold(other) for other in names}]
                if len(named) != 1:
                    fail("attribution", f"{rel(path)}:{line_no}: {name} ({year}) is carried by "
                                        f"{len(entries)} Appendix C entries and the call names no co-author "
                                        f"that separates them")


# --------------------------------------------------------- 14. falsifiers

# §I states each collapse proposition so that it could fail. Chapter 50 seals
# the four falsifiers in one place, one per component of the framework; a
# falsifier that loses its failure condition, or two that test one component,
# is a proposition that cannot fail and a component that is not tested.
FALSIFIER_SECTION = "The Four Canonical Falsifiers"
FALSIFIER_COMPONENTS = {
    r"S\_{\mathrm{epi}}": "the epistemic margin",
    r"S\_{\mathrm{vital}}": "the vital margin",
    r"S\_{\mathrm{gov}}": "the governance coordinate",
    r"M\_{\mathrm{Leontief}}": "the Leontief minimum",
}


def check_falsifiers() -> None:
    located = False
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        lines = read(path).splitlines()
        start = next(
            (index for index, line in enumerate(lines)
             if line.startswith("#") and FALSIFIER_SECTION in line),
            None,
        )
        if start is None:
            continue
        located = True
        items: list[tuple[int, int, str]] = []
        for index in range(start + 1, len(lines)):
            if re.match(r"^#{2,4} ", lines[index]):
                break
            numbered = re.match(r"^(\d+)\.\s+(.*)$", lines[index])
            if numbered:
                items.append((index + 1, int(numbered.group(1)), numbered.group(2)))
        if len(items) != 4:
            fail("falsifiers", f"{rel(path)}:{start + 1}: {len(items)} falsifiers under "
                               f"'{FALSIFIER_SECTION}', expected 4")
            continue
        carried: set[str] = set()
        for ordinal, (line_no, number, body) in enumerate(items, 1):
            if number != ordinal:
                fail("falsifiers", f"{rel(path)}:{line_no}: falsifier {ordinal} numbered {number}")
            if not re.search(r"\$F\\_" + str(ordinal) + r"\$", body):
                fail("falsifiers", f"{rel(path)}:{line_no}: item {ordinal} does not name F_{ordinal}")
            if "fails if" not in body:
                fail("falsifiers", f"{rel(path)}:{line_no}: F_{ordinal} states no condition under which it fails")
            named = [component for component in FALSIFIER_COMPONENTS if component in body]
            if len(named) != 1:
                fail("falsifiers", f"{rel(path)}:{line_no}: item {ordinal} names {len(named)} "
                                   f"framework components, expected 1")
                continue
            if named[0] in carried:
                fail("falsifiers", f"{rel(path)}:{line_no}: {FALSIFIER_COMPONENTS[named[0]]} "
                                   f"is tested by more than one falsifier")
            carried.add(named[0])
        for component in FALSIFIER_COMPONENTS:
            if component not in carried:
                fail("falsifiers", f"{rel(path)}: no falsifier tests {FALSIFIER_COMPONENTS[component]}")
    if not located:
        fail("falsifiers", f"no chapter carries the '{FALSIFIER_SECTION}' section")


# ------------------------------------------- 14b. concordance polarity

# §V-9 matches literal substrings, so a sentence can print the canonical string
# and assert its opposite in the same breath: "not one reading among three, but
# it does enter the minimum as one reading among three" matches the canonical and
# is its negation. The opposite of each canonical string is derived from the
# string itself, so the table of §V-9 is read and not extended.

def opposite_of(canonical: str) -> str | None:
    """The polarity flip of a canonical string, derived, not authored."""
    match = re.search(r"\b(is|are) not\b", canonical)
    if match:
        return re.sub(r"\b(is|are) not\b", r"\1", canonical)
    if re.match(r"^not\b", canonical):
        return re.sub(r"^not\b\s*", "", canonical)
    match = re.search(r"\b(\w+) rather than (\w+)\b", canonical)
    if match:
        return canonical[:match.start()] + f"{match.group(2)} rather than {match.group(1)}" + canonical[match.end():]
    match = re.search(r"\b(is|are)\b", canonical)
    if match:
        return canonical[:match.end()] + " not" + canonical[match.end():]
    return None


CLAUSE_BOUNDARY = re.compile(r"[.;:—–]|,\s")


def check_concordance_polarity() -> None:
    texts = {
        path: collapse(read(path))
        for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
    }
    for proposition, canonical, _variants in CONCORDANCE:
        opposite = opposite_of(canonical)
        if opposite is None:
            fail("concordance-polarity", f"{proposition}: no polarity flip derivable from '{canonical}'")
            continue
        for path, body in texts.items():
            for match in re.finditer(re.escape(opposite), body):
                back = body[max(0, match.start() - 120):match.start()]
                boundaries = [found.end() for found in CLAUSE_BOUNDARY.finditer(back)]
                clause = back[boundaries[-1]:] if boundaries else back
                if re.search(r"\b(not|never|cannot|n't)\b", clause):
                    continue
                fail("concordance-polarity",
                     f"{rel(path)}: asserts the opposite of {proposition} -> '{body[max(0, match.start() - 40):match.end() + 20].strip()}'")


# ------------------------------------------- 14c. collapse refutation

# §I states each collapse proposition so that it could fail. The propositions are
# stated in the chapters the contract names for them; a statement with no
# condition under which it is refuted is a proposition that cannot fail, and §I
# says what that costs. The site is the paragraph that opens on the proposition,
# and that paragraph is the whole of what is read: a refutation carried in a later
# sentence of the same section, or in a restatement that does not open on the bold
# form, lies outside this gate's reach.
REFUTATION = re.compile(
    r"\bfails? if\b|\bwould be (?:false|refuted)\b|\brefut(?:ed|es|ing)\b|"
    r"\bfalsif(?:ied|ies|ication)\b|\bis refuted\b|\bwould show\b[^.]{0,60}\bfalse\b"
)


def contract_chapters(topic: str) -> set[str]:
    """Chapter numbers the contract's §II names for a topic, read from AGENTS.md."""
    body = collapse(read(os.path.join(ROOT, "AGENTS.md")))
    for part in re.split(r"(?<=[.;])\s", body):
        named = set(re.findall(r"`(\d{2})`", part))
        if topic in part.lower() and named:
            return named
    return set()


def check_collapse_refutation() -> None:
    chapters = contract_chapters("collapse propositions")
    if not chapters:
        fail("collapse-refutation", "the contract names no chapter for the collapse propositions")
        return
    opener = re.compile(r"(?:^|\n\n)(\*\*Collapse Proposition (\d))")
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        if os.path.basename(path)[:2] not in chapters:
            continue
        text = read(path)
        for match in opener.finditer(text):
            ordinal = match.group(2)
            start = match.start(1)
            end = text.find("\n\n", start)
            paragraph = text[start: end if end > 0 else len(text)]
            if REFUTATION.search(paragraph):
                continue
            line_no = text.count("\n", 0, start) + 1
            if declare("collapse-refutation", path, f"Collapse Proposition {ordinal}"):
                continue
            fail("collapse-refutation",
                 f"{rel(path)}:{line_no}: Collapse Proposition {ordinal} carries no sentence "
                 f"saying what would refute it")


# --------------------------------------------- 14d. guard-site footing

# §V-10 fixes the footing: seven is six ordered crossings plus the closure,
# because n = P(P-1)+1 at three planes. Calling the circuit "three transits"
# derives the seven from the wrong number, and it is the one footing error a
# substring ban does not catch, because the phrase names no count at all.
FORBIDDEN_TRANSITS = re.compile(r"three transits of the [^.]{0,60}?circuit", re.IGNORECASE)


def check_guard_footing() -> None:
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        for match in FORBIDDEN_TRANSITS.finditer(collapse(read(path))):
            if declare("guard-footing", path, collapse(match.group(0))):
                continue
            fail("guard-footing", f"{rel(path)}: the circuit is credited with 'three transits' -> "
                                  f"'{collapse(match.group(0))}'")


# ------------------------------------------------------------ 15. reports

# Four readings below decide nothing: each is a judgement about prose that a
# gate cannot make without a false positive it cannot justify. They print and
# the reader decides. §V-7 sets the precedent (the numeral census).

NOTATION_DRIFT = [
    (r"\\rho\\_\{R, Y\}", "the archive correlation with a space after the comma"),
    (r"(?<!\\)[A-Za-z]_\{", "a symbol subscript with an unescaped underscore"),
]


def report_orphan_entries() -> list[str]:
    """An Appendix C entry no passage cites. A bibliography may legitimately be
    wider than its text, so this reports and does not fail."""
    def prose(path: str) -> str:
        text = read(path)
        for pattern in (DISPLAY_BLOCK, SVG_BLOCK, HTML_BLOCK, FENCE):
            text = pattern.sub(blank_lines, text)
        # inline math may wrap a line, so the span removal must cross newlines;
        # a span that is never closed is left alone rather than eating the prose
        # between it and the next dollar.
        text = re.sub(r"\$[^$]*?\$", blank_lines, text, flags=re.DOTALL)
        return re.sub(r"\]\([^)]*\)", "]", text)

    corpus = fold(" ".join(
        prose(path)
        for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md")))
        if not path.endswith(CITATION_EXEMPT)
    ))
    words = Counter(re.findall(r"[a-z0-9]+", corpus))
    orphans = []
    for surname, _year, _coauthors in bibliography_keys():
        parts = re.findall(r"[a-z0-9]+", surname)
        if parts and not all(words[part] for part in parts):
            orphans.append(surname)
    return sorted(set(orphans))


GENERIC_SUBSCRIPTS = {
    "eff", "rel", "req", "obs", "post", "total", "crit", "plan", "corrob", "tail", "treat",
    "odds", "true", "global", "discounted", "undiscounted", "coupled", "survival", "restore",
    "retrain", "debiased", "wellformed", "tolerable", "cumulative", "disp", "spill", "vol",
    "sym", "lin", "mm", "tol", "esc", "fs", "oper", "buff", "buf", "consent", "voice",
}


def report_undefined_symbols() -> list[str]:
    """A display block whose subscript or Greek symbol never appears in the
    chapter's prose: the symbol may be defined in an earlier block, so
    proximity alone does not decide it."""
    greek = set("alpha beta gamma delta epsilon varepsilon theta lambda mu rho sigma tau phi "
                "psi chi eta kappa pi omega nu xi zeta".split())
    findings = []
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/[0-9][0-9]-*.md"))):
        text = read(path)
        prose = DISPLAY_BLOCK.sub("", text)
        for match in DISPLAY_BLOCK.finditer(text):
            symbols = set()
            for command, subscript in re.findall(r"\\([A-Za-z]+)|\_\{?([A-Za-z][A-Za-z0-9]*)", match.group(0)):
                if command in greek:
                    symbols.add("\\" + command)
                elif subscript:
                    symbols.add(subscript)
            # a multi-letter name set in Roman is a symbol too: an abbreviation
            # that appears in no prose line was introduced inside the display.
            symbols |= {name for name in re.findall(r"\\mathrm\{([A-Za-z][A-Za-z0-9]{2,})\}", match.group(0))
                        if name.lower() not in GENERIC_SUBSCRIPTS}
            missing = sorted(symbol for symbol in symbols if symbol not in prose)
            if missing:
                line_no = text.count("\n", 0, match.start()) + 1
                findings.append(f"{rel(path)}:{line_no}: {', '.join(missing[:4])} appears in no prose line")
    return findings


NEAR_DUPLICATE_TEMPLATE = (
    "It builds on [Layer", "It presupposes >", "The order this chapter carries",
    "> Layer ", "This chapter's guard", "stoch-logo-figure",
)


def report_near_duplicates(threshold: float = 0.9) -> list[str]:
    """Sentences repeated across chapters. Part V's chapters are built from one
    template, so the template sentences are excluded before comparing: what
    remains is a proposition said twice, which is how a contradiction starts.
    Candidates are found through a trigram index, so the pass stays linear in
    the number of sentences rather than quadratic."""
    import difflib

    sentences: list[tuple[str, str]] = []
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/[0-9][0-9]-*.md"))):
        text = read(path)
        for pattern in (DISPLAY_BLOCK, SVG_BLOCK, HTML_BLOCK, FENCE):
            text = pattern.sub(blank_lines, text)
        text = INLINE_SPAN.sub(blank_lines, text)
        for sentence in re.split(r"(?<=[.!?])\s+", text):
            sentence = re.sub(r"\s+", " ", sentence).strip()
            if len(sentence) < 140 or any(marker in sentence for marker in NEAR_DUPLICATE_TEMPLATE):
                continue
            sentences.append((path, sentence))

    def trigrams(sentence: str) -> set[str]:
        words = re.findall(r"[a-z]{2,}", sentence.lower())
        return {" ".join(words[index:index + 3]) for index in range(len(words) - 2)}

    index: dict[str, list[int]] = {}
    grams = [trigrams(sentence) for _path, sentence in sentences]
    for position, gram_set in enumerate(grams):
        for gram in gram_set:
            index.setdefault(gram, []).append(position)

    findings: list[str] = []
    seen: set[tuple[int, int]] = set()
    for position, (_path, sentence) in enumerate(sentences):
        candidates: Counter = Counter()
        for gram in grams[position]:
            for other in index[gram]:
                if other > position:
                    candidates[other] += 1
        for other, shared in candidates.most_common():
            left_path, left = sentences[position]
            right_path, right = sentences[other]
            if left_path == right_path:
                continue
            # a trigram overlap below half the shorter set never reaches 0.9
            if shared * 2 < min(len(grams[position]), len(grams[other])):
                continue
            if abs(len(left) - len(right)) > 0.2 * max(len(left), len(right)):
                continue
            key = (position, other)
            if key in seen:
                continue
            seen.add(key)
            ratio = difflib.SequenceMatcher(None, left, right).ratio()
            if ratio > threshold:
                findings.append(f"{ratio:.2f} {rel(left_path)} ~ {rel(right_path)}: {left[:70]}")
    return findings


def report_cross_references() -> list[str]:
    """A 'Chapter NN' whose title shares no content word with the sentence that
    invokes it. Range citations ('Chapters 43-45') and part-level references
    name a span and not a title, so they are read as such."""
    titles: dict[str, set[str]] = {}
    named: dict[str, str] = {}
    stop = set("the a an of and or in on to for with from by as is are be at into its their "
               "this that what how why not no than then under over between after before more "
               "most less least".split())
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/[0-9][0-9]-*.md"))):
        number = os.path.basename(path)[:2]
        heading = re.search(r"^# (.+)$", read(path), re.MULTILINE)
        titles[number] = {word for word in re.findall(r"[a-z]{4,}", heading.group(1).lower()) if word not in stop} if heading else set()
        named[number] = heading.group(1).strip() if heading else "(untitled)"
    findings = []
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/[0-9][0-9]-*.md"))):
        for line_no, line in enumerate(read(path).splitlines(), 1):
            for match in re.finditer(r"\bChapter (\d{2})\b", line):
                number = match.group(1)
                if number not in titles:
                    continue
                content = {word for word in re.findall(r"[a-z]{4,}", line.lower()) if word not in stop}
                if not content & titles[number]:
                    findings.append(f"{rel(path)}:{line_no}: Chapter {number} "
                                    f"'{named[number][:48]}' -> {line.strip()[:60]}")
    return findings


def report_notation_drift() -> list[str]:
    findings = []
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        text = read(path)
        for variant, label in NOTATION_DRIFT:
            for match in re.finditer(variant, text):
                line_no = text.count("\n", 0, match.start()) + 1
                findings.append(f"{rel(path)}:{line_no}: {label} -> '{match.group(0)}'")
    return findings


def report() -> None:
    for title, producer in (
        ("orphan Appendix C entries (advisory: a bibliography may be wider than its text)", report_orphan_entries),
        ("symbols first set inside a display block (advisory: an abbreviation may be expanded in words)", report_undefined_symbols),
        ("near-duplicate sentences across chapters (advisory: template sentences excluded)", report_near_duplicates),
        ("cross-references sharing no content word with their sentence (advisory)", report_cross_references),
        ("notation drift that renders identically (advisory)", report_notation_drift),
    ):
        findings = producer()
        print(f"\n{title}: {len(findings)}")
        for item in findings[:12]:
            print("  " + item)
        if len(findings) > 12:
            print(f"  ... {len(findings) - 12} more")


# -------------------------------------------------- 16. year without citation

# A year in prose that no citation pair covers is a date the sentence asserts and
# no source supports — the failure mode §IV names, and the one the numeral census
# cannot see, because the census accepts "1878)" as a citation year. The unit is
# the paragraph: a citation anywhere in it covers the dates in it, since a reader
# takes the paragraph's sources for the paragraph.
#
# Two shapes are excluded because they are not date claims about the world: a
# decade ("the 1990s") and a table cell, where the year belongs to a row whose
# source sits in another column. Dates inside math, links, code, display blocks
# and headings are not prose and are removed before the reading.

YEAR_TOKEN = (
    r"(?:1[5-9]\d\d|20\d\d)[a-z]?"
    r"(?:(?:\s*[/\u2013-]\s*|\s*,\s*)(?:1[5-9]\d\d|20\d\d)[a-z]?)*"
)
YEAR = re.compile(r"(?<![\d.])" + YEAR_TOKEN + r"(?![\d])")
DECADE = re.compile(r"^(?:1[5-9]\d\d|20\d\d)s$")
YEAR_AUTHOR = r"[A-Z][A-Za-z'\u00c0-\u017f-]{1,}"
PAREN_OPEN = re.compile(r"\(")


def paren_year_attributed(prose: str) -> bool:
    """A parenthetical holding a date, with a name inside it or just before it."""
    for match in PAREN_OPEN.finditer(prose):
        inside = re.split(r"\)", prose[match.end():match.end() + 100])[0]
        if not YEAR.search(inside):
            continue
        before = re.split(r"[.;:!?]\s", prose[max(0, match.start() - 100):match.start()])[-1]
        for token in re.findall(YEAR_AUTHOR, before + " " + inside):
            if token not in YEAR_NOT_NAMES:
                return True
    return False


# A date is attributed when a name stands next to it, with or without parentheses:
# "Bernoulli's Ars Conjectandi, published ... in 1713" and "In 1961, John Kingman
# proved" are citations the parenthetical shapes miss. Function words are not
# names — "The 1878 Act" names an act, not an author, so it is not exempt.
YEAR_NOT_NAMES = {
    "A", "All", "An", "And", "Any", "Appendix", "As", "At", "But", "By", "Chapter",
    "Each", "Every", "Figure", "Five", "For", "Four", "From", "His", "Her", "If",
    "In", "It", "Its", "Layer", "No", "Not", "Of", "On", "One", "Or", "Our", "Part",
    "Section", "Seven", "She", "Six", "So", "Some", "Table", "That", "The", "Their",
    "Then", "There", "These", "They", "This", "Those", "Three", "Thus", "To", "Two",
    "What", "When", "Where", "Which", "While", "With",
}
YEAR_MONTHS = re.compile(
    r"(?:January|February|March|April|May|June|July|August|September|October|"
    r"November|December)\s+$"
)


def year_has_name(prose: str, start: int) -> bool:
    before = prose[max(0, start - 90):start]
    if YEAR_MONTHS.search(before):
        return False
    names = [n for n in re.findall(YEAR_AUTHOR, before) if n not in YEAR_NOT_NAMES]
    return bool(names)


def year_has_following_name(prose: str, end: int) -> bool:
    after = prose[end:end + 60]
    return bool(
        re.match(
            r"\)?[^.;:]{0,90}?(?:[A-Z]\.\s*)*[A-Z][A-Za-z'\u00c0-\u017f-]{2,}",
            after,
        )
    )

YEAR_PARAGRAPH = re.compile(r"(?:[^\n]+\n?)+(?=\n\s*\n|\Z)")
YEAR_TABLE_ROW = re.compile(r"^\s*\|")


def year_prose(text: str) -> str:
    """Blank what is not prose, keeping every offset where it was."""
    for pattern in (DISPLAY_BLOCK, SVG_BLOCK, HTML_BLOCK, FENCE):
        text = pattern.sub(lambda m: " " * len(m.group(0)), text)
    text = re.sub(r"\$[^$]*?\$", lambda m: " " * len(m.group(0)), text, flags=re.DOTALL)
    text = re.sub(r"\[[^\]]*\]\([^)]*\)", lambda m: " " * len(m.group(0)), text)
    text = re.sub(r"`[^`]*`", lambda m: " " * len(m.group(0)), text)
    return text


def year_sites() -> list[tuple[str, int, str]]:
    sites: list[tuple[str, int, str]] = []
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        text = read(path)
        for match in YEAR_PARAGRAPH.finditer(text):
            para = match.group(0)
            if not para.strip() or YEAR_TABLE_ROW.match(para):
                continue
            if para.lstrip().startswith(("#", ">")):
                continue
            prose = year_prose(para)
            if paren_year_attributed(prose):
                continue
            if paragraph_attributes(prose):
                continue
            for year in YEAR.finditer(prose):
                token = year.group(0)
                if DECADE.match(token):
                    continue
                if year_has_name(prose, year.start()) or year_has_following_name(prose, year.end()):
                    continue
                line_no = text.count("\n", 0, match.start() + year.start()) + 1
                sites.append((rel(path), line_no, token))
    return sites


def paragraph_attributes(prose: str) -> bool:
    """True when some name stands next to some date, nested parentheses aside."""
    flat = re.sub(r"[()\[\]]", "", prose)
    for match in re.finditer(YEAR_AUTHOR + r"(?:[\u2019']s)?", flat):
        name = match.group(0).rstrip("\u2019's")
        if name in YEAR_NOT_NAMES:
            continue
        window = flat[match.end():match.end() + 90]
        window = re.split(r"[.;:]\s", window)[0]
        year = YEAR.search(window)
        if not year:
            continue
        if YEAR_MONTHS.search(flat[max(0, year.start() - 20):year.start()]):
            continue
        return True
    return False


def check_year_citation() -> None:
    for path, line_no, token in year_sites():
        if declare("year-citation", path, token):
            continue
        fail(
            "year-citation",
            f"{path}:{line_no}: the year {token} is asserted in a paragraph that carries "
            "no citation pair, so the date has no stated warrant (§IV)",
        )


# ----------------------------------------------------- 17. ledger coherence

# audit/repair-report.md is the only record that a repair has a finding behind it.
# A path it names that does not exist, or a gate it names that is not a reading
# this file runs, makes the record unverifiable — the ledger becomes prose about
# a repair rather than evidence of one.
#
# What is checked: (a) every book-src path the ledger names exists, (b) every
# backticked token in a gate column is `leitura`, a gate name this file prints, or
# a check_* function defined here. What cannot be checked without git: that every
# file the tree changed appears in the ledger. The ledger is the reference list of
# its own claims, and its completeness is asserted by the wave that wrote it; this
# gate cannot make an omission visible, and says so in the report it feeds.

LEDGER = "audit/repair-report.md"
LEDGER_PATH = re.compile(r"book-src/(?:[0-9]{2}-[A-Za-z0-9._-]+|appendix-[A-Za-z0-9._-]+)\.md")
LEDGER_TOKEN = re.compile(r"`([^`\n]+)`")
LEDGER_PROSE = {
    "censo", "leitura", "git", "grep", "diff", "stat", "structure.py", "echo",
    "DECLARED_OPEN", "summary", "md", "py", "advisory", "verde", "vermelho",
}


def gate_names() -> set[str]:
    """The gate labels this file can print, read from its own calls to fail()."""
    source = read(os.path.abspath(__file__))
    return set(re.findall(r'fail\(\s*"([a-z0-9-]+)"', source))


def check_ledger_coherence() -> None:
    path = os.path.join(ROOT, LEDGER)
    if not os.path.exists(path):
        # The ledger is an input of the revision, not a part of the manuscript: the
        # repository keeps the repaired text and the gates, and a wave that keeps a
        # ledger elsewhere still has one to check when it is present. Absence is
        # therefore a state this gate does not judge — but it says so, rather than
        # passing silently, because a record that is gone cannot vouch for anything.
        print(f"  ledger-coherence: {LEDGER} is absent — no ledger to check")
        return
    text = read(path)
    known_gates = gate_names()
    defined_checks = {name for name in globals() if name.startswith("check_")}

    for match in LEDGER_PATH.finditer(text):
        target = os.path.join(ROOT, match.group(0))
        if not os.path.exists(target):
            line_no = text.count("\n", 0, match.start()) + 1
            fail(
                "ledger-coherence",
                f"{LEDGER}:{line_no}: names {match.group(0)}, which does not exist in the tree",
            )

    for line_no, body in enumerate(text.splitlines(), start=1):
        if not body.lstrip().startswith("|"):
            continue
        cells = [cell.strip() for cell in body.strip().strip("|").split("|")]
        if len(cells) < 2:
            continue
        # The gate column is the last cell in both table shapes: the four-column
        # repair rows end with it, and the two-column coverage rows are
        # "Arquivo | Portão".
        for token in LEDGER_TOKEN.findall(cells[-1]):
            token = token.strip()
            if not token or token in LEDGER_PROSE:
                continue
            if token.startswith("check_"):
                if token not in defined_checks:
                    fail(
                        "ledger-coherence",
                        f"{LEDGER}:{line_no}: the gate column names {token}, "
                        "which is not a function in this file",
                    )
                continue
            if re.fullmatch(r"[a-z][a-z0-9-]*", token) and token not in known_gates:
                fail(
                    "ledger-coherence",
                    f"{LEDGER}:{line_no}: the gate column names '{token}', "
                    "which is neither a gate this file prints nor 'leitura'",
                )


# ------------------------------------------- 18. ratio limit without hypothesis

# A limit that lands on a ratio of two symbols is a statement about a domain, not
# only about an expression: "tends to $r/(r - \alpha)$ as the diffusion vanishes"
# is false when $r \le \alpha$, because the denominator is not then positive and
# the root the ratio came from does not sit above one. The gate reads the section
# the limit sits in — a section is the block between headings — and asks for one
# inequality between the numerator symbol and the symbols of the denominator, or
# for a positivity gloss on both. The numerator is the expression before `\to`
# only when the ratio is what the arrow points at; a ratio that merely occurs
# beside an arrow is not a limit and is not read.
SECTION_HEADING = re.compile(r"^#{1,4}\s", re.M)
LIMIT_ARROW = re.compile(r"\\to\b|\\(?:long)?rightarrow\b|\u2192")
LIMIT_PROSE = re.compile(r"tend(?:s|ing)?\s+(?:to|toward)\s*$", re.I)
LIMIT_RATIOS = (
    re.compile(r"\\frac\s*\{([^{}]+)\}\s*\{([^{}]+)\}"),
    re.compile(r"([A-Za-z\\][A-Za-z0-9_^{}\\ ]*?)\s*/\s*\(([^()]+)\)"),
    re.compile(r"\b([A-Za-z])\s*/\s*([A-Za-z])\b"),
)
MATH_SYMBOL = re.compile(r"\\[A-Za-z]+|[A-Za-z]")
NOT_A_SYMBOL = {
    "frac", "sqrt", "cdot", "left", "right", "bigl", "bigr", "Bigl", "Bigr",
    "ln", "exp", "log", "max", "min", "lim", "to", "infty", "mathrm", "text",
    "ge", "le", "geq", "leq",
}
ORDER = r"(?:>|<|\\ge\b|\\le\b|\\geq\b|\\leq\b)"
VALID_SYMBOL = re.compile(r"^(?:\\[A-Za-z]+|[A-Za-z])$")


def sections_of(text: str):
    starts = [0] + [m.start() for m in SECTION_HEADING.finditer(text)]
    return [(start, text[start:(starts[i + 1] if i + 1 < len(starts) else len(text))])
            for i, start in enumerate(starts)]


def limit_expression(text: str, match: re.Match) -> str | None:
    span = match.group(1)
    arrow = LIMIT_ARROW.search(span)
    if arrow:
        return span[arrow.end():]
    if LIMIT_PROSE.search(text[max(0, match.start() - 40):match.start()]):
        return span
    return None


def symbol_set(expr: str) -> set[str]:
    bare = re.sub(r"\\mathrm\{[^}]*\}|\\text\{[^}]*\}", " ", expr)
    return {token for token in MATH_SYMBOL.findall(bare)} - NOT_A_SYMBOL


def ratio_parts(limit: str):
    for pattern in LIMIT_RATIOS:
        for match in pattern.finditer(limit):
            yield match.group(1), match.group(2)


def positive_in(section: str, symbol: str) -> bool:
    left = rf"{re.escape(symbol)}\s*{ORDER}\s*0"
    right = rf"0\s*{ORDER}\s*{re.escape(symbol)}"
    return bool(re.search(left, section) or re.search(right, section))


def denominator_positive(body: str, bottoms: list[str]) -> bool:
    """The denominator cannot vanish when every symbol in it is declared positive.

    Numerator positivity does not buy the limit: $r/(r-\alpha)$ is finite only
    when $r \neq \alpha$, and $r > 0$, $\alpha > 0$ leave that open. It is the
    denominator's sign that the hypothesis has to fix.
    """
    return bool(bottoms) and all(positive_in(body, b) for b in bottoms)


def constrained(section: str, numerator: str, denominator: str) -> bool:
    pairs = [
        rf"{re.escape(numerator)}\s*{ORDER}\s*{re.escape(denominator)}",
        rf"{re.escape(denominator)}\s*{ORDER}\s*{re.escape(numerator)}",
    ]
    if any(re.search(pattern, section) for pattern in pairs):
        return True
    return positive_in(section, numerator) and positive_in(section, denominator)


def check_ratio_limit() -> None:
    for path in sorted(glob.glob(os.path.join(ROOT, "book-src/*.md"))):
        text = read(path)
        for offset, section in sections_of(text):
            for match in re.finditer(r"\$([^$\n]+)\$", section):
                limit = limit_expression(section, match)
                if limit is None:
                    continue
                for numerator, denominator in ratio_parts(limit):
                    nums = symbol_set(numerator)
                    if len(nums) != 1:
                        continue
                    top = next(iter(nums))
                    bottoms = symbol_set(denominator) - {top}
                    if not bottoms or not all(VALID_SYMBOL.match(b) for b in bottoms):
                        continue
                    # An inequality is local to the section; a positivity gloss is a
                    # convention of the chapter, so it is read over the whole file.
                    missing = [b for b in sorted(bottoms)
                               if not constrained(section, top, b)
                               and not denominator_positive(text, bottoms)]
                    if not missing:
                        continue
                    line_no = text.count("\n", 0, offset + match.start()) + 1
                    key = f"{top}/({','.join(sorted(bottoms))})"
                    if declare("ratio-limit", path, key):
                        continue
                    fail(
                        "ratio-limit",
                        f"{rel(path)}:{line_no}: the limit {key} names no inequality "
                        f"between {top} and {', '.join(missing)} in its section",
                    )


# --------------------------------------------------------------------- driver

def main() -> int:
    check_apparatus()
    check_spine()
    check_links()
    check_figures()
    check_b1()
    check_ledger()
    check_citations()
    check_contents()
    check_concordance()
    check_guard_sites()
    check_math()
    check_notation()
    check_attribution()
    check_falsifiers()
    check_concordance_polarity()
    check_collapse_refutation()
    check_guard_footing()
    check_year_citation()
    check_ledger_coherence()
    check_ratio_limit()

    print("numeral census (advisory, not a gate):")
    flagged = census()
    print(f"  {flagged} candidate lines without a citation year, math span or editorial purpose")

    report_stale_declarations()
    report()

    if failures:
        print(f"\n{len(failures)} structural failure(s):")
        for item in failures:
            print("  " + item)
        return 1
    print("\nall structural gates pass")
    return 0


if __name__ == "__main__":
    sys.exit(main())
