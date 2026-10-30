//! Registry of the 27 Negative Defect Proofs (`ALL_DEFECTS`, `4 + 5 + 3 + 4 + 3 + 8 = 27`)
//! across the Six Mnemonic Pillars of the CAPIBA Protocol (§II.B). Every count of negative proofs or
//! pathways elsewhere in the workspace derives from `DEFECT_COUNT`.

/// The Six Mnemonic Pillars of the CAPIBA Protocol (`C-A-P-I-B-A`, §II.B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapibaPillar {
    /// **C — Calibrated** (`cromwell`, `brier_dominance`, `concept_drift`, `crps_miscalibration`)
    Calibrated,
    /// **A — Anytime-valid** (`moulton`, `peeking`, `var_underreport`, `phi5`, `multiplicity`)
    AnytimeValid,
    /// **P — Performative-aware** (`performativity`, `replace_collapse`, `understated_epsilon`)
    PerformativeAware,
    /// **I — Invariant** (`lee_point_estimate`, `sniedovich`, `machine`, `anchor_extrapolation`)
    Invariant,
    /// **B — Bitemporal** (`clock`, `vault`, `vintage_leak`)
    Bitemporal,
    /// **A — Auditing** (`chouldechova`, `analog_wall`, `instrumental_reason`, `stochary_cybernetics`, `triadic_technium`, `strategic_accommodation`, `coupled_society`, `subgroup_miscalibration`)
    Auditing,
}

/// Metadata descriptor for a negative defect proof (`gates -- --defect <name>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefectDescriptor {
    pub name: &'static str,
    pub pillar: CapibaPillar,
    pub gate: &'static str,
    pub summary: &'static str,
}

/// Number of negative defect proofs (one competency pathway each).
pub const DEFECT_COUNT: usize = 27;

/// Canonical registry of all 27 negative defect proofs (`4 + 5 + 3 + 4 + 3 + 8 = 27`).
pub const ALL_DEFECTS: [DefectDescriptor; DEFECT_COUNT] = [
    DefectDescriptor {
        name: "cromwell",
        pillar: CapibaPillar::Calibrated,
        gate: "G6",
        summary: "Dogmatic 0% or 100% probability violating Cromwell's Rule",
    },
    DefectDescriptor {
        name: "phi5",
        pillar: CapibaPillar::AnytimeValid,
        gate: "G2/G5",
        summary: "Coarse annual aggregate action answering a daily/shift decision unit",
    },
    DefectDescriptor {
        name: "moulton",
        pillar: CapibaPillar::AnytimeValid,
        gate: "G2",
        summary: "Nominal pseudoreplication ignoring Moulton-Campbell cluster design effect",
    },
    DefectDescriptor {
        name: "performativity",
        pillar: CapibaPillar::PerformativeAware,
        gate: "G6",
        summary: "Unstable RRM contraction radius eps*beta/gamma >= 1 under performative feedback",
    },
    DefectDescriptor {
        name: "sniedovich",
        pillar: CapibaPillar::Invariant,
        gate: "G6",
        summary: "Fragile local Info-Gap optimum collapsing in Sniedovich No-Man's Land",
    },
    DefectDescriptor {
        name: "clock",
        pillar: CapibaPillar::Bitemporal,
        gate: "G7",
        summary: "Bitemporal clock disabled, leaking future canary column into past features",
    },
    DefectDescriptor {
        name: "vault",
        pillar: CapibaPillar::Bitemporal,
        gate: "G7",
        summary: "Illegal second opening of single-opening SHA-256 Temporal Vault",
    },
    DefectDescriptor {
        name: "concept_drift",
        pillar: CapibaPillar::Calibrated,
        gate: "G6/G7",
        summary: "Collapsed CORP resolution tripping Concept Drift circuit breaker",
    },
    DefectDescriptor {
        name: "chouldechova",
        pillar: CapibaPillar::Auditing,
        gate: "G4",
        summary: "False claim of simultaneous PPV, FNR, and FPR equality under unequal base rates",
    },
    DefectDescriptor {
        name: "machine",
        pillar: CapibaPillar::Invariant,
        gate: "G8",
        summary: "Lossy representation violating DPI sufficiency and silently dropping spurious atoms",
    },
    DefectDescriptor {
        name: "analog_wall",
        pillar: CapibaPillar::Auditing,
        gate: "G8",
        summary: "Unshielded analog crossbar collapsing at Ferran Alia threshold cliff without NAT/HP-INV",
    },
    DefectDescriptor {
        name: "instrumental_reason",
        pillar: CapibaPillar::Auditing,
        gate: "G8",
        summary: "Positivist tail erasure, status-quo reification, and Stochastic Parrot intervention failure",
    },
    DefectDescriptor {
        name: "stochary_cybernetics",
        pillar: CapibaPillar::Auditing,
        gate: "G8",
        summary: "Unbuffered bullwhip, full-exposure non-ergodic ruin, and ungated VSM/PLAT governance",
    },
    DefectDescriptor {
        name: "peeking",
        pillar: CapibaPillar::AnytimeValid,
        gate: "G6",
        summary: "Optional stopping z-test peeking inflating Type-I false-alarm rate above alpha",
    },
    DefectDescriptor {
        name: "brier_dominance",
        pillar: CapibaPillar::Calibrated,
        gate: "G6",
        summary: "Lower Brier score failing Murphy diagram uniform dominance across decision thresholds",
    },
    DefectDescriptor {
        name: "var_underreport",
        pillar: CapibaPillar::AnytimeValid,
        gate: "G6",
        summary: "Underreported VaR99 forecast rejected by sequential GREE e-backtest martingale",
    },
    DefectDescriptor {
        name: "lee_point_estimate",
        pillar: CapibaPillar::Invariant,
        gate: "G3",
        summary: "Naive MNAR point estimate spanning zero under fractional Lee trimming bounds",
    },
    DefectDescriptor {
        name: "replace_collapse",
        pillar: CapibaPillar::PerformativeAware,
        gate: "G8",
        summary: "Synthetic data replacement recursion breaching Gerstgrasser Basel pi^2/6 ceiling",
    },
    DefectDescriptor {
        name: "triadic_technium",
        pillar: CapibaPillar::Auditing,
        gate: "G8",
        summary: "Naive real dictionary, uncompensated PCM drift, OLS under heavy tails, and RRM gap",
    },
    DefectDescriptor {
        name: "strategic_accommodation",
        pillar: CapibaPillar::Auditing,
        gate: "G8",
        summary: "Diameter-limited perceptrons, Rung-1 SHAP, single imputation, rank-reversed AHP, and L2 Kalman",
    },
    DefectDescriptor {
        name: "coupled_society",
        pillar: CapibaPillar::Auditing,
        gate: "G8",
        summary: "Event-time MNAR RRM procyclical liquidation, Stiglitz-Weiss credit rationing, and proportional-rebate VCG commons collapse",
    },
    DefectDescriptor {
        name: "crps_miscalibration",
        pillar: CapibaPillar::Calibrated,
        gate: "G6",
        summary: "Sharp-but-biased ensemble beating climatology on a lone CRPS while CORP miscalibration exceeds half its discrimination",
    },
    DefectDescriptor {
        name: "multiplicity",
        pillar: CapibaPillar::AnytimeValid,
        gate: "G6",
        summary: "Peeking each of K e-processes at 1/alpha with K = 1 per hypothesis, breaking FDR control that e-BH keeps",
    },
    DefectDescriptor {
        name: "understated_epsilon",
        pillar: CapibaPillar::PerformativeAware,
        gate: "G6",
        summary: "Passive agent declared for an action whose randomized arms measure Wasserstein sensitivity epsilon > 0",
    },
    DefectDescriptor {
        name: "anchor_extrapolation",
        pillar: CapibaPillar::Invariant,
        gate: "G6",
        summary: "Invariant coefficients claimed where anchor regression departs from OLS under hidden confounding",
    },
    DefectDescriptor {
        name: "vintage_leak",
        pillar: CapibaPillar::Bitemporal,
        gate: "G7",
        summary: "Backtest fed latest-vintage revisions at past decision dates, tripping the bitemporal clock",
    },
    DefectDescriptor {
        name: "subgroup_miscalibration",
        pillar: CapibaPillar::Auditing,
        gate: "G4",
        summary: "Globally calibrated predictor hiding subgroup miscalibration that the multicalibration audit certifies",
    },
];

