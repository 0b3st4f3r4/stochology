#!/usr/bin/env bash
# Invariant Phi_3 Cryptographic Pre-Registration & Vault Protection Hook
# Requires explicit human review (force_ask) before modifying:
#   1. data/SHA256SUMS
#   2. cards/*.yaml
#   3. crates/stochology-governance/src/vault.rs (TemporalVault)
set -euo pipefail

PAYLOAD="$(cat || true)"

if printf '%s' "$PAYLOAD" | grep -Eq '(data/SHA256SUMS|cards/[^"]*\.yaml|crates/stochology-governance/src/vault\.rs)'; then
    printf '{"decision":"force_ask","reason":"Invariant Phi_3 Cryptographic Gate: Modifying data/SHA256SUMS, cards/*.yaml, or TemporalVault (vault.rs) requires explicit human inline-diff approval to prevent silent p-hacking or seal tampering."}\n'
    exit 0
fi

printf '{"decision":"allow"}\n'

