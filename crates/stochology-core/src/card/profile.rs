//! Protocol Profile (`ProtocolProfile`): the pre-registered "clay" an application shapes around the
//! fixed kernel of the CAPIBA Protocol.
//!
//! The kernel is the set of principles — the 4 Prohibitions and the 5 Boundary Invariants
//! ($\Phi_1$–$\Phi_5$) — and their mechanisms (declared seed, SHA-256 pre-registration, attempt counter
//! $K \ge 1$, findings conservation, Cromwell's Rule, the CORP-Murphy identity, cross-authorship, and a
//! manifest-sealed dataset). A profile never removes a principle: it calibrates the numbers and the
//! vocabulary each principle is checked with, and it may waive a small set of *instruments* whose
//! applicability depends on the domain, always with a written reason that enters the pre-registration
//! seal. Every profile parameter is an `Assumed:` input in the sense of `AGENTS.md` §II.C.
//!
//! [`ProtocolProfile::canonical`] reproduces, value for value, the thresholds the repository applied
//! before profiles existed, so the four canonical cards keep their pre-registration hashes.

use std::collections::BTreeMap;
use std::fmt;

use crate::layers::UncertaintyClass;
use crate::rng::DEFAULT_SEED;
use crate::sha256::sha256_hex;

use super::model::EpistemologicalCard;
use super::yaml::flatten_yaml;

/// The nine principles of the kernel; every rule of [`super::CardSpecification`] guards one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KernelPrinciple {
    Prohibition1,
    Prohibition2,
    Prohibition3,
    Prohibition4,
    Phi1,
    Phi2,
    Phi3,
    Phi4,
    Phi5,
}

impl KernelPrinciple {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Prohibition1 => "PROHIBITION_1",
            Self::Prohibition2 => "PROHIBITION_2",
            Self::Prohibition3 => "PROHIBITION_3",
            Self::Prohibition4 => "PROHIBITION_4",
            Self::Phi1 => "PHI1",
            Self::Phi2 => "PHI2",
            Self::Phi3 => "PHI3",
            Self::Phi4 => "PHI4",
            Self::Phi5 => "PHI5",
        }
    }
}

/// An instrument a profile may waive, the principle it serves, and the condition under which the
/// waiver is admissible. Every gate code absent from [`WAIVABLE_INSTRUMENTS`] belongs to the kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaivableInstrument {
    pub gate_code: &'static str,
    pub principle: KernelPrinciple,
    pub admissible_when: &'static str,
}

/// Instruments whose applicability depends on the domain. The principle each one serves stays binding:
/// $\Phi_4$ still demands scope and a numeric exit clause, Knightian cards still demand Sniedovich's
/// floor, and performative agents still demand the RRM contraction.
pub const WAIVABLE_INSTRUMENTS: &[WaivableInstrument] = &[
    WaivableInstrument {
        gate_code: "PHI4_CHOULDECHOVA",
        principle: KernelPrinciple::Phi4,
        admissible_when: "the decision does not allocate an action across groups with distinct base rates",
    },
    WaivableInstrument {
        gate_code: "INFO_GAP_SNIEDOVICH",
        principle: KernelPrinciple::Phi4,
        admissible_when: "the card's uncertainty class is not 'knightian'",
    },
    WaivableInstrument {
        gate_code: "PROHIBITION_3_RRM_CONTRACTION",
        principle: KernelPrinciple::Prohibition3,
        admissible_when: "the card declares agent_type 'passive' (no Decision -> World feedback)",
    },
];

/// A pre-registered waiver of one instrument in [`WAIVABLE_INSTRUMENTS`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentWaiver {
    pub gate_code: String,
    pub reason: String,
}

/// Vocabulary used by $\Phi_5$ to recognise a fine operational unit answered by a coarse aggregate.
///
/// The canonical profile leaves it unset and delegates to [`crate::invariants::verify_resolution_phi5`];
/// an application in another language or domain declares its own terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionVocabulary {
    pub fine_unit_terms: Vec<String>,
    pub coarse_horizon_terms: Vec<String>,
    pub coarse_aggregation_terms: Vec<String>,
    pub coarse_phrases: Vec<String>,
}

