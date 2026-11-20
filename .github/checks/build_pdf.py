#!/usr/bin/env python3
"""Deterministic PDF edition exporter and validator for the Stochology manuscript.

Reads the built mdBook HTML pages (default `book/`), `book-src/SUMMARY.md`,
`CITATION.cff`, `book.toml`, and `book-src/figures/*.svg`, compiles LaTeX math
spans into Presentation MathML, extracts `<main>` fragments into a single
collision-free HTML5 print document with a vector cover and clickable TOC,
renders the PDF via headless Chromium/Chrome, and validates structure and page count.

Usage:
    python3 .github/checks/build_pdf.py [BUILD_DIR] [OUTPUT_PDF]
"""

from __future__ import annotations

import base64
import datetime
import html
import math
import os
import re
import shutil
import subprocess
import sys
import urllib.parse
import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from html.parser import HTMLParser

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
BOOK_SRC = os.path.join(ROOT, "book-src")
PLATES_DIR = os.path.join(BOOK_SRC, "plates")

HTML_VOID_TAGS = frozenset(
    {
        "area",
        "base",
        "br",
        "col",
        "embed",
        "hr",
        "img",
        "input",
        "link",
        "meta",
        "param",
        "source",
        "track",
        "wbr",
    }
)

HTML_SKIP_MATH_TAGS = frozenset({"code", "pre", "svg", "script", "style", "math"})

SVG_TAG_CASE = {
    "lineargradient": "linearGradient",
    "radialgradient": "radialGradient",
    "clippath": "clipPath",
    "textpath": "textPath",
    "foreignobject": "foreignObject",
    "fegaussianblur": "feGaussianBlur",
    "fecolormatrix": "feColorMatrix",
    "feoffset": "feOffset",
    "feblend": "feBlend",
    "femerge": "feMerge",
    "femergenode": "feMergeNode",
    "feflood": "feFlood",
    "fecomposite": "feComposite",
    "fedropshadow": "feDropShadow",
    "animatemotion": "animateMotion",
    "animatetransform": "animateTransform",
}

SVG_ATTR_CASE = {
    "viewbox": "viewBox",
    "gradientunits": "gradientUnits",
    "gradienttransform": "gradientTransform",
    "patternunits": "patternUnits",
    "patterntransform": "patternTransform",
    "patterncontentunits": "patternContentUnits",
    "preserveaspectratio": "preserveAspectRatio",
    "spreadmethod": "spreadMethod",
    "textlength": "textLength",
    "lengthadjust": "lengthAdjust",
    "clippathunits": "clipPathUnits",
    "maskunits": "maskUnits",
    "maskcontentunits": "maskContentUnits",
    "markerwidth": "markerWidth",
    "markerheight": "markerHeight",
    "markerunits": "markerUnits",
    "refx": "refX",
    "refy": "refY",
    "stddeviation": "stdDeviation",
    "basefrequency": "baseFrequency",
    "numoctaves": "numOctaves",
    "startoffset": "startOffset",
    "pathlength": "pathLength",
}

URL_HASH_RE = re.compile(r"url\((['\"]?)#([^)'\"\s]+)\1\)")


# ---------------------------------------------------------------------- DOM helpers


def xml_escape_text(text: str) -> str:
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def xml_escape_attr(text: str) -> str:
    return xml_escape_text(text).replace('"', "&quot;")


@dataclass
class Node:
    tag: str
    attrs: list[tuple[str, str]] = field(default_factory=list)
    children: list[Node | str] = field(default_factory=list)
    is_svg: bool = False


def render_node(node: Node | str) -> str:
    if isinstance(node, str):
        return xml_escape_text(node)

    attrs_str = "".join(
        f' {k}="{xml_escape_attr(v)}"' if v is not None else f' {k}="{k}"'
        for k, v in node.attrs
    )
    if not node.children:
        if node.is_svg or node.tag in HTML_VOID_TAGS:
            return f"<{node.tag}{attrs_str} />"
        return f"<{node.tag}{attrs_str}></{node.tag}>"

    # Keep CSS inside SVG <style> raw so selectors/quotes are preserved untouched.
    if node.tag == "style" and node.is_svg:
        inner = "".join(ch if isinstance(ch, str) else render_node(ch) for ch in node.children)
        return f"<{node.tag}{attrs_str}>{inner}</{node.tag}>"

    inner = "".join(render_node(ch) for ch in node.children)
    return f"<{node.tag}{attrs_str}>{inner}</{node.tag}>"


# ------------------------------------------------------------------ TeX -> MathML compiler


TEX_SYMBOLS: dict[str, tuple[str, str]] = {
    "alpha": ("mi", "α"),
    "beta": ("mi", "β"),
    "gamma": ("mi", "γ"),
    "delta": ("mi", "δ"),
    "epsilon": ("mi", "ϵ"),
    "varepsilon": ("mi", "ε"),
    "zeta": ("mi", "ζ"),
    "eta": ("mi", "η"),
    "theta": ("mi", "θ"),
    "vartheta": ("mi", "ϑ"),
    "iota": ("mi", "ι"),
    "kappa": ("mi", "κ"),
    "lambda": ("mi", "λ"),
    "mu": ("mi", "μ"),
    "nu": ("mi", "ν"),
    "xi": ("mi", "ξ"),
    "pi": ("mi", "π"),
    "varpi": ("mi", "ϖ"),
    "rho": ("mi", "ρ"),
    "varrho": ("mi", "ϱ"),
    "sigma": ("mi", "σ"),
    "varsigma": ("mi", "ς"),
    "tau": ("mi", "τ"),
    "upsilon": ("mi", "υ"),
    "phi": ("mi", "ϕ"),
    "varphi": ("mi", "φ"),
    "chi": ("mi", "χ"),
    "psi": ("mi", "ψ"),
    "omega": ("mi", "ω"),
    "Gamma": ("mi", "Γ"),
    "Delta": ("mi", "Δ"),
    "Theta": ("mi", "Θ"),
    "Lambda": ("mi", "Λ"),
    "Xi": ("mi", "Ξ"),
    "Pi": ("mi", "Π"),
    "Sigma": ("mi", "Σ"),
    "Upsilon": ("mi", "Υ"),
    "Phi": ("mi", "Φ"),
    "Psi": ("mi", "Ψ"),
    "Omega": ("mi", "Ω"),
    "ell": ("mi", "ℓ"),
    "infty": ("mi", "∞"),
    "emptyset": ("mi", "∅"),
    "varnothing": ("mi", "∅"),
    "partial": ("mo", "∂"),
    "nabla": ("mo", "∇"),
    "dots": ("mo", "…"),
    "cdots": ("mo", "⋯"),
    "vdots": ("mo", "⋮"),
    "ddots": ("mo", "⋱"),
    "prime": ("mo", "′"),
    "square": ("mo", "□"),
    "dagger": ("mo", "†"),
    "top": ("mo", "⊤"),
    "bot": ("mo", "⊥"),
    "perp": ("mo", "⊥"),
    "cdot": ("mo", "·"),
    "times": ("mo", "×"),
    "pm": ("mo", "±"),
    "mp": ("mo", "∓"),
    "circ": ("mo", "∘"),
    "ast": ("mo", "∗"),
    "star": ("mo", "⋆"),
    "oplus": ("mo", "⊕"),
    "otimes": ("mo", "⊗"),
    "cap": ("mo", "∩"),
    "cup": ("mo", "∪"),
    "setminus": ("mo", "∖"),
    "wedge": ("mo", "∧"),
    "land": ("mo", "∧"),
    "vee": ("mo", "∨"),
    "lor": ("mo", "∨"),
    "neg": ("mo", "¬"),
    "lnot": ("mo", "¬"),
    "forall": ("mo", "∀"),
    "exists": ("mo", "∃"),
    "lbrace": ("mo", "{"),
    "rbrace": ("mo", "}"),
    "langle": ("mo", "⟨"),
    "rangle": ("mo", "⟩"),
    "lceil": ("mo", "⌈"),
    "rceil": ("mo", "⌉"),
    "lfloor": ("mo", "⌊"),
    "rfloor": ("mo", "⌋"),
    "lvert": ("mo", "|"),
    "rvert": ("mo", "|"),
    "Vert": ("mo", "‖"),
    "mid": ("mo", "∣"),
    "parallel": ("mo", "∥"),
    "le": ("mo", "≤"),
    "leq": ("mo", "≤"),
    "ge": ("mo", "≥"),
    "geq": ("mo", "≥"),
    "ne": ("mo", "≠"),
    "neq": ("mo", "≠"),
    "approx": ("mo", "≈"),
    "equiv": ("mo", "≡"),
    "sim": ("mo", "∼"),
    "simeq": ("mo", "≃"),
    "cong": ("mo", "≅"),
    "asymp": ("mo", "≍"),
    "propto": ("mo", "∝"),
    "ll": ("mo", "≪"),
    "gg": ("mo", "≫"),
    "lesssim": ("mo", "≲"),
    "gtrsim": ("mo", "≳"),
    "lessgtr": ("mo", "≶"),
    "gtrless": ("mo", "≷"),
    "lesseqgtr": ("mo", "⋚"),
    "gtreqless": ("mo", "⋛"),
    "prec": ("mo", "≺"),
    "succ": ("mo", "≻"),
    "preceq": ("mo", "⪯"),
    "succeq": ("mo", "⪰"),
    "in": ("mo", "∈"),
    "notin": ("mo", "∉"),
    "ni": ("mo", "∋"),
    "subset": ("mo", "⊂"),
    "subseteq": ("mo", "⊆"),
    "subsetneq": ("mo", "⊊"),
    "supset": ("mo", "⊃"),
    "supseteq": ("mo", "⊇"),
    "vdash": ("mo", "⊢"),
    "to": ("mo", "→"),
    "rightarrow": ("mo", "→"),
    "leftarrow": ("mo", "←"),
    "leftrightarrow": ("mo", "↔"),
    "longrightarrow": ("mo", "⟶"),
    "longleftarrow": ("mo", "⟵"),
    "longleftrightarrow": ("mo", "⟷"),
    "Longrightarrow": ("mo", "⟹"),
    "Longrightarrow ": ("mo", "⟹"),
    "Longleftarrow": ("mo", "⟸"),
    "Longleftrightarrow": ("mo", "⟺"),
    "implies": ("mo", "⟹"),
    "iff": ("mo", "⟺"),
    "mapsto": ("mo", "↦"),
    "rightleftharpoons": ("mo", "⇌"),
    "uparrow": ("mo", "↑"),
    "downarrow": ("mo", "↓"),
}

BIG_OPS: dict[str, str] = {
    "sum": "∑",
    "prod": "∏",
    "int": "∫",
    "bigcup": "⋃",
    "bigcap": "⋂",
    "bigoplus": "⨁",
    "bigotimes": "⨂",
}

NAMED_LIMIT_OPS = frozenset({"lim", "liminf", "limsup", "sup", "inf", "max", "min", "argmax", "argmin"})
NAMED_FUNCS = frozenset(
    {
        "Pr",
        "det",
        "dim",
        "ker",
        "arg",
        "ln",
        "log",
        "exp",
        "sin",
        "cos",
        "tan",
        "arcsin",
        "arccos",
        "arctan",
        "bmod",
    }
)

SPACING_CMDS: dict[str, str] = {
    ",": "0.167em",
    ":": "0.222em",
    ";": "0.278em",
    "!": "-0.167em",
    " ": "0.25em",
    "quad": "1em",
    "qquad": "2em",
    "thinspace": "0.167em",
    "medspace": "0.222em",
    "thickspace": "0.278em",
    "enspace": "0.5em",
}

TEX_ESCAPED_CHARS: dict[str, tuple[str, str]] = {
    "{": ("mo", "{"),
    "}": ("mo", "}"),
    "%": ("mo", "%"),
    "$": ("mo", "$"),
    "&": ("mo", "&"),
    "#": ("mo", "#"),
    "_": ("mo", "_"),
    "|": ("mo", "‖"),
}

ACCENT_CMDS: dict[str, str] = {
    "hat": "^",
    "widehat": "^",
    "bar": "¯",
    "overline": "¯",
    "tilde": "~",
    "widetilde": "~",
    "dot": "˙",
    "ddot": "¨",
    "vec": "→",
}

VARIANT_CMDS: dict[str, str] = {
    "mathrm": "normal",
    "mathbf": "bold",
    "boldsymbol": "bold-italic",
    "mathbb": "double-struck",
    "mathcal": "script",
    "mathsf": "sans-serif",
}

SIZE_DELIM_CMDS = frozenset(
    {"left", "right", "bigl", "bigr", "Bigl", "Bigr", "big", "Big", "biggl", "biggr"}
)


def _preprocess_tex(raw: str) -> str:
    s = html.unescape(raw)
    s = s.replace("’", "'").replace("‘", "'").replace("–", "-").replace("—", "-")
    s = re.sub(
        r"\\unicode\{\s*(?:0x|x)([0-9A-Fa-f]+)\s*\}",
        lambda m: chr(int(m.group(1), 16)),
        s,
    )
    return s.strip()


def _tokenize_tex(s: str) -> list[tuple[str, str]]:
    tokens: list[tuple[str, str]] = []
    i = 0
    n = len(s)
    while i < n:
        ch = s[i]
        if ch.isspace():
            i += 1
            continue
        if ch == "\\":
            if i + 1 < n and s[i + 1] == "\\":
                tokens.append(("rowsep", "\\\\"))
                i += 2
                continue
            if i + 1 < n and s[i + 1].isalpha():
                j = i + 1
                while j < n and s[j].isalpha():
                    j += 1
                tokens.append(("cmd", s[i + 1 : j]))
                i = j
                continue
            if i + 1 < n:
                tokens.append(("esc", s[i + 1]))
                i += 2
                continue
            i += 1
            continue
        if ch in "{}_^&":
            tokens.append((ch, ch))
            i += 1
            continue
        if ch.isdigit():
            j = i
            while j < n and (s[j].isdigit() or (s[j] == "." and j + 1 < n and s[j + 1].isdigit())):
                j += 1
            tokens.append(("num", s[i:j]))
            i = j
            continue
        if ch.isalpha():
            tokens.append(("ident", ch))
            i += 1
            continue
        tokens.append(("op", ch))
        i += 1
    return tokens


def _extract_braced_raw(s: str, pos: int) -> tuple[str, int]:
    """Read `{...}` starting near pos, preserving spaces literally for \\text."""
    while pos < len(s) and s[pos].isspace():
        pos += 1
    if pos >= len(s) or s[pos] != "{":
        if pos < len(s):
            return s[pos], pos + 1
        return "", pos
    depth = 0
    start = pos + 1
    i = pos
    while i < len(s):
        if s[i] == "\\" and i + 1 < len(s):
            i += 2
            continue
        if s[i] == "{":
            depth += 1
        elif s[i] == "}":
            depth -= 1
            if depth == 0:
                return s[start:i], i + 1
        i += 1
    return s[start:], len(s)


class TexParser:
    """Compile a LaTeX math expression into a Presentation MathML `Node`."""

    def __init__(self, tex: str, display: bool = False) -> None:
        self.raw = _preprocess_tex(tex)
        self.display = display
        self.text_literals: list[str] = []
        masked = self._mask_text_commands(self.raw)
        self.tokens = _tokenize_tex(masked)
        self.pos = 0

    def _mask_text_commands(self, s: str) -> str:
        out: list[str] = []
        i = 0
        n = len(s)
        while i < n:
            matched = None
            for cmd in ("operatorname", "textit", "texttt", "text"):
                prefix = "\\" + cmd
                if s.startswith(prefix, i) and (i + len(prefix) == n or not s[i + len(prefix)].isalpha()):
                    matched = cmd
                    i += len(prefix)
                    break
            if matched is not None:
                content, i = _extract_braced_raw(s, i)
                clean = (
                    content.replace("\\&", "&")
                    .replace("\\%", "%")
                    .replace("\\_", "_")
                    .replace("\\#", "#")
                    .replace("\\$", "$")
                    .replace("\\{", "{")
                    .replace("\\}", "}")
                )
                idx = len(self.text_literals)
                self.text_literals.append(f"{matched}:{clean}")
                out.append(f"\\textlit{{{idx}}}")
                continue
            out.append(s[i])
            i += 1
        return "".join(out)

    def peek(self, offset: int = 0) -> tuple[str, str] | None:
        idx = self.pos + offset
        if 0 <= idx < len(self.tokens):
            return self.tokens[idx]
        return None

    def consume(self) -> tuple[str, str] | None:
        tok = self.peek()
        if tok is not None:
            self.pos += 1
        return tok

    def parse(self) -> Node:
        nodes = self._parse_seq(stop_tokens=frozenset())
        body = nodes[0] if len(nodes) == 1 else Node("mrow", children=nodes)
        attrs = [
            ("xmlns", "http://www.w3.org/1998/Math/MathML"),
            ("display", "block" if self.display else "inline"),
        ]
        return Node("math", attrs=attrs, children=[body])

    def _wrap_row(self, items: list[Node | str]) -> Node:
        if not items:
            return Node("mrow")
        if len(items) == 1 and isinstance(items[0], Node):
            return items[0]
        return Node("mrow", children=items)

    def _parse_group_or_atom(self, variant: str | None = None) -> Node:
        tok = self.peek()
        if tok is None:
            return Node("mrow")
        if tok[0] == "{":
            self.consume()
            items = self._parse_seq(stop_tokens=frozenset({"}"}), variant=variant)
            if self.peek() == ("}", "}"):
                self.consume()
            return self._wrap_row(items)
        atom = self._parse_atom(variant=variant)
        return atom if atom is not None else Node("mrow")

    def _parse_optional_bracket(self) -> Node | None:
        if self.peek() != ("op", "["):
            return None
        self.consume()
        items: list[Node | str] = []
        while (tok := self.peek()) is not None and tok != ("op", "]"):
            items.append(self._parse_scripted_atom())
        if self.peek() == ("op", "]"):
            self.consume()
        return self._wrap_row(items)

    def _collect_variant_run(self, variant: str) -> Node:
        if self.peek() != ("{", "{"):
            return self._parse_group_or_atom(variant=variant)
        self.consume()
        items = self._parse_seq(stop_tokens=frozenset({"}"}), variant=variant)
        if self.peek() == ("}", "}"):
            self.consume()
        if variant in ("normal", "bold", "sans-serif"):
            merged: list[Node | str] = []
            for item in items:
                if (
                    merged
                    and isinstance(merged[-1], Node)
                    and merged[-1].tag == "mi"
                    and isinstance(item, Node)
                    and item.tag == "mi"
                    and dict(merged[-1].attrs).get("mathvariant") == dict(item.attrs).get("mathvariant") == variant
                ):
                    merged[-1].children[0] = str(merged[-1].children[0]) + str(item.children[0])
                else:
                    merged.append(item)
            items = merged
        return self._wrap_row(items)

    def _parse_delim_token(self) -> Node | None:
        tok = self.peek()
        if tok is None:
            return None
        kind, val = tok
        if kind == "op" and val == ".":
            self.consume()
            return None
        if kind == "esc" and val in TEX_ESCAPED_CHARS:
            self.consume()
            t, ch = TEX_ESCAPED_CHARS[val]
            return Node(t, [("stretchy", "true")], [ch])
        if kind == "cmd" and val in TEX_SYMBOLS:
            self.consume()
            t, ch = TEX_SYMBOLS[val]
            return Node(t, [("stretchy", "true")], [ch])
        if kind == "op" and val in "()[]|/":
            self.consume()
            return Node("mo", [("stretchy", "true")], [val])
        return Node("mo", [("stretchy", "true")], [self.consume()[1]])

    def _parse_env(self, env_name: str) -> Node:
        rows: list[list[Node]] = []
        current_row: list[Node] = []
        current_cell: list[Node | str] = []

        def flush_cell() -> None:
            current_row.append(self._wrap_row(current_cell.copy()))
            current_cell.clear()

        def flush_row() -> None:
            flush_cell()
            rows.append(current_row.copy())
            current_row.clear()

        while (tok := self.peek()) is not None:
            if tok == ("cmd", "end"):
                self.consume()
                _ = self._parse_raw_brace_text()
                break
            if tok == ("rowsep", "\\\\"):
                self.consume()
                flush_row()
                continue
            if tok == ("&", "&"):
                self.consume()
                flush_cell()
                continue
            current_cell.append(self._parse_scripted_atom())

        if current_cell or current_row:
            flush_row()
        if rows and len(rows[-1]) == 1 and isinstance(rows[-1][0], Node) and rows[-1][0].tag == "mrow" and not rows[-1][0].children:
            rows.pop()

        mtrs = [Node("mtr", children=[Node("mtd", children=[cell]) for cell in r]) for r in rows]
        if env_name == "pmatrix":
            return Node(
                "mrow",
                children=[
                    Node("mo", [("stretchy", "true")], ["("]),
                    Node("mtable", children=mtrs),
                    Node("mo", [("stretchy", "true")], [")"]),
                ],
            )
        if env_name == "smallmatrix":
            return Node(
                "mtable",
                attrs=[("displaystyle", "false"), ("rowspacing", "0.2em"), ("columnspacing", "0.4em")],
                children=mtrs,
            )
        if env_name == "cases":
            return Node(
                "mrow",
                children=[
                    Node("mo", [("stretchy", "true")], ["{"]),
                    Node("mtable", attrs=[("columnalign", "left left"), ("columnspacing", "1em")], children=mtrs),
                ],
            )
        if env_name == "aligned":
            max_cols = max((len(r) for r in rows), default=2)
            aligns = " ".join("right" if idx % 2 == 0 else "left" for idx in range(max_cols))
            return Node("mtable", attrs=[("columnalign", aligns), ("columnspacing", "0.25em")], children=mtrs)
        return Node("mtable", children=mtrs)

    def _parse_raw_brace_text(self) -> str:
        tok = self.peek()
        if tok == ("{", "{"):
            self.consume()
            parts: list[str] = []
            depth = 1
            while (t := self.consume()) is not None:
                if t[0] == "{":
                    depth += 1
                elif t[0] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                parts.append(t[1])
            return "".join(parts)
        return self.consume()[1] if tok else ""

    def _parse_atom(self, variant: str | None = None) -> Node | None:
        tok = self.peek()
        if tok is None:
            return None
        kind, val = tok

        if kind == "{":
            return self._parse_group_or_atom(variant=variant)
        if kind == "num":
            self.consume()
            attrs = [("mathvariant", variant)] if variant else []
            return Node("mn", attrs=attrs, children=[val])
        if kind == "ident":
            self.consume()
            attrs = [("mathvariant", variant)] if variant else []
            return Node("mi", attrs=attrs, children=[val])
        if kind == "op":
            self.consume()
            if val == "'":
                return Node("mo", children=["′"])
            return Node("mo", children=[val])
        if kind == "esc":
            self.consume()
            if val in SPACING_CMDS:
                return Node("mspace", [("width", SPACING_CMDS[val])])
            if val in TEX_ESCAPED_CHARS:
                t, ch = TEX_ESCAPED_CHARS[val]
                return Node(t, children=[ch])
            return Node("mo", children=[val])
        if kind == "cmd":
            self.consume()
            if val == "textlit":
                idx = int(self._parse_raw_brace_text())
                kind_lit, content = self.text_literals[idx].split(":", 1)
                if kind_lit == "operatorname":
                    return Node("mo", [("mathvariant", "normal")], [content])
                return Node("mtext", children=[content])
            if val == "mkern":
                num_parts: list[str] = []
                while (nt := self.peek()) is not None and not (nt[0] == "ident" and nt[1] in ("m", "u", "e", "p")):
                    if nt[0] in ("num", "op"):
                        num_parts.append(self.consume()[1])
                    else:
                        break
                for _ in range(2):
                    if self.peek() is not None and self.peek()[0] == "ident" and self.peek()[1] in ("m", "u", "e", "x", "p", "t"):
                        self.consume()
                try:
                    mu = float("".join(num_parts) or "3")
                    em = max(0.05, mu / 18.0)
                    return Node("mspace", [("width", f"{em:.3f}em")])
                except ValueError:
                    return Node("mspace", [("width", "0.2em")])
            if val in SPACING_CMDS:
                return Node("mspace", [("width", SPACING_CMDS[val])])
            if val in SIZE_DELIM_CMDS:
                delim = self._parse_delim_token()
                return delim if delim is not None else Node("mrow")
            if val == "begin":
                env_name = self._parse_raw_brace_text()
                return self._parse_env(env_name)
            if val == "frac":
                return Node("mfrac", children=[self._parse_group_or_atom(), self._parse_group_or_atom()])
            if val == "tfrac":
                return Node(
                    "mstyle",
                    [("displaystyle", "false")],
                    [Node("mfrac", children=[self._parse_group_or_atom(), self._parse_group_or_atom()])],
                )
            if val == "binom":
                top = self._parse_group_or_atom()
                bot = self._parse_group_or_atom()
                return Node(
                    "mrow",
                    children=[
                        Node("mo", [("stretchy", "true")], ["("]),
                        Node("mfrac", [("linethickness", "0")], [top, bot]),
                        Node("mo", [("stretchy", "true")], [")"]),
                    ],
                )
            if val == "sqrt":
                idx = self._parse_optional_bracket()
                rad = self._parse_group_or_atom()
                return Node("mroot", children=[rad, idx]) if idx is not None else Node("msqrt", children=[rad])
            if val in ACCENT_CMDS:
                base = self._parse_group_or_atom(variant=variant)
                return Node("mover", [("accent", "true")], [base, Node("mo", children=[ACCENT_CMDS[val]])])
            if val == "underline":
                base = self._parse_group_or_atom(variant=variant)
                return Node("munder", children=[base, Node("mo", [("stretchy", "true")], ["_"])])
            if val == "underbrace":
                base = self._parse_group_or_atom(variant=variant)
                return Node("munder", children=[base, Node("mo", [("stretchy", "true")], ["⏟"])])
            if val == "stackrel":
                top = self._parse_group_or_atom()
                base = self._parse_group_or_atom()
                return Node("mover", children=[base, top])
            if val == "xrightarrow":
                sub = self._parse_optional_bracket()
                sup = self._parse_group_or_atom()
                arrow = Node("mo", [("stretchy", "true")], ["→"])
                return Node("munderover", children=[arrow, sub, sup]) if sub is not None else Node("mover", children=[arrow, sup])
            if val in VARIANT_CMDS:
                return self._collect_variant_run(VARIANT_CMDS[val])
            if val == "not":
                nxt = self._parse_atom()
                if isinstance(nxt, Node) and nxt.children and isinstance(nxt.children[0], str):
                    neg_map = {"→": "↛", "=": "≠", "≡": "≢", "∈": "∉", "⊂": "⊄", "⊆": "⊈", "∼": "≁", "≈": "≉"}
                    ch = nxt.children[0]
                    nxt.children[0] = neg_map.get(ch, ch + "\u0338")
                    return nxt
                return Node("mo", children=["¬"])
            if val in BIG_OPS:
                return Node("mo", [("movablelimits", "true")], [BIG_OPS[val]])
            if val in NAMED_LIMIT_OPS:
                op_text = {"liminf": "lim\u2009inf", "limsup": "lim\u2009sup"}.get(val, val)
                return Node("mo", [("mathvariant", "normal"), ("movablelimits", "true")], [op_text])
            if val in NAMED_FUNCS:
                return Node("mi", [("mathvariant", "normal")], [val])
            if val == "pmod":
                arg = self._parse_group_or_atom()
                return Node(
                    "mrow",
                    children=[
                        Node("mspace", [("width", "0.444em")]),
                        Node("mo", children=["("]),
                        Node("mi", [("mathvariant", "normal")], ["mod"]),
                        Node("mspace", [("width", "0.278em")]),
                        arg,
                        Node("mo", children=[")"]),
                    ],
                )
            if val in TEX_SYMBOLS:
                t, ch = TEX_SYMBOLS[val]
                attrs = [("mathvariant", variant)] if variant and t == "mi" else []
                return Node(t, attrs=attrs, children=[ch])
            raise ValueError(f"Unsupported LaTeX command '\\{val}' in TeX: {self.raw!r}")

        self.consume()
        return Node("mtext", children=[val])

    def _parse_scripted_atom(self, variant: str | None = None) -> Node:
        base = self._parse_atom(variant=variant)
        if base is None:
            return Node("mrow")

        sub: Node | None = None
        sup: Node | None = None
        primes: list[str] = []

        while (tok := self.peek()) is not None:
            if tok[0] == "op" and tok[1] == "'":
                self.consume()
                primes.append("′")
                continue
            if tok == ("cmd", "prime"):
                self.consume()
                primes.append("′")
                continue
            if tok[0] == "_" and sub is None:
                self.consume()
                sub = self._parse_group_or_atom(variant=variant)
                continue
            if tok[0] == "^" and sup is None:
                self.consume()
                sup = self._parse_group_or_atom(variant=variant)
                continue
            break

        if primes:
            prime_node = Node("mo", children=["".join(primes)])
            sup = prime_node if sup is None else Node("mrow", children=[prime_node, sup])

        if sub is None and sup is None:
            return base

        use_underover = (
            isinstance(base, Node)
            and (
                (
                    base.tag == "munder"
                    and len(base.children) == 2
                    and isinstance(base.children[1], Node)
                    and base.children[1].children == ["⏟"]
                )
                or (dict(base.attrs).get("movablelimits") == "true" and self.display)
            )
        )
        if use_underover:
            if sub is not None and sup is not None:
                return Node("munderover", children=[base, sub, sup])
            if sub is not None:
                return Node("munder", children=[base, sub])
            return Node("mover", children=[base, sup])  # type: ignore[list-item]

        if sub is not None and sup is not None:
            return Node("msubsup", children=[base, sub, sup])
        if sub is not None:
            return Node("msub", children=[base, sub])
        return Node("msup", children=[base, sup])  # type: ignore[list-item]

    def _parse_seq(self, stop_tokens: frozenset[str], variant: str | None = None) -> list[Node | str]:
        items: list[Node | str] = []
        while (tok := self.peek()) is not None:
            if tok[0] in stop_tokens or tok == ("cmd", "end") or tok[0] in ("rowsep", "&"):
                break
            items.append(self._parse_scripted_atom(variant=variant))
        return items


