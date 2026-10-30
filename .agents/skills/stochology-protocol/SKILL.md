---
name: stochology-protocol
description: Model-agnostic end-to-end engineering, mathematical modeling, cryptographic auditing, and textbook synchronization workflow for the Stochology Protocol. Use when adding or auditing Rust primitives, Epistemological Cards (cards/*.yaml), laboratories, gates, or textbook chapters (book-src/).
---

# Stochology Protocol Skill (`stochology-protocol`)

Use this skill when extending, auditing, or documenting the **Stochology Protocol**.

## 1. Multi-Agent Workflow (Filter 4 Cross-Authorship)
1. **Model (`stochology-modeler`, `branch` mode):**
   Implement pure-Rust primitives in `stochology-core`, `stochology-lenses`, `stochology-siga`, or `stochology-governance` with `SplitMix64` (`DEFAULT_SEED = 2026`).
2. **Adversarial Audit (`stochology-adversary`, `inherit` mode — VSM $S3^*$):**
   Run `cargo run --release --bin gates` (must exit `0`) and all 27 negative defect proofs `cargo run --release --bin gates -- --defect <name>` (each must exit `1`), checking `IdentityClosureAudit` in Gate `G8`.
3. **Documentary Synchronization (`stochology-scribe`, `inherit` mode):**
   Update `docs/PROTOCOL.md`, `book-src/appendix-protocol.md`, and the matching chapters in `book-src/`, regenerating quoted `labs`/`gates` output and labelling every number's provenance (`AGENTS.md` §II.C).
4. **Derived-Text Audit (`stochology-adversary`):**
   Before publishing any text derived from Stochology (summary, post, slides, notebook remix), audit each claim's provenance class — measured, simulated, assumed, or literature/informative — against the code and current output.

## 2. Cryptographic Integrity ($\Phi_3$)
- Do not overwrite `data/SHA256SUMS`, `cards/*.yaml` pre-registration hashes, or `crates/stochology-governance/src/vault.rs` (`TemporalVault`) without human review and recomputing `card.compute_preregistration_hash()`.

## 3. Verification Gate
Run the consolidation script and require exit code `0`:
```bash
scripts/verify.sh
```