impl ResolutionVocabulary {
    /// Same rule as [`crate::invariants::verify_resolution_phi5`] over this vocabulary (Unicode-lowercased).
    pub fn verify_resolution(&self, independent_unit: &str, altered_action: &str) -> bool {
        let unit = independent_unit.trim().to_lowercase();
        let action = altered_action.trim().to_lowercase();
        if unit.is_empty() || action.is_empty() {
            return false;
        }
        let any_in = |text: &str, terms: &[String]| terms.iter().any(|t| text.contains(t.as_str()));
        let fine_unit = any_in(&unit, &self.fine_unit_terms);
        let coarse_aggregate_action = (any_in(&action, &self.coarse_horizon_terms)
            && any_in(&action, &self.coarse_aggregation_terms))
            || any_in(&action, &self.coarse_phrases);
        !(fine_unit && coarse_aggregate_action)
    }
}

/// Nonlinear loss $f$ applied to the value column when a profile maps a new dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JensenLoss {
    /// $f(x) = x^2$
    Square,
    /// $f(x) = e^{x}$
    Exp,
    /// $f(x) = \max(x, 0)$
    Hinge,
}

impl JensenLoss {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Square => "square",
            Self::Exp => "exp",
            Self::Hinge => "hinge",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "square" => Some(Self::Square),
            "exp" => Some(Self::Exp),
            "hinge" => Some(Self::Hinge),
            _ => None,
        }
    }

    pub fn apply(self, x: f64) -> f64 {
        match self {
            Self::Square => x * x,
            Self::Exp => x.exp(),
            Self::Hinge => x.max(0.0),
        }
    }
}

/// Column mapping that lets the severe test read a dataset outside the canonical four.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetColumns {
    pub cluster: String,
    pub forecast: String,
    pub outcome: String,
    pub value: String,
    pub jensen_loss: JensenLoss,
}

/// Replication counts and Funnel/SIGA thresholds used by the empirical severe test.
#[derive(Debug, Clone, PartialEq)]
pub struct SevereTestParameters {
    pub permutation_replications: usize,
    pub sbc_replications: usize,
    pub sbc_max_rank: usize,
    pub ville_blocks: usize,
    pub decision_volume_units: f64,
    pub total_modeling_cost: f64,
    pub minimum_required_n_eff: f64,
    pub sbc_chi2_critical: f64,
    pub null_p_threshold: f64,
}

/// The pre-registered Protocol Profile of one application (see the module documentation).
#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolProfile {
    pub name: String,
    pub rationale: String,
    /// $\Phi_1$: the seed every card under this profile must declare.
    pub seed: u64,
    /// Prohibition 4: minimum $1/\alpha$ of the e-process ($\alpha$ is its inverse).
    pub e_process_min_inverse_alpha: f64,
    pub p_star_tolerance: f64,
    pub murphy_corp_tolerance: f64,
    pub chouldechova_ratio_tolerance: f64,
    pub min_dominant_cost_share: f64,
    pub forbidden_decision_maker_tokens: Vec<String>,
    pub lone_mean_terms: Vec<String>,
    pub resolution: Option<ResolutionVocabulary>,
    /// $\Phi_3$: directory the dataset must live in, and the SHA-256 manifest that seals it.
    pub dataset_root: String,
    pub dataset_manifest: String,
    pub dataset_columns: Option<DatasetColumns>,
    pub severe_test: SevereTestParameters,
    pub waivers: Vec<InstrumentWaiver>,
}

/// Typed error returned when a `protocol_profile:` section cannot be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileParseError {
    MissingField(String),
    UnknownField(String),
    InvalidValue { field: String, detail: String },
}

impl fmt::Display for ProfileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(k) => write!(f, "protocol_profile is missing mandatory field '{k}'"),
            Self::UnknownField(k) => write!(f, "protocol_profile has unknown field '{k}'"),
            Self::InvalidValue { field, detail } => {
                write!(f, "invalid value in protocol_profile.{field}: {detail}")
            }
        }
    }
}