def _split_wide_display_tex(expr: str) -> list[str]:
    """Break wide single-line display equations into readable stacked clauses for 16x23cm print."""
    cleaned = expr.strip()
    if r"\begin{" in cleaned or len(cleaned) <= 125:
        return [cleaned]

    def _scan_step(text: str, i: int, depth: int, lr_depth: int) -> tuple[int, int, int, str]:
        ch = text[i]
        if ch == "\\" and i + 1 < len(text):
            if text[i + 1].isalpha():
                j = i + 1
                while j < len(text) and text[j].isalpha():
                    j += 1
                cmd = text[i + 1 : j]
                if cmd == "left":
                    lr_depth += 1
                elif cmd == "right":
                    lr_depth = max(0, lr_depth - 1)
                return j, depth, lr_depth, cmd
            return i + 2, depth, lr_depth, text[i + 1]
        if ch == "{":
            return i + 1, depth + 1, lr_depth, "{"
        if ch == "}":
            return i + 1, max(0, depth - 1), lr_depth, "}"
        return i + 1, depth, lr_depth, ch

    # Pass 1: split top-level \qquad / \quad separators.
    pass1: list[str] = []
    start = 0
    i = 0
    depth = 0
    lr_depth = 0
    while i < len(cleaned):
        nxt, new_depth, new_lr, tok = _scan_step(cleaned, i, depth, lr_depth)
        if depth == 0 and lr_depth == 0 and tok in ("quad", "qquad"):
            piece = cleaned[start:i].strip()
            if piece:
                pass1.append(piece)
            start = nxt
            while start < len(cleaned) and cleaned[start].isspace():
                start += 1
        i = nxt
        depth = new_depth
        lr_depth = new_lr
    tail = cleaned[start:].strip()
    if tail:
        pass1.append(tail)
    if not pass1:
        pass1 = [cleaned]

    # Pass 2: wrap remaining overly long single clauses at top-level relations or + \underbrace.
    relation_cmds = frozenset({"le", "leq", "ge", "geq", "implies"})
    result: list[str] = []
    for clause in pass1:
        if r"\begin{" in clause or len(clause) <= 165:
            result.append(clause)
            continue
        sub_start = 0
        j = 0
        c_depth = 0
        c_lr = 0
        while j < len(clause):
            nxt, new_depth, new_lr, tok = _scan_step(clause, j, c_depth, c_lr)
            can_split = (
                c_depth == 0
                and c_lr == 0
                and (j - sub_start) >= 78
                and (
                    tok in relation_cmds
                    or (tok == "=" and (j == 0 or clause[j - 1] != ":"))
                    or (tok == "+" and clause[nxt:].lstrip().startswith(r"\underbrace"))
                )
            )
            if can_split:
                head = clause[sub_start:j].strip()
                if head:
                    result.append(head)
                    sub_start = j
            j = nxt
            c_depth = new_depth
            c_lr = new_lr
        rem = clause[sub_start:].strip()
        if rem:
            result.append(rem)
    return result or [cleaned]


def compile_math_in_text(text: str) -> list[Node | str]:
    """Split text around $$...$$ and $...$ math spans and compile each to MathML."""
    if "$" not in text:
        return [text]

    parts: list[Node | str] = []
    pos = 0
    pattern = re.compile(r"\$\$([\s\S]*?)\$\$|\$([^$]+?)\$")
    for match in pattern.finditer(text):
        start, end = match.span()
        if start > pos:
            parts.append(text[pos:start])
        if match.group(1) is not None:
            clauses = _split_wide_display_tex(match.group(1))
            math_nodes: list[Node | str] = []
            for clause in clauses:
                math_node = TexParser(clause, display=True).parse()
                if len(clause) > 135 or (r"\begin{" in clause and len(clause) > 220):
                    math_node.attrs.append(("class", "math-compact"))
                math_nodes.append(math_node)
            parts.append(Node("span", [("class", "math-display")], math_nodes))
        else:
            parts.append(TexParser(match.group(2), display=False).parse())
        pos = end
    if pos < len(text):
        parts.append(text[pos:])
    return parts


def clean_heading_tex(title: str) -> str:
    """Turn inline LaTeX markers in SUMMARY/title strings into readable plain text."""
    s = re.sub(r"\$\$?([^$]+?)\$\$?", lambda m: _plain_tex(m.group(1)), title)
    s = s.replace(r"\_", "_").replace("*", "").replace("`", "")
    return re.sub(r"\s+", " ", s).strip()


def _plain_tex(expr: str) -> str:
    expr = html.unescape(expr)
    expr = expr.replace(r"\_", "_")
    expr = expr.replace(r"\binom{7}{3}", "C(7,3)")
    expr = re.sub(r"\\(?:mathrm|mathcal|mathbb|text)\{([^}]*)\}", r"\1", expr)
    expr = expr.replace("_{", "_").replace("}", "").replace("{", "")
    expr = expr.replace(r"\ge", "≥").replace(r"\le", "≤").replace(r"\to", "→")
    return expr


# ------------------------------------------------------------------ SVG loading & ID scoping


def _collect_svg_ids(node: Node | str, out: set[str]) -> None:
    if not isinstance(node, Node):
        return
    for k, v in node.attrs:
        if k == "id" and v:
            out.add(v)
    for ch in node.children:
        _collect_svg_ids(ch, out)


def _rewrite_svg_node_ids(node: Node | str, id_set: set[str], prefix: str) -> None:
    if isinstance(node, str):
        return
    new_attrs: list[tuple[str, str]] = []
    for k, v in node.attrs:
        if k == "id" and v in id_set:
            v = prefix + v
        elif k in ("href", "xlink:href") and v.startswith("#") and v[1:] in id_set:
            v = "#" + prefix + v[1:]
        elif k in ("aria-labelledby", "aria-describedby"):
            v = " ".join(prefix + tok if tok in id_set else tok for tok in v.split())
        elif "#" in v and "url(" in v:
            v = URL_HASH_RE.sub(
                lambda m: f"url({m.group(1)}#{prefix + m.group(2) if m.group(2) in id_set else m.group(2)}{m.group(1)})",
                v,
            )
        new_attrs.append((k, v))
    node.attrs = new_attrs

    if node.tag == "style":
        node.children = [
            URL_HASH_RE.sub(
                lambda m: f"url({m.group(1)}#{prefix + m.group(2) if m.group(2) in id_set else m.group(2)}{m.group(1)})",
                ch,
            )
            if isinstance(ch, str)
            else ch
            for ch in node.children
        ]

    for ch in node.children:
        _rewrite_svg_node_ids(ch, id_set, prefix)


def scope_svg_ids(svg_node: Node, prefix: str) -> Node:
    """Prefix internal `id` and `url(#...)` references inside an SVG so inline SVGs never collide."""
    id_set: set[str] = set()
    _collect_svg_ids(svg_node, id_set)
    if id_set:
        _rewrite_svg_node_ids(svg_node, id_set, prefix)
    return svg_node


def _et_to_svg_node(elem: ET.Element) -> Node:
    local = elem.tag.split("}")[-1]
    tag = SVG_TAG_CASE.get(local.lower(), local)
    attrs: list[tuple[str, str]] = []
    has_xmlns = False
    strip_box_dims = tag == "svg" and any(rk.split("}")[-1].lower() == "viewbox" for rk in elem.attrib)
    for raw_k, raw_v in elem.attrib.items():
        if raw_k.startswith("{http://www.w3.org/1999/xlink}"):
            k = "xlink:" + raw_k.split("}")[-1]
        elif "}" in raw_k:
            k = raw_k.split("}")[-1]
        else:
            k = SVG_ATTR_CASE.get(raw_k.lower(), raw_k)
        if strip_box_dims and k in ("width", "height"):
            continue
        if k == "xmlns":
            has_xmlns = True
        attrs.append((k, raw_v))
    if tag == "svg" and not has_xmlns:
        attrs.append(("xmlns", "http://www.w3.org/2000/svg"))

    children: list[Node | str] = []
    if elem.text:
        children.append(elem.text)
    for child in elem:
        children.append(_et_to_svg_node(child))
        if child.tail:
            children.append(child.tail)
    return Node(tag=tag, attrs=attrs, children=children, is_svg=True)


def load_figure_svg_node(rel_src: str, alt_text: str, svg_seq: list[int]) -> Node | None:
    """Load `book-src/figures/<basename>.svg` as an inline scoped `<svg>` Node."""
    fname = os.path.basename(urllib.parse.unquote(rel_src))
    fig_path = os.path.join(BOOK_SRC, "figures", fname)
    if not os.path.isfile(fig_path):
        return None
    root = ET.fromstring(open(fig_path, encoding="utf-8").read())
    node = _et_to_svg_node(root)
    if alt_text and not any(k == "aria-label" or k == "aria-labelledby" for k, _ in node.attrs):
        node.attrs.append(("aria-label", alt_text))
    svg_seq[0] += 1
    return scope_svg_ids(node, f"s{svg_seq[0]}-")


# ------------------------------------------------------------------ HTML <main> -> combined HTML5


def _rewrite_pdf_href(href: str, current_slug: str) -> str:
    if href.startswith(("http:", "https:", "mailto:")):
        return href
    path, has_hash, frag = href.partition("#")
    unquoted_frag = urllib.parse.unquote(frag)
    if not path:
        return f"#doc-{current_slug}--{unquoted_frag}" if unquoted_frag else f"#doc-{current_slug}"
    base = os.path.basename(urllib.parse.unquote(path))
    target_slug = re.sub(r"\.(?:x?html|md)$", "", base)
    if has_hash and unquoted_frag:
        return f"#doc-{target_slug}--{unquoted_frag}"
    return f"#doc-{target_slug}"


class MainHTMLParser(HTMLParser):
    """Parse an mdBook `<main>` fragment into HTML5 + MathML + scoped inline SVGs."""

    def __init__(self, chapter_slug: str, svg_seq: list[int]) -> None:
        super().__init__(convert_charrefs=True)
        self.chapter_slug = chapter_slug
        self.svg_seq = svg_seq
        self.root = Node("article", [("class", "pdf-chapter"), ("id", f"doc-{chapter_slug}")])
        self.stack: list[Node] = [self.root]
        self.skip_depth = 0

    def _in_svg(self) -> bool:
        return self.stack[-1].is_svg

    def _in_skip_math(self) -> bool:
        return any(n.tag in HTML_SKIP_MATH_TAGS for n in self.stack)

    def _normalize_attrs(self, tag: str, attrs: list[tuple[str, str | None]], in_svg: bool) -> list[tuple[str, str]]:
        out: list[tuple[str, str]] = []
        seen_alt = False
        has_xmlns = False
        strip_box_dims = tag == "svg" and any(rk.lower() == "viewbox" for rk, _ in attrs)
        for raw_k, raw_v in attrs:
            k = SVG_ATTR_CASE.get(raw_k.lower(), raw_k) if in_svg else raw_k.lower()
            v = "" if raw_v is None else raw_v
            if strip_box_dims and k in ("width", "height"):
                continue
            if not in_svg:
                if k == "id":
                    if not v:
                        continue
                    v = f"doc-{self.chapter_slug}--{v}"
                elif k == "href":
                    v = _rewrite_pdf_href(v, self.chapter_slug)
                elif tag == "img" and k == "alt":
                    seen_alt = True
            else:
                if k == "xmlns":
                    has_xmlns = True
            out.append((k, v))
        if tag == "img" and not seen_alt:
            out.append(("alt", ""))
        if tag == "svg" and not has_xmlns:
            out.append(("xmlns", "http://www.w3.org/2000/svg"))
        return out

    def _maybe_upgrade_figure_paragraph(self, p_node: Node) -> None:
        meaningful = [ch for ch in p_node.children if not (isinstance(ch, str) and not ch.strip())]
        if len(meaningful) != 1 or not isinstance(meaningful[0], Node) or meaningful[0].tag != "img":
            return
        img_node = meaningful[0]
        attr_map = dict(img_node.attrs)
        src = attr_map.get("src", "")
        if "figures/" not in src or not src.endswith(".svg"):
            return
        alt_text = attr_map.get("alt", "").strip()
        svg_node = load_figure_svg_node(src, alt_text, self.svg_seq)
        p_node.tag = "figure"
        p_node.attrs = [("class", "stoch-figure")]
        children: list[Node | str] = [svg_node if svg_node is not None else img_node]
        if alt_text:
            children.append(Node("figcaption", children=[Node("strong", children=[alt_text])]))
        p_node.children = children

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        ltag = tag.lower()
        if self.skip_depth > 0:
            if ltag not in HTML_VOID_TAGS:
                self.skip_depth += 1
            return
        if ltag == "script" or (ltag == "style" and not self._in_svg()):
            self.skip_depth += 1
            return
        if ltag == "figure":
            cls = dict((k.lower(), v or "") for k, v in attrs).get("class", "")
            if "stoch-art-plate" in cls.split():
                self.skip_depth += 1
                return

        if ltag == "a" and self.stack[-1].tag in ("h1", "h2", "h3", "h4", "h5", "h6"):
            cls = dict((k.lower(), v or "") for k, v in attrs).get("class", "")
            if "header" in cls.split():
                unwrap = Node("__unwrap__")
                self.stack[-1].children.append(unwrap)
                self.stack.append(unwrap)
                return

        in_svg = self._in_svg() or ltag == "svg"
        norm_tag = SVG_TAG_CASE.get(ltag, tag if in_svg else ltag)
        norm_attrs = self._normalize_attrs(norm_tag, attrs, in_svg)
        node = Node(norm_tag, norm_attrs, is_svg=in_svg)
        self.stack[-1].children.append(node)
        if norm_tag not in HTML_VOID_TAGS:
            self.stack.append(node)

    def handle_startendtag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if self.skip_depth > 0:
            return
        ltag = tag.lower()
        in_svg = self._in_svg() or ltag == "svg"
        norm_tag = SVG_TAG_CASE.get(ltag, tag if in_svg else ltag)
        norm_attrs = self._normalize_attrs(norm_tag, attrs, in_svg)
        self.stack[-1].children.append(Node(norm_tag, norm_attrs, is_svg=in_svg))

    def handle_endtag(self, tag: str) -> None:
        ltag = tag.lower()
        if self.skip_depth > 0:
            if ltag not in HTML_VOID_TAGS:
                self.skip_depth -= 1
            return
        norm_target = SVG_TAG_CASE.get(ltag, ltag) if self._in_svg() else ltag
        if norm_target in HTML_VOID_TAGS:
            return
        if len(self.stack) > 1 and self.stack[-1].tag == "__unwrap__" and ltag == "a":
            unwrap = self.stack.pop()
            parent = self.stack[-1]
            idx = parent.children.index(unwrap)
            parent.children[idx : idx + 1] = unwrap.children
            return
        for i in range(len(self.stack) - 1, 0, -1):
            cur = self.stack[i].tag
            if cur == norm_target or cur.lower() == ltag:
                closed = self.stack[i]
                del self.stack[i:]
                if closed.tag == "svg":
                    self.svg_seq[0] += 1
                    scope_svg_ids(closed, f"s{self.svg_seq[0]}-")
                elif closed.tag == "p":
                    self._maybe_upgrade_figure_paragraph(closed)
                break

    def handle_data(self, data: str) -> None:
        if self.skip_depth > 0:
            return
        if self._in_skip_math() or "$" not in data:
            self.stack[-1].children.append(data)
            return
        self.stack[-1].children.extend(compile_math_in_text(data))


# ------------------------------------------------------------------ SUMMARY & Metadata


@dataclass
class TocEntry:
    title: str
    slug: str
    html_name: str
    part_key: str = "front"
    part_title: str = "Front Matter"
    group_title: str | None = None


@dataclass
class TocSubgroup:
    title: str | None
    entries: list[TocEntry] = field(default_factory=list)


@dataclass
class TocPart:
    title: str
    key: str = "front"
    groups: list[TocSubgroup] = field(default_factory=list)


def _classify_part_key(part_title: str) -> str:
    low = part_title.strip().lower()
    if low.startswith("front"):
        return "front"
    if low.startswith("appendices"):
        return "appendices"
    m = re.match(r"^part\s+(vii|vi|iv|v|iii|ii|i)\b", low)
    if m:
        return f"part-{m.group(1)}"
    return "front"


def parse_summary(summary_path: str) -> tuple[list[TocPart], list[TocEntry]]:
    """Parse book-src/SUMMARY.md into a hierarchical TOC tree and flat ordered spine."""
    parts: list[TocPart] = []
    spine: list[TocEntry] = []
    front_part = TocPart(title="Front Matter", key="front", groups=[TocSubgroup(title=None)])
    current_part: TocPart | None = None
    current_group: TocSubgroup | None = None

    with open(summary_path, encoding="utf-8") as handle:
        for raw in handle:
            line = raw.strip()
            if not line:
                continue
            if line.startswith("# ") and not parts and not spine:
                continue
            if line.startswith("# "):
                ptitle = clean_heading_tex(line[2:])
                current_part = TocPart(title=ptitle, key=_classify_part_key(ptitle), groups=[])
                current_group = None
                parts.append(current_part)
                continue
            if line.startswith("## "):
                if current_part is None:
                    current_part = TocPart(title="Book", key="front", groups=[])
                    parts.append(current_part)
                current_group = TocSubgroup(title=clean_heading_tex(line[3:]))
                current_part.groups.append(current_group)
                continue
            m = re.match(r"^(?:-\s+)?\[([^\]]+)\]\(([^)]+)\)$", line)
            if m:
                label = clean_heading_tex(m.group(1))
                md_href = urllib.parse.unquote(m.group(2))
                slug = re.sub(r"\.md$", "", md_href)
                if slug == "index":
                    continue
                if current_part is None:
                    entry = TocEntry(
                        title=label,
                        slug=slug,
                        html_name=f"{slug}.html",
                        part_key=front_part.key,
                        part_title=front_part.title,
                        group_title=None,
                    )
                    spine.append(entry)
                    front_part.groups[0].entries.append(entry)
                else:
                    if re.match(r"^part-[ivx]+-conclusion$", slug) and current_group is not None and current_group.title is not None:
                        current_group = TocSubgroup(title=None)
                        current_part.groups.append(current_group)
                    elif current_group is None:
                        current_group = TocSubgroup(title=None)
                        current_part.groups.append(current_group)
                    entry = TocEntry(
                        title=label,
                        slug=slug,
                        html_name=f"{slug}.html",
                        part_key=current_part.key,
                        part_title=current_part.title,
                        group_title=current_group.title,
                    )
                    spine.append(entry)
                    current_group.entries.append(entry)

    all_parts: list[TocPart] = []
    if front_part.groups[0].entries:
        all_parts.append(front_part)
    all_parts.extend(parts)
    return all_parts, spine


def load_metadata() -> dict[str, str]:
    meta = {
        "title": "Stochology: The Theory and Ethics of Aim under Uncertainty",
        "author": "Silvano Neto",
        "language": "en",
        "url": "https://stochology.org/",
        "rights": "CC BY 4.0",
        "description": (
            "Stochology is an integrative framework and transversal research program "
            "for the theory and ethics of aim (stókhos) for judgment and decision-making "
            "under structural uncertainty when action must precede the outcome."
        ),
    }
    toml_path = os.path.join(ROOT, "book.toml")
    if os.path.exists(toml_path):
        toml = open(toml_path, encoding="utf-8").read()
        if m := re.search(r'^language\s*=\s*"([^"]+)"', toml, re.M):
            meta["language"] = m.group(1)
        if m := re.search(r'^site-url\s*=\s*"([^"]+)"', toml, re.M):
            meta["url"] = m.group(1)

    cff_path = os.path.join(ROOT, "CITATION.cff")
    if os.path.exists(cff_path):
        cff = open(cff_path, encoding="utf-8").read()
        if m := re.search(r'^title:\s*"([^"]+)"', cff, re.M):
            meta["title"] = m.group(1)
        if m := re.search(r'^license:\s*(\S+)', cff, re.M):
            meta["rights"] = m.group(1).replace("-", " ")

    epoch = os.environ.get("SOURCE_DATE_EPOCH")
    if not epoch:
        try:
            epoch = subprocess.check_output(
                ["git", "log", "-1", "--format=%ct"], cwd=ROOT, text=True, stderr=subprocess.DEVNULL
            ).strip()
        except Exception:
            epoch = ""
    ts = int(epoch) if epoch and epoch.isdigit() else 1794692400
    dt = datetime.datetime.fromtimestamp(ts, tz=datetime.timezone.utc)
    meta["date"] = dt.strftime("%Y-%m-%d")
    return meta


# ------------------------------------------------------------------ Print layout & cover