/// Returns the list of all defect names in canonical verification order.
pub fn all_defect_names() -> [&'static str; DEFECT_COUNT] {
    ALL_DEFECTS.map(|d| d.name)
}

/// The six pillars in mnemonic order `C-A-P-I-B-A`.
pub const PILLAR_ORDER: [CapibaPillar; 6] = [
    CapibaPillar::Calibrated,
    CapibaPillar::AnytimeValid,
    CapibaPillar::PerformativeAware,
    CapibaPillar::Invariant,
    CapibaPillar::Bitemporal,
    CapibaPillar::Auditing,
];

/// The pillar partition of the registry rendered as `"a + b + c + d + e + f = n"` (the form quoted in
/// `AGENTS.md`, `docs/PROTOCOL.md`, and the book).
pub fn pillar_partition() -> String {
    let counts: Vec<String> = PILLAR_ORDER
        .iter()
        .map(|&p| {
            ALL_DEFECTS
                .iter()
                .filter(|d| d.pillar == p)
                .count()
                .to_string()
        })
        .collect();
    format!("{} = {}", counts.join(" + "), ALL_DEFECTS.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capiba_pillar_defect_partition_matches_4_5_3_4_3_8_contract() {
        let count_for =
            |pillar: CapibaPillar| ALL_DEFECTS.iter().filter(|d| d.pillar == pillar).count();
        assert_eq!(count_for(CapibaPillar::Calibrated), 4);
        assert_eq!(count_for(CapibaPillar::AnytimeValid), 5);
        assert_eq!(count_for(CapibaPillar::PerformativeAware), 3);
        assert_eq!(count_for(CapibaPillar::Invariant), 4);
        assert_eq!(count_for(CapibaPillar::Bitemporal), 3);
        assert_eq!(count_for(CapibaPillar::Auditing), 8);
        assert_eq!(ALL_DEFECTS.len(), DEFECT_COUNT);
    }

    /// The pillar partition rendered as `"a + b + ... = n"`, in pillar order.
    #[test]
    fn pillar_partition_string_is_derived_from_the_registry() {
        assert_eq!(pillar_partition(), "4 + 5 + 3 + 4 + 3 + 8 = 27");
    }
}