impl std::error::Error for ProfileParseError {}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

impl ProtocolProfile {
    /// The canonical profile: exactly the thresholds hard-coded before profiles existed.
    pub fn canonical() -> Self {
        Self {
            name: "canonical".into(),
            rationale: "CAPIBA canonical defaults applied to the repository's own cards".into(),
            seed: DEFAULT_SEED,
            e_process_min_inverse_alpha: 20.0,
            p_star_tolerance: 1e-3,
            murphy_corp_tolerance: 1e-4,
            chouldechova_ratio_tolerance: 1e-3,
            min_dominant_cost_share: 0.50,
            forbidden_decision_maker_tokens: strings(&[
                "system",
                "team",
                "generic",
                "analyst",
                "model",
                "ai",
                "algorithm",
            ]),
            lone_mean_terms: strings(&[
                "f(e[x])", "e[f(x)]", "jensen", "median", "q05", "q95", "crps",
            ]),
            resolution: None,
            dataset_root: "data/".into(),
            dataset_manifest: "data/SHA256SUMS".into(),
            dataset_columns: None,
            severe_test: SevereTestParameters {
                permutation_replications: 200,
                sbc_replications: 200,
                sbc_max_rank: 49,
                ville_blocks: 30,
                decision_volume_units: 100.0,
                total_modeling_cost: 500.0,
                minimum_required_n_eff: 50.0,
                sbc_chi2_critical: 9.49,
                null_p_threshold: 0.05,
            },
            waivers: Vec::new(),
        }
    }

    /// True when this profile is value-for-value the canonical one.
    pub fn is_canonical(&self) -> bool {
        *self == Self::canonical()
    }

    /// Significance level $\alpha = 1 / (1/\alpha)$ of the e-process.
    pub fn e_process_alpha(&self) -> f64 {
        1.0 / self.e_process_min_inverse_alpha
    }

    /// Returns the waiver declared for `gate_code`, if any.
    pub fn waiver_for(&self, gate_code: &str) -> Option<&InstrumentWaiver> {
        self.waivers.iter().find(|w| w.gate_code == gate_code)
    }

    /// Deterministic serialization of every profile field, hashed into the card's pre-registration seal.
    pub fn fingerprint(&self) -> String {
        let join = |v: &[String]| v.join(",");
        let resolution = self.resolution.as_ref().map_or_else(
            || "canonical".to_string(),
            |r| {
                format!(
                    "fine={};horizon={};aggregation={};phrases={}",
                    join(&r.fine_unit_terms),
                    join(&r.coarse_horizon_terms),
                    join(&r.coarse_aggregation_terms),
                    join(&r.coarse_phrases)
                )
            },
        );
        let columns = self.dataset_columns.as_ref().map_or_else(
            || "canonical".to_string(),
            |c| {
                format!(
                    "cluster={};forecast={};outcome={};value={};loss={}",
                    c.cluster,
                    c.forecast,
                    c.outcome,
                    c.value,
                    c.jensen_loss.code()
                )
            },
        );
        let st = &self.severe_test;
        let mut waivers: Vec<String> = self
            .waivers
            .iter()
            .map(|w| format!("{}={}", w.gate_code, w.reason.trim()))
            .collect();
        waivers.sort();
        format!(
            "stochology_profile_v1|name={}|rationale={}|seed={}|inv_alpha={}|tol_p_star={}|tol_murphy={}|tol_chouldechova={}|min_cost_share={}|forbidden_dm={}|lone_mean={}|resolution={}|root={}|manifest={}|columns={}|perm={}|sbc={}|sbc_rank={}|ville={}|volume={}|cost={}|min_n_eff={}|chi2={}|null_p={}|waivers={}",
            self.name.trim(),
            self.rationale.trim(),
            self.seed,
            self.e_process_min_inverse_alpha,
            self.p_star_tolerance,
            self.murphy_corp_tolerance,
            self.chouldechova_ratio_tolerance,
            self.min_dominant_cost_share,
            join(&self.forbidden_decision_maker_tokens),
            join(&self.lone_mean_terms),
            resolution,
            self.dataset_root,
            self.dataset_manifest,
            columns,
            st.permutation_replications,
            st.sbc_replications,
            st.sbc_max_rank,
            st.ville_blocks,
            st.decision_volume_units,
            st.total_modeling_cost,
            st.minimum_required_n_eff,
            st.sbc_chi2_critical,
            st.null_p_threshold,
            waivers.join(";"),
        )
    }