PDF_CSS = """\
:root {
  color-scheme: light;
  --canvas: #ffffff;
  --surface: #f9f6f0;
  --elevated: #f1ece2;
  --elevated-bg: #f1ece2;
  --ink: #1b1714;
  --ink-strong: #141210;
  --muted: #685e52;
  --border: #cfc6b6;
  --hairline: #e5dfd2;
  --part-accent: #8a3a06;
  --part-tint: #f8f1e8;
  --ch-accent: var(--part-accent);
  --ch-tint: var(--part-tint);
  --accent: var(--ch-accent);
  --accent-2: #ad1839;
  --code-bg: #f1ece2;
  --code-border: #dcd4c5;
  --code-ink: #1b1714;
  --sky: #1b52c4;
  --sky-bg: #edf2fb;
  --emerald: #0d7d57;
  --emerald-bg: #ebf6f1;
  --amber: #9c5c04;
  --amber-bg: #faf3e4;
  --rose: #ad1839;
  --rose-bg: #fbeff2;
  --font-sans: "Open Sans", "Inter", "Liberation Sans", "DejaVu Sans", system-ui, sans-serif;
  --font-serif: "Literata", "Charter", Georgia, serif;
  --mono-font: "Source Code Pro", "JetBrains Mono", "Liberation Mono", ui-monospace, monospace;
  --logo-ink: #161310;
  --logo-sub: var(--ch-accent);
  --logo-meta: #685e52;
  --logo-frame: #cfc6b6;
  --logo-ticks: #7a7063;
  --logo-node-bg: #f1ece2;
  --logo-well-hi: #ffffff;
  --logo-blue: #1b52c4;
  --logo-green: #0d7d57;
  --logo-crimson: #c82838;
  --logo-gold: #c07a06;
}

@page {
  size: 160mm 230mm;
  margin: 18mm 15mm 19mm 22mm;
  @top-left {
    content: "STOCHOLOGY";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.2pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #8a3a06;
    border-bottom: 0.5pt solid #d8cfc0;
    vertical-align: bottom;
    padding-bottom: 4pt;
    margin-bottom: 5pt;
  }
  @top-right {
    content: "THEORY & ETHICS OF AIM";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.2pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #584e43;
    border-bottom: 0.5pt solid #d8cfc0;
    vertical-align: bottom;
    padding-bottom: 4pt;
    margin-bottom: 5pt;
  }
  @bottom-left {
    content: "STOCHOLOGY · SILVANO NETO";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.0pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #7a7063;
    border-top: 0.5pt solid #d8cfc0;
    vertical-align: top;
    padding-top: 4pt;
    margin-top: 5pt;
  }
  @bottom-right {
    content: counter(page);
    font-family: "Open Sans", "Inter", "Liberation Sans", sans-serif;
    font-size: 7.5pt;
    font-weight: 700;
    color: #161310;
    border-top: 0.5pt solid #d8cfc0;
    vertical-align: top;
    padding-top: 3.5pt;
    margin-top: 5pt;
  }
}

@page :left {
  margin: 18mm 22mm 19mm 15mm;
}

@page :right {
  margin: 18mm 15mm 19mm 22mm;
}

@page :first {
  size: 160mm 230mm;
  margin: 0;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-toc:right {
  @top-left { content: "STOCHOLOGY"; color: #8a3a06; }
  @top-right { content: "TABLE OF CONTENTS"; color: #3d352c; }
  @bottom-left {
    content: "THE THEORY AND ETHICS OF AIM";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.0pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #7a7063;
  }
  @bottom-right {
    content: counter(page);
    font-family: "Open Sans", "Inter", "Liberation Sans", sans-serif;
    font-size: 7.5pt;
    font-weight: 700;
    color: #161310;
  }
}

@page pdf-toc:left {
  @top-left { content: "TABLE OF CONTENTS"; color: #3d352c; }
  @top-right { content: "STOCHOLOGY"; color: #8a3a06; }
  @bottom-left {
    content: counter(page);
    font-family: "Open Sans", "Inter", "Liberation Sans", sans-serif;
    font-size: 7.5pt;
    font-weight: 700;
    color: #161310;
  }
  @bottom-right {
    content: "THE THEORY AND ETHICS OF AIM";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.0pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #7a7063;
  }
}

@page pdf-title-page {
  size: 160mm 230mm;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-title-page:left {
  size: 160mm 230mm;
  margin: 18mm 22mm 19mm 15mm;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-title-page:right {
  size: 160mm 230mm;
  margin: 18mm 15mm 19mm 22mm;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-epigraph {
  size: 160mm 230mm;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-epigraph:left {
  size: 160mm 230mm;
  margin: 18mm 22mm 19mm 15mm;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-epigraph:right {
  size: 160mm 230mm;
  margin: 18mm 15mm 19mm 22mm;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-presentation:right {
  @top-left { content: "STOCHOLOGY"; color: #8a3a06; }
  @top-right { content: "Ø · PRESENTATION"; color: #3d352c; }
  @bottom-left {
    content: "THE THEORY AND ETHICS OF AIM";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.0pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #7a7063;
  }
  @bottom-right {
    content: counter(page);
    font-family: "Open Sans", "Inter", "Liberation Sans", sans-serif;
    font-size: 7.5pt;
    font-weight: 700;
    color: #161310;
  }
}

@page pdf-presentation:left {
  @top-left { content: "Ø · PRESENTATION"; color: #3d352c; }
  @top-right { content: "STOCHOLOGY"; color: #8a3a06; }
  @bottom-left {
    content: counter(page);
    font-family: "Open Sans", "Inter", "Liberation Sans", sans-serif;
    font-size: 7.5pt;
    font-weight: 700;
    color: #161310;
  }
  @bottom-right {
    content: "THE THEORY AND ETHICS OF AIM";
    font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace;
    font-size: 6.0pt;
    font-weight: 500;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #7a7063;
  }
}

@page pdf-part-divider {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-part-divider:left {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-part-divider:right {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-back-cover {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-back-cover:left {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-back-cover:right {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-art-plate {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-art-plate:left {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

@page pdf-art-plate:right {
  size: 160mm 230mm;
  margin: 0;
  background: #12100e;
  @top-left { content: none; border: none; }
  @top-right { content: none; border: none; }
  @bottom-left { content: none; border: none; }
  @bottom-right { content: none; border: none; }
}

* {
  box-sizing: border-box;
}

html, body {
  margin: 0;
  padding: 0;
  width: 100%;
  max-width: 100%;
  overflow-x: clip;
  background: #ffffff;
  color: var(--ink);
  font-family: var(--font-sans);
  font-size: 8.6pt;
  line-height: 1.70;
  word-spacing: 0.035em;
  letter-spacing: 0.004em;
  font-variant-numeric: tabular-nums;
}

/* Cover & full-bleed Part Dividers */
.pdf-cover {
  width: 160mm;
  height: 229.8mm;
  margin: 0;
  padding: 0;
  background: #f4efe4;
  display: block;
  break-after: page;
  page-break-after: always;
  overflow: hidden;
}

.pdf-cover svg {
  width: 160mm;
  height: 229.8mm;
  display: block;
}

.pdf-part-divider {
  page: pdf-part-divider;
  width: 160mm;
  height: 229.8mm;
  margin: 0;
  padding: 0;
  background: #f4efe4;
  display: block;
  break-before: page;
  page-break-before: always;
  break-after: page;
  page-break-after: always;
  overflow: hidden;
}

.pdf-part-divider svg {
  width: 160mm;
  height: 229.8mm;
  display: block;
}

.pdf-back-cover {
  page: pdf-back-cover;
  width: 160mm;
  height: 229.8mm;
  margin: 0;
  padding: 0;
  background: #f4efe4;
  display: block;
  break-before: page;
  page-break-before: always;
  overflow: hidden;
}

.pdf-back-cover svg {
  width: 160mm;
  height: 229.8mm;
  display: block;
}

.pdf-art-plate {
  page: pdf-art-plate;
  width: 160mm;
  height: 229.8mm;
  margin: 0;
  padding: 0;
  background: #12100e;
  display: block;
  break-before: page;
  page-break-before: always;
  break-after: page;
  page-break-after: always;
  overflow: hidden;
}

.pdf-art-plate img {
  width: 160mm;
  height: 229.8mm;
  object-fit: cover;
  object-position: center;
  display: block;
}

.pdf-cover .cov-mono,
.pdf-part-divider .cov-mono,
.pdf-back-cover .cov-mono {
  font-family: "Source Code Pro", "JetBrains Mono", "Liberation Mono", ui-monospace, monospace;
}

.pdf-cover .cov-sans,
.pdf-part-divider .cov-sans,
.pdf-back-cover .cov-sans {
  font-family: "Open Sans", "Inter", "Liberation Sans", system-ui, sans-serif;
}

.pdf-cover .cov-serif,
.pdf-part-divider .cov-serif,
.pdf-back-cover .cov-serif {
  font-family: "Literata", "Charter", Georgia, serif;
}

/* Title Page (Folha de Rosto) */
.pdf-title-page {
  page: pdf-title-page;
  height: 190mm;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  break-before: page;
  page-break-before: always;
  break-after: page;
  page-break-after: always;
  overflow: hidden;
}

.pdf-title-page .tp-eyebrow {
  font-family: var(--mono-font);
  font-size: 6.7pt;
  font-weight: 500;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: #8a3a06;
  margin: 0 0 5pt 0;
}

.pdf-title-page .tp-triad-rule {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  height: 2pt;
  width: 100%;
}

.pdf-title-page .tp-triad-rule > span:nth-child(1) { background: #0d7d57; }
.pdf-title-page .tp-triad-rule > span:nth-child(2) { background: #1b52c4; }
.pdf-title-page .tp-triad-rule > span:nth-child(3) { background: #c82838; }

.pdf-title-page .tp-center {
  margin: auto 0;
  padding: 10pt 0;
}

.pdf-title-page .tp-author {
  font-family: var(--font-sans);
  font-size: 9.8pt;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: #584e43;
  margin: 0 0 10pt 0;
}

.pdf-title-page .tp-title {
  font-family: var(--font-sans);
  font-size: 33pt;
  font-weight: 800;
  letter-spacing: -0.03em;
  line-height: 1.04;
  color: var(--ink-strong);
  margin: 0 0 8pt 0;
  padding: 0;
  border: 0;
  background: none;
}

.pdf-title-page .tp-subtitle {
  font-family: "Literata", "Charter", Georgia, serif;
  font-size: 13pt;
  font-weight: 400;
  line-height: 1.40;
  color: #3d352c;
  max-width: 96mm;
  margin: 0 0 18pt 0;
}

.pdf-title-page .tp-emblem-wrap {
  display: flex;
  align-items: center;
  gap: 10pt;
  padding: 10pt 11pt;
  background: var(--surface);
  border: 0.6pt solid var(--border);
  border-left: 2.5pt solid #8a3a06;
  border-radius: 1.5pt;
}

.pdf-title-page .tp-emblem {
  flex: 0 0 auto;
  width: 56pt;
  height: 56pt;
  display: block;
}

.pdf-title-page .tp-emblem-meta {
  flex: 1 1 auto;
  min-width: 0;
}

.pdf-title-page .tp-emblem-kicker {
  font-family: var(--mono-font);
  font-size: 6.1pt;
  font-weight: 500;
  letter-spacing: 0.09em;
  text-transform: uppercase;
  color: #8a3a06;
  margin: 0 0 3pt 0;
}

.pdf-title-page .tp-emblem-eq {
  font-family: var(--mono-font);
  font-size: 6.8pt;
  font-weight: 500;
  line-height: 1.45;
  color: var(--ink-strong);
  margin: 0;
}

.pdf-title-page .tp-colophon {
  padding-top: 8pt;
  border-top: 1pt solid var(--ink-strong);
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 10pt;
  font-family: var(--mono-font);
  font-size: 6.2pt;
  font-weight: 500;
  line-height: 1.50;
  color: var(--muted);
}

.pdf-title-page .tp-colophon-right {
  text-align: right;
  flex-shrink: 0;
}

.pdf-title-page .tp-colophon strong {
  color: var(--ink-strong);
  font-weight: 500;
}

/* Epigraph Page (between Title Page and Table of Contents) */
.pdf-epigraph {
  page: pdf-epigraph;
  height: 190mm;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  break-before: page;
  page-break-before: always;
  break-after: page;
  page-break-after: always;
  overflow: hidden;
}

.pdf-epigraph .ep-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 4pt;
  border-bottom: 0.5pt solid #d8cfc0;
  font-family: var(--mono-font);
  font-size: 6.2pt;
  font-weight: 500;
  letter-spacing: 0.11em;
  text-transform: uppercase;
}

.pdf-epigraph .ep-brand {
  display: inline-flex;
  align-items: center;
  gap: 6pt;
}

.pdf-epigraph .ep-kicker {
  color: #8a3a06;
}

.pdf-epigraph .ep-lemniscate {
  width: 28pt;
  height: 11.5pt;
  display: block;
}

.pdf-epigraph .ep-tag {
  color: #7a7063;
}

.pdf-epigraph .ep-columns {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  column-gap: 6.5mm;
  flex: 1 1 auto;
  align-content: center;
  margin: 4pt 0;
}

.pdf-epigraph .ep-col {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 6pt;
  min-width: 0;
}

.pdf-epigraph .ep-entry {
  padding-left: 6pt;
  border-left: 1.1pt solid #d8cfc0;
  text-align: left;
}

.pdf-epigraph .ep-region {
  font-family: var(--mono-font);
  font-size: 5.1pt;
  font-weight: 500;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: #8a3a06;
  margin: 0 0 1.6pt 0;
}

.pdf-epigraph .ep-orig {
  font-size: 7.0pt;
  line-height: 1.40;
  color: #584e43;
  margin: 0 0 1.8pt 0;
  hyphens: none;
}

.pdf-epigraph .ep-cjk {
  font-family: "Noto Serif CJK TC", "Noto Serif CJK SC", "Droid Sans Fallback", serif;
  font-style: normal;
}

.pdf-epigraph .ep-deva {
  font-family: "Noto Serif Devanagari", "Noto Sans Devanagari", serif;
  font-style: normal;
}

.pdf-epigraph .ep-ar {
  font-family: "Noto Naskh Arabic", "Noto Sans Arabic", serif;
  font-style: normal;
  font-size: 7.6pt;
  line-height: 1.54;
  text-align: right;
}

.pdf-epigraph .ep-serif-orig {
  font-family: "Literata", "Noto Serif", "DejaVu Serif", Georgia, serif;
  font-style: italic;
}

.pdf-epigraph .ep-quote {
  font-family: "Literata", "Charter", Georgia, serif;
  font-style: italic;
  font-size: 7.2pt;
  line-height: 1.36;
  color: #141210;
  margin: 0 0 2pt 0;
  text-align: left;
  hyphens: none;
}

.pdf-epigraph .ep-attrib {
  font-family: var(--mono-font);
  font-size: 4.9pt;
  font-weight: 500;
  letter-spacing: 0.03em;
  line-height: 1.32;
  color: #584e43;
  margin: 0;
  text-align: left;
}

.pdf-epigraph .ep-foot {
  display: flex;
  justify-content: space-between;
  padding-top: 4pt;
  border-top: 0.5pt solid #d8cfc0;
  font-family: var(--mono-font);
  font-size: 5.7pt;
  font-weight: 500;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #7a7063;
}

/* Presentation Section (after Table of Contents) */
.pdf-presentation {
  page: pdf-presentation;
  break-before: page;
  page-break-before: always;
  break-after: page;
  page-break-after: always;
}

.pdf-presentation h1 {
  font-size: 18pt;
  line-height: 1.15;
  letter-spacing: -0.02em;
  margin: 4pt 0 9pt 0;
  padding-bottom: 6pt;
  border-bottom: 1.1pt solid var(--border);
  position: relative;
}

.pdf-presentation h1::after {
  content: "";
  position: absolute;
  left: 0;
  bottom: -1.1pt;
  width: 34pt;
  height: 2.4pt;
  background: var(--ch-accent);
}

.pdf-presentation h2 {
  font-size: 10.6pt;
  font-weight: 700;
  color: var(--ink-strong);
  margin: 12pt 0 5pt 0;
  padding-bottom: 2.5pt;
  border-bottom: 0.5pt solid var(--hairline);
  break-after: avoid;
  page-break-after: avoid;
}

.pdf-presentation p {
  text-align: justify;
  text-align-last: left;
  hyphens: auto;
  hyphenate-limit-chars: 6 3 3;
  margin: 0 0 0.78em 0;
  orphans: 3;
  widows: 3;
}

.pdf-presentation .pres-triad-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 6pt;
  margin: 8pt 0 10pt 0;
  break-inside: avoid;
  page-break-inside: avoid;
}

.pdf-presentation .pres-triad-card {
  background: var(--surface);
  border: 0.5pt solid var(--border);
  border-top: 2pt solid var(--card-accent, #8a3a06);
  border-radius: 1.5pt;
  padding: 5.5pt 6.5pt;
}

.pdf-presentation .pres-card-tag {
  font-family: var(--mono-font);
  font-size: 5.9pt;
  font-weight: 500;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--card-accent, #8a3a06);
  margin: 0 0 2pt 0;
  text-align: left;
}

.pdf-presentation .pres-card-title {
  font-family: var(--font-sans);
  font-size: 8.1pt;
  font-weight: 700;
  color: var(--ink-strong);
  margin: 0 0 3pt 0;
  text-align: left;
}

.pdf-presentation .pres-card-body {
  font-size: 7.4pt;
  line-height: 1.44;
  word-spacing: 0.015em;
  color: var(--ink);
  margin: 0;
  text-align: left;
  hyphens: none;
}

.pdf-presentation .pres-paths {
  list-style: none;
  margin: 4pt 0 9pt 0;
  padding: 0;
}

.pdf-presentation .pres-paths li {
  margin: 0 0 4.2pt 0;
  padding-left: 9pt;
  position: relative;
  font-size: 8.1pt;
  line-height: 1.54;
  word-spacing: 0.02em;
  text-align: left;
  hyphens: none;
}

.pdf-presentation .pres-paths li::before {
  content: "";
  position: absolute;
  left: 0;
  top: 0.54em;
  width: 3.5pt;
  height: 3.5pt;
  border-radius: 50%;
  background: #8a3a06;
}

.pdf-presentation a {
  color: #8a3a06;
  text-decoration: underline;
  text-decoration-thickness: 0.45pt;
  text-underline-offset: 1.4pt;
}

.pdf-presentation .pres-note {
  margin-top: 9pt;
  padding: 7pt 9pt;
  background: var(--elevated);
  border-left: 2.5pt solid #8a3a06;
  border-radius: 1.5pt;
  font-size: 7.9pt;
  line-height: 1.52;
  word-spacing: 0.02em;
  text-align: left;
  hyphens: none;
  break-inside: avoid;
  page-break-inside: avoid;
}

.pdf-presentation .pres-note-label {
  font-family: var(--mono-font);
  font-size: 6.1pt;
  font-weight: 500;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #8a3a06;
  display: block;
  margin-bottom: 2.5pt;
}

/* Modernized Swiss Table of Contents */
.pdf-toc {
  page: pdf-toc;
  break-after: page;
  page-break-after: always;
}

.pdf-toc .toc-header {
  margin-bottom: 11pt;
  padding-bottom: 7pt;
}

.pdf-toc .toc-eyebrow {
  font-family: var(--mono-font);
  font-size: 7pt;
  font-weight: 500;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: #8a3a06;
  margin: 0 0 2pt 0;
}

.pdf-toc .toc-title {
  font-family: var(--font-sans);
  font-size: 17.5pt;
  font-weight: 800;
  letter-spacing: -0.025em;
  color: var(--ink-strong);
  margin: 0 0 5pt 0;
  padding: 0;
  border: 0;
  background: none;
}

.pdf-toc .toc-triad-rule {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  height: 1.8pt;
  width: 100%;
}

.pdf-toc .toc-triad-rule > span:nth-child(1) { background: #0d7d57; }
.pdf-toc .toc-triad-rule > span:nth-child(2) { background: #1b52c4; }
.pdf-toc .toc-triad-rule > span:nth-child(3) { background: #c82838; }

.pdf-toc ol {
  list-style: none;
  margin: 0;
  padding: 0;
}

.pdf-toc .toc-part {
  margin: 9pt 0 5pt 0;
  padding-top: 5.5pt;
  border-top: 1.2pt solid var(--toc-part-accent, #8a3a06);
}

.pdf-toc .toc-part-head {
  display: flex;
  align-items: baseline;
  gap: 6pt;
  margin-bottom: 4pt;
  break-after: avoid;
  page-break-after: avoid;
}

.pdf-toc .toc-part-link {
  display: flex;
  align-items: baseline;
  gap: 6pt;
  width: 100%;
  text-decoration: none;
  color: inherit;
}

.pdf-toc .toc-part-badge {
  flex: 0 0 auto;
  display: inline-block;
  font-family: var(--mono-font);
  font-size: 6.7pt;
  font-weight: 500;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #ffffff;
  background: var(--toc-part-accent, #8a3a06);
  padding: 1pt 4.5pt;
  border-radius: 1.5pt;
}

.pdf-toc .toc-part-title {
  flex: 1 1 auto;
  min-width: 0;
  font-family: var(--font-sans);
  font-weight: 700;
  font-size: 9.0pt;
  letter-spacing: -0.01em;
  color: var(--ink-strong);
}

.pdf-toc .toc-part-page {
  flex: 0 0 24pt;
  text-align: right;
  font-family: var(--mono-font);
  font-size: 7.2pt;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
  color: var(--toc-part-accent, #8a3a06);
}

.pdf-toc .toc-group {
  margin: 3.5pt 0 4.5pt 0;
}

.pdf-toc .toc-group-title {
  display: block;
  font-family: var(--mono-font);
  font-weight: 500;
  font-size: 6.4pt;
  letter-spacing: 0.07em;
  text-transform: uppercase;
  color: var(--muted);
  margin: 3pt 0 2pt 0;
  padding-bottom: 1.5pt;
  border-bottom: 0.4pt solid var(--hairline);
  break-after: avoid;
  page-break-after: avoid;
}

.pdf-toc .toc-entries {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  row-gap: 1.6pt;
}

.pdf-toc .toc-entries--single {
  grid-template-columns: minmax(0, 1fr);
}

.pdf-toc .toc-entry {
  margin: 0;
  padding: 1.3pt 0;
  border-bottom: 0.35pt solid #efeae0;
  line-height: 1.35;
  break-inside: avoid;
  page-break-inside: avoid;
}

.pdf-toc .toc-link {
  display: flex;
  align-items: baseline;
  gap: 6pt;
  width: 100%;
  color: var(--ink);
  text-decoration: none;
}

.pdf-toc .toc-code {
  flex: 0 0 26pt;
  font-family: var(--mono-font);
  font-size: 6.6pt;
  font-weight: 500;
  color: var(--toc-part-accent, var(--muted));
}

.pdf-toc .toc-label {
  flex: 1 1 auto;
  min-width: 0;
  font-size: 8.0pt;
  font-weight: 400;
  color: var(--ink);
}

.pdf-toc .toc-page {
  flex: 0 0 24pt;
  text-align: right;
  font-family: var(--mono-font);
  font-size: 6.9pt;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
  color: var(--ink-strong);
}

/* Chapter & Page Standard */
article.pdf-chapter {
  max-width: 100%;
  break-before: page;
  page-break-before: always;
}

/* Per-Chapter Architectural Header Strip */
.pdf-ch-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 4pt 8pt;
  margin: 0 0 8pt 0;
  padding: 4.5pt 0 5pt 0;
  border-top: 2pt solid var(--ch-accent);
  border-bottom: 0.5pt solid var(--hairline);
}

.pdf-ch-head-left {
  display: flex;
  align-items: center;
  gap: 6pt;
}

.pdf-ch-folio {
  font-family: var(--mono-font);
  font-size: 6.4pt;
  font-weight: 500;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ch-accent);
}

.pdf-ch-head-right {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 3.5pt;
}

.pdf-ch-pill {
  display: inline-block;
  font-family: var(--mono-font);
  font-size: 5.9pt;
  font-weight: 500;
  letter-spacing: 0.05em;
  text-transform: uppercase;
  color: var(--ch-accent);
  background: var(--ch-tint);
  border: 0.5pt solid color-mix(in srgb, var(--ch-accent) 28%, var(--border));
  padding: 0.8pt 3.8pt;
  border-radius: 1.5pt;
}

.pdf-ch-pill--muted {
  color: var(--muted);
  background: var(--elevated);
  border-color: var(--border);
}

.pdf-station-bar {
  display: inline-flex;
  align-items: center;
  gap: 1.6pt;
}

.pdf-station-node {
  display: inline-block;
  font-family: var(--mono-font);
  font-size: 5.7pt;
  font-weight: 500;
  letter-spacing: 0.03em;
  padding: 0.6pt 2.6pt;
  border-radius: 1.5pt;
  color: #8a7f70;
  background: #f5f1e9;
  border: 0.45pt solid #dcd4c5;
}

.pdf-station-node.is-active {
  color: #ffffff;
  background: var(--ch-accent);
  border-color: var(--ch-accent);
}

.pdf-station-node.is-super {
  color: #ffffff;
  background: #ad1839;
  border-color: #9c5c04;
}

/* Kind-specific opening flourishes (Part Intro, Conclusion, Stratum Opening) */
article.pdf-chapter[data-kind="part-intro"] h1,
article.pdf-chapter[data-kind="part-conclusion"] h1,
article.pdf-chapter[data-kind="stratum-opening"] h1 {
  font-size: 16.0pt;
  border-bottom-color: var(--ch-accent);
}

.pdf-lead {
  font-family: var(--font-serif);
  font-size: 9.1pt;
  line-height: 1.66;
  word-spacing: 0.025em;
  color: #3a3228;
  text-align: left !important;
  hyphens: none !important;
  margin: 0 0 11pt 0;
  padding: 4pt 0 4pt 10pt;
  border-left: 2pt solid var(--ch-accent);
  background: linear-gradient(to right, var(--ch-tint), transparent 85%);
}

blockquote.pdf-chapter-note {
  margin: 0 0 11pt 0;
  padding: 6.5pt 9.5pt;
  background: var(--ch-tint);
  border: 0.6pt solid var(--border);
  border-left: 3pt solid var(--ch-accent);
  border-radius: 0 1.5pt 1.5pt 0;
  font-size: 8.0pt;
  line-height: 1.56;
  word-spacing: 0.025em;
  text-align: left;
  hyphens: none;
  color: var(--ink);
}

/* Common Headings */
h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-sans);
  line-height: 1.22;
  color: var(--ink-strong);
  break-after: avoid;
  page-break-after: avoid;
}

h1 {
  font-size: 15.2pt;
  font-weight: 800;
  letter-spacing: -0.024em;
  line-height: 1.18;
  margin: 0 0 8.5pt 0;
  padding-bottom: 5pt;
  border-bottom: 0.9pt solid var(--border);
  background: linear-gradient(var(--ch-accent), var(--ch-accent)) left bottom / 38pt 2.3pt no-repeat;
}

h2 {
  font-size: 10.6pt;
  font-weight: 700;
  letter-spacing: -0.014em;
  margin: 1.45em 0 0.5em 0;
  padding-bottom: 3pt;
  border-bottom: 0.6pt solid var(--hairline);
  background: linear-gradient(var(--ch-accent), var(--ch-accent)) left bottom / 16pt 1.5pt no-repeat;
}

h3 {
  font-size: 9.4pt;
  font-weight: 700;
  margin: 1.15em 0 0.35em 0;
  padding-left: 6.5pt;
  border-left: 2pt solid var(--ch-accent);
}

h4 {
  font-size: 8.7pt;
  font-weight: 700;
  margin: 0.95em 0 0.28em 0;
}

h5, h6 {
  font-family: var(--mono-font);
  font-size: 7.2pt;
  font-weight: 500;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--muted);
  margin: 0.85em 0 0.25em 0;
}

p {
  margin: 0 0 0.85em 0;
  orphans: 3;
  widows: 3;
}

article.pdf-chapter > p,
article.pdf-chapter section > p {
  text-align: justify;
  text-align-last: left;
  text-justify: inter-word;
  hyphens: auto;
  hyphenate-limit-chars: 6 3 3;
  text-wrap: pretty;
  overflow-wrap: break-word;
}

article.pdf-chapter :is(ul, ol, li, blockquote) {
  text-align: left;
  hyphens: none;
  word-spacing: 0.025em;
  overflow-wrap: break-word;
}

ul, ol {
  margin: 0.42em 0 0.85em 0;
  padding-left: 1.4em;
}

li {
  margin: 0.30em 0;
  line-height: 1.62;
}

li::marker {
  font-family: var(--mono-font);
  font-size: 0.88em;
  color: var(--ch-accent);
}

a {
  color: var(--ch-accent);
  text-decoration: underline;
  text-decoration-thickness: 0.55pt;
  text-underline-offset: 2pt;
  text-decoration-color: color-mix(in srgb, var(--ch-accent) 42%, transparent);
}

/* Signature Triadic Body Blockquotes */
blockquote {
  margin: 0.9em 0;
  padding: 6.5pt 10pt 6.5pt 12pt;
  line-height: 1.62;
  background: linear-gradient(
      to bottom,
      #1b52c4 0 33.33%,
      #0d7d57 33.33% 66.66%,
      #c82838 66.66% 100%
    )
    left top / 2.4pt 100% no-repeat,
    var(--surface);
  border: 0.5pt solid var(--hairline);
  border-left: 0;
  border-radius: 0 1.5pt 1.5pt 0;
  break-inside: avoid;
}

blockquote > p:last-child {
  margin-bottom: 0;
}

/* Learning Outcomes & Chapter Bridge Plates */
.pdf-outcomes {
  margin: 8pt 0 11pt 0;
  padding: 6.5pt 9.5pt;
  background: var(--ch-tint);
  border: 0.6pt solid var(--border);
  border-top: 1.6pt solid var(--ch-accent);
  border-radius: 0 0 1.5pt 1.5pt;
  text-align: left;
  hyphens: none;
  word-spacing: 0.025em;
  break-inside: avoid;
  page-break-inside: avoid;
}

.pdf-outcomes > h2,
.pdf-bridge > h2 {
  font-family: var(--mono-font);
  font-size: 6.9pt;
  font-weight: 500;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ch-accent);
  margin: 0 0 5pt 0;
  padding: 0;
  border: 0;
  background: none;
}

.pdf-outcomes > ol {
  margin: 0;
  padding-left: 1.35em;
  font-size: 8.2pt;
  line-height: 1.58;
}

.pdf-outcomes > ol > li {
  margin: 0.25em 0;
  text-align: left;
  hyphens: none;
}

.pdf-bridge {
  margin-top: 14pt;
  padding: 7.5pt 10pt;
  background: var(--surface);
  border: 0.6pt solid var(--border);
  border-top: 1.6pt solid var(--ch-accent);
  border-radius: 0 0 1.5pt 1.5pt;
  font-size: 8.3pt;
  line-height: 1.62;
  word-spacing: 0.025em;
  text-align: left;
  hyphens: none;
  break-inside: avoid;
  page-break-inside: avoid;
}

.pdf-bridge > p {
  text-align: left;
  hyphens: none;
}

.pdf-bridge > p:last-child {
  margin-bottom: 0;
}

pre, code, math {
  hyphens: none;
  word-spacing: normal;
}

pre, code {
  font-family: var(--mono-font);
}

code {
  font-size: 7.6pt;
  background-color: var(--code-bg);
  color: var(--code-ink);
  border: 0.45pt solid var(--code-border);
  padding: 0.04em 0.24em;
  border-radius: 1.5pt;
  -webkit-box-decoration-break: clone;
  box-decoration-break: clone;
  overflow-wrap: break-word;
}

pre {
  background-color: #f7f3eb;
  color: var(--code-ink);
  border: 0.6pt solid var(--code-border);
  border-left: 2pt solid #8a7f70;
  border-radius: 1.5pt;
  padding: 0.6em 0.8em;
  font-size: 6.6pt;
  line-height: 1.40;
  text-align: left;
  white-space: pre-wrap;
  overflow-wrap: break-word;
  overflow-x: hidden;
  max-width: 100%;
  break-inside: avoid;
  margin: 0.85em 0;
}

pre code {
  background: none;
  border: 0;
  padding: 0;
  font-size: inherit;
}

/* Warm Archival Ledger Tables */
table {
  width: 100%;
  max-width: 100%;
  border-collapse: collapse;
  margin: 0.95em 0;
  font-size: 7.0pt;
  line-height: 1.46;
  word-spacing: 0.01em;
  letter-spacing: 0;
  border-top: 1.5pt solid var(--ch-accent);
  border-bottom: 1pt solid var(--border);
}

thead {
  display: table-header-group;
}

tr {
  break-inside: avoid;
}

th, td,
article.pdf-chapter :is(th, td, th p, td p) {
  text-align: left;
}

th, td {
  border: 0;
  border-bottom: 0.5pt solid #e2dacb;
  padding: 3.6pt 4.5pt;
  vertical-align: top;
  overflow-wrap: break-word;
  word-break: normal;
}

td, td p, td li {
  hyphens: auto;
  hyphenate-limit-chars: 7 3 3;
}

td code, th code {
  hyphens: none;
  word-break: normal;
  overflow-wrap: break-word;
}

thead th {
  background-color: var(--elevated);
  font-family: var(--mono-font);
  font-size: 5.9pt;
  font-weight: 500;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--ink-strong);
  border-bottom: 1.1pt solid var(--ch-accent);
  hyphens: none;
  overflow-wrap: normal;
  word-break: normal;
}

table.pdf-table-wide {
  font-size: 6.35pt;
  line-height: 1.40;
}

table.pdf-table-wide th,
table.pdf-table-wide td {
  padding: 3.0pt 3.2pt;
}

table.pdf-table-wide thead th {
  font-size: 5.35pt;
  letter-spacing: 0.025em;
}

table.pdf-table-fixed {
  table-layout: fixed;
  width: 100%;
}

table.pdf-table-fixed th,
table.pdf-table-fixed td {
  padding: 2.8pt 2.6pt;
}

table.pdf-table-fixed code {
  font-size: 0.80em;
  padding: 0.3pt 1.2pt;
}

table.pdf-table-fixed math {
  font-size: 0.82em;
}

table.pdf-table-dense {
  font-size: 5.45pt;
  line-height: 1.38;
}

table.pdf-table-dense th,
table.pdf-table-dense td {
  padding: 3.0pt 1.8pt;
}

table.pdf-table-dense thead th {
  font-size: 4.75pt;
  letter-spacing: 0.01em;
}

table.pdf-table-dense code {
  font-size: 0.88em;
  padding: 0.4pt 1.6pt;
}

table.pdf-table-dense math {
  font-size: 0.90em;
}

tbody tr:last-child td {
  border-bottom: 0;
}

/* Instrument Plates & Chapter Emblems */
figure, .stoch-figure {
  position: relative;
  margin: 0.95em 0;
  padding: 7pt 9pt;
  border: 0.6pt solid var(--border);
  border-radius: 1.5pt;
  background: var(--canvas);
  text-align: center;
  break-inside: avoid;
}

.stoch-figure:not(.stoch-logo-figure)::before,
.stoch-figure:not(.stoch-logo-figure)::after {
  content: "";
  position: absolute;
  width: 5.5pt;
  height: 5.5pt;
  pointer-events: none;
  opacity: 0.72;
}

.stoch-figure:not(.stoch-logo-figure)::before {
  top: 2.5pt;
  left: 2.5pt;
  border-top: 1.2pt solid var(--ch-accent);
  border-left: 1.2pt solid var(--ch-accent);
}

.stoch-figure:not(.stoch-logo-figure)::after {
  bottom: 2.5pt;
  right: 2.5pt;
  border-bottom: 1.2pt solid var(--ch-accent);
  border-right: 1.2pt solid var(--ch-accent);
}

figure.stoch-logo-figure {
  max-width: 100%;
  margin: 2pt auto 9pt auto;
  padding: 0 0 6pt 0;
  border: 0;
  border-bottom: 0.6pt solid var(--hairline);
  border-radius: 0;
  background: transparent;
}

figure.stoch-logo-figure::before,
figure.stoch-logo-figure::after {
  content: none;
}

figure svg, .stoch-figure > svg, figure img {
  display: block;
  width: 100%;
  max-width: 100%;
  height: auto;
  margin-inline: auto;
}

figcaption {
  margin-top: 5pt;
  padding-top: 4pt;
  border-top: 0.45pt solid var(--hairline);
  font-size: 7.4pt;
  line-height: 1.44;
  color: var(--muted);
  text-align: center;
  hyphens: none;
}

figcaption strong {
  font-family: var(--mono-font);
  font-size: 6.9pt;
  font-weight: 500;
  letter-spacing: 0.04em;
  color: var(--ink-strong);
}

/* Display Math Plates */
.math-display {
  position: relative;
  display: block;
  max-width: 100%;
  overflow-x: hidden;
  margin: 0.85em 0;
  padding: 6pt 9pt;
  background-color: var(--surface);
  border: 0.6pt solid var(--border);
  border-left: 2.4pt solid var(--ch-accent);
  border-radius: 1.5pt;
  text-align: center;
  break-inside: avoid;
}

.math-display::before,
.math-display::after {
  content: "";
  position: absolute;
  right: 3pt;
  width: 5pt;
  height: 5pt;
  pointer-events: none;
  opacity: 0.45;
}

.math-display::before {
  top: 2.5pt;
  border-top: 1pt solid var(--ch-accent);
  border-right: 1pt solid var(--ch-accent);
}

.math-display::after {
  bottom: 2.5pt;
  border-bottom: 1pt solid var(--ch-accent);
  border-right: 1pt solid var(--ch-accent);
}

.math-display > math + math {
  margin-top: 0.35em;
}

math {
  font-size: 0.96em;
}

math[display="inline"] {
  margin-inline: 0.06em;
}

math.math-compact {
  font-size: 0.80em;
}

hr {
  border: 0;
  border-top: 0.6pt solid var(--hairline);
  margin: 1.25em 0;
}
"""


