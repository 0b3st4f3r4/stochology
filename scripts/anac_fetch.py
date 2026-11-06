#!/usr/bin/env python3
"""Fetch the ANAC air-transport statistical series for a future sealed dataset.

This script is the *fetch* half of a two-step provenance gate. It never writes into
``data/SHA256SUMS``: sealing a new dataset requires an explicit human decision, a pinned
digest and a schema contract (see ``AGENTS.md`` §IV), and the loader in
``crates/stochology-cli/src/loaders.rs`` plus its schema entry must exist before the
digest is added to the manifest.

Usage
-----
    # Report the digest of a fetched file for review; nothing is committed.
    python3 scripts/anac_fetch.py --pin

    # Verify a candidate file against a digest already pinned in this script.
    python3 scripts/anac_fetch.py --out data/anac_transporte.csv

The public portal rejects requests without a browser User-Agent (a bare ``curl`` receives
HTTP 403), so the request below sends an explicit one. If the portal refuses the request
or answers with HTML, the script fails loudly instead of writing a file: an unverified
table is not evidence.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import sys
import urllib.error
import urllib.request

# Public distribution of the ANAC statistical series (open data).
SOURCE_URL = (
    "https://www.gov.br/anac/pt-br/assuntos/dados-e-estatisticas/"
    "percentual-de-voos-cancelados-e-atrasados"
)
USER_AGENT = (
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/122.0 Safari/537.36"
)
TIMEOUT_SECONDS = 60

# Required columns of the series the laboratory would consume.
REQUIRED_COLUMNS = (
    "empresa",
    "aeroporto_origem",
    "aeroporto_destino",
    "partida_prevista",
    "partida_real",
    "situacao_voo",
)

# Digest of the approved artifact. Stays ``None`` until a human reviewer seals the file;
# a run with ``--out`` and no pinned digest refuses to write.
EXPECTED_SHA256: str | None = None


def fetch(url: str = SOURCE_URL) -> bytes:
    """Download ``url`` with a browser User-Agent, or raise ``SystemExit``."""
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
            payload = response.read()
    except urllib.error.HTTPError as exc:
        raise SystemExit(
            f"ERROR: {url} answered HTTP {exc.code}. The portal gates automated access; "
            "download the series manually and pass it with --check PATH."
        ) from exc
    except urllib.error.URLError as exc:
        raise SystemExit(f"ERROR: cannot reach {url}: {exc.reason}") from exc
    if payload.lstrip().startswith(b"<"):
        raise SystemExit(
            "ERROR: the response is an HTML page, not a CSV series. "
            "Download the file manually and pass it with --check PATH."
        )
    return payload


def validate(payload: bytes) -> int:
    """Fail unless the payload is a CSV table carrying every required column."""
    text = payload.decode("utf-8-sig", errors="strict")
    reader = csv.DictReader(io.StringIO(text))
    header = [name.strip() for name in (reader.fieldnames or [])]
    missing = [column for column in REQUIRED_COLUMNS if column not in header]
    if missing:
        raise SystemExit(
            f"ERROR: the series is missing required columns {missing}; found {header}"
        )
    rows = sum(1 for _ in reader)
    if rows == 0:
        raise SystemExit("ERROR: the series has a header and no rows")
    return rows


def digest(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--url", default=SOURCE_URL, help="override the distribution URL")
    parser.add_argument("--check", metavar="PATH", help="validate and hash a local file")
    parser.add_argument("--out", metavar="PATH", help="write the verified bytes to PATH")
    parser.add_argument(
        "--pin",
        action="store_true",
        help="print the digest of the fetched payload for review; writes nothing",
    )
    args = parser.parse_args(argv)

    payload = open(args.check, "rb").read() if args.check else fetch(args.url)
    rows = validate(payload)
    sha = digest(payload)
    print(f"rows = {rows}")
    print(f"sha256 = {sha}")

    if args.pin:
        print("Pinned digest printed for review. Nothing was written.")
        return 0
    if args.out:
        if EXPECTED_SHA256 is None:
            raise SystemExit(
                "ERROR: no approved digest is pinned in this script yet. Seal the artifact "
                "first (human review, data/SHA256SUMS, data/SOURCES.md, loaders.rs), then set "
                "EXPECTED_SHA256 and re-run."
            )
        if sha != EXPECTED_SHA256:
            raise SystemExit(
                f"ERROR: digest mismatch: expected {EXPECTED_SHA256}, got {sha}. "
                "The upstream series changed; the sealed manifest would be stale."
            )
        with open(args.out, "wb") as handle:
            handle.write(payload)
        print(f"wrote {args.out}")
        return 0
    print("Dry run. Pass --pin, --check or --out.")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