    /// Pre-registration payload of `card` under this profile. The canonical profile leaves the
    /// payload of [`EpistemologicalCard::compute_preregistration_payload`] untouched; any other profile
    /// appends the SHA-256 of its [`Self::fingerprint`], so calibrating the clay is itself sealed.
    pub fn preregistration_payload_for(&self, card: &EpistemologicalCard) -> String {
        let base = card.compute_preregistration_payload();
        if self.is_canonical() {
            base
        } else {
            format!(
                "{base}|profile_sha256={}",
                sha256_hex(self.fingerprint().as_bytes())
            )
        }
    }

    /// SHA-256 of [`Self::preregistration_payload_for`].
    pub fn preregistration_hash_for(&self, card: &EpistemologicalCard) -> String {
        sha256_hex(self.preregistration_payload_for(card).as_bytes())
    }

    /// Checks that every waiver names a waivable instrument, carries a reason, and meets its
    /// admissibility condition on `card`. Returns `(gate_code, description)` for each defect.
    pub fn waiver_defects(&self, card: &EpistemologicalCard) -> Vec<(&'static str, String)> {
        self.waivers
            .iter()
            .flat_map(|w| self.waiver_defects_of(card, w))
            .collect()
    }

    /// Defects of a single waiver `w` on `card` (empty when the waiver is admissible).
    pub fn waiver_defects_of(
        &self,
        card: &EpistemologicalCard,
        w: &InstrumentWaiver,
    ) -> Vec<(&'static str, String)> {
        let Some(instrument) = WAIVABLE_INSTRUMENTS
            .iter()
            .find(|i| i.gate_code == w.gate_code)
        else {
            return vec![(
                "PROFILE_KERNEL_WAIVER",
                format!(
                    "'{}' belongs to the kernel and cannot be waived by a profile",
                    w.gate_code
                ),
            )];
        };
        let mut defects = Vec::new();
        if w.reason.trim().is_empty() {
            defects.push((
                "PROFILE_WAIVER_REASON",
                format!("waiver of '{}' must state a reason", w.gate_code),
            ));
        }
        let admissible = match instrument.gate_code {
            "INFO_GAP_SNIEDOVICH" => card.uncertainty != UncertaintyClass::Knightian,
            "PROHIBITION_3_RRM_CONTRACTION" => {
                card.epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .agent_type
                    == "passive"
            }
            _ => true,
        };
        if !admissible {
            defects.push((
                "PROFILE_KERNEL_WAIVER",
                format!(
                    "waiver of '{}' is admissible only when {}",
                    w.gate_code, instrument.admissible_when
                ),
            ));
        }
        defects
    }

    /// Parses the optional `protocol_profile:` section of a card YAML. Absent section gives `Ok(None)`;
    /// every field left out inherits its canonical value, except `name` and `rationale`, which an
    /// application must always state. Unknown keys are rejected so a typo never falls back silently.
    pub fn from_card_yaml(yaml: &str) -> Result<Option<Self>, ProfileParseError> {
        let (map, _) = flatten_yaml(yaml);
        let section: BTreeMap<&str, &str> = map
            .iter()
            .filter_map(|(k, v)| {
                k.strip_prefix("protocol_profile.")
                    .map(|rest| (rest, v.as_str()))
            })
            .collect();
        if section.is_empty() {
            return Ok(None);
        }
        Self::from_section(&section).map(Some)
    }