def _wrap_cover_lines(text: str, max_chars: int = 34) -> list[str]:
    """Wrap a cover subtitle cleanly on word boundaries."""
    words = text.split()
    if not words:
        return []
    lines: list[str] = []
    current = words[0]
    for word in words[1:]:
        if len(current) + 1 + len(word) <= max_chars:
            current = f"{current} {word}"
        else:
            lines.append(current)
            current = word
    lines.append(current)
    return lines


def _lerp_rgb(stops: list[tuple[float, tuple[int, int, int]]], u: float) -> str:
    """Linearly interpolate an RGB color across normalized [0, 1] stops."""
    u = max(0.0, min(1.0, u))
    for i in range(len(stops) - 1):
        u0, c0 = stops[i]
        u1, c1 = stops[i + 1]
        if u <= u1:
            t = 0.0 if u1 == u0 else (u - u0) / (u1 - u0)
            r = round(c0[0] + (c1[0] - c0[0]) * t)
            g = round(c0[1] + (c1[1] - c0[1]) * t)
            b = round(c0[2] + (c1[2] - c0[2]) * t)
            return f"rgb({r},{g},{b})"
    r, g, b = stops[-1][1]
    return f"rgb({r},{g},{b})"


def build_cover_font_head(abs_build: str) -> str:
    """Emit stylesheet and @font-face tags for bundled fonts under `abs_build/fonts/`."""
    fonts_dir = os.path.join(abs_build, "fonts")
    if not os.path.isdir(fonts_dir):
        return ""

    parts: list[str] = []
    css_candidates = sorted(
        fname
        for fname in os.listdir(fonts_dir)
        if fname == "fonts.css" or (fname.startswith("fonts-") and fname.endswith(".css"))
    )
    if css_candidates:
        parts.append(f'  <link rel="stylesheet" href="fonts/{xml_escape_attr(css_candidates[0])}" />')

    face_rules: list[str] = []
    if os.path.isfile(os.path.join(fonts_dir, "literata-400.woff2")):
        face_rules.append(
            "@font-face {\n"
            '  font-family: "Literata";\n'
            "  font-style: normal;\n"
            "  font-weight: 400 700;\n"
            "  font-display: block;\n"
            '  src: url("fonts/literata-400.woff2") format("woff2");\n'
            "}"
        )
    if os.path.isfile(os.path.join(fonts_dir, "literata-italic.woff2")):
        face_rules.append(
            "@font-face {\n"
            '  font-family: "Literata";\n'
            "  font-style: italic;\n"
            "  font-weight: 400 700;\n"
            "  font-display: block;\n"
            '  src: url("fonts/literata-italic.woff2") format("woff2");\n'
            "}"
        )
    if face_rules:
        parts.append("  <style>\n" + "\n".join(face_rules) + "\n  </style>")
    return "\n".join(parts)


def build_cover_svg(meta: dict[str, str]) -> str:
    """Render a full-bleed A4 Swiss Editorial / Instrumentalist vector cover."""
    raw_title = meta.get("title", "Stochology: The Theory and Ethics of Aim under Uncertainty")
    if ":" in raw_title:
        main_title, subtitle_raw = (part.strip() for part in raw_title.split(":", 1))
    else:
        main_title, subtitle_raw = raw_title.strip(), ""
    subtitle_lines = _wrap_cover_lines(subtitle_raw, max_chars=30)[:3] if subtitle_raw else []
    url_display = re.sub(r"^https?://", "", meta.get("url", "stochology.org")).rstrip("/")

    mono_stack = "'Source Code Pro', 'JetBrains Mono', ui-monospace, monospace"
    sans_stack = "'Open Sans', 'Inter', 'Liberation Sans', system-ui, sans-serif"
    serif_stack = "'Literata', 'Charter', Georgia, serif"

    # 1. Calibrated astrolabe ticks (cx=1260, cy=940, r=620)
    cx, cy = 1260.0, 940.0
    r_astrolabe = 620.0
    astrolabe_ticks: list[str] = []
    for deg in range(0, 360, 5):
        rad = math.radians(deg)
        is_major = deg % 30 == 0
        r_in = 602.0 if is_major else 612.0
        r_out = 628.0 if is_major else 620.0
        stroke = "#4a4238" if is_major else "#2c2721"
        sw = "2.0" if is_major else "1.2"
        x1 = cx + r_in * math.cos(rad)
        y1 = cy + r_in * math.sin(rad)
        x2 = cx + r_out * math.cos(rad)
        y2 = cy + r_out * math.sin(rad)
        astrolabe_ticks.append(
            f'      <line x1="{x1:.2f}" y1="{y1:.2f}" x2="{x2:.2f}" y2="{y2:.2f}" '
            f'stroke="{stroke}" stroke-width="{sw}"/>'
        )

    # 2. Generative vertical-bar Pareto heavy-tail density wave (x=140..1960, baseline y=1756)
    bar_stops: list[tuple[float, tuple[int, int, int]]] = [
        (0.00, (32, 118, 92)),
        (0.30, (46, 102, 178)),
        (0.60, (194, 134, 42)),
        (1.00, (200, 40, 56)),
    ]
    n_bars = 92
    p_shape = 3.4
    q_tail = 1.45
    pareto_bars: list[str] = []
    pareto_pts: list[str] = []
    for i in range(n_bars):
        u = i / (n_bars - 1)
        x = 140.0 + 1820.0 * u
        t = (x - 140.0) / 455.0
        if t <= 0.0:
            val = 0.0
        else:
            val = (t**p_shape) * (
                ((p_shape + q_tail) / (q_tail + p_shape * (t**p_shape))) ** ((p_shape + q_tail) / p_shape)
            )
        y_top = 1756.0 - 536.0 * val
        pareto_pts.append(f"{x:.1f},{y_top:.1f}")
        if y_top < 1754.5:
            col = _lerp_rgb(bar_stops, u)
            pareto_bars.append(
                f'      <line x1="{x:.1f}" y1="1756" x2="{x:.1f}" y2="{y_top:.1f}" '
                f'stroke="{col}" stroke-opacity="0.78" stroke-width="3.2"/>'
            )

    # 3. Crosswind ballistic trajectory sheaf: telos (180, 1490) -> Stokhos reticle (1355, 1015)
    sheaf_paths: list[str] = []
    for k in range(-7, 8):
        if k == 0:
            continue
        s = k / 7.0
        c1x = 510.0 + 25.0 * s
        c1y = 1010.0 + 90.0 * s
        c2x = 975.0 + 35.0 * s
        c2y = 855.0 + 125.0 * s
        ex = 1355.0 + 36.0 * s
        ey = 1015.0 + 145.0 * s + 16.0 * s * s
        op = 0.07 + 0.13 * (1.0 - abs(s))
        sheaf_paths.append(
            f'      <path d="M 180 1490 C {c1x:.1f} {c1y:.1f}, {c2x:.1f} {c2y:.1f}, {ex:.1f} {ey:.1f}" '
            f'fill="none" stroke="#f4efe4" stroke-opacity="{op:.3f}" stroke-width="1.4"/>'
        )
    principal_path = (
        '      <path d="M 180 1490 C 510 1010, 975 855, 1355 1015" '
        'fill="none" stroke="#f0ad38" stroke-width="4.2" stroke-linecap="round"/>'
    )

    # 4. Leontief inner triangle (R_tri=415) & clockwise mediation arcs (R_arc=475)
    r_tri = 415.0
    r_arc = 475.0
    v_epi = (cx + r_tri * math.cos(math.radians(-90)), cy + r_tri * math.sin(math.radians(-90)))
    v_gov = (cx + r_tri * math.cos(math.radians(30)), cy + r_tri * math.sin(math.radians(30)))
    v_vital = (cx + r_tri * math.cos(math.radians(150)), cy + r_tri * math.sin(math.radians(150)))
    tri_points = f"{v_epi[0]:.2f},{v_epi[1]:.2f} {v_gov[0]:.2f},{v_gov[1]:.2f} {v_vital[0]:.2f},{v_vital[1]:.2f}"

    arc_specs = [
        (168.0, 252.0, "#36cf97", "T", "1"),
        (-72.0, 12.0, "#5ca8ff", "T", "2"),
        (48.0, 132.0, "#ef4b5b", "T", "3"),
    ]
    arc_svg: list[str] = []
    guard_svg: list[str] = []
    for deg_s, deg_e, color, t_sym, t_idx in arc_specs:
        rad_s = math.radians(deg_s)
        rad_arc_e = math.radians(deg_e - 0.5)
        rad_e = math.radians(deg_e)
        sx, sy = cx + r_arc * math.cos(rad_s), cy + r_arc * math.sin(rad_s)
        ax, ay = cx + r_arc * math.cos(rad_arc_e), cy + r_arc * math.sin(rad_arc_e)
        tx, ty = cx + r_arc * math.cos(rad_e), cy + r_arc * math.sin(rad_e)
        ux, uy = -math.sin(rad_e), math.cos(rad_e)
        nx, ny = math.cos(rad_e), math.sin(rad_e)
        arrow_len, arrow_half = 26.0, 14.0
        w1x, w1y = tx - arrow_len * ux + arrow_half * nx, ty - arrow_len * uy + arrow_half * ny
        w2x, w2y = tx - arrow_len * ux - arrow_half * nx, ty - arrow_len * uy - arrow_half * ny
        arc_svg.append(
            f'      <path d="M {sx:.2f} {sy:.2f} A {r_arc:.0f} {r_arc:.0f} 0 0 1 {ax:.2f} {ay:.2f}" '
            f'fill="none" stroke="{color}" stroke-width="7.5" stroke-linecap="butt"/>'
        )
        arc_svg.append(
            f'      <path d="M {w1x:.2f} {w1y:.2f} L {tx:.2f} {ty:.2f} L {w2x:.2f} {w2y:.2f}" '
            f'fill="none" stroke="{color}" stroke-width="7.0" stroke-linecap="round" stroke-linejoin="round"/>'
        )
        mid_rad = math.radians(0.5 * (deg_s + deg_e))
        mx, my = cx + r_arc * math.cos(mid_rad), cy + r_arc * math.sin(mid_rad)
        lx, ly = cx + 528.0 * math.cos(mid_rad), cy + 528.0 * math.sin(mid_rad)
        anchor = "middle"
        if math.cos(mid_rad) < -0.35:
            anchor = "end"
        elif math.cos(mid_rad) > 0.35:
            anchor = "start"
        if abs(math.sin(mid_rad)) > 0.85:
            ly += 12.0
        guard_svg.append(
            f'      <circle cx="{mx:.2f}" cy="{my:.2f}" r="6.0" fill="#12100e" stroke="{color}" stroke-width="2.6"/>'
        )
        guard_svg.append(
            f'      <text class="cov-mono" x="{lx:.1f}" y="{ly:.1f}" text-anchor="{anchor}" fill="#b8aea0" '
            f'font-family="{mono_stack}" font-size="26" font-weight="500">{t_sym}<tspan dy="5" font-size="18">{t_idx}</tspan></text>'
        )

    # Inner chord guard sites T4, T5, T6 and centroid closure T7
    r_chord = r_tri * 0.5
    inner_guards = [
        (210.0, "4", 176.0, "start"),
        (-30.0, "5", 176.0, "end"),
        (90.0, "6", 178.0, "middle"),
    ]
    for deg_m, t_idx, r_lbl, anchor in inner_guards:
        rad_m = math.radians(deg_m)
        mx, my = cx + r_chord * math.cos(rad_m), cy + r_chord * math.sin(rad_m)
        lx, ly = cx + r_lbl * math.cos(rad_m), cy + r_lbl * math.sin(rad_m)
        if deg_m == 90.0:
            ly += 8.0
        else:
            ly += 14.0
        guard_svg.append(
            f'      <circle cx="{mx:.2f}" cy="{my:.2f}" r="4.2" fill="#f4efe4" fill-opacity="0.55"/>'
        )
        guard_svg.append(
            f'      <text class="cov-mono" x="{lx:.1f}" y="{ly:.1f}" text-anchor="{anchor}" fill="#9d9386" '
            f'font-family="{mono_stack}" font-size="24" font-weight="500">T<tspan dy="5" font-size="17">{t_idx}</tspan></text>'
        )
    # T7 centroid closure
    guard_svg.append(
        f'      <circle cx="{cx:.0f}" cy="{cy:.0f}" r="5.5" fill="#171512" stroke="#9d9386" stroke-width="2"/>'
    )
    guard_svg.append(
        f'      <text class="cov-mono" x="{cx:.0f}" y="{cy - 22:.0f}" text-anchor="middle" fill="#9d9386" '
        f'font-family="{mono_stack}" font-size="24" font-weight="500">T<tspan dy="5" font-size="17">7</tspan></text>'
    )

    subtitle_svg = "\n".join(
        f'  <text class="cov-serif" x="140" y="{2248 + idx * 88}" fill="#3d352c" font-family="{serif_stack}" '
        f'font-size="70" font-weight="400">{xml_escape_text(line)}</text>'
        for idx, line in enumerate(subtitle_lines)
    )

    return f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2100 3019" preserveAspectRatio="xMidYMid slice" role="img" aria-label="{xml_escape_attr(meta["title"])} cover">
  <defs>
    <clipPath id="cov-chamber-clip">
      <rect x="0" y="0" width="2100" height="1756"/>
    </clipPath>
  </defs>

  <!-- Full-bleed split grounds: warm archival paper plinth & matte obsidian optical chamber -->
  <rect width="2100" height="3019" fill="#f4efe4"/>
  <rect x="0" y="0" width="2100" height="1756" fill="#12100e"/>

  <!-- Upper optical chamber -->
  <g clip-path="url(#cov-chamber-clip)">
    <!-- Subtle three-column drafting grid -->
    <g stroke="#1e1b17" stroke-width="1">
      <line x1="140" y1="0" x2="140" y2="1756"/>
      <line x1="747" y1="0" x2="747" y2="1756"/>
      <line x1="1353" y1="0" x2="1353" y2="1756"/>
      <line x1="1960" y1="0" x2="1960" y2="1756"/>
    </g>

    <!-- Running top technical header -->
    <text class="cov-mono" x="140" y="116" fill="#f4efe4" font-family="{mono_stack}" font-size="32" font-weight="500" letter-spacing="4.2">STÓKHOS · LÓGOS</text>
    <text class="cov-mono" x="1960" y="116" text-anchor="end" fill="#b4aa9c" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="2.5">00—99 · VII PARTS · XXXV ORDERS</text>
    <line x1="140" y1="164" x2="1960" y2="164" stroke="#2c2721" stroke-width="1.5"/>
    <g stroke="#4a4238" stroke-width="1.5">
      <line x1="140" y1="158" x2="140" y2="170"/>
      <line x1="747" y1="158" x2="747" y2="170"/>
      <line x1="1353" y1="158" x2="1353" y2="170"/>
      <line x1="1960" y1="158" x2="1960" y2="170"/>
    </g>

    <!-- Minimalist upper-left Nuclear Triad diagnostic spec block -->
    <g>
      <text class="cov-mono" x="140" y="264" fill="#9d9386" font-family="{mono_stack}" font-size="26" font-weight="500" letter-spacing="3.2">NUCLEAR TRIAD</text>
      <rect x="140" y="300" width="18" height="18" fill="#5ca8ff"/>
      <text class="cov-mono" x="176" y="316" fill="#e5dfd2" font-family="{mono_stack}" font-size="28" font-weight="500">S<tspan dy="6" font-size="19">epi</tspan><tspan dy="-6"> · EPISTEMIC</tspan></text>
      <rect x="140" y="354" width="18" height="18" fill="#36cf97"/>
      <text class="cov-mono" x="176" y="370" fill="#e5dfd2" font-family="{mono_stack}" font-size="28" font-weight="500">S<tspan dy="6" font-size="19">vital</tspan><tspan dy="-6"> · VITAL</tspan></text>
      <rect x="140" y="408" width="18" height="18" fill="#ef4b5b"/>
      <text class="cov-mono" x="176" y="424" fill="#e5dfd2" font-family="{mono_stack}" font-size="28" font-weight="500">S<tspan dy="6" font-size="19">gov</tspan><tspan dy="-6"> · GOVERNANCE</tspan></text>
      <line x1="140" y1="468" x2="610" y2="468" stroke="#2c2721" stroke-width="1.5"/>
      <text class="cov-mono" x="140" y="516" fill="#a89e90" font-family="{mono_stack}" font-size="26" font-weight="500">M = min(S<tspan dy="5" font-size="18">epi</tspan><tspan dy="-5">, S</tspan><tspan dy="5" font-size="18">vital</tspan><tspan dy="-5">, S</tspan><tspan dy="5" font-size="18">gov</tspan><tspan dy="-5">)</tspan></text>
      <text class="cov-mono" x="140" y="564" fill="#a89e90" font-family="{mono_stack}" font-size="26" font-weight="500">D = (M, Θ<tspan dy="5" font-size="18">basin</tspan><tspan dy="-5">) · T</tspan><tspan dy="5" font-size="18">1</tspan><tspan dy="-5">—T</tspan><tspan dy="5" font-size="18">7</tspan></text>
    </g>

    <!-- Generative heavy-tailed Pareto density wave (alpha < 2) -->
    <g>
{chr(10).join(pareto_bars)}
      <polyline points="{" ".join(pareto_pts)}" fill="none" stroke="#f4efe4" stroke-opacity="0.72" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
    </g>

    <!-- Off-center calibrated astrolabe & triad circuit -->
    <g>
      <circle cx="{cx:.0f}" cy="{cy:.0f}" r="{r_astrolabe:.0f}" fill="none" stroke="#2a251f" stroke-width="1.6"/>
{chr(10).join(astrolabe_ticks)}

      <!-- Inner dashed non-substitutable Leontief triangle -->
      <polygon points="{tri_points}" fill="#171512" fill-opacity="0.55" stroke="#f4efe4" stroke-opacity="0.38" stroke-width="2.2" stroke-dasharray="7 8" stroke-linejoin="round"/>

      <!-- Directed clockwise mediation arcs -->
{chr(10).join(arc_svg)}

      <!-- Guard sites T1..T7 -->
{chr(10).join(guard_svg)}
    </g>

    <!-- Crosswind ballistic trajectory sheaf -->
    <g>
{chr(10).join(sheaf_paths)}
{principal_path}
      <circle cx="180" cy="1490" r="10" fill="#12100e" stroke="#f4efe4" stroke-width="2.6"/>
      <circle cx="180" cy="1490" r="3.2" fill="#f0ad38"/>
      <text class="cov-serif" x="180" y="1538" text-anchor="middle" fill="#c6bdb0" font-family="{serif_stack}" font-size="30" font-style="italic">télos</text>
    </g>

    <!-- Precision solar-ochre Stokhos reticle & threshold axes -->
    <g>
      <line x1="1355" y1="1061" x2="1355" y2="1756" stroke="#f0ad38" stroke-opacity="0.45" stroke-width="1.8" stroke-dasharray="6 7"/>
      <text class="cov-mono" x="1374" y="1726" fill="#f0ad38" fill-opacity="0.92" paint-order="stroke" stroke="#12100e" stroke-width="7" stroke-linejoin="round" font-family="{mono_stack}" font-size="28" font-weight="500">p*</text>
      <line x1="860" y1="1015" x2="1309" y2="1015" stroke="#f0ad38" stroke-opacity="0.35" stroke-width="1.6" stroke-dasharray="6 7"/>
      <line x1="1401" y1="1015" x2="1780" y2="1015" stroke="#f0ad38" stroke-opacity="0.35" stroke-width="1.6" stroke-dasharray="6 7"/>
      <circle cx="1355" cy="1015" r="46" fill="none" stroke="#f0ad38" stroke-opacity="0.58" stroke-width="1.6"/>
      <circle cx="1355" cy="1015" r="25" fill="#12100e" stroke="#f0ad38" stroke-width="4.2"/>
      <line x1="1299" y1="1015" x2="1343" y2="1015" stroke="#f0ad38" stroke-width="3.2"/>
      <line x1="1367" y1="1015" x2="1411" y2="1015" stroke="#f0ad38" stroke-width="3.2"/>
      <line x1="1355" y1="959" x2="1355" y2="1003" stroke="#f0ad38" stroke-width="3.2"/>
      <line x1="1355" y1="1027" x2="1355" y2="1071" stroke="#f0ad38" stroke-width="3.2"/>
      <circle cx="1355" cy="1015" r="6" fill="#f4efe4"/>
    </g>

    <!-- Double-ring Nuclear Triad nodes & plane labels -->
    <g>
      <!-- Archive / S_epi -->
      <circle cx="{v_epi[0]:.2f}" cy="{v_epi[1]:.2f}" r="34" fill="#12100e"/>
      <circle cx="{v_epi[0]:.2f}" cy="{v_epi[1]:.2f}" r="28" fill="#12100e" stroke="#5ca8ff" stroke-width="5"/>
      <circle cx="{v_epi[0]:.2f}" cy="{v_epi[1]:.2f}" r="10" fill="#5ca8ff"/>
      <text class="cov-mono" x="{v_epi[0]:.1f}" y="{v_epi[1] - 78:.1f}" text-anchor="middle" fill="#e5dfd2" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="1.4">ARCHIVE · S<tspan dy="5" font-size="20">epi</tspan></text>

      <!-- World / S_vital -->
      <circle cx="{v_vital[0]:.2f}" cy="{v_vital[1]:.2f}" r="34" fill="#12100e"/>
      <circle cx="{v_vital[0]:.2f}" cy="{v_vital[1]:.2f}" r="28" fill="#12100e" stroke="#36cf97" stroke-width="5"/>
      <circle cx="{v_vital[0]:.2f}" cy="{v_vital[1]:.2f}" r="10" fill="#36cf97"/>
      <text class="cov-mono" x="{v_vital[0]:.1f}" y="{v_vital[1] + 88:.1f}" text-anchor="middle" fill="#e5dfd2" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="1.4">WORLD · S<tspan dy="5" font-size="20">vital</tspan></text>

      <!-- Institution / S_gov -->
      <circle cx="{v_gov[0]:.2f}" cy="{v_gov[1]:.2f}" r="34" fill="#12100e"/>
      <circle cx="{v_gov[0]:.2f}" cy="{v_gov[1]:.2f}" r="28" fill="#12100e" stroke="#ef4b5b" stroke-width="5"/>
      <circle cx="{v_gov[0]:.2f}" cy="{v_gov[1]:.2f}" r="10" fill="#ef4b5b"/>
      <text class="cov-mono" x="{v_gov[0]:.1f}" y="{v_gov[1] + 88:.1f}" text-anchor="middle" fill="#e5dfd2" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="1.4">INSTITUTION · S<tspan dy="5" font-size="20">gov</tspan></text>
    </g>
  </g>

  <!-- Crisp 12px flat three-plane horizon divider -->
  <rect x="0" y="1756" width="700" height="12" fill="#0d7d57"/>
  <rect x="700" y="1756" width="700" height="12" fill="#1b52c4"/>
  <rect x="1400" y="1756" width="700" height="12" fill="#c82838"/>

  <!-- Lower warm archival paper plinth: Swiss typography & colophon -->
  <text class="cov-mono" x="140" y="1866" fill="#8a3a06" font-family="{mono_stack}" font-size="32" font-weight="500" letter-spacing="4.2">THEORY &amp; ETHICS OF AIM</text>
  <text class="cov-sans" x="130" y="2118" fill="#141210" font-family="{sans_stack}" font-size="252" font-weight="800" letter-spacing="-9">{xml_escape_text(main_title)}</text>
{subtitle_svg}

  <line x1="140" y1="2660" x2="1960" y2="2660" stroke="#141210" stroke-width="2.5"/>
  <text class="cov-sans" x="140" y="2748" fill="#141210" font-family="{sans_stack}" font-size="48" font-weight="700" letter-spacing="4">{xml_escape_text(meta["author"].upper())}</text>
  <text class="cov-mono" x="140" y="2816" fill="#584e43" font-family="{mono_stack}" font-size="28" font-weight="500">Open Treatise · ∞, Ø, 00–99 &amp; Appendices A–G</text>
  <text class="cov-mono" x="1960" y="2748" text-anchor="end" fill="#141210" font-family="{mono_stack}" font-size="34" font-weight="500">{xml_escape_text(url_display)}</text>
  <text class="cov-mono" x="1960" y="2816" text-anchor="end" fill="#584e43" font-family="{mono_stack}" font-size="28" font-weight="500">{xml_escape_text(meta["rights"])} · {xml_escape_text(meta["date"])}</text>
</svg>"""


def build_title_page_html(meta: dict[str, str]) -> str:
    """Render the formal interior Title Page (Folha de Rosto) preceding the TOC."""
    raw_title = meta.get("title", "Stochology: The Theory and Ethics of Aim under Uncertainty")
    if ":" in raw_title:
        main_title, subtitle = (part.strip() for part in raw_title.split(":", 1))
    else:
        main_title, subtitle = raw_title.strip(), "The Theory and Ethics of Aim under Uncertainty"
    url_display = re.sub(r"^https?://", "", meta.get("url", "stochology.org")).rstrip("/")

    emblem_svg = """<svg class="tp-emblem" viewBox="0 0 120 120" role="img" aria-label="Nuclear Triad emblem">
  <circle cx="60" cy="60" r="52" fill="none" stroke="#d8cfc0" stroke-width="1.2" stroke-dasharray="3 3"/>
  <circle cx="60" cy="60" r="37" fill="none" stroke="#b4aa9c" stroke-width="1"/>
  <polygon points="60,21 94,80 26,80" fill="none" stroke="#141210" stroke-width="1.4" stroke-dasharray="4 3"/>
  <circle cx="60" cy="60" r="9" fill="none" stroke="#8a3a06" stroke-width="1.6"/>
  <line x1="46" y1="60" x2="74" y2="60" stroke="#8a3a06" stroke-width="1.2"/>
  <line x1="60" y1="46" x2="60" y2="74" stroke="#8a3a06" stroke-width="1.2"/>
  <circle cx="60" cy="60" r="2.2" fill="#8a3a06"/>
  <circle cx="60" cy="21" r="6.5" fill="#ffffff" stroke="#1b52c4" stroke-width="2.5"/>
  <circle cx="26" cy="80" r="6.5" fill="#ffffff" stroke="#0d7d57" stroke-width="2.5"/>
  <circle cx="94" cy="80" r="6.5" fill="#ffffff" stroke="#c82838" stroke-width="2.5"/>
