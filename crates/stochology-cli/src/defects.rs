//! Registry of the 21 Negative Defect Proofs (`ALL_DEFECTS`, `3 + 4 + 2 + 3 + 2 + 7 = 21`)
//! across the Six Mnemonic Pillars of the CAPIBA Protocol (§II.B).

/// The Six Mnemonic Pillars of the CAPIBA Protocol (`C-A-P-I-B-A`, §II.B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapibaPillar {
    /// **C — Calibrated** (`cromwell`, `brier_dominance`, `concept_drift`)
    Calibrated,
    /// **A — Anytime-valid** (`moulton`, `peeking`, `var_underreport`, `phi5`)
    AnytimeValid,
    /// **P — Performative-aware** (`performativity`, `replace_collapse`)
    PerformativeAware,
    /// **I — Invariant** (`lee_point_estimate`, `sniedovich`, `machine`)
    Invariant,
    /// **B — Bitemporal** (`clock`, `vault`)
    Bitemporal,
    /// **A — Auditing** (`chouldechova`, `analog_wall`, `instrumental_reason`, `stochary_cybernetics`, `triadic_technium`, `strategic_accommodation`, `coupled_society`)
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

/// Canonical registry of all 21 negative defect proofs (`3 + 4 + 2 + 3 + 2 + 7 = 21`).
pub const ALL_DEFECTS: [DefectDescriptor; 21] = [
    DefectDescriptor {
        name: "cromwell",
        pillar: CapibaPillar::Calibrated,
        gate: "G6",
        summary: "Dogmatic 0% or 100% probability violating Cromwell's Rule",
    },
    DefectDescriptor {
        name: "phi5",
        pillar: CapibaPillar::AnytimeValid,
        gate: "G5",
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
        gate: "G7",
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
];

/// Returns the list of all 21 defect names in canonical verification order.
pub fn all_defect_names() -> [&'static str; 21] {
    ALL_DEFECTS.map(|d| d.name)
}
