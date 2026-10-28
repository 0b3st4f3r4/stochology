//! Multi-seed acceptance for simulated syntheses (`stochology_lenses::acceptance`).
//!
//! A claim verified at a single seed can be seed luck. Gate `G8` therefore re-evaluates each simulated synthesis
//! (`triad`, `critical`, `stocks`) over the pre-declared seed set [`ACCEPTANCE_SEEDS`] (`2026..=2045`) and accepts it only if
//! every gated claim passes on at least [`MIN_CLAIM_PASS_RATE`] of the seeds and every identity closes on all of them.
//! The seed set and the pass rate are declared here, before any claim is evaluated, and are not tuned per claim.

/// A named verification check `(name, passed)`.
pub type NamedCheck = (&'static str, bool);

/// Pre-declared acceptance seeds: the canonical `seed = 2026` followed by the next 19 integers.
pub const ACCEPTANCE_SEEDS: [u64; 20] = [
    2026, 2027, 2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036, 2037, 2038, 2039, 2040, 2041,
    2042, 2043, 2044, 2045,
];

/// Minimum fraction of acceptance seeds on which each gated claim must pass (19 of 20).
pub const MIN_CLAIM_PASS_RATE: f64 = 0.95;

/// Per-claim pass rates of a synthesis across a seed set.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiSeedAcceptance {
    /// Seeds evaluated.
    pub seeds: Vec<u64>,
    /// Pass rate of every gated claim, in declaration order.
    pub claim_pass_rates: Vec<(&'static str, f64)>,
    /// True if the identity checks closed on every seed.
    pub identities_closed_on_all_seeds: bool,
    /// True if every claim reaches [`MIN_CLAIM_PASS_RATE`] and identities closed on every seed.
    pub accepted: bool,
}

impl MultiSeedAcceptance {
    /// Evaluates `run(seed) -> (claims, identities_closed)` on every seed and aggregates per-claim pass rates.
    pub fn evaluate<F>(seeds: &[u64], run: F) -> Self
    where
        F: Fn(u64) -> (Vec<NamedCheck>, bool),
    {
        let mut names: Vec<&'static str> = Vec::new();
        let mut passes: Vec<usize> = Vec::new();
        let mut identities_closed_on_all_seeds = true;
        for &seed in seeds {
            let (claims, identities) = run(seed);
            identities_closed_on_all_seeds &= identities;
            for (name, ok) in claims {
                let idx = match names.iter().position(|&n| n == name) {
                    Some(i) => i,
                    None => {
                        names.push(name);
                        passes.push(0);
                        names.len() - 1
                    }
                };
                if ok {
                    passes[idx] += 1;
                }
            }
        }
        let total = seeds.len().max(1) as f64;
        let claim_pass_rates: Vec<(&'static str, f64)> = names
            .into_iter()
            .zip(passes)
            .map(|(n, p)| (n, p as f64 / total))
            .collect();
        let accepted = identities_closed_on_all_seeds
            && claim_pass_rates
                .iter()
                .all(|&(_, rate)| rate >= MIN_CLAIM_PASS_RATE);
        Self {
            seeds: seeds.to_vec(),
            claim_pass_rates,
            identities_closed_on_all_seeds,
            accepted,
        }
    }

    /// Claims whose pass rate is below [`MIN_CLAIM_PASS_RATE`].
    pub fn failing_claims(&self) -> Vec<(&'static str, f64)> {
        self.claim_pass_rates
            .iter()
            .copied()
            .filter(|&(_, rate)| rate < MIN_CLAIM_PASS_RATE)
            .collect()
    }
}

/// Path of the sealed acceptance registry, relative to the workspace root; its SHA-256 is listed in `data/SHA256SUMS`.
pub const ACCEPTANCE_REGISTRY_PATH: &str = "data/preregistration/acceptance_registry.txt";

/// Canonical (non-comment) lines of the acceptance registry, regenerated from the code: the seed set, the pass rate, and every
/// gated and discarded claim of the three simulated syntheses. Claim names encode their thresholds, so changing a threshold,
/// adding or removing a claim, or moving a claim between `gated` and `discarded` changes these lines and breaks the seal.
pub fn canonical_registry_lines() -> Vec<String> {
    let seed = ACCEPTANCE_SEEDS[0];
    let critical =
        crate::critical::CriticalStochasticRationalitySynthesis::evaluate_canonical(seed);
    let stocks = crate::stocks::StocharyConjecturalCyberneticSynthesis::evaluate_canonical(seed);
    let triad = crate::triad::ComplexFractalToroidalTechniumSynthesis::evaluate_canonical(seed);
    let seeds: Vec<String> = ACCEPTANCE_SEEDS.iter().map(|s| s.to_string()).collect();
    let mut lines = vec![
        "acceptance_registry v1".to_string(),
        format!("seeds {}", seeds.join(" ")),
        format!("min_claim_pass_rate {MIN_CLAIM_PASS_RATE:.2}"),
    ];
    let groups: [(&str, Vec<NamedCheck>, Vec<NamedCheck>); 3] = [
        (
            "critical",
            critical.claim_checks(),
            critical.discarded_claim_checks(),
        ),
        (
            "stocks",
            stocks.claim_checks(),
            stocks.discarded_claim_checks(),
        ),
        (
            "triad",
            triad.claim_checks(),
            triad.discarded_claim_checks(),
        ),
    ];
    for (synthesis, gated, discarded) in groups {
        lines.extend(
            gated
                .iter()
                .map(|(name, _)| format!("gated {synthesis} {name}")),
        );
        lines.extend(
            discarded
                .iter()
                .map(|(name, _)| format!("discarded {synthesis} {name}")),
        );
    }
    lines
}

/// Checks a registry file's content against [`canonical_registry_lines`] (comment lines starting with `#` and blank lines are
/// ignored for the comparison but still covered by the file's SHA-256). Returns the first mismatch, if any.
pub fn verify_registry_content(content: &str) -> Result<(), String> {
    let found: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    let expected = canonical_registry_lines();
    for (i, exp) in expected.iter().enumerate() {
        match found.get(i) {
            Some(got) if got == exp => {}
            Some(got) => {
                return Err(format!(
                    "registry line {} is '{got}', code expects '{exp}'",
                    i + 1
                ))
            }
            None => return Err(format!("registry is missing '{exp}'")),
        }
    }
    if found.len() > expected.len() {
        return Err(format!(
            "registry has unexpected line '{}'",
            found[expected.len()]
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_rates_and_acceptance() {
        let acc = MultiSeedAcceptance::evaluate(&[1, 2, 3, 4], |seed| {
            (vec![("always", true), ("even_only", seed % 2 == 0)], true)
        });
        assert_eq!(
            acc.claim_pass_rates,
            vec![("always", 1.0), ("even_only", 0.5)]
        );
        assert!(!acc.accepted);
        assert_eq!(acc.failing_claims(), vec![("even_only", 0.5)]);
        let ok = MultiSeedAcceptance::evaluate(&[1, 2], |_| (vec![("always", true)], true));
        assert!(ok.accepted);
    }
}