</svg>"""

    return f"""  <section class="pdf-title-page" aria-label="Title page">
    <div class="tp-top">
      <p class="tp-eyebrow">STÓKHOS · LÓGOS · OPEN TREATISE</p>
      <div class="tp-triad-rule" aria-hidden="true"><span></span><span></span><span></span></div>
    </div>

    <div class="tp-center">
      <p class="tp-author">{xml_escape_text(meta["author"].upper())}</p>
      <h1 class="tp-title">{xml_escape_text(main_title)}</h1>
      <p class="tp-subtitle">{xml_escape_text(subtitle)}</p>

      <div class="tp-emblem-wrap">
        {emblem_svg}
        <div class="tp-emblem-meta">
          <p class="tp-emblem-kicker">THE FULL MEDIATION CIRCUIT · WORLD · ARCHIVE · INSTITUTION</p>
          <p class="tp-emblem-eq"><i>M</i><sub>Leontief</sub> = min(<i>S</i><sub>epi</sub>, <i>S</i><sub>vital</sub>, <i>S</i><sub>gov</sub>) &nbsp;·&nbsp; <i>D</i><sub>diag</sub> = (<i>M</i><sub>Leontief</sub>, &Theta;<sub>basin</sub>)<br/>Three Canonical Operators &nbsp;·&nbsp; Seven Guard Sites (<i>T</i><sub>1</sub>–<i>T</i><sub>7</sub>) &nbsp;·&nbsp; Thirty-Five Knowledge Orders</p>
        </div>
      </div>
    </div>

    <div class="tp-colophon">
      <div>
        <strong>∞ EPIGRAPH · Ø PRESENTATION · PROLOGUE (00) · PARTS I–VII (01–99) · APPENDICES A–G</strong><br/>
        Open-Source Manuscript · Licensed under {xml_escape_text(meta["rights"])}<br/>
        github.com/0b3st4f3r4/stochology
      </div>
      <div class="tp-colophon-right">
        <strong>{xml_escape_text(url_display)}</strong><br/>
        Print Edition · 160 &times; 230 mm<br/>
        {xml_escape_text(meta["date"])}
      </div>
    </div>
  </section>"""


def build_epigraph_html(meta: dict[str, str]) -> str:
    """Render the dedicated 10-tradition Epigraph (∞) leaf between Title Page and Contents."""
    _ = meta
    lemniscate_svg = """<svg class="ep-lemniscate" viewBox="0 0 88 36" role="img" aria-label="Infinity epigraph mark">
      <line x1="2" y1="18" x2="7" y2="18" stroke="#cfc6b6" stroke-width="1"/>
      <line x1="81" y1="18" x2="86" y2="18" stroke="#cfc6b6" stroke-width="1"/>
      <path d="M 44 18 C 34 5, 10 5, 10 18 C 10 31, 34 31, 44 18 C 54 5, 78 5, 78 18 C 78 31, 54 31, 44 18 Z" fill="none" stroke="#8a3a06" stroke-width="2.1" stroke-linejoin="round"/>
      <circle cx="22" cy="18" r="2.1" fill="#0d7d57"/>
      <circle cx="44" cy="18" r="2.1" fill="#1b52c4"/>
      <circle cx="66" cy="18" r="2.1" fill="#c82838"/>
    </svg>"""
    return f"""  <section id="front-epigraph" class="pdf-epigraph" aria-label="Epigraph">
    <div class="ep-top">
      <span class="ep-brand">
        <span class="ep-kicker">&#8734; &middot; EPIGRAPH</span>
        {lemniscate_svg}
      </span>
      <span class="ep-tag">TEN ANCESTRAL HORIZONS</span>
    </div>

    <div class="ep-columns">
      <div class="ep-col">
        <article class="ep-entry">
          <p class="ep-region">Nile Valley &middot; Ancient Africa &middot; c. 1900 BCE</p>
          <p class="ep-quote">&ldquo;The limits of art are not reached, no artist&rsquo;s skills are perfect; good speech is more hidden than greenstone, yet may be found among maids at the grindstones.&rdquo;</p>
          <p class="ep-attrib">&mdash; Ptahhotep, <em>Instruction of Ptahhotep</em>, Maxim 1 (trans. M. Lichtheim, 1973)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">Sub-Saharan Africa &amp; Diaspora &middot; Akan</p>
          <p class="ep-orig ep-serif-orig" lang="ak">&laquo;Sɛ wo were fi na wosankofa a yɛnkyi.&raquo;</p>
          <p class="ep-quote">&ldquo;It is not taboo to go back and fetch what you have forgotten.&rdquo;</p>
          <p class="ep-attrib">&mdash; Akan Proverb &middot; <em>Sankofa</em> (Twi oral tradition)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">East Asia &middot; Sinosphere &middot; c. 5th c. BCE</p>
          <p class="ep-orig ep-cjk" lang="lzh">「昔之善戰者，先為不可勝，以待敵之可勝。不可勝在己，可勝在敵。」</p>
          <p class="ep-quote">&ldquo;Anciently the skilful warriors first made themselves invincible and awaited the enemy&rsquo;s moment of vulnerability. Invincibility depends on one&rsquo;s self; the enemy&rsquo;s vulnerability on him.&rdquo;</p>
          <p class="ep-attrib">&mdash; Sunzi, <em>The Art of War</em> (<em>Sūnzǐ Bīngfǎ</em>) IV.1&ndash;2 (trans. S. B. Griffith, 1963)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">South Asia &middot; Sanskrit &middot; c. 5th&ndash;2nd c. BCE</p>
          <p class="ep-orig ep-deva" lang="sa">&laquo;कर्मण्येवाधिकारस्ते मा फलेषु कदाचन।&raquo;</p>
          <p class="ep-quote">&ldquo;To action alone hast thou a right and never at all to its fruits&hellip;&rdquo;</p>
          <p class="ep-attrib">&mdash; <em>Bhagavad Gītā</em> II.47 (trans. S. Radhakrishnan, 1948)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">Southeast Asia &middot; Pāli Canon</p>
          <p class="ep-orig ep-serif-orig" lang="pi">&laquo;Mā anussavena, mā paramparāya, mā itikirāya, mā piṭakasampadānena&hellip;&raquo;</p>
          <p class="ep-quote">&ldquo;Do not go by oral tradition, by lineage of teaching, by hearsay, by a collection of scriptures&hellip;&rdquo;</p>
          <p class="ep-attrib">&mdash; <em>Kālāma Sutta</em> (<em>Aṅguttara Nikāya</em> 3.65; trans. Bhikkhu Bodhi)</p>
        </article>
      </div>

      <div class="ep-col">
        <article class="ep-entry">
          <p class="ep-region">MENA &amp; Islamic World &middot; Arabic &middot; c. 1028 CE</p>
          <p class="ep-orig ep-ar" lang="ar" dir="rtl">&laquo;فطالب الحق ليس هو الناظر في كتب المتقدمين، المسترسل مع طبعه في حسن الظن بهم، بل طالب الحق هو المتهم لظنه فيهم، المتوقف فيما يفهمه عنهم، المتبع الحجة والبرهان&hellip;&raquo;</p>
          <p class="ep-quote">&ldquo;The seeker after the truth is not one who studies the writings of the ancients and, following his natural disposition, puts his trust in them, but rather the one who suspects his faith in them and questions what he gathers from them, the one who submits to argument and demonstration&hellip;&rdquo;</p>
          <p class="ep-attrib">&mdash; Ibn al-Haytham, <em>Al-Shukūk ʿalā Baṭlamyūs</em> (trans. A. I. Sabra, 2003; cf. Sabra &amp; Shehaby, 1971)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">Mediterranean &amp; Europe &middot; Classical Greek &middot; c. 400 BCE</p>
          <p class="ep-orig ep-serif-orig" lang="grc">&laquo;Ὁ βίος βραχύς, ἡ δὲ τέχνη μακρή, ὁ δὲ καιρὸς ὀξύς, ἡ δὲ πεῖρα σφαλερή, ἡ δὲ κρίσις χαλεπή.&raquo;</p>
          <p class="ep-quote">&ldquo;Life is short, the art long, <em>kair&oacute;s</em> fleeting, experience perilous, judgment difficult.&rdquo;</p>
          <p class="ep-attrib">&mdash; Hippocrates, <em>Aphorisms</em> I.1 (after W. H. S. Jones, LCL 150)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">Mesoamerica &middot; Nahuatl</p>
          <p class="ep-orig ep-serif-orig" lang="nci">&laquo;Tlaalahui, tlapetzcahui in tlalticpac.&raquo;</p>
          <p class="ep-quote">&ldquo;It is slippery, it is slick on the earth.&rdquo;</p>
          <p class="ep-attrib">&mdash; <em>Huehuetlahtolli</em> &middot; <em>Florentine Codex</em> VI (Anderson &amp; Dibble)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">Andes &middot; South America &middot; Aymara</p>
          <p class="ep-orig ep-serif-orig" lang="ay">&laquo;Qhipnayr uñtasis sarnaqapxañani.&raquo;</p>
          <p class="ep-quote">&ldquo;Looking to the past before our eyes and the future behind, let us walk on.&rdquo;</p>
          <p class="ep-attrib">&mdash; Aymara Maxim &middot; <em>Qhipnayra</em> (Núñez &amp; Sweetser, 2006; Rivera Cusicanqui, 2012)</p>
        </article>

        <article class="ep-entry">
          <p class="ep-region">Oceania &middot; Pacific &middot; Māori</p>
          <p class="ep-orig ep-serif-orig" lang="mi">&laquo;Kia whakatōmuri te haere whakamua.&raquo;</p>
          <p class="ep-quote">&ldquo;I walk backwards into the future with my eyes fixed on my past.&rdquo;</p>
          <p class="ep-attrib">&mdash; Māori <em>Whakataukī</em> (cf. L. Rameka, 2016)</p>
        </article>
      </div>
    </div>

    <div class="ep-foot">
      <span>&#8734; OPEN HORIZON &middot; BEFORE THRESHOLD &Oslash;</span>
      <span>ST&Oacute;KHOS &middot; L&Oacute;GOS</span>
    </div>
  </section>"""


def build_presentation_html(meta: dict[str, str]) -> str:
    """Render the Presentation (Ø) framing page immediately after Contents."""
    _ = meta
    return """  <section id="front-presentation" class="pdf-presentation" style="--part-accent: #8a3a06; --part-tint: #fdf7ef; --ch-accent: #8a3a06; --ch-tint: #fdf7ef;" aria-label="Presentation">
    <div class="pdf-ch-head">
      <span class="pdf-ch-kicker">&Oslash; &middot; PRESENTATION</span>
      <div class="pdf-ch-badges">
        <span class="pdf-badge pdf-badge-track">THRESHOLD &Oslash;</span>
      </div>
    </div>

    <h1>&Oslash; &mdash; Presentation</h1>

    <p class="pdf-lead">Before 00 enters speech and before 01 commits a measure, &Oslash; marks the empty threshold: the unframed ground from which an agent prepares to aim without counterfeiting certainty, exhausting survival reserves, or displacing ruin.</p>

    <p><strong>Stochology</strong> (from Classical Greek <em>st&oacute;khos</em>, aim or conjecture, and <em>l&oacute;gos</em>, reasoned account) studies judgment when action cannot wait for certainty. While its Greek name joins Plato&rsquo;s <em>Philebus</em>, Hippocratic <em>kair&oacute;s</em>, Stoic archery, and Bernoulli&rsquo;s <em>Ars Conjectandi</em>, the invariant it formalizes converges across Africa, Asia, the Islamic world, Abya Yala, and Oceania&mdash;and traces through Brahmagupta, Liu Hui, Ibn al-Haytham, Mahalanobis, and Rao into the joint custody of <strong>World</strong>, <strong>Archive</strong>, and <strong>Institution</strong>.</p>

    <p>Rather than treating inference, physical buffering, and institutional governance as separate disciplines, Stochology binds them through a Leontief Nuclear Triad, <i>M</i><sub>Leontief</sub> = min(<i>S</i><sub>epi</sub>, <i>S</i><sub>vital</sub>, <i>S</i><sub>gov</sub>), paired in retrospective audit with the reception-basin fold &Theta;<sub>basin</sub>. The minimum is not a weighted sum: abundant precision cannot pay off an exhausted physiological buffer, nor can ample physical slack excuse testimonial dismissal or undisclosed downstream hazard. Strictly diagnostic, the co-diagnostic pair evaluates finished trajectories without functioning as an automated control rule.</p>

    <div class="pres-triad-grid">
      <div class="pres-triad-card" style="--card-accent: #1b52c4;">
        <p class="pres-card-tag">AXIS I &middot; ARCHIVE</p>
        <p class="pres-card-title">Epistemic Margin (<i>S</i><sub>epi</sub>)</p>
        <p class="pres-card-body">Partial-identification bounds, non-random missingness (&rho;<sub><i>R,Y</i></sub> &ne; 0), and additive design effects without false point certainty.</p>
      </div>
      <div class="pres-triad-card" style="--card-accent: #0d7d57;">
        <p class="pres-card-tag">AXIS II &middot; WORLD</p>
        <p class="pres-card-title">Vital Margin (<i>S</i><sub>vital</sub>)</p>
        <p class="pres-card-body">Physical buffers, convex loss sensitivity, non-ergodic paths, and survival horizons exceeding recovery time before ruin barriers.</p>
      </div>
      <div class="pres-triad-card" style="--card-accent: #c82838;">
        <p class="pres-card-tag">AXIS III &middot; INSTITUTION</p>
        <p class="pres-card-title">Governance (<i>S</i><sub>gov</sub>)</p>
        <p class="pres-card-body">Unrigged premises, pre-sealed criteria, algedonic distress channels, and refusal to displace harm onto external reception basins.</p>
      </div>
    </div>

    <h2>Architecture &amp; Reading Paths</h2>

    <p>The book progresses through an essayistic <a href="#doc-00-preface">Prologue (00)</a>, ninety-nine numbered chapters organized in seven parts, and seven technical reference appendices:</p>

    <ul class="pres-paths">
      <li><strong>Intuition &amp; Bases (<a href="#doc-part-i-introduction">Part I</a> &amp; <a href="#doc-part-ii-introduction">Part II</a>):</strong> Enters through everyday decisions under irreversible time (<a href="#doc-08-three-things-you-cant-trade">Ch. 01&ndash;08</a>), then builds the mathematical, epistemological, and natural-science substrates (<a href="#doc-appendix-a-mathematics">Ch. 09&ndash;26 &amp; App. A</a>).</li>
      <li><strong>Foundations &amp; Worked Loop (<a href="#doc-part-iii-introduction">Part III</a> &amp; <a href="#doc-part-iv-introduction">Part IV</a>):</strong> Fixes the three-plane circuit (<a href="#doc-29-aim-and-the-three-planes">Ch. 29</a>), the triadic minimum (<a href="#doc-41-the-triad-and-the-minimum">Ch. 41</a>), and follows one running case across all three filters through the reflexive brakes and falsifiers in <a href="#doc-50-methodological-families-reflexivity">Ch. 50</a>.</li>
      <li><strong>Multilayer Graph &amp; Guard Trajectories (<a href="#doc-part-v-introduction">Parts V</a>&ndash;<a href="#doc-part-vii-introduction">VII</a>):</strong> Maps 35 orders of knowledge across seven strata (<a href="#doc-appendix-b-architecture-verification">App. B</a>), isolates guard sites <i>T</i><sub>1</sub>&ndash;<i>T</i><sub>7</sub> chronologically along a single shot (Ch. 86&ndash;92), and stress-tests simultaneous superposition and attested reopening (Ch. 93&ndash;99).</li>
      <li><strong>Reference Apparatus (<a href="#doc-appendix-c-sources">Appendices C</a>&ndash;<a href="#doc-appendix-g-glossary">G</a>):</strong> Collects primary sources (<a href="#doc-appendix-c-sources">App. C</a>), the quantitative claim ledger (<a href="#doc-appendix-d-claim-ledger">App. D</a>), guard lexicon (<a href="#doc-appendix-e-lexicon">App. E</a>), chapter synopses (<a href="#doc-appendix-f-manuscript-audit">App. F</a>), and alphabetical symbol index (<a href="#doc-appendix-g-glossary">App. G</a>).</li>
    </ul>

    <div class="pres-note">
      <span class="pres-note-label">EDITORIAL &amp; PROVENANCE STANDARD</span>
      Authority flows strictly from chapters to appendices: wherever a summary table diverges from a chapter, the chapter governs. Quantitative statements throughout the text rest on exactly two warrants stated in place—either published values cited verbatim at their source precision or explicit algebraic derivations displayed beside the prose. This edition is formatted at <strong>160 &times; 230 mm</strong> with mirrored binding margins under <strong>CC BY 4.0</strong>.
    </div>
  </section>"""


def build_art_plate_html(filename: str, css_mod: str, aria_label: str, alt_text: str) -> str:
    """Embed a full-bleed ultrarealistic raster plate from book-src/plates/."""
    plate_path = os.path.join(PLATES_DIR, filename)
    if not os.path.isfile(plate_path):
        raise FileNotFoundError(f"Missing PDF art plate asset: {plate_path}")
    ext = os.path.splitext(filename)[1].lower()
    mime = {
        ".jpg": "image/jpeg",
        ".jpeg": "image/jpeg",
        ".png": "image/png",
        ".webp": "image/webp",
    }.get(ext, "image/jpeg")
    with open(plate_path, "rb") as handle:
        b64 = base64.b64encode(handle.read()).decode("ascii")
    return (
        f'  <section class="pdf-art-plate {xml_escape_attr(css_mod)}" aria-label="{xml_escape_attr(aria_label)}">\n'
        f'    <img src="data:{mime};base64,{b64}" alt="{xml_escape_attr(alt_text)}" />\n'
        "  </section>"
    )


PART_ART_PLATES: dict[str, tuple[str, str, str]] = {
    "part-i": (
        "part-i-plate.jpg",
        "Part I artistic plate — Thinking Under Uncertainty",
        "Dusk coastal plate of a brass storm-lantern and coiled hemp mooring rope on wet dark basalt pier overlooking churning slate waves and a narrow golden shaft under dark storm clouds.",
    ),
    "part-ii": (
        "part-ii-plate.jpg",
        "Part II artistic plate — The Bases",
        "Studio macro plate on midnight-blue slate showing a crystal icosahedron, smooth brass rings, spherical glass flask with cobalt ink plumes, nautilus section, and quartz crystals.",
    ),
    "part-iii": (
        "part-iii-plate.jpg",
        "Part III artistic plate — Foundations of Judgment",
        "Chiaroscuro studio plate of a darkened steel and verdigris-copper double-pan balance scale with three stone weights and a marble plumb-weight inside a beveled glass case pierced by turquoise-teal light.",
    ),
    "part-iv": (
        "part-iv-plate.jpg",
        "Part IV artistic plate — The Framework",
        "Forest twilight architectural plate of a carved dark basalt terraced hydraulic weir with three bronze sluice gates, emerald mountain water, and a smooth overflow spillway.",
    ),
    "part-v": (
        "part-v-plate.jpg",
        "Part V artistic plate — Some Orders of Knowledge and Application",
        "Architectural gallery plate of seven suspended dark-smoked glass shelves holding five polished mineral spheres per stratum linked by vertical tensioned silver threads under amethyst skylight.",
    ),
    "part-vi": (
        "part-vi-plate.jpg",
        "Part VI artistic plate — The Trajectory of Aim",
        "High-speed studio plate of a walnut-wood and burnished-brass arrow with barred feather fletching flying diagonally through aligned translucent smoked-glass rings under solar-gold rim light.",
    ),
    "part-vii": (
        "part-vii-plate.jpg",
        "Part VII artistic plate — The Superposed Aim",
        "Overhead obsidian plate of intersecting concentric wave-interference ripples inside a brushed-steel ring refracting ruby-crimson and amber caustics through fractured obsidian prisms beside a cracked wax-and-brass seal.",
    ),
}


def build_back_cover_svg(meta: dict[str, str]) -> str:
    """Render a full-bleed 160x230mm Swiss Editorial / Instrumentalist vector back cover."""
    url_display = re.sub(r"^https?://", "", meta.get("url", "stochology.org")).rstrip("/")
    mono_stack = "'Source Code Pro', 'JetBrains Mono', ui-monospace, monospace"
    sans_stack = "'Open Sans', 'Inter', 'Liberation Sans', system-ui, sans-serif"
    serif_stack = "'Literata', 'Charter', Georgia, serif"

    para1_lines = _wrap_cover_lines(
        "Linking convergent traditions across Africa, Asia, the Islamic world, Abya Yala, and Oceania "
        "with Greek stokhastikē tékhnē and Bernoulli's Ars Conjectandi, Stochology formalizes "
        "judgment across World, Archive, and Institution.",
        max_chars=66,
    )
    para2_lines = _wrap_cover_lines(
        "Across an essayistic prologue, ninety-nine chapters in seven parts, thirty-five multilayer "
        "orders of knowledge, and seven appendices, it formalizes seven guard sites (T1–T7), "
        "three collapse propositions, and pre-registered falsifiers.",
        max_chars=66,
    )
    author_blurb_lines = _wrap_cover_lines(
        "This open monograph unifies mathematics, epistemology, statistical inference, "
        "thermodynamics, institutional cybernetics, and moral philosophy—warranting every figure "
        "by source precision or explicit derivation.",
        max_chars=46,
    )

    para1_svg = "\n".join(
        f'    <text class="cov-sans" x="140" y="{572 + idx * 54}" fill="#e5dfd2" font-family="{sans_stack}" '
        f'font-size="38" font-weight="400">{xml_escape_text(line)}</text>'
        for idx, line in enumerate(para1_lines)
    )
    para2_svg = "\n".join(
        f'    <text class="cov-sans" x="140" y="{816 + idx * 54}" fill="#cfc6b6" font-family="{sans_stack}" '
        f'font-size="38" font-weight="400">{xml_escape_text(line)}</text>'
        for idx, line in enumerate(para2_lines)
    )
    blurb_svg = "\n".join(
        f'  <text class="cov-serif" x="140" y="{2016 + idx * 52}" fill="#2b2520" font-family="{serif_stack}" '
        f'font-size="35" font-weight="400">{xml_escape_text(line)}</text>'
        for idx, line in enumerate(author_blurb_lines)
    )

    cards = [
        (
            140,
            "#5ca8ff",
            "01 · ARCHIVE",
            "Epistemic Margin",
            "Partial-identification bounds, missingness awareness, and additive design effects.",
        ),
        (
            760,
            "#36cf97",
            "02 · WORLD",
            "Vital Margin",
            "Physical buffers, convex loss protection, non-ergodicity, and recovery slack.",
        ),
        (
            1380,
            "#ef4b5b",
            "03 · INSTITUTION",
            "Governance",
            "Unrigged premises, pre-sealed criteria, algedonic channels, and basin integrity.",
        ),
    ]
    cards_svg: list[str] = []
    for cx, accent, tag, title, body in cards:
        body_lines = _wrap_cover_lines(body, max_chars=26)[:4]
        body_nodes = "\n".join(
            f'      <text class="cov-sans" x="{cx + 34}" y="{1286 + i * 44}" fill="#cfc6b6" '
            f'font-family="{sans_stack}" font-size="31" font-weight="400">{xml_escape_text(ln)}</text>'
            for i, ln in enumerate(body_lines)
        )
        cards_svg.append(
            f'    <g>\n'
            f'      <rect x="{cx}" y="1068" width="580" height="492" rx="4" fill="#191613" stroke="#2f2a24" stroke-width="1.5"/>\n'
            f'      <rect x="{cx}" y="1068" width="580" height="9" rx="2" fill="{accent}"/>\n'
            f'      <text class="cov-mono" x="{cx + 34}" y="1132" fill="{accent}" font-family="{mono_stack}" font-size="25" font-weight="500" letter-spacing="2.4">{xml_escape_text(tag)}</text>\n'
            f'      <text class="cov-sans" x="{cx + 34}" y="1192" fill="#f4efe4" font-family="{sans_stack}" font-size="38" font-weight="700">{xml_escape_text(title)}</text>\n'
            f'      <line x1="{cx + 34}" y1="1226" x2="{cx + 546}" y2="1226" stroke="#2c2721" stroke-width="1.2"/>\n'
            f'{body_nodes}\n'
            f'    </g>'
        )

    # Subtle decorative watermark arcs in upper-right obsidian field
    wm_cx, wm_cy = 1820, 440
    wm_arcs = "\n".join(
        f'      <circle cx="{wm_cx}" cy="{wm_cy}" r="{r}" fill="none" stroke="#f4efe4" stroke-opacity="0.065" stroke-width="1.4" stroke-dasharray="{6 + idx * 4} {8 + idx * 3}"/>'
        for idx, r in enumerate((190, 290, 400, 520))
    )

    return f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2100 3019" preserveAspectRatio="xMidYMid slice" role="img" aria-label="{xml_escape_attr(meta["title"])} back cover">
  <defs>
    <clipPath id="back-cov-chamber-clip">
      <rect x="0" y="0" width="2100" height="1756"/>
    </clipPath>
  </defs>

  <rect width="2100" height="3019" fill="#f4efe4"/>
  <rect x="0" y="0" width="2100" height="1756" fill="#12100e"/>

  <g clip-path="url(#back-cov-chamber-clip)">
    <g stroke="#1e1b17" stroke-width="1">
      <line x1="140" y1="0" x2="140" y2="1756"/>
      <line x1="747" y1="0" x2="747" y2="1756"/>
      <line x1="1353" y1="0" x2="1353" y2="1756"/>
      <line x1="1960" y1="0" x2="1960" y2="1756"/>
    </g>

    <g>
{wm_arcs}
      <circle cx="{wm_cx}" cy="{wm_cy}" r="14" fill="none" stroke="#f0ad38" stroke-opacity="0.30" stroke-width="2.2"/>
      <circle cx="{wm_cx}" cy="{wm_cy}" r="4" fill="#f0ad38" fill-opacity="0.45"/>
    </g>

    <text class="cov-mono" x="140" y="116" fill="#f4efe4" font-family="{mono_stack}" font-size="32" font-weight="500" letter-spacing="4.2">STOCHOLOGY</text>
    <text class="cov-mono" x="1960" y="116" text-anchor="end" fill="#b4aa9c" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="2.5">THE THEORY AND ETHICS OF AIM</text>
    <line x1="140" y1="164" x2="1960" y2="164" stroke="#2c2721" stroke-width="1.5"/>

    <text class="cov-mono" x="140" y="248" fill="#e08234" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="3.5">THE MEDIATION CIRCUIT</text>
    <text class="cov-serif" x="140" y="324" fill="#f4efe4" font-family="{serif_stack}" font-size="54" font-weight="400">Judgment under uncertainty fails whenever</text>
    <text class="cov-serif" x="140" y="396" fill="#f4efe4" font-family="{serif_stack}" font-size="54" font-weight="400">measurement, survival reserves, and</text>
    <text class="cov-serif" x="140" y="468" fill="#f4efe4" font-family="{serif_stack}" font-size="54" font-weight="400">institutional accountability are split apart.</text>
    <rect x="140" y="508" width="240" height="5" fill="#f0ad38"/>

{para1_svg}
{para2_svg}

{chr(10).join(cards_svg)}

    <line x1="140" y1="1624" x2="1960" y2="1624" stroke="#2c2721" stroke-width="1.5"/>
    <text class="cov-mono" x="140" y="1692" fill="#a89e90" font-family="{mono_stack}" font-size="26" font-weight="500" letter-spacing="1.2">M<tspan dy="5" font-size="19">Leontief</tspan><tspan dy="-5"> = min(S</tspan><tspan dy="5" font-size="19">epi</tspan><tspan dy="-5">, S</tspan><tspan dy="5" font-size="19">vital</tspan><tspan dy="-5">, S</tspan><tspan dy="5" font-size="19">gov</tspan><tspan dy="-5">) &nbsp;·&nbsp; T</tspan><tspan dy="5" font-size="19">1</tspan><tspan dy="-5">–T</tspan><tspan dy="5" font-size="19">7</tspan><tspan dy="-5"> &nbsp;·&nbsp; 35 ORDERS</tspan></text>
  </g>

  <!-- Mirrored three-plane horizon divider aligning across spine with front cover -->
  <rect x="0" y="1756" width="700" height="12" fill="#c82838"/>
  <rect x="700" y="1756" width="700" height="12" fill="#1b52c4"/>
  <rect x="1400" y="1756" width="700" height="12" fill="#0d7d57"/>

  <!-- Lower archival paper plinth -->
  <text class="cov-mono" x="140" y="1862" fill="#8a3a06" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="3.5">OPEN MONOGRAPH · PROVENANCE</text>
  <text class="cov-sans" x="140" y="1938" fill="#141210" font-family="{sans_stack}" font-size="52" font-weight="700" letter-spacing="-0.8">About the Treatise</text>
{blurb_svg}

  <line x1="140" y1="2292" x2="1210" y2="2292" stroke="#cfc6b6" stroke-width="1.5"/>
  <text class="cov-mono" x="140" y="2348" fill="#6c6154" font-family="{mono_stack}" font-size="25" font-weight="500" letter-spacing="2.8">PROGRAM SCOPE</text>
  <text class="cov-mono" x="140" y="2402" fill="#3d352c" font-family="{mono_stack}" font-size="25" font-weight="500">PARTIAL IDENTIFICATION · NON-ERGODICITY</text>
  <text class="cov-mono" x="140" y="2450" fill="#3d352c" font-family="{mono_stack}" font-size="25" font-weight="500">EPISTEMIC, VITAL &amp; GOVERNANCE MARGINS</text>
  <text class="cov-mono" x="140" y="2498" fill="#3d352c" font-family="{mono_stack}" font-size="25" font-weight="500">CAUSAL INVARIANCE · 35 KNOWLEDGE ORDERS</text>

  <!-- Right-hand Open Edition Specification Card -->
  <g>
    <rect x="1270" y="1840" width="690" height="736" rx="4" fill="#ede6d8" stroke="#cfc6b6" stroke-width="2"/>
    <rect x="1270" y="1840" width="690" height="9" rx="2" fill="#12100e"/>

    <circle cx="1615" cy="1988" r="86" fill="none" stroke="#b4aa9c" stroke-width="1.6" stroke-dasharray="6 5"/>
    <circle cx="1615" cy="1988" r="61" fill="none" stroke="#8c8275" stroke-width="1.4"/>
    <polygon points="1615,1924 1670,2020 1560,2020" fill="none" stroke="#141210" stroke-width="2.4" stroke-dasharray="6 5"/>
    <circle cx="1615" cy="1988" r="13" fill="none" stroke="#8a3a06" stroke-width="2.4"/>
    <line x1="1593" y1="1988" x2="1637" y2="1988" stroke="#8a3a06" stroke-width="2"/>
    <line x1="1615" y1="1966" x2="1615" y2="2010" stroke="#8a3a06" stroke-width="2"/>
    <circle cx="1615" cy="1924" r="10" fill="#f4efe4" stroke="#1b52c4" stroke-width="4.5"/>
    <circle cx="1560" cy="2020" r="10" fill="#f4efe4" stroke="#0d7d57" stroke-width="4.5"/>
    <circle cx="1670" cy="2020" r="10" fill="#f4efe4" stroke="#c82838" stroke-width="4.5"/>

    <line x1="1314" y1="2124" x2="1916" y2="2124" stroke="#cfc6b6" stroke-width="1.5"/>

    <text class="cov-mono" x="1314" y="2194" fill="#584e43" font-family="{mono_stack}" font-size="25" font-weight="500">SCOPE</text>
    <text class="cov-mono" x="1916" y="2194" text-anchor="end" fill="#141210" font-family="{mono_stack}" font-size="26" font-weight="500">∞ · Ø · 00–99 · A–G</text>

    <text class="cov-mono" x="1314" y="2262" fill="#584e43" font-family="{mono_stack}" font-size="25" font-weight="500">STRUCTURE</text>
    <text class="cov-mono" x="1916" y="2262" text-anchor="end" fill="#141210" font-family="{mono_stack}" font-size="26" font-weight="500">7 Strata · 35 Orders</text>

    <text class="cov-mono" x="1314" y="2330" fill="#584e43" font-family="{mono_stack}" font-size="25" font-weight="500">FORMAT</text>
    <text class="cov-mono" x="1916" y="2330" text-anchor="end" fill="#141210" font-family="{mono_stack}" font-size="26" font-weight="500">160 × 230 mm</text>

    <text class="cov-mono" x="1314" y="2398" fill="#584e43" font-family="{mono_stack}" font-size="25" font-weight="500">WARRANT</text>
    <text class="cov-mono" x="1916" y="2398" text-anchor="end" fill="#141210" font-family="{mono_stack}" font-size="26" font-weight="500">Source &amp; Derivation</text>

    <text class="cov-mono" x="1314" y="2466" fill="#584e43" font-family="{mono_stack}" font-size="25" font-weight="500">LICENSE</text>
    <text class="cov-mono" x="1916" y="2466" text-anchor="end" fill="#8a3a06" font-family="{mono_stack}" font-size="26" font-weight="500">{xml_escape_text(meta["rights"])}</text>
  </g>

  <line x1="140" y1="2660" x2="1960" y2="2660" stroke="#141210" stroke-width="2.5"/>
  <text class="cov-sans" x="140" y="2748" fill="#141210" font-family="{sans_stack}" font-size="48" font-weight="700" letter-spacing="4">{xml_escape_text(meta["author"].upper())}</text>
  <text class="cov-mono" x="140" y="2816" fill="#584e43" font-family="{mono_stack}" font-size="26" font-weight="500">Open Treatise · github.com/0b3st4f3r4/stochology</text>
  <text class="cov-mono" x="1960" y="2748" text-anchor="end" fill="#141210" font-family="{mono_stack}" font-size="34" font-weight="500">{xml_escape_text(url_display)}</text>
  <text class="cov-mono" x="1960" y="2816" text-anchor="end" fill="#584e43" font-family="{mono_stack}" font-size="28" font-weight="500">{xml_escape_text(meta["rights"])} · {xml_escape_text(meta["date"])}</text>
</svg>"""