    fn from_section(section: &BTreeMap<&str, &str>) -> Result<Self, ProfileParseError> {
        let mut p = Self::canonical();
        let mut columns: BTreeMap<&str, &str> = BTreeMap::new();
        let mut resolution: BTreeMap<&str, &str> = BTreeMap::new();
        let list = |v: &str| -> Vec<String> {
            v.split(',')
                .map(|t| t.trim().to_lowercase())
                .filter(|t| !t.is_empty())
                .collect()
        };
        fn num<T: std::str::FromStr>(field: &str, v: &str) -> Result<T, ProfileParseError>
        where
            T::Err: fmt::Display,
        {
            v.trim()
                .parse::<T>()
                .map_err(|e| ProfileParseError::InvalidValue {
                    field: field.to_string(),
                    detail: e.to_string(),
                })
        }
        fn positive(field: &str, v: f64) -> Result<f64, ProfileParseError> {
            if v.is_finite() && v > 0.0 {
                Ok(v)
            } else {
                Err(ProfileParseError::InvalidValue {
                    field: field.to_string(),
                    detail: format!("{v} must be finite and > 0"),
                })
            }
        }

        let mut name = None;
        let mut rationale = None;
        for (&key, &val) in section {
            if let Some(code) = key.strip_prefix("waivers.") {
                p.waivers.push(InstrumentWaiver {
                    gate_code: code.to_string(),
                    reason: val.to_string(),
                });
                continue;
            }
            if let Some(col) = key.strip_prefix("dataset_columns.") {
                columns.insert(col, val);
                continue;
            }
            if let Some(term) = key.strip_prefix("resolution.") {
                resolution.insert(term, val);
                continue;
            }
            let st = &mut p.severe_test;
            match key {
                "name" => name = Some(val.trim().to_string()),
                "rationale" => rationale = Some(val.trim().to_string()),
                "seed" => p.seed = num(key, val)?,
                "e_process_min_inverse_alpha" => {
                    p.e_process_min_inverse_alpha = positive(key, num(key, val)?)?
                }
                "p_star_tolerance" => p.p_star_tolerance = positive(key, num(key, val)?)?,
                "murphy_corp_tolerance" => p.murphy_corp_tolerance = positive(key, num(key, val)?)?,
                "chouldechova_ratio_tolerance" => {
                    p.chouldechova_ratio_tolerance = positive(key, num(key, val)?)?
                }
                "min_dominant_cost_share" => {
                    p.min_dominant_cost_share = positive(key, num(key, val)?)?
                }
                "forbidden_decision_maker_tokens" => p.forbidden_decision_maker_tokens = list(val),
                "lone_mean_terms" => p.lone_mean_terms = list(val),
                "dataset_root" => p.dataset_root = val.trim().to_string(),
                "dataset_manifest" => p.dataset_manifest = val.trim().to_string(),
                "severe_test.permutation_replications" => {
                    st.permutation_replications = num(key, val)?
                }
                "severe_test.sbc_replications" => st.sbc_replications = num(key, val)?,
                "severe_test.sbc_max_rank" => st.sbc_max_rank = num(key, val)?,
                "severe_test.ville_blocks" => st.ville_blocks = num(key, val)?,
                "severe_test.decision_volume_units" => {
                    st.decision_volume_units = positive(key, num(key, val)?)?
                }
                "severe_test.total_modeling_cost" => {
                    st.total_modeling_cost = positive(key, num(key, val)?)?
                }
                "severe_test.minimum_required_n_eff" => {
                    st.minimum_required_n_eff = positive(key, num(key, val)?)?
                }
                "severe_test.sbc_chi2_critical" => {
                    st.sbc_chi2_critical = positive(key, num(key, val)?)?
                }
                "severe_test.null_p_threshold" => {
                    st.null_p_threshold = positive(key, num(key, val)?)?
                }
                other => return Err(ProfileParseError::UnknownField(other.to_string())),
            }
        }
        p.name = name
            .filter(|s| !s.is_empty())
            .ok_or_else(|| ProfileParseError::MissingField("name".into()))?;
        p.rationale = rationale
            .filter(|s| !s.is_empty())
            .ok_or_else(|| ProfileParseError::MissingField("rationale".into()))?;
        if p.e_process_min_inverse_alpha < 1.0 {
            return Err(ProfileParseError::InvalidValue {
                field: "e_process_min_inverse_alpha".into(),
                detail: "1/alpha must be >= 1".into(),
            });
        }
        if !(p.dataset_root.ends_with('/') && !p.dataset_root.contains("..")) {
            return Err(ProfileParseError::InvalidValue {
                field: "dataset_root".into(),
                detail: "must be a relative directory ending in '/' without '..'".into(),
            });
        }
        if p.dataset_manifest.contains("..") {
            return Err(ProfileParseError::InvalidValue {
                field: "dataset_manifest".into(),
                detail: "must not contain '..'".into(),
            });
        }
        if p.severe_test.permutation_replications == 0
            || p.severe_test.sbc_replications == 0
            || p.severe_test.sbc_max_rank == 0
            || p.severe_test.ville_blocks == 0
        {
            return Err(ProfileParseError::InvalidValue {
                field: "severe_test".into(),
                detail: "replication counts, sbc_max_rank and ville_blocks must be >= 1".into(),
            });
        }

        if !resolution.is_empty() {
            let mut vocab = ResolutionVocabulary {
                fine_unit_terms: Vec::new(),
                coarse_horizon_terms: Vec::new(),
                coarse_aggregation_terms: Vec::new(),
                coarse_phrases: Vec::new(),
            };
            for (&k, &v) in &resolution {
                match k {
                    "fine_unit_terms" => vocab.fine_unit_terms = list(v),
                    "coarse_horizon_terms" => vocab.coarse_horizon_terms = list(v),
                    "coarse_aggregation_terms" => vocab.coarse_aggregation_terms = list(v),
                    "coarse_phrases" => vocab.coarse_phrases = list(v),
                    other => {
                        return Err(ProfileParseError::UnknownField(format!(
                            "resolution.{other}"
                        )));
                    }
                }
            }
            if vocab.fine_unit_terms.is_empty() || vocab.coarse_horizon_terms.is_empty() {
                return Err(ProfileParseError::InvalidValue {
                    field: "resolution".into(),
                    detail: "fine_unit_terms and coarse_horizon_terms are mandatory".into(),
                });
            }
            p.resolution = Some(vocab);
        }

        if !columns.is_empty() {
            let get = |k: &str| -> Result<String, ProfileParseError> {
                columns
                    .get(k)
                    .map(|v| v.trim().to_string())
                    .filter(|v| !v.is_empty())
                    .ok_or_else(|| ProfileParseError::MissingField(format!("dataset_columns.{k}")))
            };
            if let Some(other) = columns.keys().find(|k| {
                !matches!(
                    **k,
                    "cluster" | "forecast" | "outcome" | "value" | "jensen_loss"
                )
            }) {
                return Err(ProfileParseError::UnknownField(format!(
                    "dataset_columns.{other}"
                )));
            }
            let loss_raw = get("jensen_loss")?;
            p.dataset_columns = Some(DatasetColumns {
                cluster: get("cluster")?,
                forecast: get("forecast")?,
                outcome: get("outcome")?,
                value: get("value")?,
                jensen_loss: JensenLoss::from_code(&loss_raw).ok_or_else(|| {
                    ProfileParseError::InvalidValue {
                        field: "dataset_columns.jensen_loss".into(),
                        detail: format!("'{loss_raw}' (use square, exp, or hinge)"),
                    }
                })?,
            });
        }
        Ok(p)
    }
}

