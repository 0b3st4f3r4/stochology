//! Ontological Layers ($L_1, L_2, L_3$), DPI Operational Pipeline ($D_1..D_4$),
//! Uncertainty Typing (Stirling-Walker Matrix), and Cromwell's Rule (`PROHIBITION_4`).

/// Ontological Layer of the data- and decision-generating cycle ($L_1 \to L_2 \to L_3 \to L_1$).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    /// $L_1$ — World: Physical, biological, or socioeconomic reality generating the data.
    WorldL1,
    /// $L_2$ — Model: Formal, reduced, computationally simulable mathematical representation.
    ModelL2,
    /// $L_3$ — Agent: Human decision-maker, committee, or algorithmic controller acting under constraints.
    AgentL3,
}

impl Layer {
    pub const fn code(self) -> &'static str {
        match self {
            Self::WorldL1 => "L1",
            Self::ModelL2 => "L2",
            Self::AgentL3 => "L3",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "L1" => Some(Self::WorldL1),
            "L2" => Some(Self::ModelL2),
            "L3" => Some(Self::AgentL3),
            _ => None,
        }
    }
}

/// Four-stage Data Processing Inequality (DPI) operational pipeline view ($D_1 \to D_2 \to D_3 \to D_4$),
/// explicitly mapped onto the canonical three-layer ontology ($L_1, L_2, L_3$) and its three bridges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DpiOperationalStage {
    /// $D_1$ — World & Sensor/Survey Collection ($L_1$ and Bridge $L_1 \to L_2$ selection filter).
    D1DataCollection,
    /// $D_2$ — Feature Engineering & Sufficient Compression (Bridge $L_1 \to L_2$ / $L_2$ representation).
    D2FeatureEngineering,
    /// $D_3$ — Formal Mathematical/Stochastic Model ($L_2$).
    D3ModelRepresentation,
    /// $D_4$ — Human/Controller Decision & Performative Actuation ($L_3$ and Bridges $L_2 \to L_3$, $L_3 \to L_1$).
    D4HumanDecisionActuation,
}

impl DpiOperationalStage {
    pub const fn code(self) -> &'static str {
        match self {
            Self::D1DataCollection => "D1",
            Self::D2FeatureEngineering => "D2",
            Self::D3ModelRepresentation => "D3",
            Self::D4HumanDecisionActuation => "D4",
        }
    }

    /// Maps a canonical `(Layer, Option<bridge_str>)` pair onto its DPI operational stage ($D_1..D_4$).
    pub fn from_ontological(layer: Layer, bridge: Option<&str>) -> Self {
        match (layer, bridge.map(str::trim)) {
            (_, Some("L2->L3")) | (_, Some("L3->L1")) | (Layer::AgentL3, _) => {
                Self::D4HumanDecisionActuation
            }
            (Layer::ModelL2, Some("L1->L2")) => Self::D2FeatureEngineering,
            (Layer::ModelL2, _) => Self::D3ModelRepresentation,
            (Layer::WorldL1, _) => Self::D1DataCollection,
        }
    }
}

/// Uncertainty Typing in the Stirling-Walker Matrix (§1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UncertaintyClass {
    /// Aleatory ($L_1$) — Irreducible physical or stochastic variability.
    Aleatory,
    /// Parametric Epistemic ($L_2$) — Reducible ignorance about finite parameters $\theta$.
    EpistemicParameter,
    /// Structural Epistemic ($L_2$) — Doubt about functional form $\eta(x, \theta)$; requires Kennedy-O'Hagan GP $\delta(x)$.
    EpistemicSpecification,
    /// Knightian / Deep ($L_3$) — Unbounded possibility space or strategic environment; requires Info-Gap + Sniedovich safeguard.
    Knightian,
}

impl UncertaintyClass {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Aleatory => "aleatory",
            Self::EpistemicParameter => "epistemic_parameter",
            Self::EpistemicSpecification => "epistemic_specification",
            Self::Knightian => "knightian",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "aleatory" => Some(Self::Aleatory),
            "epistemic_parameter" => Some(Self::EpistemicParameter),
            "epistemic_specification" => Some(Self::EpistemicSpecification),
            "knightian" => Some(Self::Knightian),
            _ => None,
        }
    }
}

/// Laplace's Rule of Succession for small samples or rare events ($s$ successes in $n$ binary trials):
/// $$\hat{p} = \frac{s + 1}{n + 2}$$
#[inline]
pub fn laplace_succession(successes: usize, trials: usize) -> f64 {
    assert!(successes <= trials, "successes cannot exceed trials");
    ((successes as f64) + 1.0) / ((trials as f64) + 2.0)
}

/// Cromwell's Rule (Prohibition 4): clamps probability $p$ to $[\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$.
#[inline]
pub fn apply_cromwell_floor(p: f64, epsilon_floor: f64) -> f64 {
    assert!(
        epsilon_floor > 0.0 && epsilon_floor < 0.5,
        "epsilon_floor must be in (0, 0.5)"
    );
    p.clamp(epsilon_floor, 1.0 - epsilon_floor)
}

/// Verifies whether probability $p$ strictly satisfies Cromwell's Rule for a given $\varepsilon_{\text{floor}} > 0$.
#[inline]
pub fn respects_cromwell(p: f64, epsilon_floor: f64) -> bool {
    epsilon_floor > 0.0
        && epsilon_floor < 0.5
        && p >= epsilon_floor
        && p <= (1.0 - epsilon_floor)
        && p > 0.0
        && p < 1.0
}