PART_STYLES: dict[str, dict[str, str]] = {
    "front": {
        "roman": "00",
        "accent": "#8a3a06",
        "tint": "#f8f1e8",
        "dark": "#12100e",
        "bright": "#e08234",
        "short": "PROLOGUE",
        "range": "PROLOGUE · AIM AMONG MIRRORS",
    },
    "part-i": {
        "roman": "I",
        "accent": "#9a4108",
        "tint": "#f8f0e6",
        "dark": "#14100c",
        "bright": "#e58c3a",
        "short": "PART I · UNCERTAINTY",
        "range": "CHAPTERS 01–08",
    },
    "part-ii": {
        "roman": "II",
        "accent": "#1b4db8",
        "tint": "#edf2fb",
        "dark": "#0c1322",
        "bright": "#5ca8ff",
        "short": "PART II · THE BASES",
        "range": "CHAPTERS 09–26",
    },
    "part-iii": {
        "roman": "III",
        "accent": "#0b6b6b",
        "tint": "#ebf5f4",
        "dark": "#091617",
        "bright": "#3fd6cf",
        "short": "PART III · JUDGMENT",
        "range": "CHAPTERS 27–41",
    },
    "part-iv": {
        "roman": "IV",
        "accent": "#0a6e4c",
        "tint": "#ebf6f1",
        "dark": "#0a1511",
        "bright": "#36cf97",
        "short": "PART IV · THE FRAMEWORK",
        "range": "CHAPTERS 42–50",
    },
    "part-v": {
        "roman": "V",
        "accent": "#663399",
        "tint": "#f4eef9",
        "dark": "#140e1c",
        "bright": "#c594ff",
        "short": "PART V · MULTILAYER",
        "range": "CHAPTERS 51–85 · 35 ORDERS",
    },
    "part-vi": {
        "roman": "VI",
        "accent": "#9c5c04",
        "tint": "#faf3e4",
        "dark": "#16120b",
        "bright": "#f0ad38",
        "short": "PART VI · TRAJECTORY OF AIM",
        "range": "CHAPTERS 86–92 · T1–T7",
    },
    "part-vii": {
        "roman": "VII",
        "accent": "#ad1839",
        "tint": "#fbeff2",
        "dark": "#170c10",
        "bright": "#f2556b",
        "short": "PART VII · SUPERPOSED AIM",
        "range": "CHAPTERS 93–99 · INTERFERENCE",
    },
    "appendices": {
        "roman": "A–G",
        "accent": "#3b4654",
        "tint": "#f0f2f5",
        "dark": "#14171c",
        "bright": "#9aa8b8",
        "short": "APPENDICES",
        "range": "APPENDICES A–G",
    },
}

STRATUM_STYLES: dict[int, tuple[str, str, str, str]] = {
    1: ("I", "L01–L05", "#9a4108", "#f8f0e6"),
    2: ("II", "L06–L10", "#1b4db8", "#edf2fb"),
    3: ("III", "L11–L15", "#0a6e4c", "#ebf6f1"),
    4: ("IV", "L16–L20", "#0b6b6b", "#ebf5f4"),
    5: ("V", "L21–L25", "#663399", "#f4eef9"),
    6: ("VI", "L26–L30", "#9c5c04", "#faf3e4"),
    7: ("VII", "L31–L35", "#ad1839", "#fbeff2"),
}

ROMAN_TO_STRATUM = {"i": 1, "ii": 2, "iii": 3, "iv": 4, "v": 5, "vi": 6, "vii": 7}

SITE_STYLES: dict[str, tuple[str, str]] = {
    "T1": ("#1b52c4", "#edf2fb"),
    "T2": ("#1b52c4", "#edf2fb"),
    "T3": ("#1b52c4", "#edf2fb"),
    "T4": ("#0d7d57", "#ebf6f1"),
    "T5": ("#0d7d57", "#ebf6f1"),
    "T6": ("#c82838", "#fbeff2"),
    "T7": ("#9c5c04", "#faf3e4"),
}


@dataclass
class ChapterMeta:
    slug: str
    page_name: str
    part_key: str
    part_short: str
    part_accent: str
    part_tint: str
    accent: str
    tint: str
    kind: str
    folio_label: str
    top_right_text: str
    bottom_left_text: str
    pills: list[tuple[str, bool]] = field(default_factory=list)
    station_mode: str = "none"  # "none" | "laminar" | "superposed"
    active_sites: frozenset[str] = frozenset()


def _truncate_words(text: str, limit: int) -> str:
    clean = re.sub(r"\s+", " ", text).strip()
    if len(clean) <= limit:
        return clean
    cut = clean[: limit - 1].rsplit(" ", 1)[0].rstrip(" ,;:-·")
    return (cut if cut else clean[: limit - 1]) + "…"


def _css_str(text: str) -> str:
    clean = re.sub(r"\s+", " ", text).strip()
    return clean.replace("\\", "\\\\").replace('"', '\\"')


def _node_text(node: Node | str) -> str:
    if isinstance(node, str):
        return node
    return "".join(_node_text(ch) for ch in node.children)


def _add_class(node: Node, cls_name: str) -> None:
    for idx, (k, v) in enumerate(node.attrs):
        if k == "class":
            tokens = v.split()
            if cls_name not in tokens:
                node.attrs[idx] = ("class", f"{v} {cls_name}".strip())
            return
    node.attrs.append(("class", cls_name))


def _set_style_attr(node: Node, extra_style: str) -> None:
    for idx, (k, v) in enumerate(node.attrs):
        if k == "style":
            sep = ";" if v.strip() and not v.strip().endswith(";") else ""
            node.attrs[idx] = ("style", f"{v}{sep} {extra_style}".strip())
            return
    node.attrs.append(("style", extra_style))


def _find_first_tag(node: Node, tag: str) -> Node | None:
    if node.tag == tag:
        return node
    for ch in node.children:
        if isinstance(ch, Node):
            found = _find_first_tag(ch, tag)
            if found is not None:
                return found
    return None


def _extract_emblem_aria(root: Node) -> str:
    for ch in root.children:
        if isinstance(ch, Node) and ch.tag == "figure":
            cls = dict(ch.attrs).get("class", "")
            if "stoch-logo-figure" in cls.split():
                svg = _find_first_tag(ch, "svg")
                if svg is not None:
                    return dict(svg.attrs).get("aria-label", "")
    return ""


def _extract_opening_blockquote_text(root: Node) -> str:
    for ch in root.children:
        if isinstance(ch, Node):
            if ch.tag == "h2":
                break
            if ch.tag == "blockquote":
                return re.sub(r"\s+", " ", _node_text(ch)).strip()
    return ""


def _parse_emblem_keyword(aria: str) -> str:
    m = re.search(r"·\s*([^—]+?)\s*—", aria)
    if not m:
        return ""
    kw = re.sub(r"\s+", " ", m.group(1)).strip()
    if kw.startswith("CH"):
        return ""
    return kw


def _parse_active_sites(text: str) -> frozenset[str]:
    sites: set[str] = set()
    for m in re.finditer(r"T([1-7])\s*\.\.\s*T([1-7])", text):
        a, b = int(m.group(1)), int(m.group(2))
        for k in range(min(a, b), max(a, b) + 1):
            sites.add(f"T{k}")
    for m in re.finditer(r"\bT([1-7])\b", text):
        sites.add(f"T{m.group(1)}")
    return frozenset(sites)


def extract_chapter_meta(entry: TocEntry, root: Node) -> ChapterMeta:
    pstyle = PART_STYLES.get(entry.part_key, PART_STYLES["front"])
    part_accent = pstyle["accent"]
    part_tint = pstyle["tint"]
    part_short = pstyle["short"]
    accent, tint = part_accent, part_tint

    slug = entry.slug
    page_name = "p-" + re.sub(r"[^a-z0-9-]+", "-", slug.lower()).strip("-")
    aria = _extract_emblem_aria(root)
    bq_text = _extract_opening_blockquote_text(root)
    keyword = _parse_emblem_keyword(aria)

    pills: list[tuple[str, bool]] = []
    station_mode = "none"
    active_sites: frozenset[str] = frozenset()

    if slug == "00-preface":
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="prologue",
            folio_label="PROLOGUE · 00",
            top_right_text="00 · AIM AMONG MIRRORS",
            bottom_left_text="STÓKHOS · LOGOS · ARCHIVE · INSTITUTION",
            pills=[("PROLOGUE", False)],
        )

    if m := re.match(r"^part-([ivx]+)-introduction$", slug):
        roman = m.group(1).upper()
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="part-intro",
            folio_label=f"PART {roman} · INTRODUCTION",
            top_right_text=f"PART {roman} · INTRODUCTION",
            bottom_left_text=f"SCOPE & ARCHITECTURE · PART {roman}",
            pills=[("PART OPENING", False)],
        )

    if m := re.match(r"^part-([ivx]+)-conclusion$", slug):
        roman = m.group(1).upper()
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="part-conclusion",
            folio_label=f"PART {roman} · CONCLUSION",
            top_right_text=f"PART {roman} · CONCLUSION",
            bottom_left_text=f"SYNTHESIS & HANDOFF · PART {roman}",
            pills=[("PART SYNTHESIS", False)],
        )

    if m := re.match(r"^stratum-([ivx]+)-opening$", slug):
        s_idx = ROMAN_TO_STRATUM.get(m.group(1), 1)
        s_roman, orders_range, accent, tint = STRATUM_STYLES[s_idx]
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="stratum-opening",
            folio_label=f"STRATUM {s_roman} · OPENING",
            top_right_text=_truncate_words(entry.title, 34),
            bottom_left_text=f"STRATUM {s_roman} · ORDERS {orders_range}",
            pills=[(f"STRATUM {s_roman}", False), (orders_range, True)],
        )

    if m := re.match(r"^appendix-([a-g])-", slug):
        letter = m.group(1).upper()
        short_name = re.sub(r"^Appendix\s+[A-G]\s*[—-]\s*", "", entry.title)
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="appendix",
            folio_label=f"APPENDIX {letter}",
            top_right_text=_truncate_words(f"APP {letter} · {short_name}", 34),
            bottom_left_text=f"REFERENCE APPARATUS · APPENDIX {letter}",
            pills=[(f"APPENDIX {letter}", False)],
        )

    m_num = re.match(r"^(\d{2})-", slug)
    num = int(m_num.group(1)) if m_num else 0

    # Part V layers (51-85)
    if 51 <= num <= 85:
        order_num = num - 50
        s_idx = (order_num - 1) // 5 + 1
        s_roman, _, accent, tint = STRATUM_STYLES[s_idx]
        short_name = re.sub(r"^Layer\s+\d+:\s*", "", entry.title)

        short_guards: list[str] = []
        if gm := re.search(r"\{([^}]+)\}", bq_text):
            for tok in gm.group(1).split(","):
                if tm := re.search(r"T([1-7])", tok):
                    short_guards.append(f"T{tm.group(1)}")
        elif gm := re.search(r"\{(T[^}]+)\}", aria):
            for tok in gm.group(1).split(","):
                if tm := re.search(r"T([1-7])", tok):
                    short_guards.append(f"T{tm.group(1)}")

        sig = ""
        if sm := re.search(r"Signature\s*\((\d+\s*,\s*\d+\s*,\s*\d+)\)", bq_text):
            sig = re.sub(r"\s+", "", sm.group(1))

        triple_str = "{" + ", ".join(short_guards) + "}" if short_guards else ""
        pills = [(f"STRATUM {s_roman}", False)]
        if triple_str:
            pills.append((triple_str, False))
        if sig:
            pills.append((f"SIG ({sig})", True))

        bottom_parts = [f"STRATUM {s_roman}"]
        if short_guards:
            bottom_parts.append("{" + ",".join(short_guards) + "}")
        if sig:
            bottom_parts.append(f"SIG ({sig})")

        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="layer",
            folio_label=f"ORDER L{order_num:02d} · CHAPTER {num:02d}",
            top_right_text=_truncate_words(f"L{order_num:02d} · CH {num:02d} · {short_name}", 34),
            bottom_left_text=_truncate_words(" · ".join(bottom_parts), 44),
            pills=pills,
        )

    # Part VI laminar stations (86-92)
    if 86 <= num <= 92:
        site_num = num - 85
        site_tok = f"T{site_num}"
        accent, tint = SITE_STYLES[site_tok]
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=accent,
            tint=tint,
            kind="station-laminar",
            folio_label=f"CHAPTER {num:02d} · STATION {site_tok}",
            top_right_text=_truncate_words(f"CH {num:02d} · {site_tok} · {entry.title}", 34),
            bottom_left_text=f"LAMINAR TRAJECTORY · GUARD SITE {site_tok}",
            pills=[("LAMINAR", False)],
            station_mode="laminar",
            active_sites=frozenset({site_tok}),
        )

    # Part VII superposed aim (93-99)
    if 93 <= num <= 99:
        active_sites = _parse_active_sites(aria) or _parse_active_sites(bq_text)
        if not active_sites:
            active_sites = frozenset({f"T{k}" for k in range(1, 8)})
        super_expr = keyword or "⊕".join(sorted(active_sites))
        return ChapterMeta(
            slug=slug,
            page_name=page_name,
            part_key=entry.part_key,
            part_short=part_short,
            part_accent=part_accent,
            part_tint=part_tint,
            accent=part_accent,
            tint=part_tint,
            kind="station-superposed",
            folio_label=f"CHAPTER {num:02d} · SUPERPOSITION",
            top_right_text=_truncate_words(f"CH {num:02d} · {entry.title}", 34),
            bottom_left_text=_truncate_words(f"SUPERPOSED AIM · {super_expr}", 44),
            pills=[("SUPERPOSED", False)],
            station_mode="superposed",
            active_sites=active_sites,
        )

    # Parts I-IV chapters (01-50)
    bottom_left = f"CH {num:02d} · {keyword}" if keyword else part_short
    if entry.part_key == "part-ii":
        grp = (entry.group_title or "").lower()
        if "math" in grp:
            accent, tint = "#1b4db8", "#edf2fb"
            track_lbl = "TRACK I · MATHEMATICS"
        elif "philosoph" in grp:
            accent, tint = "#663399", "#f4eef9"
            track_lbl = "TRACK II · PHILOSOPHY"
        else:
            accent, tint = "#0d7d57", "#ebf6f1"
            track_lbl = "TRACK III · NATURAL SCIENCE"
        pills.append((track_lbl, False))
        if keyword:
            pills.append((keyword, True))
        bottom_left = f"{track_lbl} · {keyword}" if keyword else track_lbl
    elif entry.part_key == "part-iii":
        kw_up = keyword.upper()
        if "EPI" in kw_up:
            accent, tint = "#1b52c4", "#edf2fb"
        elif "VITAL" in kw_up:
            accent, tint = "#0d7d57", "#ebf6f1"
        elif "GOV" in kw_up or "INST" in kw_up:
            accent, tint = "#c82838", "#fbeff2"
        pills.append(("FOUNDATIONS", False))
        if keyword:
            pills.append((keyword, True))
        bottom_left = f"FOUNDATIONS OF JUDGMENT · {keyword}" if keyword else "FOUNDATIONS OF JUDGMENT"
    elif entry.part_key == "part-iv":
        grp = (entry.group_title or "").lower()
        if "filter i " in grp or "s_epi" in grp:
            accent, tint = "#1b52c4", "#edf2fb"
            flt = "FILTER I · S_EPI"
        elif "filter ii" in grp or "s_vital" in grp:
            accent, tint = "#0d7d57", "#ebf6f1"
            flt = "FILTER II · S_VITAL"
        elif "filter iii" in grp or "s_gov" in grp:
            accent, tint = "#c82838", "#fbeff2"
            flt = "FILTER III · S_GOV"
        elif "synthesis" in grp:
            accent, tint = "#9c5c04", "#faf3e4"
            flt = "SYNTHESIS & REFLEXIVITY"
        else:
            accent, tint = "#8a3a06", "#f8f1e8"
            flt = "SEQUENTIAL LOOP"
        pills.append((flt, False))
        if keyword and keyword.upper() not in flt:
            pills.append((keyword, True))
        bottom_left = f"{flt} · {keyword}" if keyword and keyword.upper() not in flt else flt
    else:
        pills.append(("PART I", False))
        if keyword:
            pills.append((keyword, True))

    return ChapterMeta(
        slug=slug,
        page_name=page_name,
        part_key=entry.part_key,
        part_short=part_short,
        part_accent=part_accent,
        part_tint=part_tint,
        accent=accent,
        tint=tint,
        kind="chapter",
        folio_label=f"CHAPTER {num:02d}",
        top_right_text=_truncate_words(f"CH {num:02d} · {entry.title}", 34),
        bottom_left_text=_truncate_words(bottom_left, 44),
        pills=pills,
    )


def _build_chapter_header_node(meta: ChapterMeta) -> Node:
    left = Node(
        "div",
        [("class", "pdf-ch-head-left")],
        [Node("span", [("class", "pdf-ch-folio")], [meta.folio_label])],
    )
    right_children: list[Node | str] = []
    for text, is_muted in meta.pills:
        cls = "pdf-ch-pill pdf-ch-pill--muted" if is_muted else "pdf-ch-pill"
        right_children.append(Node("span", [("class", cls)], [text]))

    if meta.station_mode != "none":
        active_cls = "is-super" if meta.station_mode == "superposed" else "is-active"
        nodes: list[Node | str] = []
        for idx in range(1, 8):
            site = f"T{idx}"
            cls = f"pdf-station-node {active_cls}" if site in meta.active_sites else "pdf-station-node"
            nodes.append(Node("span", [("class", cls)], [site]))
        right_children.append(Node("div", [("class", "pdf-station-bar"), ("aria-hidden", "true")], nodes))

    right = Node("div", [("class", "pdf-ch-head-right")], right_children)
    return Node("header", [("class", "pdf-ch-head")], [left, right])


_CELL_MATH_SPLIT_OPS_AFTER = frozenset({",", ";"})
_CELL_MATH_SPLIT_OPS_BEFORE = frozenset(
    {"=", "\u2264", "\u2265", "\u2260", "\u2248", "\u223c", "\u2192", "\u2194", "\u2295", "\u2297", "\u2282", "\u228a", "\u2286"}
)
_CELL_MATH_OPEN_FENCES = frozenset({"(", "[", "{", "\u27e8"})
_CELL_MATH_CLOSE_FENCES = frozenset({")", "]", "}", "\u27e9"})


def _count_table_cols(table: Node) -> int:
    first_tr = _find_first_tag(table, "tr")
    if first_tr is None:
        return 0
    return sum(1 for ch in first_tr.children if isinstance(ch, Node) and ch.tag in ("th", "td"))


def _iter_tag_nodes(node: Node, tag: str) -> list[Node]:
    found: list[Node] = []
    for ch in node.children:
        if isinstance(ch, Node):
            if ch.tag == tag:
                found.append(ch)
            found.extend(_iter_tag_nodes(ch, tag))
    return found


def _inject_wbr_in_str(
    text: str, *, split_camel: bool = False, trailing_node: bool = False
) -> list[Node | str]:
    """Insert <wbr> break opportunities after '/', long '_', em-dash, and long CamelCase identifiers."""
    out: list[Node | str] = []
    parts = re.split(r"(\s+)", text)
    for part_idx, tok in enumerate(parts):
        if not tok:
            continue
        if tok.isspace():
            out.append(tok)
            continue
        is_last_tok = part_idx == len(parts) - 1
        # Split after '/', '—', long '_', or long internal CamelCase transitions
        segs: list[str] = []
        buf: list[str] = []
        n = len(tok)
        for idx, ch in enumerate(tok):
            buf.append(ch)
            nxt = tok[idx + 1] if idx + 1 < n else ""
            insert_after = False
            if ch in ("/", "\u2014"):
                if (nxt and nxt not in (",", ".", ";", ":", ")", "]", "}")) or (
                    not nxt and is_last_tok and trailing_node
                ):
                    insert_after = True
            elif ch == "_" and n >= 12 and nxt:
                insert_after = True
            elif (
                split_camel
                and n >= 13
                and (ch.islower() or ch.isdigit())
                and nxt.isupper()
            ):
                insert_after = True
            if insert_after:
                segs.append("".join(buf))
                buf.clear()
        if buf:
            segs.append("".join(buf))
        for s_idx, seg in enumerate(segs):
            if s_idx > 0:
                out.append(Node("wbr"))
            out.append(seg)
    return out


def _split_wide_cell_math(math_node: Node) -> list[Node | str]:
    """Split long inline MathML inside narrow table cells at top-level relational/separator ops."""
    if len(_node_text(math_node)) < 9:
        return [math_node]
    children = [ch for ch in math_node.children if not (isinstance(ch, str) and not ch.strip())]
    if len(children) == 1 and isinstance(children[0], Node) and children[0].tag == "mrow":
        children = [ch for ch in children[0].children if not (isinstance(ch, str) and not ch.strip())]
    if len(children) < 3:
        return [math_node]

    def _seg_text_len(nodes: list[Node | str]) -> int:
        return sum(len(_node_text(x)) for x in nodes)

    segments: list[list[Node | str]] = []
    cur: list[Node | str] = []
    depth = 0
    for ch in children:
        txt = _node_text(ch).strip() if isinstance(ch, Node) else str(ch).strip()
        is_mo = isinstance(ch, Node) and ch.tag == "mo"
        if is_mo and txt in _CELL_MATH_CLOSE_FENCES:
            depth = max(0, depth - 1)
        elif not is_mo:
            # Handle closing fences inside scripted wrappers such as ]_A or )^k before checking op splits.
            closes = sum(1 for c in txt if c in _CELL_MATH_CLOSE_FENCES)
            opens = sum(1 for c in txt if c in _CELL_MATH_OPEN_FENCES)
            if closes > opens:
                depth = max(0, depth - (closes - opens))
        if depth == 0 and is_mo and txt in _CELL_MATH_SPLIT_OPS_BEFORE and cur and _seg_text_len(cur) >= 2:
            segments.append(cur)
            cur = [ch]
            continue
        cur.append(ch)
        if is_mo and txt in _CELL_MATH_OPEN_FENCES:
            depth += 1
        elif not is_mo:
            opens = sum(1 for c in txt if c in _CELL_MATH_OPEN_FENCES)
            closes = sum(1 for c in txt if c in _CELL_MATH_CLOSE_FENCES)
            if opens > closes:
                depth += opens - closes
        elif depth == 0 and is_mo and txt in _CELL_MATH_SPLIT_OPS_AFTER and _seg_text_len(cur) >= 3:
            segments.append(cur)
            cur = []
    if cur:
        if segments and _seg_text_len(cur) < 3:
            segments[-1].extend(cur)
        else:
            segments.append(cur)

    if len(segments) <= 1 or any(_seg_text_len(seg) < 2 for seg in segments):
        return [math_node]

    attrs = list(math_node.attrs)
    out: list[Node | str] = []
    for s_idx, seg in enumerate(segments):
        if s_idx > 0:
            out.append(" ")
        out.append(Node("math", list(attrs), seg))
    return out


def _prepare_cell_wrapping(node: Node, *, in_code: bool = False) -> None:
    """Recursively insert soft break points inside table <td> content without touching SVG or headers."""
    new_children: list[Node | str] = []
    children = node.children
    for idx, ch in enumerate(children):
        if isinstance(ch, str):
            has_next_node = any(isinstance(nxt, Node) for nxt in children[idx + 1 : idx + 2])
            new_children.extend(
                _inject_wbr_in_str(ch, split_camel=in_code, trailing_node=has_next_node)
            )
        elif ch.tag == "math":
            new_children.extend(_split_wide_cell_math(ch))
        elif ch.tag in ("svg", "wbr"):
            new_children.append(ch)
        else:
            _prepare_cell_wrapping(ch, in_code=(in_code or ch.tag == "code"))
            new_children.append(ch)
    node.children = new_children