/// An instrument waived under a profile, with the violations it suppressed (never hidden: reported).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedWaiver {
    pub gate_code: String,
    pub principle: KernelPrinciple,
    pub reason: String,
    pub suppressed: Vec<String>,
}

/// Successful outcome of [`EpistemologicalCard::audit_with`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReport {
    pub profile_name: String,
    pub applied_waivers: Vec<AppliedWaiver>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROFILE_YAML: &str = r#"
protocol_profile:
  name: "hospital_night_shift"
  rationale: "Shift-level staffing; Portuguese vocabulary"
  seed: 7
  e_process_min_inverse_alpha: 40.0
  resolution:
    fine_unit_terms: "turno, plantão"
    coarse_horizon_terms: "anual, mensal"
    coarse_aggregation_terms: "média, total"
    coarse_phrases: "todos os plantões"
  dataset_root: "examples/clay/"
  dataset_manifest: "examples/clay/SHA256SUMS"
  dataset_columns:
    cluster: "ward"
    forecast: "p_hat"
    outcome: "overflow"
    value: "excess_patients"
    jensen_loss: "square"
  severe_test:
    permutation_replications: 300
  waivers:
    PHI4_CHOULDECHOVA: "No group allocation"
"#;

    #[test]
    fn absent_section_is_none_and_canonical_matches_pre_profile_literals() {
        assert_eq!(ProtocolProfile::from_card_yaml("id: \"x\"\n"), Ok(None));
        let c = ProtocolProfile::canonical();
        assert!(c.is_canonical());
        assert_eq!(c.seed, 2026);
        assert_eq!(c.e_process_alpha(), 0.05);
        assert_eq!(c.severe_test.permutation_replications, 200);
    }

    #[test]
    fn declared_section_overrides_only_what_it_states() {
        let p = ProtocolProfile::from_card_yaml(PROFILE_YAML)
            .expect("profile must parse")
            .expect("section present");
        assert_eq!(p.seed, 7);
        assert_eq!(p.e_process_alpha(), 0.025);
        assert_eq!(p.severe_test.permutation_replications, 300);
        assert_eq!(p.severe_test.sbc_replications, 200);
        assert_eq!(p.dataset_root, "examples/clay/");
        assert_eq!(
            p.dataset_columns.as_ref().map(|c| c.jensen_loss),
            Some(JensenLoss::Square)
        );
        let vocab = p.resolution.as_ref().expect("vocabulary");
        assert!(!vocab.verify_resolution("Turno de 12h", "Fixar a média anual de leitos"));
        assert!(vocab.verify_resolution("Turno de 12h", "Abrir leito extra no turno"));
        assert_eq!(p.waivers.len(), 1);
        assert!(!p.is_canonical());
    }

    #[test]
    fn unknown_fields_and_missing_rationale_are_rejected() {
        let typo = PROFILE_YAML.replace("  seed: 7", "  sed: 7");
        assert_eq!(
            ProtocolProfile::from_card_yaml(&typo),
            Err(ProfileParseError::UnknownField("sed".into()))
        );
        let no_rationale = PROFILE_YAML.replace(
            "  rationale: \"Shift-level staffing; Portuguese vocabulary\"\n",
            "",
        );
        assert_eq!(
            ProtocolProfile::from_card_yaml(&no_rationale),
            Err(ProfileParseError::MissingField("rationale".into()))
        );
    }

    #[test]
    fn kernel_waivers_and_empty_reasons_are_defects() {
        let card = EpistemologicalCard::canonical_example();
        let mut p = ProtocolProfile::canonical();
        p.waivers.push(InstrumentWaiver {
            gate_code: "PHI1_SEED".into(),
            reason: "faster".into(),
        });
        p.waivers.push(InstrumentWaiver {
            gate_code: "PHI4_CHOULDECHOVA".into(),
            reason: "  ".into(),
        });
        let codes: Vec<&str> = p.waiver_defects(&card).iter().map(|d| d.0).collect();
        assert_eq!(codes, ["PROFILE_KERNEL_WAIVER", "PROFILE_WAIVER_REASON"]);
    }

    #[test]
    fn non_canonical_profile_changes_the_preregistration_seal() {
        let card = EpistemologicalCard::canonical_example();
        let canonical = ProtocolProfile::canonical();
        assert_eq!(
            canonical.preregistration_hash_for(&card),
            card.compute_preregistration_hash()
        );
        let mut shaped = canonical.clone();
        shaped.seed = 7;
        assert_ne!(
            shaped.preregistration_hash_for(&card),
            card.compute_preregistration_hash()
        );
    }
}
