#!/usr/bin/env python3
"""Deterministic Part V graph validator for Stochology (Layer 0).

Complements .github/checks/structure.py by validating properties that CI does not
check: G1 DAG acyclicity, root/sink topology, transitive ancestor cones Cone(L_i),
|Cone(L35)| == 28, zero-axis (E, V, G) inheritance across Cone(L_i), Johnson
graph G2 = J(7,3) 12-regularity, and Vandermonde exposure partition (12+9+9+4+1).
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys
from collections import Counter, defaultdict, deque
from pathlib import Path

ROOT = Path(os.environ.get("STOCHOLOGY_REPO_ROOT") or Path(__file__).resolve().parents[2]).resolve()
BOOK_SRC = ROOT / "book-src"
APP_B = BOOK_SRC / "appendix-b-architecture-verification.md"

ROMAN_TO_STRATUM = {
    "I": "S1",
    "II": "S2",
    "III": "S3",
    "IV": "S4",
    "V": "S5",
    "VI": "S6",
    "VII": "S7",
}
STRATUM_ANCHORS = {
    "S1": "T5",
    "S2": "T1",
    "S3": "T4",
    "S4": "T2",
    "S5": "T3",
    "S6": "T6",
    "S7": "T7",
}
AXES_SITES = {
    "E": {"T1", "T2", "T3"},
    "V": {"T4", "T5"},
    "G": {"T6", "T7"},
}
EXPECTED_SINKS = {"L20", "L21", "L22", "L25", "L35"}
EXPECTED_VANDERMONDE = Counter({
    "Trans-Triadic Keystone": 12,
    "Governance Axial Exposure": 9,
    "Vital Axial Exposure": 9,
    "Epistemic Axial Exposure": 4,
    "Pure Epistemic Concentration": 1,
})

B1_ROW_RE = re.compile(r"^\| `(L\d\d)` \| (\d+) \| `(S\d)` \|")
BQ_RE = re.compile(
    r"^>\s*Layer\s+(\d+)\s+of\s+Part\s+V\s+·\s+Stratum\s+([IVX]+)\s+·\s+"
    r"(?:Chapter\s+(\d+)\s+·\s+)?Unique\s+Guard\s+Combination\s+`?\{([^}]+)\}`?\s+·\s+"
    r"Signature\s+`?\(([^)]+)\)`?",
    re.MULTILINE,
)
PARENT_LINK_RE = re.compile(r"\[Layer\s+(\d+)\]\([^)]+\)")


def parse_table_b1() -> dict[str, dict]:
    rows: dict[str, dict] = {}
    for line in APP_B.read_text(encoding="utf-8").splitlines():
        m = B1_ROW_RE.match(line)
        if not m:
            continue
        cells = [c.strip() for c in line.strip("|").split("|")]
        sig_nums = tuple(int(x) for x in re.findall(r"\d+", cells[6]))
        rows[m.group(1)] = {
            "order": m.group(1),
            "chapter": int(m.group(2)),
            "stratum": m.group(3),
            "name": cells[3],
            "primary_site": cells[4].strip("`"),
            "triple": sorted(re.findall(r"T\d", cells[5])),
            "signature": sig_nums,
            "regime": cells[7],
        }
    return rows


def parse_part_v_chapters() -> dict[str, dict]:
    chapters: dict[str, dict] = {}
    for path_str in sorted(glob.glob(str(BOOK_SRC / "[5-8][0-9]-*.md"))):
        path = Path(path_str)
        ch_num = int(path.name[:2])
        if not (51 <= ch_num <= 85):
            continue
        text = path.read_text(encoding="utf-8")
        # Restrict parent parsing strictly to the opening > Layer NN blockquote line(s)
        bq_lines: list[str] = []
        in_bq = False
        for line in text.splitlines():
            if line.startswith("> Layer "):
                in_bq = True
                bq_lines.append(line)
            elif in_bq and line.startswith(">"):
                bq_lines.append(line)
            elif in_bq:
                break
        bq_block = "\n".join(bq_lines)
        m = BQ_RE.search(bq_block)
        if not m:
            continue
        order = f"L{int(m.group(1)):02d}"
        parents = [f"L{int(p):02d}" for p in PARENT_LINK_RE.findall(bq_block)]
        triple = sorted(re.findall(r"T\d", m.group(4)))
        sig_nums = tuple(int(x) for x in re.findall(r"\d+", m.group(5)))
        primary_svg = re.search(rf"CH{ch_num:02d}\s*·\s*(T\d)\*", text)
        chapters[order] = {
            "order": order,
            "file": path.name,
            "chapter": ch_num,
            "declared_chapter": int(m.group(3)) if m.group(3) else ch_num,
            "stratum": ROMAN_TO_STRATUM.get(m.group(2), m.group(2)),
            "triple": triple,
            "signature": sig_nums,
            "parents": parents,
            "primary_svg": primary_svg.group(1) if primary_svg else None,
        }
    return chapters


def validate_graphs() -> dict:
    b1 = parse_table_b1()
    chs = parse_part_v_chapters()
    blocking: list[str] = []
    advisory: list[str] = []

    if len(b1) != 35:
        blocking.append(f"Table B.1 has {len(b1)} rows, expected 35.")
    if len(chs) != 35:
        blocking.append(f"Found {len(chs)} Part V layer blockquotes, expected 35.")

    orders = sorted(set(b1) | set(chs))
    children: dict[str, list[str]] = {o: [] for o in orders}
    parents: dict[str, list[str]] = {o: [] for o in orders}

    triples_seen: dict[tuple[str, ...], list[str]] = defaultdict(list)
    stratum_primaries: dict[str, list[str]] = defaultdict(list)
    regime_counts: Counter = Counter()

    for o in orders:
        row = b1.get(o)
        ch = chs.get(o)
        if not row or not ch:
            blocking.append(f"Order {o} missing in {'Table B.1' if not row else 'chapter blockquote'}.")
            continue
        if row["chapter"] != ch["chapter"] or ch["chapter"] != 50 + int(o[1:]):
            blocking.append(f"{o}: non-monotone chapter mapping B.1={row['chapter']} vs file={ch['chapter']}.")
        if row["stratum"] != ch["stratum"]:
            blocking.append(f"{o}: stratum mismatch B.1={row['stratum']} vs blockquote={ch['stratum']}.")
        if row["triple"] != ch["triple"] or len(row["triple"]) != 3:
            blocking.append(f"{o}: triple mismatch B.1={row['triple']} vs blockquote={ch['triple']}.")
        if row["signature"] != ch["signature"] or sum(row["signature"]) != 3:
            blocking.append(f"{o}: signature mismatch B.1={row['signature']} vs blockquote={ch['signature']}.")
        expected_sig = tuple(len(set(row["triple"]) & AXES_SITES[a]) for a in ("E", "V", "G"))
        if row["signature"] != expected_sig:
            blocking.append(f"{o}: signature {row['signature']} != triple axis count {expected_sig}.")
        if ch["primary_svg"] and ch["primary_svg"] != row["primary_site"]:
            blocking.append(f"{o}: masthead primary {ch['primary_svg']} != B.1 {row['primary_site']}.")
        if row["primary_site"] not in row["triple"]:
            blocking.append(f"{o}: primary site {row['primary_site']} not in triple {row['triple']}.")

        triples_seen[tuple(row["triple"])].append(o)
        stratum_primaries[row["stratum"]].append(row["primary_site"])
        regime_counts[row["regime"]] += 1

        for p in ch["parents"]:
            if p not in orders:
                blocking.append(f"{o}: unknown parent {p} in G1.")
            elif p == o:
                blocking.append(f"{o}: self-loop in G1.")
            else:
                parents[o].append(p)
                children[p].append(o)

    if len(triples_seen) != 35 or any(len(v) != 1 for v in triples_seen.values()):
        blocking.append(f"Part V triples do not form 35 unique 3-subsets (unique={len(triples_seen)}).")

    for s, sites in sorted(stratum_primaries.items()):
        if len(sites) != 5 or len(set(sites)) != 5:
            blocking.append(f"Stratum {s} primary sites are not 5 pairwise distinct sites: {sites}.")

    if regime_counts != EXPECTED_VANDERMONDE:
        blocking.append(
            f"Vandermonde exposure regime counts {dict(regime_counts)} != expected {dict(EXPECTED_VANDERMONDE)}."
        )

    # Topological sort (Kahn) for G1 acyclicity
    in_deg = {o: len(parents[o]) for o in orders}
    q = deque(sorted(o for o, d in in_deg.items() if d == 0))
    topo: list[str] = []
    while q:
        u = q.popleft()
        topo.append(u)
        for v in sorted(children[u]):
            in_deg[v] -= 1
            if in_deg[v] == 0:
                q.append(v)

    acyclic = len(topo) == len(orders)
    if not acyclic:
        cycle_nodes = sorted(set(orders) - set(topo))
        blocking.append(f"G1 is NOT acyclic! Nodes involved in/below cycle: {cycle_nodes}.")

    roots = sorted(o for o in orders if not parents[o])
    sinks = sorted(o for o in orders if not children[o])
    if roots != ["L01"]:
        blocking.append(f"G1 roots {roots} != expected ['L01'].")
    if set(sinks) != EXPECTED_SINKS:
        blocking.append(f"G1 sinks {sinks} != expected {sorted(EXPECTED_SINKS)}.")

    # Compute transitive ancestor cones Cone(L_i)
    cones: dict[str, list[str]] = {}
    if acyclic:
        cone_sets: dict[str, set[str]] = {}
        for o in topo:
            anc = {o}
            for p in parents[o]:
                anc |= cone_sets[p]
            cone_sets[o] = anc
            cones[o] = sorted(anc)
    else:
        for o in orders:
            seen: set[str] = set()
            stack = [o]
            while stack:
                cur = stack.pop()
                if cur in seen:
                    continue
                seen.add(cur)
                stack.extend(parents[cur])
            cones[o] = sorted(seen)

    cone_l35 = cones.get("L35", [])
    outside_l35 = sorted(set(orders) - set(cone_l35))
    if len(cone_l35) != 28:
        blocking.append(f"|Cone(L35)| = {len(cone_l35)}, expected 28 (outside: {outside_l35}).")

    # Check zero-axis inheritance across Cone(L_i)
    zero_axis_details: dict[str, dict] = {}
    for o in orders:
        if o not in b1:
            continue
        sig = b1[o]["signature"]
        missing_local = [ax for ax, cnt in zip(("E", "V", "G"), sig) if cnt == 0]
        if not missing_local:
            continue
        coverage: dict[str, list[str]] = {}
        for ax in missing_local:
            idx = ("E", "V", "G").index(ax)
            suppliers = [a for a in cones[o] if a != o and a in b1 and b1[a]["signature"][idx] > 0]
            coverage[ax] = suppliers
            if not suppliers:
                blocking.append(f"{o} has signature {sig} (zero {ax}) with NO upstream {ax} supplier in Cone({o}).")
        zero_axis_details[o] = coverage

    # Build G2 = J(7,3) Johnson graph
    j73_neighbors: dict[str, list[str]] = {o: [] for o in orders}
    edge_count = 0
    for i, u in enumerate(orders):
        if u not in b1:
            continue
        su = set(b1[u]["triple"])
        for v in orders[i + 1 :]:
            if v not in b1:
                continue
            if len(su & set(b1[v]["triple"])) == 2:
                j73_neighbors[u].append(v)
                j73_neighbors[v].append(u)
                edge_count += 1
    for o in orders:
        j73_neighbors[o].sort()
        if o in b1 and len(j73_neighbors[o]) != 12:
            blocking.append(f"{o} has J(7,3) degree {len(j73_neighbors[o])}, expected 12.")
    if edge_count != 210:
        blocking.append(f"J(7,3) total undirected edges = {edge_count}, expected 210.")

    layer_records = {}
    for o in orders:
        if o not in b1 or o not in chs:
            continue
        layer_records[o] = {
            **b1[o],
            "file": chs[o]["file"],
            "parents": sorted(parents[o]),
            "children": sorted(children[o]),
            "cone": cones[o],
            "cone_size": len(cones[o]),
            "j73_neighbors": j73_neighbors[o],
            "zero_axis_suppliers": zero_axis_details.get(o, {}),
        }

    return {
        "summary": {
            "orders_count": len(layer_records),
            "g1_acyclic": acyclic,
            "g1_roots": roots,
            "g1_sinks": sinks,
            "cone_l35_size": len(cone_l35),
            "outside_cone_l35": outside_l35,
            "g2_edge_count": edge_count,
            "g2_regular_12": all(len(j73_neighbors[o]) == 12 for o in layer_records),
            "vandermonde_counts": dict(regime_counts),
            "blocking_count": len(blocking),
            "advisory_count": len(advisory),
        },
        "blocking": blocking,
        "advisory": advisory,
        "layers": layer_records,
    }


def format_markdown(report: dict, stratum: str | None = None, layer: str | None = None) -> str:
    s = report["summary"]
    lines = ["# Part V Graph Audit (`G1` DAG & `G2 = J(7,3)`)", ""]
    status = "PASS" if s["blocking_count"] == 0 else "FAIL"
    lines.append(f"- **Status:** `{status}` (`{s['blocking_count']}` blocking defects)")
    lines.append(
        f"- **G1 Topology:** Acyclic=`{s['g1_acyclic']}` · Roots=`{s['g1_roots']}` · "
        f"Sinks=`{s['g1_sinks']}` · `|Cone(L35)|={s['cone_l35_size']}` (outside: `{s['outside_cone_l35']}`)"
    )
    lines.append(
        f"- **G2 = J(7,3):** Orders=`{s['orders_count']}` · Edges=`{s['g2_edge_count']}` · "
        f"12-Regular=`{s['g2_regular_12']}`"
    )
    vc = s["vandermonde_counts"]
    lines.append(
        "- **Vandermonde Regimes:** "
        + ", ".join(f"{k}={v}" for k, v in sorted(vc.items(), key=lambda kv: -kv[1]))
    )
    if report["blocking"]:
        lines.extend(["", "## Blocking Graph Defects", ""])
        for item in report["blocking"]:
            lines.append(f"- `{item}`")

    selected = sorted(report["layers"].values(), key=lambda r: r["order"])
    if layer:
        norm = f"L{int(layer.lstrip('Ll')):02d}"
        selected = [r for r in selected if r["order"] == norm]
    elif stratum:
        norm_s = f"S{stratum.lstrip('Ss')}"
        selected = [r for r in selected if r["stratum"] == norm_s]

    if selected:
        lines.extend([
            "",
            "## Selected Order Graph Neighborhood",
            "",
            "| Order | Ch | Stratum | Primary | Triple | Sig `(E,V,G)` | Parents (`G1`) | `|Cone|` | Zero-Axis Upstream Suppliers | `J(7,3)` Neighbors (12) |",
            "| :--- | :---: | :---: | :---: | :--- | :---: | :--- | :---: | :--- | :--- |",
        ])
        for r in selected:
            p_str = ", ".join(r["parents"]) if r["parents"] else "ROOT"
            sup = (
                "; ".join(f"{ax}: {len(v)} (`{', '.join(v[:3])}`{'...' if len(v) > 3 else ''})" for ax, v in r["zero_axis_suppliers"].items())
                if r["zero_axis_suppliers"]
                else "—"
            )
            j_str = ", ".join(r["j73_neighbors"])
            lines.append(
                f"| `{r['order']}` | {r['chapter']} | `{r['stratum']}` (anchor `{STRATUM_ANCHORS[r['stratum']]}`) | "
                f"`{r['primary_site']}` | `{','.join(r['triple'])}` | `{r['signature']}` | "
                f"{p_str} | {r['cone_size']} | {sup} | {j_str} |"
            )
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate Stochology Part V G1 DAG and G2 = J(7,3).")
    parser.add_argument("--stratum", help="Filter output table to stratum S1..S7")
    parser.add_argument("--layer", help="Filter output table to order L01..L35")
    parser.add_argument("--json", action="store_true", help="Emit JSON output")
    parser.add_argument("--markdown", action="store_true", help="Emit Markdown summary (default)")
    parser.add_argument("--out", help="Optional file path to write output")
    args = parser.parse_args()

    report = validate_graphs()
    if args.json:
        output = json.dumps(report, indent=2) + "\n"
    else:
        output = format_markdown(report, stratum=args.stratum, layer=args.layer)

    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(output, encoding="utf-8")
    else:
        sys.stdout.write(output)
    return 0 if report["summary"]["blocking_count"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
