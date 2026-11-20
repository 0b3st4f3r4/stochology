#!/usr/bin/env python3
"""Published-surface check (AGENTS.md section V, check 8).

The other gates read the sources. This one reads the pages mdbook builds, because
§V-8 is stated over a page: a page that belongs to exactly one chapter and carries
a masthead must print that chapter's identity consistently in four objects — the
masthead's order label, its chapter token, its guard triple, and the `Layer NN`
line of the blockquote — and the masthead prints its label twice, as an accessible
name and as a visible node.

The predicate is internal to the page. It compares the printed objects with one
another and with the chapter the page carries (the page's own file name, which is
editorial); it never compares the page with the source that produced it. It reads
no prose, audits no literature value, and returns no verdict on any substantive
claim.

Usage: python3 .github/checks/surface.py [BUILD_DIR]   (default: book)

Exit code 0 means every Part V page passes; 1 means at least one failed. Run it
after `mdbook build`; the render job of .github/workflows/ci.yml does exactly that.
"""

from __future__ import annotations

import glob
import os
import re
import sys

FIRST_PART_V_CHAPTER = 51
LAST_PART_V_CHAPTER = 85

PAGE = re.compile(r"^(\d\d)-[^/]*\.html$")
BLOCKQUOTE = re.compile(r"<blockquote>(.*?)</blockquote>", re.DOTALL)
LAYER_LINE = re.compile(
    r"Layer (\d+) of Part V · Stratum ([IVX]+) · Chapter (\d+) · "
    r"Unique Guard Combination \{([^}]*)\} · Signature \(([^)]*)\)"
)
MASTHEAD = re.compile(
    r'<figure class="stoch-figure stoch-logo-figure">.*?</figure>', re.DOTALL
)
ARIA_LABEL = re.compile(
    r'aria-label="Stochology Order (L\d\d) \(CH(\d\d) · T\d\* · \{([^}]*)\}\)'
)
VISIBLE_ORDER = re.compile(r"PART V · STRATUM S\d · ORDER (L\d\d)")
VISIBLE_CHAPTER = re.compile(r"CH(\d\d) · T\d\* · \{([^}]*)\}")
TEXT_NODE = re.compile(r">([^<>]+)<")
TAG = re.compile(r"<[^>]+>")

failures: list[str] = []


def fail(page: str, detail: str) -> None:
    failures.append(f"[surface] {page}: {detail}")


def read(path: str) -> str:
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def sites(triple: str) -> frozenset[str]:
    return frozenset(re.findall(r"T\d", triple))


def check_page(path: str, chapter: str) -> None:
    name = os.path.basename(path)
    html = read(path)

    quoted = [block for block in BLOCKQUOTE.findall(html) if "Layer " in block and "of Part V" in block]
    if not quoted:
        fail(name, "carries no '> Layer NN' blockquote")
        return
    if len(quoted) > 1:
        fail(name, f"carries {len(quoted)} Layer blockquotes, expected 1")
        return
    line = LAYER_LINE.search(re.sub(r"\s+", " ", TAG.sub(" ", quoted[0])))
    if not line:
        fail(name, "its Layer blockquote does not parse as an order line")
        return
    layer, quoted_chapter, quoted_triple = (
        line.group(1),
        line.group(3),
        sites(line.group(4)),
    )

    mastheads = MASTHEAD.findall(html)
    if not mastheads:
        fail(name, "carries a Layer NN blockquote but no masthead")
        return
    if len(mastheads) > 1:
        fail(name, f"carries {len(mastheads)} mastheads and no single chapter")
        return
    masthead = mastheads[0]

    aria = ARIA_LABEL.search(masthead)
    if not aria:
        fail(name, "its masthead prints no accessible name of the order's label")
        return
    aria_order, aria_chapter, aria_triple = aria.group(1), aria.group(2), sites(aria.group(3))

    visible = " ".join(TEXT_NODE.findall(masthead))
    order_node = VISIBLE_ORDER.search(visible)
    chapter_node = VISIBLE_CHAPTER.search(visible)
    if not order_node or not chapter_node:
        fail(name, "its masthead prints no visible order label and chapter token")
        return
    shown_order, shown_chapter, shown_triple = (
        order_node.group(1),
        chapter_node.group(1),
        sites(chapter_node.group(2)),
    )

    # The two printings of the label must agree with each other.
    if (aria_order, aria_chapter, aria_triple) != (shown_order, shown_chapter, shown_triple):
        fail(
            name,
            "masthead accessible name "
            f"({aria_order}, CH{aria_chapter}, {{{', '.join(sorted(aria_triple))}}}) "
            f"!= visible node ({shown_order}, CH{shown_chapter}, {{{', '.join(sorted(shown_triple))}}})",
        )

    # The masthead must name the chapter whose text the page carries.
    if shown_chapter != chapter:
        fail(name, f"masthead names CH{shown_chapter}, the page carries chapter {chapter}")
    if aria_chapter != chapter:
        fail(name, f"masthead accessible name names CH{aria_chapter}, the page carries chapter {chapter}")
    if quoted_chapter != chapter:
        fail(name, f"blockquote names Chapter {quoted_chapter}, the page carries chapter {chapter}")

    # The masthead must name the order its own blockquote declares.
    declared = "L" + layer
    if shown_order != declared:
        fail(name, f"masthead names {shown_order}, its blockquote declares {declared}")
    if aria_order != declared:
        fail(name, f"masthead accessible name names {aria_order}, its blockquote declares {declared}")

    # The guard triple must be the one the page prints as its order's unique
    # guard combination.
    if shown_triple != quoted_triple:
        fail(
            name,
            f"masthead prints {{{', '.join(sorted(shown_triple))}}}, "
            f"its blockquote declares {{{', '.join(sorted(quoted_triple))}}}",
        )
    if aria_triple != quoted_triple:
        fail(
            name,
            f"masthead accessible name prints {{{', '.join(sorted(aria_triple))}}}, "
            f"its blockquote declares {{{', '.join(sorted(quoted_triple))}}}",
        )


def main(argv: list[str]) -> int:
    build_dir = argv[1] if len(argv) > 1 else "book"
    if not os.path.isdir(build_dir):
        print(f"surface: no build directory {build_dir!r} — run mdbook build first", file=sys.stderr)
        return 1

    pages: dict[str, str] = {}
    for path in sorted(glob.glob(os.path.join(build_dir, "*.html"))):
        match = PAGE.match(os.path.basename(path))
        if not match:
            continue
        chapter = match.group(1)
        if FIRST_PART_V_CHAPTER <= int(chapter) <= LAST_PART_V_CHAPTER:
            pages[chapter] = path

    for chapter in range(FIRST_PART_V_CHAPTER, LAST_PART_V_CHAPTER + 1):
        key = f"{chapter:02d}"
        if key not in pages:
            fail("book/", f"chapter {key} has no published page")

    for chapter, path in sorted(pages.items()):
        check_page(path, chapter)

    if failures:
        print(f"\n{len(failures)} published-surface failure(s):")
        for item in failures:
            print("  " + item)
        return 1

    print(f"published surface: {len(pages)} Part V pages, every masthead agrees with its blockquote")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
