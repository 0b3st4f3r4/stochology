"""Convert the Philadelphia Fed RTDSM ROUTPUT vintage matrix (xlsx) to `data/rtdsm_routput_vintages.csv`.

Standard library only; documented in data/SOURCES.md §3b.

Usage: python3 -I rtdsm_to_csv.py <routputqvqd.xlsx> <out.csv>
Output columns: vintage,obs_quarter,routput  (vintage = YYYYQq, obs_quarter = YYYYQq, level in the vintage's own units)
"""
import re
import sys
import zipfile
import xml.etree.ElementTree as ET

NS = {"m": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}


def col_index(ref):
    letters = re.match(r"[A-Z]+", ref).group(0)
    idx = 0
    for ch in letters:
        idx = idx * 26 + (ord(ch) - 64)
    return idx - 1


def vintage_label(name):
    m = re.fullmatch(r"ROUTPUT(\d{2})Q([1-4])", name)
    yy = int(m.group(1))
    year = 1900 + yy if yy >= 65 else 2000 + yy
    return f"{year}Q{m.group(2)}"


def obs_label(text):
    m = re.fullmatch(r"(\d{4}):Q([1-4])", text.strip())
    return f"{m.group(1)}Q{m.group(2)}"


def main(src, dst):
    with zipfile.ZipFile(src) as z:
        strings = [
            "".join(t.text or "" for t in si.iter(f"{{{NS['m']}}}t"))
            for si in ET.fromstring(z.read("xl/sharedStrings.xml")).findall("m:si", NS)
        ]
        sheet = ET.fromstring(z.read("xl/worksheets/sheet1.xml"))
    grid = []
    for row in sheet.find("m:sheetData", NS).findall("m:row", NS):
        cells = {}
        for c in row.findall("m:c", NS):
            v = c.find("m:v", NS)
            if v is None:
                continue
            val = strings[int(v.text)] if c.get("t") == "s" else v.text
            cells[col_index(c.get("r"))] = val
        grid.append(cells)
    header = grid[0]
    vintages = {j: vintage_label(name) for j, name in header.items() if j > 0}
    out = ["vintage,obs_quarter,routput"]
    for cells in grid[1:]:
        if 0 not in cells:
            continue
        obs = obs_label(cells[0])
        for j, vin in vintages.items():
            raw = cells.get(j)
            if raw is None or raw.strip() in ("#N/A", ""):
                continue
            out.append(f"{vin},{obs},{float(raw):.1f}")
    body = sorted(out[1:])
    with open(dst, "w", newline="\n") as fh:
        fh.write(out[0] + "\n" + "\n".join(body) + "\n")
    print(f"{len(body)} rows, {len(vintages)} vintages")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