def _apply_fixed_colgroup(table: Node, ncols: int) -> None:
    """Apply proportional <colgroup> + fixed table layout to dense 5-6 col prose tables."""
    trs = _iter_tag_nodes(table, "tr")
    hdr_max_word = [1.0] * ncols
    body_lens = [[] for _ in range(ncols)]
    for tr in trs:
        cells = [ch for ch in tr.children if isinstance(ch, Node) and ch.tag in ("th", "td")]
        if len(cells) != ncols:
            continue
        is_hdr = all(c.tag == "th" for c in cells)
        for ci, cell in enumerate(cells):
            txt = re.sub(r"\s+", " ", _node_text(cell)).strip()
            if is_hdr:
                words = [w for w in txt.split(" ") if w]
                if words:
                    hdr_max_word[ci] = max(hdr_max_word[ci], float(max(len(w) for w in words)))
            else:
                body_lens[ci].append(len(txt))

    if not any(body_lens):
        return
    avg_len = [sum(col) / len(col) if col else 10.0 for col in body_lens]
    if sum(avg_len) < 140.0:
        return

    raw_w = [
        min(
            27.0,
            max(
                18.5
                if (c == 0 and avg_len[c] >= 12.0)
                else (17.5 if (c == 1 and hdr_max_word[c] >= 9.0) else (12.0 if c >= 2 else 8.5)),
                hdr_max_word[c] * 1.15,
                avg_len[c] ** 0.58,
            ),
        )
        for c in range(ncols)
    ]
    total = sum(raw_w) or 100.0
    widths = [w * 100.0 / total for w in raw_w]
    colgroup = Node(
        "colgroup",
        [],
        [Node("col", [("style", f"width: {w:.2f}%;")]) for w in widths],
    )
    table.children = [ch for ch in table.children if not (isinstance(ch, Node) and ch.tag == "colgroup")]
    table.children.insert(0, colgroup)
    _add_class(table, "pdf-table-fixed")


def _tag_tables(node: Node) -> None:
    """Classify 5-6 col and 7+ col tables so dense ledgers fit cleanly on 123 mm measure."""
    for ch in node.children:
        if isinstance(ch, Node):
            if ch.tag == "table":
                ncols = _count_table_cols(ch)
                if ncols >= 7:
                    _add_class(ch, "pdf-table-dense")
                elif ncols >= 5:
                    _add_class(ch, "pdf-table-wide")
                    if ncols <= 6:
                        _apply_fixed_colgroup(ch, ncols)
                for td in _iter_tag_nodes(ch, "td"):
                    _prepare_cell_wrapping(td)
            else:
                _tag_tables(ch)


def decorate_chapter_article(root: Node, meta: ChapterMeta) -> None:
    """Attach per-chapter theme variables and wrap common structural blocks."""
    root.attrs.append(("data-part", meta.part_key))
    root.attrs.append(("data-kind", meta.kind))
    _set_style_attr(
        root,
        f"page: {meta.page_name}; --part-accent: {meta.part_accent}; --part-tint: {meta.part_tint}; "
        f"--ch-accent: {meta.accent}; --ch-tint: {meta.tint};",
    )

    # 1. Tag opening blockquote before first <h2> as .pdf-chapter-note
    seen_h2 = False
    for ch in root.children:
        if isinstance(ch, Node):
            if ch.tag == "h2":
                seen_h2 = True
                break
            if ch.tag == "blockquote" and not seen_h2:
                _add_class(ch, "pdf-chapter-note")

    # 2. Tag italic epigraph lead on part-intro, part-conclusion, stratum-opening
    if meta.kind in ("part-intro", "part-conclusion", "stratum-opening"):
        seen_h1 = False
        for ch in root.children:
            if isinstance(ch, Node):
                if ch.tag == "h1":
                    seen_h1 = True
                    continue
                if seen_h1:
                    if ch.tag in ("h2", "hr"):
                        break
                    if ch.tag == "p":
                        meaningful = [c for c in ch.children if not (isinstance(c, str) and not c.strip())]
                        if len(meaningful) == 1 and isinstance(meaningful[0], Node) and meaningful[0].tag in ("em", "i"):
                            _add_class(ch, "pdf-lead")
                        break

    # 3. Wrap "By the end of this chapter you can" + <ol> into <aside class="pdf-outcomes">
    i = 0
    while i < len(root.children):
        ch = root.children[i]
        if isinstance(ch, Node) and ch.tag == "h2":
            heading_txt = re.sub(r"\s+", " ", _node_text(ch)).strip().lower()
            if heading_txt == "by the end of this chapter you can":
                j = i + 1
                while j < len(root.children) and isinstance(root.children[j], str) and not root.children[j].strip():
                    j += 1
                if j < len(root.children) and isinstance(root.children[j], Node) and root.children[j].tag == "ol":
                    aside = Node("aside", [("class", "pdf-outcomes")], [ch, root.children[j]])
                    root.children[i : j + 1] = [aside]
                    break
        i += 1

    # 4. Wrap closing <h2>Bridge</h2> + trailing siblings into <footer class="pdf-bridge">
    bridge_idx = None
    for idx, ch in enumerate(root.children):
        if isinstance(ch, Node) and ch.tag == "h2":
            if re.sub(r"\s+", " ", _node_text(ch)).strip().lower() == "bridge":
                bridge_idx = idx
    if bridge_idx is not None:
        bridge_items = [c for c in root.children[bridge_idx:] if not (isinstance(c, str) and not c.strip())]
        footer = Node("footer", [("class", "pdf-bridge")], bridge_items)
        root.children[bridge_idx:] = [footer]

    # 5. Insert per-chapter header strip right before first <h1>
    h1_idx = 0
    for idx, ch in enumerate(root.children):
        if isinstance(ch, Node) and ch.tag == "h1":
            h1_idx = idx
            break
    root.children.insert(h1_idx, _build_chapter_header_node(meta))

    # 6. Classify multi-column tables for responsive print typography
    _tag_tables(root)


def build_dynamic_page_css(chapter_metas: list[ChapterMeta]) -> str:
    """Emit mirrored recto/verso @page rules carrying per-part and per-chapter headers/footers."""
    rules: list[str] = []
    mono_footer = (
        'font-family: "Source Code Pro", "Liberation Mono", ui-monospace, monospace; '
        "font-size: 6.0pt; font-weight: 500; letter-spacing: 0.06em; text-transform: uppercase; color: #7a7063;"
    )
    folio_footer = (
        'font-family: "Open Sans", "Inter", "Liberation Sans", sans-serif; '
        "font-size: 7.5pt; font-weight: 700; color: #161310;"
    )
    for m in chapter_metas:
        part_s = _css_str(m.part_short)
        top_r = _css_str(m.top_right_text)
        bot_l = _css_str(m.bottom_left_text)
        rules.append(
            f"@page {m.page_name}:right {{\n"
            f'  @top-left {{ content: "{part_s}"; color: {m.part_accent}; }}\n'
            f'  @top-right {{ content: "{top_r}"; color: #3d352c; }}\n'
            f'  @bottom-left {{ content: "{bot_l}"; {mono_footer} }}\n'
            f'  @bottom-right {{ content: counter(page); {folio_footer} }}\n'
            "}\n"
            f"@page {m.page_name}:left {{\n"
            f'  @top-left {{ content: "{top_r}"; color: #3d352c; }}\n'
            f'  @top-right {{ content: "{part_s}"; color: {m.part_accent}; }}\n'
            f'  @bottom-left {{ content: counter(page); {folio_footer} }}\n'
            f'  @bottom-right {{ content: "{bot_l}"; {mono_footer} }}\n'
            "}"
        )
    return "\n".join(rules)


def _build_part_motif_svg(part_key: str, bright: str, accent: str) -> str:
    """Generate a bespoke architectural vector motif centered around (1400, 860)."""
    cx, cy = 1400.0, 860.0
    elems: list[str] = [
        f'<circle cx="{cx:.0f}" cy="{cy:.0f}" r="430" fill="none" stroke="#ffffff" stroke-opacity="0.10" stroke-width="1.5"/>',
        f'<circle cx="{cx:.0f}" cy="{cy:.0f}" r="430" fill="none" stroke="{bright}" stroke-opacity="0.22" stroke-width="1.5" stroke-dasharray="4 14"/>',
    ]

    if part_key == "part-i":
        # Crosswind ballistic arc sheaf aiming at a reticle
        tx, ty = 1580.0, 740.0
        ox, oy = 860.0, 1210.0
        for k in range(-4, 5):
            if k == 0:
                continue
            s = k / 4.0
            op = 0.14 + 0.14 * (1.0 - abs(s))
            elems.append(
                f'<path d="M {ox:.0f} {oy:.0f} C {1080 + 20*s:.0f} {820 + 60*s:.0f}, '
                f'{1340 + 25*s:.0f} {660 + 85*s:.0f}, {tx + 35*s:.0f} {ty + 110*s:.0f}" '
                f'fill="none" stroke="#f4efe4" stroke-opacity="{op:.2f}" stroke-width="1.6"/>'
            )
        elems.append(
            f'<path d="M {ox:.0f} {oy:.0f} C 1080 820, 1340 660, {tx:.0f} {ty:.0f}" '
            f'fill="none" stroke="{bright}" stroke-width="4.5" stroke-linecap="round"/>'
        )
        elems.append(f'<circle cx="{ox:.0f}" cy="{oy:.0f}" r="9" fill="#14100c" stroke="#f4efe4" stroke-width="2.5"/>')
        elems.append(f'<circle cx="{tx:.0f}" cy="{ty:.0f}" r="36" fill="none" stroke="{bright}" stroke-opacity="0.55" stroke-width="1.5"/>')
        elems.append(f'<circle cx="{tx:.0f}" cy="{ty:.0f}" r="18" fill="#14100c" stroke="{bright}" stroke-width="3.6"/>')
        elems.append(f'<circle cx="{tx:.0f}" cy="{ty:.0f}" r="4.5" fill="#f4efe4"/>')

    elif part_key == "part-ii":
        # Three orthogonal disciplinary strata planes (Math, Philosophy, Natural Science)
        planes = [(-175, "#5ca8ff"), (0, "#c594ff"), (175, "#36cf97")]
        for dy, col in planes:
            y = cy + dy
            pts = f"{cx - 260:.0f},{y:.0f} {cx:.0f},{y - 95:.0f} {cx + 260:.0f},{y:.0f} {cx:.0f},{y + 95:.0f}"
            elems.append(f'<polygon points="{pts}" fill="{col}" fill-opacity="0.08" stroke="{col}" stroke-width="2.6" stroke-linejoin="round"/>')
        for px, py_off in [(cx - 130, -47), (cx, 0), (cx + 130, 47)]:
            elems.append(
                f'<line x1="{px:.0f}" y1="{cy - 175 + py_off:.0f}" x2="{px:.0f}" y2="{cy + 175 + py_off:.0f}" '
                f'stroke="#f4efe4" stroke-opacity="0.32" stroke-width="1.5" stroke-dasharray="5 7"/>'
            )
            for dy, col in planes:
                elems.append(f'<circle cx="{px:.0f}" cy="{cy + dy + py_off:.0f}" r="6" fill="#0c1322" stroke="{col}" stroke-width="2.5"/>')

    elif part_key == "part-iii":
        # Three-plane convergence triangle + nested Leontief minimum contours
        for r_scale, op in [(340, 0.82), (245, 0.52), (150, 0.34)]:
            pts = " ".join(
                f"{cx + r_scale * math.cos(math.radians(a)):.1f},{cy + r_scale * math.sin(math.radians(a)):.1f}"
                for a in (-90, 30, 150)
            )
            dash = ' stroke-dasharray="6 7"' if r_scale < 340 else ""
            elems.append(
                f'<polygon points="{pts}" fill="none" stroke="{bright}" stroke-opacity="{op:.2f}" '
                f'stroke-width="2.6"{dash} stroke-linejoin="round"/>'
            )
        for a, col in [(-90, "#5ca8ff"), (150, "#36cf97"), (30, "#ef4b5b")]:
            vx = cx + 340 * math.cos(math.radians(a))
            vy = cy + 340 * math.sin(math.radians(a))
            elems.append(f'<line x1="{cx:.0f}" y1="{cy:.0f}" x2="{vx:.1f}" y2="{vy:.1f}" stroke="#f4efe4" stroke-opacity="0.22" stroke-width="1.5"/>')
            elems.append(f'<circle cx="{vx:.1f}" cy="{vy:.1f}" r="18" fill="#091617" stroke="{col}" stroke-width="4"/>')
            elems.append(f'<circle cx="{vx:.1f}" cy="{vy:.1f}" r="6" fill="{col}"/>')
        elems.append(f'<circle cx="{cx:.0f}" cy="{cy:.0f}" r="11" fill="#091617" stroke="#f0ad38" stroke-width="3"/>')

    elif part_key == "part-iv":
        # Cyclic sequential mediation loop with 3 filter gates
        r_loop = 310.0
        for d0, d1, col in [(166, 254, "#36cf97"), (-74, 14, "#5ca8ff"), (46, 134, "#ef4b5b")]:
            r0, r1 = math.radians(d0), math.radians(d1)
            x0, y0 = cx + r_loop * math.cos(r0), cy + r_loop * math.sin(r0)
            x1, y1 = cx + r_loop * math.cos(r1), cy + r_loop * math.sin(r1)
            elems.append(f'<path d="M {x0:.1f} {y0:.1f} A {r_loop:.0f} {r_loop:.0f} 0 0 1 {x1:.1f} {y1:.1f}" fill="none" stroke="{col}" stroke-width="6"/>')
            mid = math.radians(0.5 * (d0 + d1))
            gx, gy = cx + r_loop * math.cos(mid), cy + r_loop * math.sin(mid)
            elems.append(f'<rect x="{gx - 18:.1f}" y="{gy - 18:.1f}" width="36" height="36" fill="#0a1511" stroke="{col}" stroke-width="3"/>')
        for a, col in [(-90, "#5ca8ff"), (150, "#36cf97"), (30, "#ef4b5b")]:
            vx, vy = cx + r_loop * math.cos(math.radians(a)), cy + r_loop * math.sin(math.radians(a))
            elems.append(f'<circle cx="{vx:.1f}" cy="{vy:.1f}" r="16" fill="#0a1511" stroke="{col}" stroke-width="4"/>')

    elif part_key == "part-v":
        # 7 x 5 = 35 multilayer order lattice
        stratum_cols = ["#e58c3a", "#5ca8ff", "#36cf97", "#3fd6cf", "#c594ff", "#f0ad38", "#f2556b"]
        x0, y0 = cx - 220.0, cy - 270.0
        dx, dy = 110.0, 90.0
        for r in range(7):
            col = stratum_cols[r]
            ry = y0 + r * dy
            elems.append(f'<line x1="{x0 - 28:.0f}" y1="{ry:.0f}" x2="{x0 + 4*dx + 28:.0f}" y2="{ry:.0f}" stroke="{col}" stroke-opacity="0.28" stroke-width="1.5"/>')
            for c in range(5):
                px = x0 + c * dx
                elems.append(f'<rect x="{px - 15:.1f}" y="{ry - 15:.1f}" width="30" height="30" fill="#140e1c" stroke="{col}" stroke-width="2.6"/>')
                elems.append(f'<circle cx="{px:.1f}" cy="{ry:.1f}" r="4.5" fill="{col}"/>')

    elif part_key == "part-vi":
        # Single laminar trajectory passing 7 sequentially spaced guard stations T1..T7
        x_start, x_end = cx - 360.0, cx + 360.0
        elems.append(f'<line x1="{x_start:.0f}" y1="{cy:.0f}" x2="{x_end:.0f}" y2="{cy:.0f}" stroke="#ffffff" stroke-opacity="0.16" stroke-width="1.5" stroke-dasharray="5 6"/>')
        pts: list[str] = []
        for k in range(7):
            u = k / 6.0
            sx = x_start + (x_end - x_start) * u
            sy = cy - 95.0 * math.sin(u * math.pi)
            pts.append(f"{sx:.1f},{sy:.1f}")
        elems.append(f'<polyline points="{" ".join(pts)}" fill="none" stroke="{bright}" stroke-width="4" stroke-linejoin="round"/>')
        for k in range(7):
            u = k / 6.0
            sx = x_start + (x_end - x_start) * u
            sy = cy - 95.0 * math.sin(u * math.pi)
            col = "#5ca8ff" if k < 3 else ("#36cf97" if k < 5 else ("#ef4b5b" if k == 5 else bright))
            elems.append(f'<line x1="{sx:.1f}" y1="{cy + 40:.0f}" x2="{sx:.1f}" y2="{sy:.1f}" stroke="{col}" stroke-opacity="0.45" stroke-width="1.5"/>')
            elems.append(f'<circle cx="{sx:.1f}" cy="{sy:.1f}" r="18" fill="#16120b" stroke="{col}" stroke-width="3.6"/>')
            elems.append(f'<circle cx="{sx:.1f}" cy="{sy:.1f}" r="5.5" fill="#f4efe4"/>')

    elif part_key == "part-vii":
        # Superposed 7-site K7 chord interference ring
        r_ring = 320.0
        nodes: list[tuple[float, float]] = []
        for k in range(7):
            a = math.radians(-90.0 + k * (360.0 / 7.0))
            nodes.append((cx + r_ring * math.cos(a), cy + r_ring * math.sin(a)))
        for i in range(7):
            for j in range(i + 1, 7):
                x1, y1 = nodes[i]
                x2, y2 = nodes[j]
                is_hi = (i, j) in {(0, 1), (1, 5), (5, 6), (0, 6)}
                col = bright if is_hi else "#f4efe4"
                op = "0.85" if is_hi else "0.18"
                sw = "3.2" if is_hi else "1.3"
                elems.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{col}" stroke-opacity="{op}" stroke-width="{sw}"/>')
        for nx, ny in nodes:
            elems.append(f'<circle cx="{nx:.1f}" cy="{ny:.1f}" r="16" fill="#170c10" stroke="{bright}" stroke-width="3.4"/>')
            elems.append(f'<circle cx="{nx:.1f}" cy="{ny:.1f}" r="4.5" fill="#f0ad38"/>')
        elems.append(f'<circle cx="{cx:.0f}" cy="{cy:.0f}" r="24" fill="#170c10" stroke="#f0ad38" stroke-width="3"/>')
        elems.append(f'<circle cx="{cx:.0f}" cy="{cy:.0f}" r="6" fill="#f4efe4"/>')

    else:
        # Appendices: 7 calibrated vertical apparatus columns A-G
        x0 = cx - 270.0
        for idx in range(7):
            col_x = x0 + idx * 90.0
            h_top = cy - 250.0 + (idx % 3) * 35.0
            h_bot = cy + 250.0 - ((6 - idx) % 2) * 30.0
            elems.append(f'<line x1="{col_x:.0f}" y1="{cy - 280:.0f}" x2="{col_x:.0f}" y2="{cy + 280:.0f}" stroke="#ffffff" stroke-opacity="0.14" stroke-width="1.5"/>')
            elems.append(f'<line x1="{col_x:.0f}" y1="{h_top:.0f}" x2="{col_x:.0f}" y2="{h_bot:.0f}" stroke="{bright}" stroke-width="5" stroke-linecap="butt"/>')
            elems.append(f'<circle cx="{col_x:.0f}" cy="{h_top:.0f}" r="8" fill="#14171c" stroke="{bright}" stroke-width="3"/>')
            elems.append(f'<circle cx="{col_x:.0f}" cy="{h_bot:.0f}" r="8" fill="{bright}"/>')

    return "\n      ".join(elems)


