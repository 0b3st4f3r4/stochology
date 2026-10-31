#!/usr/bin/env bash
# Cryptographic Dataset Manifest & TemporalVault Protection Hook
set -euo pipefail

PAYLOAD="$(cat || true)"

if printf '%s' "$PAYLOAD" | grep -Eq '(data/SHA256SUMS|crates/stochology/src/governance\.rs)'; then
    printf '{"decision":"force_ask","reason":"Cryptographic Seal Gate: Modifying data/SHA256SUMS or TemporalVault in crates/stochology/src/governance.rs requires explicit human inline-diff approval."}\n'
    exit 0
fi

printf '{"decision":"allow"}\n'