def build_part_divider_html(part: TocPart) -> str:
    """Render a full-bleed A4 architectural divider page opening a Part or Appendices."""
    pstyle = PART_STYLES[part.key]
    roman = pstyle["roman"]
    accent = pstyle["accent"]
    dark = pstyle["dark"]
    bright = pstyle["bright"]
    mono_stack = "'Source Code Pro', 'JetBrains Mono', ui-monospace, monospace"
    sans_stack = "'Open Sans', 'Inter', 'Liberation Sans', system-ui, sans-serif"

    # Strip "Part X — " prefix for clean monumental title on lower paper plinth
    clean_title = re.sub(r"^Part\s+[IVX]+\s*[—-]\s*", "", part.title).strip() or part.title
    if len(clean_title) <= 32:
        title_fs, line_h, wrap_cols = 104, 114, 24
    elif len(clean_title) <= 58:
        title_fs, line_h, wrap_cols = 80, 92, 33
    else:
        title_fs, line_h, wrap_cols = 66, 76, 40
    title_lines = _wrap_cover_lines(clean_title, max_chars=wrap_cols)

    numeral_size = 380 if len(roman) == 1 else (310 if len(roman) == 2 else 230)
    eyebrow = "REFERENCE APPARATUS" if part.key == "appendices" else f"PART {roman}"

    title_svg = "\n".join(
        f'  <text class="cov-sans" x="135" y="{1895 + i * line_h}" fill="#141210" font-family="{sans_stack}" '
        f'font-size="{title_fs}" font-weight="800" letter-spacing="{-3 if title_fs >= 90 else -2}">{xml_escape_text(line)}</text>'
        for i, line in enumerate(title_lines)
    )
    y_rule = 1895 + (len(title_lines) - 1) * line_h + 48
    y_list = y_rule + 64

    # Build concise mini-contents index (subgroups if named, else entries across 1 or 2 columns)
    named_groups = [g for g in part.groups if g.title]
    index_nodes: list[str] = []
    if named_groups:
        row_h = 74 if len(named_groups) <= 6 else 66
        for idx, g in enumerate(named_groups[:8]):
            gy = y_list + idx * row_h
            ch_count = len(g.entries)
            first_code = _entry_toc_code(g.entries[0]) if g.entries else ""
            last_code = _entry_toc_code(g.entries[-1]) if g.entries else ""
            span_lbl = f"{first_code}–{last_code}" if first_code != last_code else first_code
            index_nodes.append(
                f'  <text class="cov-mono" x="140" y="{gy}" fill="{accent}" font-family="{mono_stack}" '
                f'font-size="29" font-weight="500">{xml_escape_text(span_lbl)}</text>'
            )
            index_nodes.append(
                f'  <text class="cov-sans" x="340" y="{gy}" fill="#2b2520" font-family="{sans_stack}" '
                f'font-size="33" font-weight="600">{xml_escape_text(_truncate_words(g.title or "", 58))}</text>'
            )
            index_nodes.append(
                f'  <text class="cov-mono" x="1960" y="{gy}" text-anchor="end" fill="#584e43" font-family="{mono_stack}" '
                f'font-size="26" font-weight="500">{ch_count} SECTIONS</text>'
            )
    else:
        flat_entries = [e for g in part.groups for e in g.entries]
        use_two_cols = len(flat_entries) > 10
        row_h = 68
        half = (len(flat_entries) + 1) // 2 if use_two_cols else len(flat_entries)
        for idx, entry in enumerate(flat_entries[:18]):
            col = 1 if (use_two_cols and idx >= half) else 0
            row = idx - half if col == 1 else idx
            bx = 140 if col == 0 else 1090
            by = y_list + row * row_h
            code = _entry_toc_code(entry)
            label = re.sub(r"^Appendix\s+[A-G]\s*[—-]\s*", "", entry.title) if part.key == "appendices" else entry.title
            max_w = 32 if use_two_cols else 54
            code_fs = 27 if use_two_cols else 30
            lbl_fs = 30 if use_two_cols else 35
            lbl_off = 135 if use_two_cols else 160
            index_nodes.append(
                f'  <text class="cov-mono" x="{bx}" y="{by}" fill="{accent}" font-family="{mono_stack}" '
                f'font-size="{code_fs}" font-weight="500">{xml_escape_text(code)}</text>'
            )
            index_nodes.append(
                f'  <text class="cov-sans" x="{bx + lbl_off}" y="{by}" fill="#2b2520" font-family="{sans_stack}" '
                f'font-size="{lbl_fs}" font-weight="500">{xml_escape_text(_truncate_words(label, max_w))}</text>'
            )

    clip_id = f"pdiv-{part.key}-clip"
    motif_svg = _build_part_motif_svg(part.key, bright, accent)
    svg = f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2100 3019" preserveAspectRatio="xMidYMid slice" role="img" aria-label="{xml_escape_attr(part.title)}">
  <defs>
    <clipPath id="{clip_id}">
      <rect x="0" y="0" width="2100" height="1660"/>
    </clipPath>
  </defs>
  <rect width="2100" height="3019" fill="#f4efe4"/>
  <rect x="0" y="0" width="2100" height="1660" fill="{dark}"/>
  <g clip-path="url(#{clip_id})">
    <g stroke="#ffffff" stroke-opacity="0.06" stroke-width="1">
      <line x1="140" y1="0" x2="140" y2="1660"/>
      <line x1="747" y1="0" x2="747" y2="1660"/>
      <line x1="1353" y1="0" x2="1353" y2="1660"/>
      <line x1="1960" y1="0" x2="1960" y2="1660"/>
    </g>
    <text class="cov-mono" x="140" y="116" fill="#f4efe4" font-family="{mono_stack}" font-size="32" font-weight="500" letter-spacing="4.2">STOCHOLOGY</text>
    <text class="cov-mono" x="1960" y="116" text-anchor="end" fill="#f4efe4" fill-opacity="0.78" font-family="{mono_stack}" font-size="28" font-weight="500" letter-spacing="2.5">{xml_escape_text(pstyle["range"])}</text>
    <line x1="140" y1="164" x2="1960" y2="164" stroke="#ffffff" stroke-opacity="0.16" stroke-width="1.5"/>

    <text class="cov-mono" x="140" y="408" fill="{bright}" font-family="{mono_stack}" font-size="32" font-weight="500" letter-spacing="4">{xml_escape_text(eyebrow)}</text>
    <text class="cov-sans" x="130" y="755" fill="#f4efe4" font-family="{sans_stack}" font-size="{numeral_size}" font-weight="800" letter-spacing="-8">{xml_escape_text(roman)}</text>
    <line x1="140" y1="815" x2="480" y2="815" stroke="{bright}" stroke-width="4"/>

    <g>
      {motif_svg}
    </g>
  </g>
  <rect x="0" y="1660" width="2100" height="10" fill="{accent}"/>
  <text class="cov-mono" x="140" y="1765" fill="{accent}" font-family="{mono_stack}" font-size="32" font-weight="500" letter-spacing="4">{xml_escape_text(eyebrow)}</text>
{title_svg}
  <line x1="140" y1="{y_rule}" x2="1960" y2="{y_rule}" stroke="#cfc6b6" stroke-width="2"/>
{chr(10).join(index_nodes)}
</svg>"""
    return (
        f'  <section id="part-{xml_escape_attr(part.key)}" class="pdf-part-divider" '
        f'data-part="{xml_escape_attr(part.key)}" aria-label="{xml_escape_attr(part.title)}">\n'
        f"{svg}\n"
        "  </section>"
    )


def _entry_toc_code(entry: TocEntry) -> str:
    slug = entry.slug
    if slug == "00-preface":
        return "00"
    if re.match(r"^part-[ivx]+-introduction$", slug):
        return "INTRO"
    if re.match(r"^part-[ivx]+-conclusion$", slug):
        return "CONCL"
    if m := re.match(r"^stratum-([ivx]+)-opening$", slug):
        return f"S-{m.group(1).upper()}"
    if m := re.match(r"^appendix-([a-g])-", slug):
        return m.group(1).upper()
    if m := re.match(r"^(\d{2})-", slug):
        num = int(m.group(1))
        if 51 <= num <= 85:
            return f"L{num - 50:02d}"
        return f"{num:02d}"
    return "·"


def _entry_toc_label(entry: TocEntry) -> str:
    if re.match(r"^(?:5[1-9]|[67]\d|8[0-5])-", entry.slug):
        return re.sub(r"^Layer\s+\d+:\s*", "", entry.title)
    if entry.part_key == "appendices":
        return re.sub(r"^Appendix\s+[A-G]\s*[—-]\s*", "", entry.title)
    return entry.title


def build_toc_html(parts: list[TocPart], page_numbers: dict[str, int] | None = None) -> str:
    def _toc_page_str(dest_id: str) -> str:
        if page_numbers is None:
            return "0000"
        num = page_numbers.get(dest_id)
        return str(num) if num is not None else "—"

    items: list[str] = []
    for part in parts:
        pstyle = PART_STYLES.get(part.key, PART_STYLES["front"])
        accent = pstyle["accent"]
        roman = pstyle["roman"]

        if part.key == "front":
            epig_li = (
                '          <li class="toc-entry"><a class="toc-link" href="#front-epigraph">'
                '<span class="toc-code">∞</span>'
                '<span class="toc-label">Epigraph</span>'
                f'<span class="toc-page">{xml_escape_text(_toc_page_str("front-epigraph"))}</span>'
                "</a></li>"
            )
            pres_li = (
                '          <li class="toc-entry"><a class="toc-link" href="#front-presentation">'
                '<span class="toc-code">Ø</span>'
                '<span class="toc-label">Presentation</span>'
                f'<span class="toc-page">{xml_escape_text(_toc_page_str("front-presentation"))}</span>'
                "</a></li>"
            )
            chap_lis = "\n".join(
                f'          <li class="toc-entry"><a class="toc-link" href="#doc-{xml_escape_attr(e.slug)}">'
                f'<span class="toc-code">{_entry_toc_code(e)}</span>'
                f'<span class="toc-label">{xml_escape_text(_entry_toc_label(e))}</span>'
                f'<span class="toc-page">{xml_escape_text(_toc_page_str(f"doc-{e.slug}"))}</span>'
                "</a></li>"
                for g in part.groups
                for e in g.entries
            )
            front_lis = [epig_li, pres_li]
            if chap_lis:
                front_lis.append(chap_lis)
            lis = "\n".join(front_lis)
            items.append(
                f'      <li class="toc-part" style="--toc-part-accent: {accent};">\n'
                '        <div class="toc-part-head">\n'
                '<span class="toc-part-badge">∞ · Ø–00</span>\n'
                '          <span class="toc-part-title">Front Matter &amp; Prologue</span>\n'
                '        </div>\n'
                f'        <ol class="toc-entries toc-entries--single">\n{lis}\n        </ol>\n'
                '      </li>'
            )
            continue

        group_blocks: list[str] = []
        for group in part.groups:
            if not group.entries:
                continue
            grid_cls = "toc-entries toc-entries--single" if len(group.entries) == 1 else "toc-entries"
            entry_lis = "\n".join(
                f'            <li class="toc-entry"><a class="toc-link" href="#doc-{xml_escape_attr(e.slug)}">'
                f'<span class="toc-code">{_entry_toc_code(e)}</span>'
                f'<span class="toc-label">{xml_escape_text(_entry_toc_label(e))}</span>'
                f'<span class="toc-page">{xml_escape_text(_toc_page_str(f"doc-{e.slug}"))}</span>'
                "</a></li>"
                for e in group.entries
            )
            if group.title:
                group_blocks.append(
                    '        <div class="toc-group">\n'
                    f'          <span class="toc-group-title">{xml_escape_text(group.title)}</span>\n'
                    f'          <ol class="{grid_cls}">\n{entry_lis}\n          </ol>\n'
                    '        </div>'
                )
            else:
                group_blocks.append(f'        <ol class="{grid_cls}">\n{entry_lis}\n        </ol>')

        badge_lbl = roman if part.key == "appendices" else f"PART {roman}"
        part_dest = f"part-{part.key}"
        part_page_str = _toc_page_str(part_dest)
        items.append(
            f'      <li class="toc-part" style="--toc-part-accent: {accent};">\n'
            '        <div class="toc-part-head">\n'
            f'          <a class="toc-part-link" href="#{xml_escape_attr(part_dest)}">\n'
            f'            <span class="toc-part-badge">{xml_escape_text(badge_lbl)}</span>\n'
            f'            <span class="toc-part-title">{xml_escape_text(part.title)}</span>\n'
            f'            <span class="toc-part-page">{xml_escape_text(part_page_str)}</span>\n'
            '          </a>\n'
            '        </div>\n'
            f'{chr(10).join(group_blocks)}\n'
            '      </li>'
        )

    toc_body = "\n".join(items)
    return (
        '  <nav class="pdf-toc" aria-label="Table of contents">\n'
        '    <header class="toc-header">\n'
        '      <p class="toc-eyebrow">STOCHOLOGY · EDITION ARCHITECTURE</p>\n'
        '      <h2 class="toc-title">Contents</h2>\n'
        '      <div class="toc-triad-rule" aria-hidden="true"><span></span><span></span><span></span></div>\n'
        '    </header>\n'
        '    <ol>\n'
        f'{toc_body}\n'
        '    </ol>\n'
        '  </nav>'
    )


def convert_html_chapter(
    html_path: str, entry: TocEntry, svg_seq: list[int]
) -> tuple[str, ChapterMeta]:
    raw = open(html_path, encoding="utf-8").read()
    m = re.search(r"<main>(.*?)</main>", raw, re.S)
    if not m:
        raise ValueError(f"Missing <main> in {html_path}")

    parser = MainHTMLParser(chapter_slug=entry.slug, svg_seq=svg_seq)
    parser.feed(m.group(1))
    parser.close()
    chapter_meta = extract_chapter_meta(entry, parser.root)
    decorate_chapter_article(parser.root, chapter_meta)
    return render_node(parser.root), chapter_meta


# ------------------------------------------------------------------ Chromium & PDF validation


def find_chromium_binary() -> str:
    candidates: list[str] = []
    env_bin = os.environ.get("CHROME_BIN", "").strip()
    if env_bin:
        candidates.append(env_bin)
    candidates.extend(
        [
            "google-chrome",
            "google-chrome-stable",
            "chromium",
            "chromium-browser",
            "/snap/bin/chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        ]
    )
    for cand in candidates:
        resolved = shutil.which(cand) or (cand if os.path.isfile(cand) and os.access(cand, os.X_OK) else None)
        if resolved:
            return resolved
    raise RuntimeError(
        "No Chromium or Google Chrome executable found (checked CHROME_BIN and standard paths)"
    )


def validate_pdf(output_pdf: str, min_pages: int) -> tuple[int, float]:
    """Verify PDF signature, trailer marker, size, and page object count."""
    if not os.path.isfile(output_pdf):
        raise ValueError(f"PDF output missing: {output_pdf}")
    size_bytes = os.path.getsize(output_pdf)
    if size_bytes < 200_000:
        raise ValueError(f"PDF suspiciously small ({size_bytes} bytes)")

    with open(output_pdf, "rb") as handle:
        data = handle.read()
    if not data.startswith(b"%PDF-1."):
        raise ValueError("Invalid PDF header (expected %PDF-1.x)")
    if b"%%EOF" not in data[-1024:]:
        raise ValueError("Missing %%EOF trailer marker at end of PDF")

    page_count = len(re.findall(rb"/Type\s*/Page(?![sA-Za-z])", data))
    if page_count < min_pages:
        raise ValueError(f"Expected at least {min_pages} PDF pages, found {page_count}")
    return page_count, size_bytes / 1024.0


def _decode_pdf_name(raw_name: bytes) -> str:
    decoded = re.sub(
        rb"#([0-9A-Fa-f]{2})",
        lambda m: bytes([int(m.group(1), 16)]),
        raw_name,
    )
    return decoded.decode("utf-8", errors="replace")


def _extract_balanced_token(data: bytes, start: int, open_tok: bytes, close_tok: bytes) -> tuple[bytes, int]:
    """Extract balanced delimiter span starting at open_tok, ignoring string literals."""
    idx = start
    depth = 0
    n = len(data)
    olen = len(open_tok)
    clen = len(close_tok)
    while idx < n:
        ch = data[idx : idx + 1]
        if ch == b"(":
            idx += 1
            paren = 1
            while idx < n and paren > 0:
                b = data[idx : idx + 1]
                if b == b"\\":
                    idx += 2
                    continue
                if b == b"(":
                    paren += 1
                elif b == b")":
                    paren -= 1
                idx += 1
            continue
        if data[idx : idx + olen] == open_tok:
            depth += 1
            idx += olen
            continue
        if data[idx : idx + clen] == close_tok:
            depth -= 1
            idx += clen
            if depth == 0:
                return data[start:idx], idx
            continue
        idx += 1
    return data[start:], n


def _parse_pdf_xref(data: bytes) -> tuple[dict[int, int], int | None]:
    """Parse classic PDF xref table(s) via startxref; fallback to scanning outside streams."""
    tail = data[max(0, len(data) - 4096) :]
    m_sx = re.findall(rb"startxref\s+(\d+)\s+%%EOF", tail)
    offsets: dict[int, int] = {}
    root_obj: int | None = None
    visited: set[int] = set()

    if m_sx:
        xref_off = int(m_sx[-1])
        while 0 <= xref_off < len(data) and xref_off not in visited:
            visited.add(xref_off)
            sub = data[xref_off:]
            if not sub.startswith(b"xref"):
                break
            nl_pos = sub.find(b"\n")
            if nl_pos == -1:
                break
            pos = nl_pos + 1
            while pos < len(sub):
                line_start = pos
                line_end = sub.find(b"\n", pos)
                if line_end == -1:
                    line_end = len(sub)
                line = sub[pos:line_end].strip()
                pos = line_end + 1
                if not line:
                    continue
                if line.startswith(b"trailer"):
                    tr_chunk = sub[line_start : min(len(sub), line_start + 4096)]
                    if root_obj is None:
                        m_root = re.search(rb"/Root\s+(\d+)\s+0\s+R", tr_chunk)
                        if m_root:
                            root_obj = int(m_root.group(1))
                    m_prev = re.search(rb"/Prev\s+(\d+)", tr_chunk)
                    xref_off = int(m_prev.group(1)) if m_prev else -1
                    break
                parts = line.split()
                if len(parts) == 2 and parts[0].isdigit() and parts[1].isdigit():
                    start_num = int(parts[0])
                    count = int(parts[1])
                    for idx in range(count):
                        rec_end = sub.find(b"\n", pos)
                        if rec_end == -1:
                            rec_end = len(sub)
                        rec = sub[pos:rec_end].strip().split()
                        pos = rec_end + 1
                        if len(rec) >= 3 and rec[2] == b"n" and rec[0].isdigit():
                            offsets.setdefault(start_num + idx, int(rec[0]))
                else:
                    break
            else:
                break

    if not offsets or root_obj is None:
        cleaned = re.sub(rb"stream\r?\n.*?\r?\nendstream", b"stream endstream", data, flags=re.S)
        for m in re.finditer(rb"(?:^|[\r\n])(\d+)\s+0\s+obj\b", cleaned):
            offsets.setdefault(int(m.group(1)), m.start(1))
        m_cat = re.search(rb"(\d+)\s+0\s+obj\b[^]*?/Type\s*/Catalog\b", cleaned)
        if m_cat:
            root_obj = int(m_cat.group(1))
        else:
            m_root = re.search(rb"/Root\s+(\d+)\s+0\s+R", tail)
            if m_root:
                root_obj = int(m_root.group(1))
        if root_obj is not None and cleaned is not data:
            # Map direct body fallback via offsets on cleaned buffer only if classic xref missing
            pass

    return offsets, root_obj


def _read_pdf_obj(data: bytes, offsets: dict[int, int], obj_num: int) -> bytes:
    off = offsets.get(obj_num)
    if off is None or off < 0 or off >= len(data):
        return b""
    m_head = re.match(rb"\s*\d+\s+\d+\s+obj\b", data[off : off + 64])
    if not m_head:
        return b""
    start = off + m_head.end()
    end = data.find(b"endobj", start)
    if end == -1:
        return data[start : start + 262144]
    return data[start:end]


def _collect_pdf_page_order(data: bytes, offsets: dict[int, int], pages_root: int) -> dict[int, int]:
    """Traverse the PDF /Pages tree in reading order and return {page_obj_num: 1-based page}."""
    page_objs: list[int] = []
    seen: set[int] = set()

    def walk(node_obj: int) -> None:
        if node_obj in seen:
            return
        seen.add(node_obj)
        body = _read_pdf_obj(data, offsets, node_obj)
        if re.search(rb"/Type\s*/Pages\b", body):
            m_kids = re.search(rb"/Kids\s*\[", body)
            if m_kids:
                arr, _ = _extract_balanced_token(body, m_kids.end() - 1, b"[", b"]")
                for kid in re.findall(rb"(\d+)\s+0\s+R", arr):
                    walk(int(kid))
        elif re.search(rb"/Type\s*/Page(?![sA-Za-z])", body):
            page_objs.append(node_obj)

    walk(pages_root)
    return {obj_num: idx for idx, obj_num in enumerate(page_objs, 1)}


def _resolve_dest_value(
    val: bytes, data: bytes, offsets: dict[int, int], page_map: dict[int, int], depth: int = 0
) -> int | None:
    if depth > 3:
        return None
    val_s = val.lstrip()
    if val_s.startswith(b"["):
        m_p = re.match(rb"\[\s*(\d+)\s+0\s+R\b", val_s)
        if m_p:
            return page_map.get(int(m_p.group(1)))
    elif val_s.startswith(b"<<"):
        m_d = re.search(rb"/D\s*\[", val_s)
        if m_d:
            arr, _ = _extract_balanced_token(val_s, m_d.end() - 1, b"[", b"]")
            m_p = re.match(rb"\[\s*(\d+)\s+0\s+R\b", arr)
            if m_p:
                return page_map.get(int(m_p.group(1)))
    else:
        m_ref = re.match(rb"(\d+)\s+0\s+R\b", val_s)
        if m_ref:
            target = _read_pdf_obj(data, offsets, int(m_ref.group(1))).strip()
            return _resolve_dest_value(target, data, offsets, page_map, depth + 1)
    return None


def _parse_pdf_dests_dict(
    dict_bytes: bytes, data: bytes, offsets: dict[int, int], page_map: dict[int, int]
) -> dict[str, int]:
    inner = dict_bytes.strip()
    if inner.startswith(b"<<") and inner.endswith(b">>"):
        inner = inner[2:-2]
    result: dict[str, int] = {}
    i = 0
    n = len(inner)
    ws = b" \t\r\n\f\x00"
    delim = ws + b"<>[]()/{}%"

    while i < n:
        while i < n and inner[i : i + 1] in ws:
            i += 1
        if i >= n:
            break
        if inner[i : i + 1] != b"/":
            i += 1
            continue
        k_start = i + 1
        k_end = k_start
        while k_end < n and inner[k_end : k_end + 1] not in delim:
            k_end += 1
        name = _decode_pdf_name(inner[k_start:k_end])
        i = k_end
        while i < n and inner[i : i + 1] in ws:
            i += 1
        if i >= n:
            break
        if inner[i : i + 1] == b"[":
            tok, i = _extract_balanced_token(inner, i, b"[", b"]")
            pg = _resolve_dest_value(tok, data, offsets, page_map)
            if pg is not None and name:
                result[name] = pg
        elif inner[i : i + 2] == b"<<":
            tok, i = _extract_balanced_token(inner, i, b"<<", b">>")
            pg = _resolve_dest_value(tok, data, offsets, page_map)
            if pg is not None and name:
                result[name] = pg
        else:
            m_ref = re.match(rb"(\d+\s+0\s+R)\b", inner[i:])
            if m_ref:
                pg = _resolve_dest_value(m_ref.group(1), data, offsets, page_map)
                if pg is not None and name:
                    result[name] = pg
                i += m_ref.end()
            else:
                i += 1
    return result


def _decode_pdf_literal_str(raw: bytes, start: int) -> tuple[str, int]:
    """Decode PDF (...) literal string starting at '(' and return (text, next_pos)."""
    i = start + 1
    n = len(raw)
    out = bytearray()
    depth = 1
    while i < n and depth > 0:
        b = raw[i]
        if b == ord("\\"):
            i += 1
            if i >= n:
                break
            esc = raw[i]
            esc_map = {ord("n"): 10, ord("r"): 13, ord("t"): 9, ord("b"): 8, ord("f"): 12}
            if esc in esc_map:
                out.append(esc_map[esc])
                i += 1
            elif esc in (ord("("), ord(")"), ord("\\")):
                out.append(esc)
                i += 1
            elif 48 <= esc <= 55:
                oct_digits = raw[i : i + 3]
                m_oct = re.match(rb"[0-7]{1,3}", oct_digits)
                if m_oct:
                    out.append(int(m_oct.group(0), 8) & 0xFF)
                    i += m_oct.end()
            else:
                out.append(esc)
                i += 1
        elif b == ord("("):
            depth += 1
            out.append(b)
            i += 1
        elif b == ord(")"):
            depth -= 1
            if depth > 0:
                out.append(b)
            i += 1
        else:
            out.append(b)
            i += 1
    return out.decode("utf-8", errors="replace"), i


def _walk_names_dest_tree(
    node_obj: int, data: bytes, offsets: dict[int, int], page_map: dict[int, int]
) -> dict[str, int]:
    out: dict[str, int] = {}
    seen: set[int] = set()

    def walk(obj_id: int) -> None:
        if obj_id in seen:
            return
        seen.add(obj_id)
        body = _read_pdf_obj(data, offsets, obj_id)
        m_kids = re.search(rb"/Kids\s*\[", body)
        if m_kids:
            arr, _ = _extract_balanced_token(body, m_kids.end() - 1, b"[", b"]")
            for kid in re.findall(rb"(\d+)\s+0\s+R", arr):
                walk(int(kid))
        m_names = re.search(rb"/Names\s*\[", body)
        if m_names:
            arr, _ = _extract_balanced_token(body, m_names.end() - 1, b"[", b"]")
            inner = arr[1:-1]
            j = 0
            m_len = len(inner)
            ws = b" \t\r\n\f\x00"
            while j < m_len:
                while j < m_len and inner[j : j + 1] in ws:
                    j += 1
                if j >= m_len:
                    break
                if inner[j : j + 1] == b"(":
                    key, j = _decode_pdf_literal_str(inner, j)
                elif inner[j : j + 1] == b"<" and inner[j : j + 2] != b"<<":
                    end_hex = inner.find(b">", j + 1)
                    if end_hex == -1:
                        break
                    hex_bytes = re.sub(rb"\s+", b"", inner[j + 1 : end_hex])
                    if len(hex_bytes) % 2:
                        hex_bytes += b"0"
                    key = bytes.fromhex(hex_bytes.decode("ascii", errors="ignore")).decode(
                        "utf-8", errors="replace"
                    )
                    j = end_hex + 1
                else:
                    j += 1
                    continue
                while j < m_len and inner[j : j + 1] in ws:
                    j += 1
                if j >= m_len:
                    break
                if inner[j : j + 1] == b"[":
                    tok, j = _extract_balanced_token(inner, j, b"[", b"]")
                    pg = _resolve_dest_value(tok, data, offsets, page_map)
                    if pg is not None and key:
                        out[key] = pg
                elif inner[j : j + 2] == b"<<":
                    tok, j = _extract_balanced_token(inner, j, b"<<", b">>")
                    pg = _resolve_dest_value(tok, data, offsets, page_map)
                    if pg is not None and key:
                        out[key] = pg
                else:
                    m_ref = re.match(rb"(\d+\s+0\s+R)\b", inner[j:])
                    if m_ref:
                        pg = _resolve_dest_value(m_ref.group(1), data, offsets, page_map)
                        if pg is not None and key:
                            out[key] = pg
                        j += m_ref.end()
                    else:
                        j += 1

    walk(node_obj)
    return out


def extract_pdf_dest_pages(pdf_path: str) -> dict[str, int]:
    """Resolve named PDF destinations (/Dests dict and /Names -> /Dests tree) to 1-based page numbers."""
    with open(pdf_path, "rb") as handle:
        data = handle.read()
    offsets, root_obj = _parse_pdf_xref(data)
    if root_obj is None:
        raise ValueError(f"Could not locate PDF /Root catalog in {pdf_path}")

    catalog = _read_pdf_obj(data, offsets, root_obj)
    m_pages = re.search(rb"/Pages\s+(\d+)\s+0\s+R", catalog)
    if not m_pages:
        raise ValueError(f"Could not locate /Pages root in catalog of {pdf_path}")
    page_map = _collect_pdf_page_order(data, offsets, int(m_pages.group(1)))
    if not page_map:
        raise ValueError(f"No PDF leaf pages found in {pdf_path}")

    dests: dict[str, int] = {}

    m_dests_ref = re.search(rb"/Dests\s+(\d+)\s+0\s+R", catalog)
    if m_dests_ref:
        dest_obj_bytes = _read_pdf_obj(data, offsets, int(m_dests_ref.group(1)))
        dests.update(_parse_pdf_dests_dict(dest_obj_bytes, data, offsets, page_map))
    else:
        m_dests_inline = re.search(rb"/Dests\s*<<", catalog)
        if m_dests_inline:
            inline_dict, _ = _extract_balanced_token(catalog, m_dests_inline.end() - 2, b"<<", b">>")
            dests.update(_parse_pdf_dests_dict(inline_dict, data, offsets, page_map))

    m_names_ref = re.search(rb"/Names\s+(\d+)\s+0\s+R", catalog)
    names_body = _read_pdf_obj(data, offsets, int(m_names_ref.group(1))) if m_names_ref else b""
    if not names_body:
        m_names_inline = re.search(rb"/Names\s*<<", catalog)
        if m_names_inline:
            names_body, _ = _extract_balanced_token(catalog, m_names_inline.end() - 2, b"<<", b">>")
    if names_body:
        m_nd = re.search(rb"/Dests\s+(\d+)\s+0\s+R", names_body)
        if m_nd:
            for k, v in _walk_names_dest_tree(int(m_nd.group(1)), data, offsets, page_map).items():
                dests.setdefault(k, v)

    return dests


def _run_chromium_print(
    browser: str, html_doc: str, tmp_html: str, tmp_profile: str, tmp_pdf: str
) -> None:
    with open(tmp_html, "w", encoding="utf-8") as handle:
        handle.write(html_doc)
    if os.path.exists(tmp_pdf):
        os.remove(tmp_pdf)
    os.makedirs(tmp_profile, exist_ok=True)
    cmd = [
        browser,
        "--headless=new",
        "--no-sandbox",
        "--disable-gpu",
        "--disable-dev-shm-usage",
        f"--user-data-dir={tmp_profile}",
        "--no-pdf-header-footer",
        "--generate-pdf-document-outline",
        f"--print-to-pdf={tmp_pdf}",
        f"file://{tmp_html}",
    ]
    res = subprocess.run(cmd, capture_output=True, text=True, timeout=180)
    if res.returncode != 0 or not os.path.isfile(tmp_pdf):
        tail = res.stderr[-2000:] if res.stderr else ""
        raise RuntimeError(f"Chromium failed (exit {res.returncode}): {tail}")


def build_pdf(build_dir: str, output_pdf: str) -> int:
    abs_build = os.path.abspath(build_dir)
    if not os.path.isdir(abs_build):
        print(f"build_pdf: missing HTML build directory {build_dir!r}", file=sys.stderr)
        return 1

    parts, spine = parse_summary(os.path.join(BOOK_SRC, "SUMMARY.md"))
    parts_by_key = {p.key: p for p in parts}
    meta = load_metadata()
    svg_seq = [0]
    body_blocks: list[str] = []
    chapter_metas: list[ChapterMeta] = []
    seen_part_keys: set[str] = set()

    for entry in spine:
        html_path = os.path.join(abs_build, entry.html_name)
        if not os.path.isfile(html_path):
            print(f"build_pdf: missing rendered chapter HTML {html_path}", file=sys.stderr)
            return 1
        if entry.part_key != "front" and entry.part_key not in seen_part_keys:
            seen_part_keys.add(entry.part_key)
            if entry.part_key in parts_by_key:
                body_blocks.append(build_part_divider_html(parts_by_key[entry.part_key]))
        if entry.part_key in PART_ART_PLATES and entry.slug == f"{entry.part_key}-introduction":
            plate_file, plate_aria, plate_alt = PART_ART_PLATES[entry.part_key]
            body_blocks.append(
                build_art_plate_html(plate_file, f"pdf-art-plate-{entry.part_key}", plate_aria, plate_alt)
            )
        chapter_html, ch_meta = convert_html_chapter(html_path, entry, svg_seq)
        chapter_metas.append(ch_meta)
        body_blocks.append(chapter_html)

    cover_fonts_head = build_cover_font_head(abs_build)
    dynamic_page_css = build_dynamic_page_css(chapter_metas)
    cover_svg = build_cover_svg(meta)
    opening_plate_html = build_art_plate_html(
        "opening-plate.jpg",
        "pdf-art-plate-opening",
        "Opening artistic plate — Télos",
        "Fine-art studio macro plate of an uninscribed brass astrolabe, forged bronze arrow tip on golden silk thread, refracted cobalt, emerald, and crimson rays on cotton paper, and hourglass on dark obsidian slate.",
    )
    title_page_html = build_title_page_html(meta)
    epigraph_html = build_epigraph_html(meta)
    presentation_plate_html = build_art_plate_html(
        "presentation-plate.jpg",
        "pdf-art-plate-presentation",
        "Synthesis artistic plate — Stochology End to End",
        "Architectural observatory plate unifying all of Stochology end to end: emerald basalt vital basin and poised bronze arrow along a golden thread, sculptural brass triadic armillary uniting cobalt crystal, emerald sphere, and crimson bronze balance across concentric transit arcs, seven ascending glass strata ledges, and a coastal twilight horizon beyond a stone archway.",
    )
    presentation_html = build_presentation_html(meta)
    closing_plate_html = build_art_plate_html(
        "closing-plate.jpg",
        "pdf-art-plate-closing",
        "Closing artistic plate — Skopós",
        "Fine-art twilight photograph of a dark honed basalt reflection basin with a suspended brass plumb-bob over concentric ripples and three translucent glass spheres in cobalt blue, emerald green, and ruby crimson.",
    )
    back_cover_svg = build_back_cover_svg(meta)
    joined_body = "\n".join(body_blocks)
    expected_dests = (
        ["front-epigraph", "front-presentation"]
        + [f"doc-{e.slug}" for e in spine]
        + [f"part-{p.key}" for p in parts if p.key != "front"]
    )

    def compose_doc(page_numbers: dict[str, int] | None) -> str:
        return (
            "<!DOCTYPE html>\n"
            f'<html lang="{xml_escape_attr(meta["language"])}">\n'
            "<head>\n"
            '  <meta charset="utf-8" />\n'
            f'  <title>{xml_escape_text(meta["title"])}</title>\n'
            '  <meta name="author" content="Silvano Neto" />\n'
            + (f"{cover_fonts_head}\n" if cover_fonts_head else "")
            + f'  <style>\n{PDF_CSS}\n{dynamic_page_css}\n  </style>\n'
            "</head>\n"
            "<body>\n"
            '  <section class="pdf-cover" aria-label="Cover">\n'
            f'{cover_svg}\n'
            "  </section>\n"
            f'{opening_plate_html}\n'
            f'{title_page_html}\n'
            f'{epigraph_html}\n'
            f'{build_toc_html(parts, page_numbers=page_numbers)}\n'
            f'{presentation_html}\n'
            f'{presentation_plate_html}\n'
            + joined_body
            + f"\n{closing_plate_html}"
            + '\n  <section class="pdf-back-cover" aria-label="Back cover">\n'
            f'{back_cover_svg}\n'
            "  </section>\n"
            "</body>\n</html>\n"
        )

    abs_out = os.path.abspath(output_pdf)
    os.makedirs(os.path.dirname(abs_out), exist_ok=True)
    # Keep temp HTML, Chrome profile, and temp PDF inside abs_build so Snap Chromium confinement works cleanly.
    tmp_html = os.path.join(abs_build, "_print_edition.html")
    tmp_profile = os.path.join(abs_build, "_chrome_profile")
    tmp_pdf = os.path.join(abs_build, "_stochology.pdf.tmp")

    browser = find_chromium_binary()
    try:
        # Pass 1: print with 4-digit placeholders so TOC height is pre-reserved, then extract page map.
        _run_chromium_print(browser, compose_doc(None), tmp_html, tmp_profile, tmp_pdf)
        dest_pages = extract_pdf_dest_pages(tmp_pdf)
        missing = [k for k in expected_dests if k not in dest_pages]
        if missing:
            raise ValueError(f"Missing PDF destinations after pass 1: {missing[:10]}")

        # Pass 2 (+ Pass 3 max): inject page numbers and verify fixed-point convergence.
        for attempt in (2, 3):
            current_map = {k: dest_pages[k] for k in expected_dests}
            _run_chromium_print(browser, compose_doc(current_map), tmp_html, tmp_profile, tmp_pdf)
            verified = extract_pdf_dest_pages(tmp_pdf)
            missing_v = [k for k in expected_dests if k not in verified]
            if missing_v:
                raise ValueError(f"Missing PDF destinations after pass {attempt}: {missing_v[:10]}")
            next_map = {k: verified[k] for k in expected_dests}
            if next_map == current_map:
                break
            if attempt == 3:
                diffs = [
                    (k, current_map[k], next_map[k])
                    for k in expected_dests
                    if current_map[k] != next_map[k]
                ]
                raise ValueError(f"TOC page numbers did not converge within 3 passes: {diffs[:8]}")
            dest_pages = verified

        os.replace(tmp_pdf, abs_out)
    finally:
        for artifact in (tmp_html, tmp_pdf):
            if os.path.exists(artifact):
                os.remove(artifact)
        if os.path.exists(tmp_profile):
            shutil.rmtree(tmp_profile, ignore_errors=True)

    pages, size_kb = validate_pdf(abs_out, min_pages=len(spine))
    print(
        f"pdf: built {output_pdf} ({len(spine)} spine sections, {svg_seq[0]} scoped inline SVGs, "
        f"{pages} pages, {size_kb:.1f} KiB)"
    )
    return 0


def main(argv: list[str]) -> int:
    build_dir = argv[1] if len(argv) > 1 else os.path.join(ROOT, "book")
    output_pdf = argv[2] if len(argv) > 2 else os.path.join(build_dir, "stochology.pdf")
    return build_pdf(build_dir, output_pdf)


if __name__ == "__main__":
    sys.exit(main(sys.argv))
