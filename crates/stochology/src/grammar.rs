//! **Grammar of Aim (`grammar.rs`) — Syntax Layer, Acceptance Predicate, and the Two Movements.**
//!
//! Companion to [`notes/The Grammar of Aim.md`](../../../notes/The%20Grammar%20of%20Aim.md).
//! This module is the **executable shadow of the formal grammar**: it owns syntax and the acceptance
//! predicate, never a second dynamics. Margins are *declared* by the Three Canonical Operators; the
//! grammar only folds them.
//!
//! Structure:
//! 1. **Lexicon:** the three planes ([`Plane`]), the three axes ([`TriadAxis`]), and the seven guard
//!    sites ([`GuardSite`]) — one site per constitutive tension of the treatise (Tensions 1–3 on
//!    $S_{\mathrm{epi}}$, 4–5 on $S_{\mathrm{vital}}$, 6–7 on $S_{\mathrm{gov}}$);
//! 2. **Two tapes:** a [`Shot`] pairs the control tape (*télos*, [`AgentDecision`]) with the
//!    contingency tape (*skopós*, [`StepOutcome`]);
//! 3. **Sentence:** a [`Sentence`] is a guarded trajectory sealed by a [`TemporalVault`];
//! 4. **Judgment:** [`judge`] returns one of four verdicts ([`GrammarVerdict`]) — interior, frontier,
//!    collapse (reusing [`CollapseProposition`]), or unparsable (quarantine, never collapse);
//! 5. **Generation:** [`enumerate`] (exhaustive over a finite grid) and [`shortest_violation`]
//!    (breadth-first search for the shortest counterexample against the declared thresholds).

use crate::engine::{AgentDecision, StepOutcome, TrajectoryReport};
use crate::governance::{TelicGrade, TemporalVault};
use crate::kernel::sha256_hex;
use crate::synthesis::{CollapseProposition, TriadicIntegrity};
use std::collections::VecDeque;
use std::fmt::Write;

/// Near-critical band width above the acceptance threshold $\tau$ below which a sentence is graded
/// as sitting on the frontier rather than in the interior of the language (`[Assumed]`).
pub const BOUNDARY_BAND: f64 = 0.05;

/// Upper bound on the generative depth admitted by [`enumerate`] and [`shortest_violation`].
pub const MAX_DEPTH: usize = 3;

/// Upper bound on the number of guard vectors ($g^7 \le 300\,000$, admitting grids up to $g = 6$: $6^7 = 279\,936$)
/// enumerated before uniform quantile subsampling is applied.
pub const MAX_ENUMERATION: usize = 300_000;

/// Canonical single-step social-harm ceiling $H_{\max} = 40.0$ (`[Assumed]`) used to bound vital-margin
/// slack in [`assess_report`] (`M4 · DANO-01`).
pub const HARM_MAX_ASSUMED: f64 = 40.0;

/// The three planes of the mediation circuit (treatise §1.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plane {
    /// Material and historical world: events in irreversible time.
    World,
    /// Representation, model, and documentary archive.
    Archive,
    /// Deliberation, rules, and institutional liability.
    Institution,
}

/// The three axes of the Nuclear Triad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriadAxis {
    /// Axis 1 — epistemic margin $S_{\mathrm{epi}}$ (Filter I).
    Epistemic,
    /// Axis 2 — vital margin $S_{\mathrm{vital}}$ (Filter II).
    Vital,
    /// Axis 3 — institutional governance $S_{\mathrm{gov}}$ (Filter III).
    Institutional,
}

/// Directed mediation transit across the three-plane circuit (`World`, `Archive`, `Institution`).
///
/// Between the three planes $\mathcal{P} = \{\mathsf{World}, \mathsf{Archive}, \mathsf{Institution}\}$
/// there are $3 \times 2 = 6$ ordered binary transits ($P_{\mathrm{from}} \to P_{\mathrm{to}}$, $P_{\mathrm{from}} \neq P_{\mathrm{to}}$) —
/// decomposing into the **3 direct transits** of the sequential loop and the **3 reflexive locks**
/// (counter-transits) — plus **1 integral three-plane closure** ($3 + 3 + 1 = 7$), in exact 1-to-1
/// bijection with [`GuardSite`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitTransit {
    /// Directed binary passage from `from` plane to `to` plane ($P_{\mathrm{from}} \neq P_{\mathrm{to}}$).
    Directed {
        /// Origin plane of the mediation transit.
        from: Plane,
        /// Destination plane of the mediation transit.
        to: Plane,
        /// `true` if this is one of the 3 reflexive counter-transits (locks), `false` if one of the 3 direct loop transits.
        reflexive_lock: bool,
    },
    /// Integral closure across all three planes (`World <-> Archive <-> Institution`, [`GuardSite::T7Liability`]).
    IntegralCircuit,
}

/// The seven guard sites: one per constitutive tension, in chronological loop order,
/// forming an exact bijection onto the $3 + 3 + 1 = 7$ [`CircuitTransit`]s of the three-plane
/// circuit and the 7 [`crate::multilayer::KnowledgeStratum`] strata of the knowledge graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GuardSite {
    /// Tension 1 (`Institution -> Archive`, Reflexive Lock 1, $S_{\mathrm{epi}}$, Stratum `S2`):
    /// inquiry framing, cost-loss threshold $p^* = C/(\eta L)$, Simon premise-rigging $\mathrm{PRI}$,
    /// Cromwell reserve, and Type III error openness before querying the archive.
    T1Framing,
    /// Tension 2 (`Archive -> World`, Reflexive Lock 2, $S_{\mathrm{epi}}$, Stratum `S4`):
    /// hypothesis/bounds tested against empirical interventions in the world — sharp Manski/Lee/Peterson
    /// identification bounds $[L_t, U_t]$, Ville *e*-processes $\mathcal{E}_t$, and Mayo severity $\mathrm{SEV}$.
    T2Severity,
    /// Tension 3 (`World -> Archive`, Direct Transit 1, $S_{\mathrm{epi}}$, Stratum `S5`):
    /// passage from the living population in the world to the documentary archive — sampling fraction $f$,
    /// Meng selection $\rho_{R,Y}$, cluster $\mathrm{DEFF}$, Kish floor, Chao $\hat{n}_{00}$, Keynes weight $V$, and vintages.
    T3Archive,
    /// Tension 4 (`Archive -> Institution`, Direct Transit 2, $S_{\mathrm{vital}}$, Stratum `S3`):
    /// passage from archival distribution to formal decision summary — preserving the Non-Identical,
    /// ordinal scale invariance, Jensen convexity gap $\Delta_J$, CRPS, and Hill tail index $\hat{\alpha}$.
    T4Formalization,
    /// Tension 5 (`World -> Institution`, Reflexive Lock 3, $S_{\mathrm{vital}}$, Stratum `S1`):
    /// physical, thermodynamic, and biological irreversible time in the world bounding institutional action —
    /// Itô variance drain, Kelly shrinkage, Kingman queue $\rho/(1-\rho)$, Holling basin $D_t/D_0$, and $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$.
    T5Temporal,
    /// Tension 6 (`Institution -> World`, Direct Transit 3, $S_{\mathrm{gov}}$, Stratum `S6`):
    /// institutional policy rule or proxy metric intervening in a reactive social/material world —
    /// `CORP-PAV` calibration, `ICP` causal invariance, KOH discrepancy, Perdomo performativity, and Goodhart $\beta_{\mathrm{proxy}}$.
    T6Performative,
    /// Tension 7 (`World <-> Archive <-> Institution`, Integral Circuit Closure, $S_{\mathrm{gov}}$, Stratum `S7`):
    /// constitutional synthesis binding aim (*télos*) and outcome (*skopós*) across all three planes —
    /// `TemporalVault` *ex-ante* seal, real-time $S3^*$ algedonic veto ($\delta_{\mathrm{cred}} = 0$, $10\text{ bits/s}$ queue),
    /// Sosa `TelicGrade`, and *ex-post* Holmström–Innes downside liability symmetry.
    T7Liability,
}

impl GuardSite {
    /// All seven guard sites in canonical loop order.
    pub const ALL: [GuardSite; 7] = [
        GuardSite::T1Framing,
        GuardSite::T2Severity,
        GuardSite::T3Archive,
        GuardSite::T4Formalization,
        GuardSite::T5Temporal,
        GuardSite::T6Performative,
        GuardSite::T7Liability,
    ];

    /// 1-based tension index (`1..=7`).
    pub const fn index(&self) -> u8 {
        match self {
            GuardSite::T1Framing => 1,
            GuardSite::T2Severity => 2,
            GuardSite::T3Archive => 3,
            GuardSite::T4Formalization => 4,
            GuardSite::T5Temporal => 5,
            GuardSite::T6Performative => 6,
            GuardSite::T7Liability => 7,
        }
    }

    /// Short machine-readable code (`"T1"`..`"T7"`).
    pub const fn code(&self) -> &'static str {
        match self {
            GuardSite::T1Framing => "T1",
            GuardSite::T2Severity => "T2",
            GuardSite::T3Archive => "T3",
            GuardSite::T4Formalization => "T4",
            GuardSite::T5Temporal => "T5",
            GuardSite::T6Performative => "T6",
            GuardSite::T7Liability => "T7",
        }
    }

    /// The Nuclear Triad axis that owns this guard site (`3 / 2 / 2` partition).
    pub const fn axis(&self) -> TriadAxis {
        match self {
            GuardSite::T1Framing | GuardSite::T2Severity | GuardSite::T3Archive => {
                TriadAxis::Epistemic
            }
            GuardSite::T4Formalization | GuardSite::T5Temporal => TriadAxis::Vital,
            GuardSite::T6Performative | GuardSite::T7Liability => TriadAxis::Institutional,
        }
    }

    /// The primary mediation plane anchored by this guard site:
    /// - [`Plane::World`]: [`GuardSite::T2Severity`] (empirical test in the world) and [`GuardSite::T5Temporal`] (irreversible physical/biological time in the world);
    /// - [`Plane::Archive`]: [`GuardSite::T3Archive`] (documentary recording) and [`GuardSite::T4Formalization`] (formal summary of the archive);
    /// - [`Plane::Institution`]: [`GuardSite::T1Framing`] (institutional premise framing), [`GuardSite::T6Performative`] (institutional policy intervention), and [`GuardSite::T7Liability`] (integral constitutional closure).
    pub const fn plane(&self) -> Plane {
        match self {
            GuardSite::T2Severity | GuardSite::T5Temporal => Plane::World,
            GuardSite::T3Archive | GuardSite::T4Formalization => Plane::Archive,
            GuardSite::T1Framing | GuardSite::T6Performative | GuardSite::T7Liability => {
                Plane::Institution
            }
        }
    }

    /// The directed [`CircuitTransit`] across `(World, Archive, Institution)` guarded by this site.
    ///
    /// Across [`GuardSite::ALL`], `T1`–`T6` biject onto the $3 \times 2 = 6$ ordered pairs of distinct planes
    /// (3 direct loop transits + 3 reflexive locks), and `T7` is the `IntegralCircuit` closure ($3 + 3 + 1 = 7$).
    pub const fn transit(&self) -> CircuitTransit {
        match self {
            GuardSite::T1Framing => CircuitTransit::Directed {
                from: Plane::Institution,
                to: Plane::Archive,
                reflexive_lock: true,
            },
            GuardSite::T2Severity => CircuitTransit::Directed {
                from: Plane::Archive,
                to: Plane::World,
                reflexive_lock: true,
            },
            GuardSite::T3Archive => CircuitTransit::Directed {
                from: Plane::World,
                to: Plane::Archive,
                reflexive_lock: false,
            },
            GuardSite::T4Formalization => CircuitTransit::Directed {
                from: Plane::Archive,
                to: Plane::Institution,
                reflexive_lock: false,
            },
            GuardSite::T5Temporal => CircuitTransit::Directed {
                from: Plane::World,
                to: Plane::Institution,
                reflexive_lock: true,
            },
            GuardSite::T6Performative => CircuitTransit::Directed {
                from: Plane::Institution,
                to: Plane::World,
                reflexive_lock: false,
            },
            GuardSite::T7Liability => CircuitTransit::IntegralCircuit,
        }
    }

    /// Verifies that the 7 guard sites form an exact bijection onto the $3 \times 2 = 6$ directed
    /// binary transits (3 direct + 3 reflexive locks) plus 1 integral circuit closure ($3 + 3 + 1 = 7$).
    pub fn is_bijection_over_circuit_transits() -> bool {
        let mut direct_count = 0usize;
        let mut reflexive_count = 0usize;
        let mut integral_count = 0usize;
        let mut pairs: Vec<(Plane, Plane)> = Vec::with_capacity(6);
        for site in Self::ALL {
            match site.transit() {
                CircuitTransit::Directed {
                    from,
                    to,
                    reflexive_lock,
                } => {
                    if from == to || pairs.contains(&(from, to)) {
                        return false;
                    }
                    pairs.push((from, to));
                    if reflexive_lock {
                        reflexive_count += 1;
                    } else {
                        direct_count += 1;
                    }
                }
                CircuitTransit::IntegralCircuit => {
                    integral_count += 1;
                }
            }
        }
        pairs.len() == 6 && direct_count == 3 && reflexive_count == 3 && integral_count == 1
    }
}

/// A single guard reading: the margin declared at one site, plus the step where a violation was
/// *witnessed* (`None` means the operator declared the margin without per-step evidence —
/// absence of evidence is not evidence that the guard held).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GuardReading {
    /// Guard site.
    pub site: GuardSite,
    /// Declared margin in $[0, 1]$.
    pub margin: f64,
    /// First step at which a violation was witnessed, if any.
    pub step_t: Option<usize>,
}

impl GuardReading {
    /// Creates a guard reading.
    pub const fn new(site: GuardSite, margin: f64, step_t: Option<usize>) -> Self {
        Self {
            site,
            margin,
            step_t,
        }
    }
}

/// Mirror of [`GuardReading`] for autonomous read operations (`§10`, `M5 · AUTOR-01`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReadReading {
    /// Guard site governing the read transit.
    pub site: GuardSite,
    /// Declared margin in $[0, 1]$.
    pub margin: f64,
    /// Step index at which the read occurred, if any.
    pub step_t: Option<usize>,
    /// Whether the read was authorized under the active institutional mandate.
    pub authorized: bool,
}

impl ReadReading {
    /// Creates a read reading.
    pub const fn new(
        site: GuardSite,
        margin: f64,
        step_t: Option<usize>,
        authorized: bool,
    ) -> Self {
        Self {
            site,
            margin,
            step_t,
            authorized,
        }
    }
}

/// Kind of institutional or tool action executed by an agent (`§10`, `M5 · AUTOR-01`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentActionKind {
    /// Passive observation / query on the archive (Pearl Rung 1).
    Read,
    /// State-mutating intervention or actuation (Pearl Rung 2).
    Write,
    /// Sub-delegation of authority to a downstream agent.
    Delegate,
    /// Emergency or constitutional halt ($S3^*$ stop).
    Halt,
}

/// Identifier of an institutional, archival, or physical resource scope (`§10`, `M5 · AUTOR-01`).
pub type ResourceId = String;

/// Audited agent action carrying principal identity, delegation chain, resource scope, permission envelope,
/// halt authority, and token cost (`§10`, `M5 · AUTOR-01`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentAction {
    /// Action category (`Read`, `Write`, `Delegate`, `Halt`).
    pub kind: AgentActionKind,
    /// Target resource scopes touched by this action.
    pub scope: Vec<ResourceId>,
    /// Executing agent identifier.
    pub principal: String,
    /// Human or institutional principal on whose behalf the agent acts.
    pub on_behalf_of: String,
    /// Task contract / pre-registered authorization identifier.
    pub contract_id: String,
    /// Resource scopes explicitly authorized by `contract_id`.
    pub authorized_scopes: Vec<ResourceId>,
    /// Whether the agent holds constitutional authority to invoke `Halt`.
    pub can_halt: bool,
    /// Token or compute cost consumed by this action.
    pub token_cost: usize,
}

/// One shot of the loop: control tape (*télos*) paired with contingency tape (*skopós*)
/// and its three-plane mediation circuit (`[Plane::World, Plane::Archive, Plane::Institution]`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shot {
    /// Plane transit sequence across the mediation circuit for this step.
    pub planes: [Plane; 3],
    /// What the agent issued — the discipline of the aim.
    pub telos: AgentDecision,
    /// What the world returned — the contingency of the hit.
    pub skopos: StepOutcome,
    /// True iff all eight [`AgentDecision`] fields on `telos` were attested by the step report (`B2 · FITA-01`).
    pub has_complete_control_tape: bool,
}

impl Shot {
    /// Canonical mediation circuit across world, archive, and institution.
    pub const CANONICAL_CIRCUIT: [Plane; 3] = [Plane::World, Plane::Archive, Plane::Institution];

    /// Builds a shot from one report step (`B2 · FITA-01`, `M6 · SENT-01`):
    /// copies the full attested [`AgentDecision`] from `skopos.attested_decision` when present (preserving the
    /// agent's declared `provisioned_capacity` rather than promoting the post-ruin environment sentinel
    /// `effective_capacity = 0.0`). When `skopos.attested_decision` is `None`, marks
    /// `has_complete_control_tape = false` so [`judge`] quarantines the trajectory with
    /// [`UnparsableReason::MissingControlTape`].
    pub fn attested(bounds: (f64, f64), effective_capacity: f64, skopos: StepOutcome) -> Self {
        match skopos.attested_decision {
            Some(decision) => Self {
                planes: Self::CANONICAL_CIRCUIT,
                telos: decision,
                skopos,
                has_complete_control_tape: true,
            },
            None => Self {
                planes: Self::CANONICAL_CIRCUIT,
                telos: AgentDecision {
                    epistemic_demand_bounds: bounds,
                    recognized_n_eff: 0.0,
                    provisioned_capacity: effective_capacity,
                    vital_reserve_allocation: 0.0,
                    growth_leverage_f: 0.0,
                    proxy_incentive_pressure: 0.0,
                    frontline_testimonial_discount: 0.0,
                    constitutional_brake_active: false,
                },
                skopos,
                has_complete_control_tape: false,
            },
        }
    }

    /// Builds a canonical *declared* shot for generative use (`[Simulated]`; not a measurement).
    pub fn canonical(step_t: usize, utilization: f64) -> Self {
        let capacity = 1.0 / utilization.clamp(0.05, 0.99);
        let telos = AgentDecision {
            epistemic_demand_bounds: (0.80, 1.20),
            recognized_n_eff: 24.0,
            provisioned_capacity: capacity,
            vital_reserve_allocation: 0.10,
            growth_leverage_f: 0.50,
            proxy_incentive_pressure: 0.20,
            frontline_testimonial_discount: 0.0,
            constitutional_brake_active: true,
        };
        let skopos = StepOutcome {
            step_t,
            true_latent_demand: 1.0,
            raw_archive_mean: 1.0,
            epistemic_bounds: (0.80, 1.20),
            effective_capacity: capacity,
            epistemic_bounds_covered_truth: true,
            realized_utilization: utilization,
            realized_harm: 0.0,
            next_reserve: 100.0,
            hit_ruin_barrier: false,
            sosa_grade: TelicGrade::FullyAptPerformance,
            attested_decision: Some(telos),
            already_ruined: false,
        };
        Self {
            planes: Self::CANONICAL_CIRCUIT,
            telos,
            skopos,
            has_complete_control_tape: true,
        }
    }
}

/// One production rule `lhs ::= rhs_tokens` parsed from the EBNF specification in [`notes/The Grammar of Aim.md`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EbnfRule {
    /// Non-terminal symbol on the left-hand side.
    pub lhs: String,
    /// Right-hand side token sequence (with comments stripped).
    pub rhs: Vec<String>,
}

/// Executable EBNF grammar parsed from [`notes/The Grammar of Aim.md`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EbnfGrammar {
    /// Ordered production rules extracted from the EBNF block.
    pub rules: Vec<EbnfRule>,
}

/// Tokenizes a surface derivation string or EBNF right-hand side into lexical tokens.
pub fn lex_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        // Strip EBNF comments (* ... *)
        if ch == '(' {
            let mut clone = chars.clone();
            clone.next();
            if clone.peek() == Some(&'*') {
                chars.next();
                chars.next();
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&')') {
                        chars.next();
                        break;
                    }
                }
                continue;
            }
        }
        if ch == '"' {
            chars.next();
            let mut lit = String::from("\"");
            for c in chars.by_ref() {
                lit.push(c);
                if c == '"' {
                    break;
                }
            }
            tokens.push(lit);
            continue;
        }
        if matches!(ch, '[' | ']' | '(' | ')' | ',' | '|') {
            tokens.push(ch.to_string());
            chars.next();
            continue;
        }
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() || matches!(c, '[' | ']' | '(' | ')' | ',' | '|' | '"') {
                break;
            }
            word.push(c);
            chars.next();
        }
        if !word.is_empty() {
            tokens.push(word);
        }
    }
    tokens
}

/// Extracts and validates the executable EBNF grammar from `markdown_or_ebnf`.
/// Fails if any of the 11 canonical productions (`Sentence`, `Framing`, `Counting`, `Body`, `Step`,
/// `Decision`, `Brake`, `Write`, `Read`, `Halt`, `Interval`) is missing or structurally altered (e.g., `Sentence ::= banana`).
pub fn parse_ebnf_grammar(markdown_or_ebnf: &str) -> Result<EbnfGrammar, &'static str> {
    let ebnf_block = if let Some(start) = markdown_or_ebnf.find("```ebnf") {
        let after = &markdown_or_ebnf[start + "```ebnf".len()..];
        let end = after.find("```").ok_or("unclosed ```ebnf block")?;
        &after[..end]
    } else {
        markdown_or_ebnf
    };

    let mut rules = Vec::new();
    for line in ebnf_block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (lhs_raw, rhs_raw) = trimmed
            .split_once("::=")
            .ok_or("EBNF production line missing '::='")?;
        let lhs = lhs_raw.trim().to_string();
        let rhs = lex_tokens(rhs_raw);
        if lhs.is_empty() || rhs.is_empty() {
            return Err("EBNF production has empty LHS or RHS");
        }
        rules.push(EbnfRule { lhs, rhs });
    }

    let expected: [(&str, &[&str]); 11] = [
        ("Sentence", &["Framing", "Counting", "Body"]),
        (
            "Framing",
            &[
                "\"frame\"",
                "[",
                "\"insulate\"",
                "|",
                "\"open\"",
                "]",
                "[",
                "\"type3\"",
                "]",
            ],
        ),
        (
            "Counting",
            &[
                "\"count\"",
                "Interval",
                "[",
                "\"deff\"",
                "]",
                "[",
                "\"vintages\"",
                "]",
            ],
        ),
        (
            "Body",
            &[
                "Decision", "[", "Brake", "]", "(", "Write", "|", "Read", ")", "*", "[", "Halt",
                "]", "[", "Step*", "]",
            ],
        ),
        ("Step", &["Framing", "Counting", "Body"]),
        (
            "Decision",
            &[
                "\"act\"",
                "\"lex\"",
                "\"(\"",
                "\"survival\"",
                "\",\"",
                "\"minimax_regret\"",
                "\")\"",
            ],
        ),
        ("Brake", &["\"brake\"", "\"tts\"", "\">=\"", "\"ttr\""]),
        ("Write", &["\"write\"", "Scope"]),
        ("Read", &["\"read\"", "Scope"]),
        ("Halt", &["\"halt\""]),
        ("Interval", &["\"[\"", "Scalar", "\",\"", "Scalar", "\"]\""]),
    ];

    if rules.len() != expected.len() {
        return Err("EBNF rule count mismatch against canonical 11-production grammar");
    }
    for ((exp_lhs, exp_rhs), rule) in expected.iter().zip(rules.iter()) {
        if rule.lhs != *exp_lhs {
            return Err("EBNF non-terminal LHS mismatch");
        }
        let exp_vec: Vec<String> = exp_rhs.iter().map(|s| (*s).to_string()).collect();
        if rule.rhs != exp_vec {
            return Err("EBNF production RHS tokens mismatch against canonical grammar");
        }
    }

    Ok(EbnfGrammar { rules })
}

impl EbnfGrammar {
    /// Recursive-descent parser executing the grammar productions over `surface_input`
    /// and constructing a [`Sentence`] annotated with `guards`.
    pub fn parse_sentence(
        &self,
        surface_input: &str,
        guards: Vec<GuardReading>,
    ) -> Result<Sentence, GrammarVerdict> {
        if self.rules.len() != 11 {
            return Err(GrammarVerdict::Unparsable(
                UnparsableReason::SkeletonMismatch,
            ));
        }
        let tokens = lex_tokens(surface_input);
        if tokens.is_empty() {
            return Err(GrammarVerdict::Unparsable(UnparsableReason::EmptySentence));
        }
        let mut cursor = 0usize;
        let mut shots = Vec::new();
        let mut actions = Vec::new();
        Self::parse_step_sequence(&tokens, &mut cursor, &mut shots, &mut actions)?;
        if cursor != tokens.len() {
            return Err(GrammarVerdict::Unparsable(
                UnparsableReason::SkeletonMismatch,
            ));
        }
        Ok(Sentence::with_agent_audit(
            shots,
            guards,
            actions,
            Vec::new(),
            None,
            None,
        ))
    }

    fn parse_step_sequence(
        tokens: &[String],
        cursor: &mut usize,
        shots: &mut Vec<Shot>,
        actions: &mut Vec<AgentAction>,
    ) -> Result<(), GrammarVerdict> {
        while *cursor < tokens.len() {
            // 1. Plane::World — Framing ::= "frame" [ "insulate" | "open" ] [ "type3" ]
            if tokens.get(*cursor).map(String::as_str) != Some("frame") {
                return Err(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ));
            }
            *cursor += 1;
            if matches!(
                tokens.get(*cursor).map(String::as_str),
                Some("insulate") | Some("open")
            ) {
                *cursor += 1;
            }
            if tokens.get(*cursor).map(String::as_str) == Some("type3") {
                *cursor += 1;
            }

            // 2. Plane::Archive — Counting ::= "count" Interval [ "deff" ] [ "vintages" ]
            if tokens.get(*cursor).map(String::as_str) != Some("count") {
                return Err(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ));
            }
            *cursor += 1;
            if tokens.get(*cursor).map(String::as_str) != Some("[") {
                return Err(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ));
            }
            *cursor += 1;
            let lo: f64 = tokens
                .get(*cursor)
                .and_then(|s| s.parse::<f64>().ok())
                .ok_or(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ))?;
            *cursor += 1;
            if tokens.get(*cursor).map(String::as_str) != Some(",") {
                return Err(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ));
            }
            *cursor += 1;
            let hi: f64 = tokens
                .get(*cursor)
                .and_then(|s| s.parse::<f64>().ok())
                .ok_or(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ))?;
            *cursor += 1;
            if tokens.get(*cursor).map(String::as_str) != Some("]") {
                return Err(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ));
            }
            *cursor += 1;
            if !lo.is_finite() || !hi.is_finite() || lo > hi {
                return Err(GrammarVerdict::Unparsable(
                    UnparsableReason::SkeletonMismatch,
                ));
            }
            let has_deff = if tokens.get(*cursor).map(String::as_str) == Some("deff") {
                *cursor += 1;
                true
            } else {
                false
            };
            if tokens.get(*cursor).map(String::as_str) == Some("vintages") {
                *cursor += 1;
            }

            // 3. Plane::Institution — Body ::= Decision [ Brake ] ( Write | Read )* [ Halt ] [ Step* ]
            let expected_dec = ["act", "lex", "(", "survival", ",", "minimax_regret", ")"];
            for &tok in &expected_dec {
                if tokens.get(*cursor).map(String::as_str) != Some(tok) {
                    return Err(GrammarVerdict::Unparsable(
                        UnparsableReason::SkeletonMismatch,
                    ));
                }
                *cursor += 1;
            }
            let has_brake = if tokens.get(*cursor).map(String::as_str) == Some("brake") {
                let expected_brake = ["brake", "tts", ">=", "ttr"];
                for &tok in &expected_brake {
                    if tokens.get(*cursor).map(String::as_str) != Some(tok) {
                        return Err(GrammarVerdict::Unparsable(
                            UnparsableReason::SkeletonMismatch,
                        ));
                    }
                    *cursor += 1;
                }
                true
            } else {
                false
            };

            while let Some(tok) = tokens.get(*cursor).map(String::as_str) {
                if tok == "write" || tok == "read" {
                    let kind = if tok == "write" {
                        AgentActionKind::Write
                    } else {
                        AgentActionKind::Read
                    };
                    *cursor += 1;
                    let scope_tok =
                        tokens
                            .get(*cursor)
                            .cloned()
                            .ok_or(GrammarVerdict::Unparsable(
                                UnparsableReason::SkeletonMismatch,
                            ))?;
                    *cursor += 1;
                    actions.push(AgentAction {
                        kind,
                        scope: vec![scope_tok.clone()],
                        principal: "ebnf-agent".to_string(),
                        on_behalf_of: "institution".to_string(),
                        contract_id: "canonical-contract".to_string(),
                        authorized_scopes: vec![scope_tok],
                        can_halt: has_brake,
                        token_cost: 1,
                    });
                } else {
                    break;
                }
            }
            if tokens.get(*cursor).map(String::as_str) == Some("halt") {
                *cursor += 1;
                actions.push(AgentAction {
                    kind: AgentActionKind::Halt,
                    scope: Vec::new(),
                    principal: "ebnf-agent".to_string(),
                    on_behalf_of: "institution".to_string(),
                    contract_id: "canonical-contract".to_string(),
                    authorized_scopes: Vec::new(),
                    can_halt: has_brake,
                    token_cost: 1,
                });
            }

            let step_t = shots.len();
            let mut shot = Shot::canonical(step_t, 0.80);
            shot.telos.epistemic_demand_bounds = (lo, hi);
            shot.telos.recognized_n_eff = if has_deff { 22.0 } else { 46.0 };
            shot.telos.constitutional_brake_active = has_brake;
            shots.push(shot);
        }
        Ok(())
    }
}

/// A guarded trajectory: a derivable sentence of the grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct Sentence {
    /// Shots of the loop, in order.
    pub shots: Vec<Shot>,
    /// Guard readings, one per site in the well-formed case.
    pub guards: Vec<GuardReading>,
    /// Audited agent actions (`Read`, `Write`, `Delegate`, `Halt`) executed along the derivation (`§10`).
    pub actions: Vec<AgentAction>,
    /// Read guard readings (`ReadReading`) recorded along the derivation (`§10`).
    pub reads: Vec<ReadReading>,
    /// Optional token budget ceiling (`§10`).
    pub token_budget: Option<usize>,
    /// Optional tool-call budget ceiling (`§10`).
    pub tool_call_budget: Option<usize>,
    /// Bitemporal seal committing to the sentence's canonical payload.
    pub seal: TemporalVault,
}

impl Sentence {
    /// Builds and seals a sentence.
    pub fn new(shots: Vec<Shot>, guards: Vec<GuardReading>) -> Self {
        Self::with_agent_audit(shots, guards, Vec::new(), Vec::new(), None, None)
    }

    /// Builds and seals a sentence including agent actions, read readings, and resource budgets (`§10`).
    pub fn with_agent_audit(
        shots: Vec<Shot>,
        guards: Vec<GuardReading>,
        actions: Vec<AgentAction>,
        reads: Vec<ReadReading>,
        token_budget: Option<usize>,
        tool_call_budget: Option<usize>,
    ) -> Self {
        let payload = Self::payload_of_full(
            &shots,
            &guards,
            &actions,
            &reads,
            token_budget,
            tool_call_budget,
        );
        Self {
            shots,
            guards,
            actions,
            reads,
            token_budget,
            tool_call_budget,
            seal: TemporalVault::seal(payload.as_bytes()),
        }
    }

    /// Formats the sentence's control tape as a surface derivation string conforming to the EBNF grammar.
    pub fn to_ebnf_surface(&self) -> String {
        let mut out = String::new();
        for (idx, s) in self.shots.iter().enumerate() {
            if idx > 0 {
                out.push(' ');
            }
            let (lo, hi) = s.telos.epistemic_demand_bounds;
            let framing_mode = if hi - lo > 4.0 {
                "open type3"
            } else {
                "insulate"
            };
            let deff_tok = if s.telos.recognized_n_eff < 35.0 {
                " deff vintages"
            } else {
                ""
            };
            let brake_tok = if s.telos.constitutional_brake_active {
                " brake tts >= ttr"
            } else {
                ""
            };
            let _ = write!(
                &mut out,
                "frame {framing_mode} count [ {lo:.4} , {hi:.4} ]{deff_tok} act lex ( survival , minimax_regret ){brake_tok}"
            );
        }
        out
    }

    /// Canonical deterministic payload encoding (no external serialization dependency).
    /// Encodes all fields of `Shot` (`planes`, `telos`, `skopos`) and `GuardReading` using exact
    /// IEEE-754 bit representations (`f64::to_bits()`) so any sub-$10^{-7}$ or field perturbation
    /// alters the SHA-256 seal.
    pub fn payload_of(shots: &[Shot], guards: &[GuardReading]) -> String {
        Self::payload_of_full(shots, guards, &[], &[], None, None)
    }

    /// Full canonical payload encoding including `actions`, `reads`, and budgets (`§10`).
    pub fn payload_of_full(
        shots: &[Shot],
        guards: &[GuardReading],
        actions: &[AgentAction],
        reads: &[ReadReading],
        token_budget: Option<usize>,
        tool_call_budget: Option<usize>,
    ) -> String {
        let mut out = String::new();
        for s in shots {
            let _ = writeln!(
                &mut out,
                "shot {} {:?} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {} {} {:016x} {:016x} {:016x} {:016x} {:016x} {} {:016x} {:016x} {:016x} {} {:?}",
                s.skopos.step_t,
                s.planes,
                s.telos.epistemic_demand_bounds.0.to_bits(),
                s.telos.epistemic_demand_bounds.1.to_bits(),
                s.telos.recognized_n_eff.to_bits(),
                s.telos.provisioned_capacity.to_bits(),
                s.telos.vital_reserve_allocation.to_bits(),
                s.telos.growth_leverage_f.to_bits(),
                s.telos.proxy_incentive_pressure.to_bits(),
                s.telos.frontline_testimonial_discount.to_bits(),
                s.telos.constitutional_brake_active,
                s.has_complete_control_tape,
                s.skopos.true_latent_demand.to_bits(),
                s.skopos.raw_archive_mean.to_bits(),
                s.skopos.epistemic_bounds.0.to_bits(),
                s.skopos.epistemic_bounds.1.to_bits(),
                s.skopos.effective_capacity.to_bits(),
                s.skopos.epistemic_bounds_covered_truth,
                s.skopos.realized_utilization.to_bits(),
                s.skopos.realized_harm.to_bits(),
                s.skopos.next_reserve.to_bits(),
                s.skopos.hit_ruin_barrier,
                s.skopos.sosa_grade,
            );
        }
        for g in guards {
            let _ = writeln!(
                &mut out,
                "guard {:?} {:016x} {}",
                g.site,
                g.margin.to_bits(),
                match g.step_t {
                    Some(t) => t.to_string(),
                    None => "-".to_string(),
                }
            );
        }
        for a in actions {
            let _ = writeln!(
                &mut out,
                "action {:?} {:?} {} {} {} {:?} {} {}",
                a.kind,
                a.scope,
                a.principal,
                a.on_behalf_of,
                a.contract_id,
                a.authorized_scopes,
                a.can_halt,
                a.token_cost
            );
        }
        for r in reads {
            let _ = writeln!(
                &mut out,
                "read {:?} {:016x} {:?} {}",
                r.site,
                r.margin.to_bits(),
                r.step_t,
                r.authorized
            );
        }
        if token_budget.is_some() || tool_call_budget.is_some() {
            let _ = writeln!(&mut out, "budget {:?} {:?}", token_budget, tool_call_budget);
        }
        out
    }

    /// The sentence's canonical payload.
    pub fn payload(&self) -> String {
        Self::payload_of_full(
            &self.shots,
            &self.guards,
            &self.actions,
            &self.reads,
            self.token_budget,
            self.tool_call_budget,
        )
    }

    /// Recomputes the seal and compares it with the stored commitment.
    pub fn verify_seal(&self) -> bool {
        sha256_hex(self.payload().as_bytes()) == self.seal.expected_sha256
    }
}

/// Why a candidate is not a sentence of the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnparsableReason {
    /// No shot and no guard: the empty derivation.
    EmptySentence,
    /// A transit carries no guard reading at all: an unguarded production.
    UnguardedTransit(GuardSite),
    /// The control tape violates a declared invariant (inverted bounds, non-positive capacity, broken plane circuit).
    SkeletonMismatch,
    /// A guard returned a non-finite margin: the reading cannot support any verdict.
    NonFiniteMargin(GuardSite),
    /// A report step omitted the attested [`AgentDecision`] control tape (`B2 · FITA-01`).
    MissingControlTape,
    /// An agent attempted a `Write` or `Delegate` action outside its authorized scope or contract (`§10`, `M5`).
    OutOfScope,
    /// An agent attempted a `Read` action without authorization (`§10`, `M5`).
    UnauthorizedRead,
    /// An agent exceeded its declared token or tool-call budget (`§10`, `M5`).
    BudgetExhausted,
}

/// The four verdicts of the acceptance predicate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrammarVerdict {
    /// Every axis clears $\tau + $ [`BOUNDARY_BAND`].
    Interior,
    /// Some axis clears $\tau$ but sits inside the near-critical band.
    Boundary {
        /// The weakest guard site.
        site: GuardSite,
        /// Its margin.
        margin: f64,
    },
    /// At least one axis falls below $\tau$: an ill-formed derivation, classified by the
    /// corresponding [`CollapseProposition`].
    Collapse {
        /// The active collapse proposition (mask over the triad).
        proposition: CollapseProposition,
        /// The first violating guard site in step order, when one was witnessed.
        at: Option<GuardSite>,
        /// The step at which that site was witnessed.
        step_t: Option<usize>,
    },
    /// Quarantine: not a sentence of the language, and therefore not a collapse either.
    Unparsable(UnparsableReason),
}

/// Validates the control tape and mediation circuit (the *télos* side of the skeleton,
/// completely independent of the *skopós* contingency tape — `B3 · NCOM-01`).
fn telos_skeleton_ok(shot: &Shot) -> bool {
    let (l, u) = shot.telos.epistemic_demand_bounds;
    shot.planes == Shot::CANONICAL_CIRCUIT
        && l.is_finite()
        && u.is_finite()
        && l <= u
        && u > 0.0
        && shot.telos.provisioned_capacity.is_finite()
        && shot.telos.provisioned_capacity > 0.0
        && shot.telos.vital_reserve_allocation.is_finite()
        && shot.telos.vital_reserve_allocation >= 0.0
}

/// Minimum margin over the readings of one axis, or `None` if the axis is unguarded.
fn axis_score(guards: &[GuardReading], axis: TriadAxis) -> Option<f64> {
    guards
        .iter()
        .filter(|g| g.site.axis() == axis)
        .map(|g| g.margin)
        .reduce(f64::min)
}

/// Checks whether at least one guard reading is present on `plane`.
fn plane_guarded(guards: &[GuardReading], plane: Plane) -> bool {
    guards.iter().any(|g| g.site.plane() == plane)
}

/// The weakest guard site of one axis.
fn weakest_site(guards: &[GuardReading], axis: TriadAxis) -> Option<GuardSite> {
    guards
        .iter()
        .filter(|g| g.site.axis() == axis)
        .min_by(|a, b| a.margin.total_cmp(&b.margin))
        .map(|g| g.site)
}

/// The first violating site in step order (witnessed steps first, then unwitnessed; ties by site).
fn first_violation(guards: &[GuardReading], threshold: f64) -> Option<(GuardSite, Option<usize>)> {
    let mut violators: Vec<&GuardReading> =
        guards.iter().filter(|g| g.margin < threshold).collect();
    violators.sort_by(|a, b| match (a.step_t, b.step_t) {
        (Some(x), Some(y)) => x.cmp(&y).then_with(|| a.site.cmp(&b.site)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.site.cmp(&b.site),
    });
    violators.first().map(|g| (g.site, g.step_t))
}

/// Normalizes an acceptance threshold into the admissible band `[0.05, 0.95]`.
///
/// A non-finite threshold falls back to the canonical `0.50` instead of propagating `NaN` through
/// the Leontief bottleneck and the frontier band, where every comparison would silently evaluate to
/// `false` and strip the collapse of its witness site.
fn normalize_threshold(threshold: f64) -> f64 {
    if threshold.is_finite() {
        threshold.clamp(0.05, 0.95)
    } else {
        0.50
    }
}

/// **The acceptance predicate.** Parses a sentence and returns one of the four verdicts.
///
/// Order of evaluation is fixed: control-tape attestation, skeleton, agent authority/budget, plane circuit,
/// and guard completeness first (an ill-formed derivation has no verdict), then the Leontief bottleneck
/// [`TriadicIntegrity::evaluate`], then the frontier band. `threshold` is normalized by
/// [`normalize_threshold`] before any comparison.
pub fn judge(sentence: &Sentence, threshold: f64) -> GrammarVerdict {
    let threshold = normalize_threshold(threshold);
    if sentence.shots.is_empty() && sentence.guards.is_empty() {
        return GrammarVerdict::Unparsable(UnparsableReason::EmptySentence);
    }
    if sentence.shots.is_empty() {
        return GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch);
    }
    for shot in &sentence.shots {
        if !shot.has_complete_control_tape {
            return GrammarVerdict::Unparsable(UnparsableReason::MissingControlTape);
        }
        if !telos_skeleton_ok(shot) {
            return GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch);
        }
    }
    if let Some(max_calls) = sentence.tool_call_budget
        && sentence.actions.len() > max_calls
    {
        return GrammarVerdict::Unparsable(UnparsableReason::BudgetExhausted);
    }
    if let Some(max_tok) = sentence.token_budget {
        let used_tok: usize = sentence.actions.iter().map(|a| a.token_cost).sum();
        if used_tok > max_tok {
            return GrammarVerdict::Unparsable(UnparsableReason::BudgetExhausted);
        }
    }
    for r in &sentence.reads {
        if !r.authorized {
            return GrammarVerdict::Unparsable(UnparsableReason::UnauthorizedRead);
        }
        if !r.margin.is_finite() {
            return GrammarVerdict::Unparsable(UnparsableReason::NonFiniteMargin(r.site));
        }
    }
    for (idx, act) in sentence.actions.iter().enumerate() {
        match act.kind {
            AgentActionKind::Read => {
                if act.scope.is_empty()
                    || !act.scope.iter().all(|s| act.authorized_scopes.contains(s))
                {
                    return GrammarVerdict::Unparsable(UnparsableReason::UnauthorizedRead);
                }
            }
            AgentActionKind::Write | AgentActionKind::Delegate => {
                if act.scope.is_empty()
                    || act.contract_id.is_empty()
                    || act.principal.is_empty()
                    || !act.scope.iter().all(|s| act.authorized_scopes.contains(s))
                {
                    return GrammarVerdict::Unparsable(UnparsableReason::OutOfScope);
                }
            }
            AgentActionKind::Halt => {
                if !act.can_halt {
                    return GrammarVerdict::Collapse {
                        proposition: CollapseProposition::Proposition3CynicalCapture,
                        at: Some(GuardSite::T7Liability),
                        step_t: Some(idx),
                    };
                }
            }
        }
    }
    for g in &sentence.guards {
        if !g.margin.is_finite() {
            return GrammarVerdict::Unparsable(UnparsableReason::NonFiniteMargin(g.site));
        }
    }
    for plane in [Plane::World, Plane::Archive, Plane::Institution] {
        if !plane_guarded(&sentence.guards, plane) {
            let missing = GuardSite::ALL
                .iter()
                .copied()
                .find(|s| s.plane() == plane)
                .unwrap_or(GuardSite::T1Framing);
            return GrammarVerdict::Unparsable(UnparsableReason::UnguardedTransit(missing));
        }
    }
    let axes = [
        TriadAxis::Epistemic,
        TriadAxis::Vital,
        TriadAxis::Institutional,
    ];
    let mut scores = [0.0_f64; 3];
    for (idx, axis) in axes.iter().enumerate() {
        match axis_score(&sentence.guards, *axis) {
            Some(s) => scores[idx] = s,
            None => {
                let missing = GuardSite::ALL
                    .iter()
                    .copied()
                    .find(|s| s.axis() == *axis)
                    .unwrap_or(GuardSite::T1Framing);
                return GrammarVerdict::Unparsable(UnparsableReason::UnguardedTransit(missing));
            }
        }
    }
    let triad = TriadicIntegrity::evaluate(scores[0], scores[1], scores[2], threshold);
    if triad.diagnose == CollapseProposition::IntactTriad {
        for (idx, axis) in axes.iter().enumerate() {
            if scores[idx] < threshold + BOUNDARY_BAND {
                let site = weakest_site(&sentence.guards, *axis).unwrap_or(GuardSite::T1Framing);
                return GrammarVerdict::Boundary {
                    site,
                    margin: scores[idx],
                };
            }
        }
        return GrammarVerdict::Interior;
    }
    let (at, step_t) = match first_violation(&sentence.guards, threshold) {
        Some((site, t)) => (Some(site), t),
        None => (None, None),
    };
    GrammarVerdict::Collapse {
        proposition: triad.diagnose,
        at,
        step_t,
    }
}

/// **Independent step-log recognizer of a sequential report:** invokes the Three Canonical
/// Mathematical Operators ([`EpistemicMarginState::from_single_arm_bounds`],
/// [`VitalMarginState::from_queue_basin_and_survival`], [`GovernanceState::evaluate_shift_governance`])
/// directly on the realized step trace `report.steps` and trajectory observables without reading
/// `report.triadic_integrity` (`S-01`).
pub fn assess_report(report: &TrajectoryReport, threshold: f64) -> GrammarVerdict {
    use crate::epistemic::EpistemicMarginState;
    use crate::governance::GovernanceState;
    use crate::vital::VitalMarginState;

    if report.steps.is_empty() {
        return GrammarVerdict::Unparsable(UnparsableReason::EmptySentence);
    }
    let n_steps = report.steps.len();
    let covered_indicators: Vec<f64> = report
        .steps
        .iter()
        .filter(|s| s.epistemic_bounds_covered_truth)
        .map(|_| 1.0)
        .collect();
    let missing_steps = n_steps.saturating_sub(covered_indicators.len());
    let epi_op = EpistemicMarginState::from_single_arm_bounds(
        &covered_indicators,
        missing_steps,
        0.0,
        1.0,
        0.05,
    );
    let n_eff_ratio = (report.mean_recognized_n_eff / 100.0).clamp(0.0, 1.0);
    let m_t1 =
        (0.75 * report.epistemic_coverage_rate + 0.25 * report.shock_coverage_rate).clamp(0.0, 1.0);
    let m_t2 = (0.65 * report.shock_coverage_rate + 0.35 * epi_op.epistemic_margin_score)
        .clamp(0.0, 1.0)
        .max(m_t1);
    let m_t3 =
        (0.70 * report.epistemic_coverage_rate + 0.20 * epi_op.keynes_weight + 0.10 * n_eff_ratio)
            .clamp(0.0, 1.0)
            .max(m_t1);

    let survival_steps = report.steps.iter().filter(|s| !s.hit_ruin_barrier).count() as f64;
    let mean_sizing_util = report
        .steps
        .iter()
        .filter(|s| !s.hit_ruin_barrier)
        .map(|s| {
            let mid = 0.50 * (s.epistemic_bounds.0 + s.epistemic_bounds.1);
            (mid / s.effective_capacity.max(1.0)).clamp(0.0, 1.0)
        })
        .sum::<f64>()
        / survival_steps.max(1.0);
    let vital_op = VitalMarginState::from_queue_basin_and_survival(
        mean_sizing_util,
        survival_steps,
        1.0,
        4.0,
        1.80,
        10.0,
        (1.0 - report.terminal_basin_radius_ratio).max(0.0) * 3.0,
    );
    let util_slack = ((0.95 - mean_sizing_util) / (0.95 - 0.76)).clamp(0.0, 1.0);
    let reserve_ratio = (report.terminal_reserve / 45.0).clamp(0.0, 1.0);
    let peak_harm = report.max_step_harm.max(report.mean_social_harm);
    let harm_slack = (1.0 - peak_harm / HARM_MAX_ASSUMED).clamp(0.0, 1.0);

    let adroit_steps = report
        .steps
        .iter()
        .filter(|s| {
            matches!(
                s.sosa_grade,
                TelicGrade::FullyAptPerformance | TelicGrade::AdroitExAnteUnluckyOutcome
            )
        })
        .count();
    let obs_apt_rate = report
        .apt_or_adroit_rate
        .max((adroit_steps as f64) / (n_steps as f64));
    let brake_active = obs_apt_rate > 0.10;
    let m_t5_base =
        (0.65 * util_slack + 0.20 * report.terminal_basin_radius_ratio + 0.15 * reserve_ratio)
            .clamp(0.0, 1.0);
    let m_t5 = if brake_active {
        m_t5_base.min(harm_slack)
    } else {
        m_t5_base
    };
    let m_t4 = if brake_active {
        vital_op.vital_margin_score.max(m_t5).min(harm_slack)
    } else {
        vital_op.vital_margin_score.max(m_t5)
    };
    let discount_est = if brake_active { 0.05 } else { 0.65 };
    let proxy_est = if brake_active { 0.10 } else { 0.85 };
    let gov_op = GovernanceState::evaluate_shift_governance(
        1.85,
        10.0,
        discount_est,
        survival_steps.max(6.0),
        proxy_est,
        brake_active,
        (0.75 + 0.20 * obs_apt_rate).clamp(0.20, 0.95),
        brake_active,
        report.ruin_step.is_none(),
    );
    let m_t6 = gov_op.gate1_ex_ante_score.max(gov_op.nested_leontief_score);
    let m_t7 = if brake_active {
        gov_op.nested_leontief_score
    } else {
        gov_op.nested_leontief_score.min(harm_slack)
    };

    let coverage_break = report
        .steps
        .iter()
        .find(|s| !s.epistemic_bounds_covered_truth)
        .map(|s| s.step_t);
    let archive_omission = report
        .steps
        .iter()
        .find(|s| s.raw_archive_mean < s.true_latent_demand)
        .map(|s| s.step_t);
    let lucky = report
        .steps
        .iter()
        .find(|s| s.sosa_grade == TelicGrade::LuckyAccurateOnly)
        .map(|s| s.step_t);
    let guards = vec![
        GuardReading::new(GuardSite::T1Framing, m_t1, coverage_break),
        GuardReading::new(GuardSite::T2Severity, m_t2, None),
        GuardReading::new(GuardSite::T3Archive, m_t3, archive_omission),
        GuardReading::new(GuardSite::T4Formalization, m_t4, None),
        GuardReading::new(GuardSite::T5Temporal, m_t5, report.ruin_step),
        GuardReading::new(GuardSite::T6Performative, m_t6, None),
        GuardReading::new(GuardSite::T7Liability, m_t7, lucky),
    ];
    let shots: Vec<Shot> = report
        .steps
        .iter()
        .map(|s| Shot::attested(s.epistemic_bounds, s.effective_capacity, *s))
        .collect();
    judge(&Sentence::new(shots, guards), threshold)
}

/// Explicit enumeration coverage ledger (`M1 · COB-01`): records requested vs. enumerated
/// guard-vector combinations and whether quantile subsampling truncated the grid at [`MAX_ENUMERATION`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coverage {
    /// Requested combinations $g_{\text{raw}}^7$ before deduplication and subsampling.
    pub requested: usize,
    /// Enumerated combinations $g_{\text{eff}}^7$ after deduplication and subsampling.
    pub enumerated: usize,
    /// Number of distinct finite levels in the effective grid.
    pub distinct_levels: usize,
    /// Whether the deduplicated grid exceeded [`MAX_ENUMERATION`] and was quantile-subsampled to 6 levels.
    pub truncated: bool,
}

/// Deterministic generative budget: depth, threshold, and the finite grid of guard margins.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrammarBudget {
    /// Derivation depth (number of shots); clamped to [`MAX_DEPTH`].
    pub max_depth: usize,
    /// Acceptance threshold $\tau$.
    pub threshold: f64,
    /// Finite grid of admissible margin values.
    pub grid: &'static [f64],
}

impl GrammarBudget {
    /// Clamped depth.
    pub fn depth(&self) -> usize {
        self.max_depth.clamp(1, MAX_DEPTH)
    }

    /// Sorted and deduplicated (`1e-12`) finite grid levels, with `[0.0]` fallback if empty.
    fn deduped_finite_grid(&self) -> Vec<f64> {
        let mut grid: Vec<f64> = self
            .grid
            .iter()
            .copied()
            .filter(|v| v.is_finite())
            .collect();
        if grid.is_empty() {
            grid.push(0.0);
            return grid;
        }
        grid.sort_by(|a, b| a.total_cmp(b));
        let mut deduped = Vec::with_capacity(grid.len());
        for v in grid {
            if deduped
                .last()
                .is_none_or(|prev: &f64| (v - *prev).abs() > 1e-12)
            {
                deduped.push(v);
            }
        }
        deduped
    }

    /// Effective grid: deduplicated non-empty finite levels, preserving min and max via uniform quantile spacing
    /// whenever $g_{\text{dedup}}^7 > \mathrm{MAX\_ENUMERATION}$ ($300\,000$). For $g_{\text{dedup}} \le 6$, returns all levels untouched.
    fn effective_grid(&self) -> Vec<f64> {
        let deduped = self.deduped_finite_grid();
        if deduped.len().saturating_pow(7) <= MAX_ENUMERATION {
            return deduped;
        }
        // Uniform quantile subsampling to 6 levels preserving endpoints (min and max)
        let target_g = 6usize;
        let last = deduped.len() - 1;
        (0..target_g)
            .map(|k| deduped[(k * last) / (target_g - 1)])
            .collect()
    }

    /// Computes the explicit [`Coverage`] ledger (`M1 · COB-01`) for this budget.
    pub fn coverage(&self) -> Coverage {
        let raw_finite_count = self.grid.iter().filter(|v| v.is_finite()).count().max(1);
        let deduped = self.deduped_finite_grid();
        let eff = self.effective_grid();
        let truncated = deduped.len().saturating_pow(7) > MAX_ENUMERATION;
        Coverage {
            requested: raw_finite_count.saturating_pow(7),
            enumerated: eff.len().saturating_pow(7),
            distinct_levels: eff.len(),
            truncated,
        }
    }
}

/// **Generative movement, coverage regime with explicit [`Coverage`] ledger (`M1 · COB-01`):**
/// enumerates every guard vector ($g_{\text{eff}}^7$) over the effective grid up to the declared depth.
pub fn enumerate_with_coverage(budget: &GrammarBudget) -> (Vec<Sentence>, Coverage) {
    let cov = budget.coverage();
    let grid = budget.effective_grid();
    let base_shots: Vec<Shot> = (0..budget.depth())
        .map(|t| Shot::canonical(t, 0.80))
        .collect();
    let mut digits = [0_usize; 7];
    let mut out = Vec::with_capacity(cov.enumerated);
    loop {
        let guards = GuardSite::ALL
            .iter()
            .enumerate()
            .map(|(i, site)| GuardReading::new(*site, grid[digits[i]], Some(0)))
            .collect::<Vec<_>>();
        out.push(Sentence::new(base_shots.clone(), guards));
        let mut pos = 0;
        loop {
            if pos == 7 {
                return (out, cov);
            }
            digits[pos] += 1;
            if digits[pos] < grid.len() {
                break;
            }
            digits[pos] = 0;
            pos += 1;
        }
    }
}

/// **Generative movement, coverage regime (`M1 · COB-01`):** enumerates every guard vector ($g^7$) over the
/// finite grid up to the declared depth (`[Assumed]` combinatorial grid for the enumeration itself,
/// `[Simulated]` for the shots) and always returns the [`Coverage`] ledger (`requested`, `enumerated`,
/// `distinct_levels`, `truncated`) so truncation can never be silently discarded.
pub fn enumerate(budget: &GrammarBudget) -> (Vec<Sentence>, Coverage) {
    enumerate_with_coverage(budget)
}

/// **Generative movement, adversarial regime with explicit [`Coverage`] ledger (`M1 · COB-01`).**
pub fn shortest_violation_with_coverage(
    budget: &GrammarBudget,
) -> (Option<(Sentence, GuardSite)>, Coverage) {
    (shortest_violation(budget), budget.coverage())
}

/// **Generative movement, adversarial regime:** true breadth-first search (`VecDeque`) over derivation
/// `(depth, site_idx, grid_level_idx)` for the shortest derivation and first violated site that triggers collapse
/// according to [`judge`], without pre-filtering the grid by `budget.threshold`.
pub fn shortest_violation(budget: &GrammarBudget) -> Option<(Sentence, GuardSite)> {
    let grid = budget.effective_grid();
    let strongest = grid.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for site_idx in 0..GuardSite::ALL.len() {
        for level_idx in 0..grid.len() {
            queue.push_back((1, site_idx, level_idx));
        }
    }

    while let Some((depth, site_idx, level_idx)) = queue.pop_front() {
        let first_site = GuardSite::ALL[site_idx];
        let step_erosion = (depth.saturating_sub(1) as f64) * BOUNDARY_BAND;
        let candidate_margin = (grid[level_idx] - step_erosion).max(0.0);
        let guards = GuardSite::ALL
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                let margin = if idx == site_idx {
                    candidate_margin
                } else {
                    strongest
                };
                GuardReading::new(*s, margin, Some(depth.saturating_sub(1)))
            })
            .collect::<Vec<_>>();
        let shots = (0..depth)
            .map(|t| Shot::canonical(t, 0.80))
            .collect::<Vec<_>>();
        let sentence = Sentence::new(shots, guards);
        if let GrammarVerdict::Collapse { .. } = judge(&sentence, budget.threshold) {
            return Some((sentence, first_site));
        }
        if depth < budget.depth() {
            queue.push_back((depth + 1, site_idx, level_idx));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{CommunityReserveEnv, StochologicalAgent};

    const TAU: f64 = 0.50;

    fn uniform(margin: f64) -> Vec<GuardReading> {
        GuardSite::ALL
            .iter()
            .map(|s| GuardReading::new(*s, margin, Some(0)))
            .collect()
    }

    fn sentence(margin: f64) -> Sentence {
        Sentence::new(vec![Shot::canonical(0, 0.80)], uniform(margin))
    }

    #[test]
    fn seven_sites_partition_three_two_two() {
        assert_eq!(GuardSite::ALL.len(), 7);
        let count = |axis: TriadAxis| GuardSite::ALL.iter().filter(|s| s.axis() == axis).count();
        assert_eq!(count(TriadAxis::Epistemic), 3);
        assert_eq!(count(TriadAxis::Vital), 2);
        assert_eq!(count(TriadAxis::Institutional), 2);

        let plane_count =
            |plane: Plane| GuardSite::ALL.iter().filter(|s| s.plane() == plane).count();
        assert_eq!(plane_count(Plane::World), 2);
        assert_eq!(plane_count(Plane::Archive), 2);
        assert_eq!(plane_count(Plane::Institution), 3);
        assert_eq!(GuardSite::T5Temporal.plane(), Plane::World);
        assert_eq!(GuardSite::T1Framing.plane(), Plane::Institution);
        assert!(GuardSite::is_bijection_over_circuit_transits());
    }

    #[test]
    fn interior_boundary_and_collapse_are_ordered() {
        assert_eq!(judge(&sentence(0.95), TAU), GrammarVerdict::Interior);
        match judge(&sentence(0.52), TAU) {
            GrammarVerdict::Boundary { margin, .. } => assert!((margin - 0.52).abs() < 1e-12),
            other => panic!("expected Boundary, got {other:?}"),
        }
        match judge(&sentence(0.10), TAU) {
            GrammarVerdict::Collapse {
                proposition, at, ..
            } => {
                assert_eq!(proposition, CollapseProposition::TotalTriadicCollapse);
                assert_eq!(at, Some(GuardSite::T1Framing));
            }
            other => panic!("expected Collapse, got {other:?}"),
        }
    }

    #[test]
    fn single_axis_failure_maps_to_its_proposition() {
        let mut guards = uniform(0.95);
        for g in guards.iter_mut() {
            if g.site.axis() == TriadAxis::Vital {
                g.margin = 0.0;
            }
        }
        let s = Sentence::new(vec![Shot::canonical(0, 0.80)], guards);
        match judge(&s, TAU) {
            GrammarVerdict::Collapse {
                proposition, at, ..
            } => {
                assert_eq!(
                    proposition,
                    CollapseProposition::Proposition2ImpotentLucidity
                );
                assert_eq!(at, Some(GuardSite::T4Formalization));
            }
            other => panic!("expected Collapse, got {other:?}"),
        }
    }

    #[test]
    fn non_finite_threshold_falls_back_to_the_canonical_band() {
        // A non-finite acceptance threshold must not strip the frontier band or the collapse witness:
        // `judge` normalizes it to the canonical 0.50 before any comparison.
        assert_eq!(judge(&sentence(0.95), f64::NAN), GrammarVerdict::Interior);
        assert_eq!(
            judge(&sentence(0.95), f64::INFINITY),
            GrammarVerdict::Interior
        );
        assert_eq!(
            judge(&sentence(0.95), f64::NEG_INFINITY),
            GrammarVerdict::Interior
        );
        match judge(&sentence(0.52), f64::NAN) {
            GrammarVerdict::Boundary { margin, .. } => assert!((margin - 0.52).abs() < 1e-12),
            other => panic!("expected Boundary, got {other:?}"),
        }
        match judge(&sentence(0.10), f64::NAN) {
            GrammarVerdict::Collapse {
                proposition,
                at,
                step_t,
            } => {
                assert_eq!(proposition, CollapseProposition::TotalTriadicCollapse);
                assert_eq!(at, Some(GuardSite::T1Framing));
                assert_eq!(step_t, Some(0));
            }
            other => panic!("expected Collapse, got {other:?}"),
        }
    }

    #[test]
    fn non_finite_margin_is_quarantined_not_satisfied() {
        let mut guards = uniform(0.95);
        guards[2].margin = f64::NAN;
        let s = Sentence::new(vec![Shot::canonical(0, 0.80)], guards);
        assert_eq!(
            judge(&s, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::NonFiniteMargin(GuardSite::T3Archive))
        );
    }

    #[test]
    fn unguarded_transit_and_empty_sentence_are_unparsable() {
        let guards: Vec<GuardReading> = uniform(0.95)
            .into_iter()
            .filter(|g| g.site.axis() != TriadAxis::Institutional)
            .collect();
        let s = Sentence::new(vec![Shot::canonical(0, 0.80)], guards);
        assert_eq!(
            judge(&s, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::UnguardedTransit(
                GuardSite::T6Performative
            ))
        );
        let empty = Sentence::new(Vec::new(), Vec::new());
        assert_eq!(
            judge(&empty, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::EmptySentence)
        );
    }

    #[test]
    fn malformed_telos_is_a_skeleton_mismatch() {
        let mut shot = Shot::canonical(0, 0.80);
        shot.telos.epistemic_demand_bounds = (2.0, 1.0);
        let s = Sentence::new(vec![shot], uniform(0.95));
        assert_eq!(
            judge(&s, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch)
        );
        // S-07: Zero provisioned capacity or empty shots with non-empty guards must be SkeletonMismatch
        let mut zero_cap_shot = Shot::canonical(0, 0.80);
        zero_cap_shot.telos.provisioned_capacity = 0.0;
        assert_eq!(
            judge(&Sentence::new(vec![zero_cap_shot], uniform(0.95)), TAU),
            GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch)
        );
        assert_eq!(
            judge(&Sentence::new(Vec::new(), uniform(0.95)), TAU),
            GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch)
        );
    }

    #[test]
    fn seal_detects_tampering() {
        let mut s = sentence(0.95);
        assert!(s.verify_seal());
        s.guards[0].margin = 0.0;
        assert!(!s.verify_seal());
    }

    #[test]
    fn enumeration_covers_the_grid_and_preserves_seals() {
        static GRID: [f64; 3] = [0.0, 0.60, 0.95];
        let budget = GrammarBudget {
            max_depth: 2,
            threshold: TAU,
            grid: &GRID,
        };
        let (sentences, cov) = enumerate(&budget);
        assert_eq!(sentences.len(), 3_usize.pow(7));
        assert!(!cov.truncated);
        assert!(sentences.iter().all(|s| s.verify_seal()));
        assert!(sentences.iter().all(|s| s.shots.len() == 2));
        let collapses = sentences
            .iter()
            .filter(|s| matches!(judge(s, TAU), GrammarVerdict::Collapse { .. }))
            .count();
        assert_eq!(collapses, 3_usize.pow(7) - 2_usize.pow(7));
    }

    #[test]
    fn shortest_counterexample_is_single_site_at_depth_one() {
        static GRID: [f64; 4] = [0.0, 0.40, 0.60, 0.95];
        let budget = GrammarBudget {
            max_depth: 3,
            threshold: TAU,
            grid: &GRID,
        };
        let (sentence, site) = shortest_violation(&budget).expect("counterexample exists");
        assert_eq!(site, GuardSite::T1Framing);
        assert_eq!(sentence.shots.len(), 1);
        let violators = sentence.guards.iter().filter(|g| g.margin < TAU).count();
        assert_eq!(violators, 1);

        // M11: When the grid sits inside the Boundary band [tau, tau + BOUNDARY_BAND), depth 1 is Boundary
        // and BFS reaches depth 2 as the shortest counterexample!
        static BOUNDARY_GRID: [f64; 2] = [0.52, 0.85];
        let boundary_budget = GrammarBudget {
            max_depth: 2,
            threshold: TAU,
            grid: &BOUNDARY_GRID,
        };
        let (depth2_sentence, depth2_site) =
            shortest_violation(&boundary_budget).expect("depth-2 counterexample exists");
        assert_eq!(depth2_site, GuardSite::T1Framing);
        assert_eq!(depth2_sentence.shots.len(), 2);
    }

    #[test]
    fn no_counterexample_when_every_level_clears_tau() {
        static GRID: [f64; 2] = [0.80, 0.95];
        let budget = GrammarBudget {
            max_depth: 2,
            threshold: TAU,
            grid: &GRID,
        };
        assert!(shortest_violation(&budget).is_none());
    }

    #[test]
    fn grammar_verdict_never_disagrees_with_the_engine() {
        let env = CommunityReserveEnv::new(crate::kernel::DEFAULT_SEED, 24, 45.0);
        let cells = [
            ("111", true, true, true),
            ("011", false, true, true),
            ("101", true, false, true),
            ("110", true, true, false),
            ("100", true, false, false),
            ("010", false, true, false),
            ("001", false, false, true),
            ("000", false, false, false),
        ];
        for (key, e, v, g) in cells {
            let report = env.run_trajectory(&mut StochologicalAgent::with_axes(e, v, g));
            let engine = report.triadic_integrity.diagnose;
            match assess_report(&report, TAU) {
                GrammarVerdict::Collapse { proposition, .. } => assert_eq!(
                    proposition, engine,
                    "cell {key}: grammar and engine disagree on the collapse proposition"
                ),
                GrammarVerdict::Interior | GrammarVerdict::Boundary { .. } => assert_eq!(
                    engine,
                    CollapseProposition::IntactTriad,
                    "cell {key}: engine reports a collapse the grammar failed to see"
                ),
                GrammarVerdict::Unparsable(reason) => {
                    panic!("cell {key}: recognised trajectory was quarantined ({reason:?})")
                }
            }
        }
    }

    #[test]
    fn blind_undersizing_cell_names_its_first_violated_site() {
        let env = CommunityReserveEnv::new(crate::kernel::DEFAULT_SEED, 24, 45.0);
        let report = env.run_trajectory(&mut StochologicalAgent::without_epistemic_margin());
        match assess_report(&report, TAU) {
            GrammarVerdict::Collapse {
                proposition,
                at,
                step_t,
            } => {
                assert_eq!(
                    proposition,
                    CollapseProposition::Proposition1BlindUndersizing
                );
                assert_eq!(at, Some(GuardSite::T1Framing));
                assert_eq!(step_t, Some(0));
            }
            other => panic!("expected Collapse for the 011 ablation, got {other:?}"),
        }
    }

    #[test]
    fn budget_clamps_depth_and_empty_grid() {
        static EMPTY: [f64; 0] = [];
        let budget = GrammarBudget {
            max_depth: 99,
            threshold: TAU,
            grid: &EMPTY,
        };
        assert_eq!(budget.depth(), MAX_DEPTH);
        assert_eq!(budget.effective_grid(), vec![0.0]);
        let (empty_sentences, empty_cov) = enumerate(&budget);
        assert!(!empty_sentences.is_empty());
        assert!(!empty_cov.truncated);
    }

    #[test]
    fn p1_telos_skeleton_is_independent_of_skopos_and_missing_tape_is_quarantined() {
        let mut shot_a = Shot::canonical(0, 0.80);
        shot_a.telos.provisioned_capacity = 0.0;
        shot_a.skopos.hit_ruin_barrier = false;
        let mut shot_b = shot_a;
        shot_b.skopos.hit_ruin_barrier = true;

        assert_eq!(
            judge(&Sentence::new(vec![shot_a], uniform(0.95)), TAU),
            GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch)
        );
        assert_eq!(
            judge(&Sentence::new(vec![shot_b], uniform(0.95)), TAU),
            GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch)
        );

        let env = CommunityReserveEnv::new(crate::kernel::DEFAULT_SEED, 24, 45.0);
        let mut report = env.run_trajectory(&mut StochologicalAgent::full());
        report.steps[0].attested_decision = None;
        assert_eq!(
            assess_report(&report, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::MissingControlTape)
        );

        let mut high_harm_report = env.run_trajectory(&mut StochologicalAgent::full());
        high_harm_report.max_step_harm = 35.0;
        assert!(matches!(
            assess_report(&high_harm_report, TAU),
            GrammarVerdict::Collapse { .. }
        ));
    }

    #[test]
    fn p2_coverage_ledger_reports_truncation_and_deduplicates_before_limit() {
        static GRID_7: [f64; 7] = [0.0, 0.15, 0.30, 0.45, 0.60, 0.75, 0.90];
        let budget_7 = GrammarBudget {
            max_depth: 1,
            threshold: TAU,
            grid: &GRID_7,
        };
        let (violation, cov) = shortest_violation_with_coverage(&budget_7);
        assert!(violation.is_some());
        assert_eq!(cov.requested, 7_usize.pow(7));
        assert_eq!(cov.enumerated, 6_usize.pow(7));
        assert_eq!(cov.distinct_levels, 6);
        assert!(cov.truncated);
        assert_eq!(cov.enumerated, 279_936);

        static DUP_GRID: [f64; 7] = [0.0, 0.0, 0.30, 0.30, 0.70, 0.70, 0.95];
        let dup_budget = GrammarBudget {
            max_depth: 1,
            threshold: TAU,
            grid: &DUP_GRID,
        };
        let (dup_sentences, dup_cov) = enumerate_with_coverage(&dup_budget);
        assert_eq!(dup_cov.requested, 7_usize.pow(7));
        assert_eq!(dup_cov.distinct_levels, 4);
        assert_eq!(dup_cov.enumerated, 4_usize.pow(7));
        assert_eq!(dup_sentences.len(), 4_usize.pow(7));
        assert!(!dup_cov.truncated);
    }

    #[test]
    fn p3_agent_actions_reads_budgets_and_ebnf_extensions_are_enforced() {
        let shots = vec![Shot::canonical(0, 0.80)];
        let guards = uniform(0.95);

        let unauthorized_write = Sentence::with_agent_audit(
            shots.clone(),
            guards.clone(),
            vec![AgentAction {
                kind: AgentActionKind::Write,
                scope: vec!["reserve/vault".to_string()],
                principal: "agent-01".to_string(),
                on_behalf_of: "citizen-board".to_string(),
                contract_id: "contract-2026".to_string(),
                authorized_scopes: vec!["telemetry/read".to_string()],
                can_halt: true,
                token_cost: 10,
            }],
            Vec::new(),
            Some(100),
            Some(10),
        );
        assert_eq!(
            judge(&unauthorized_write, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::OutOfScope)
        );

        let unauthorized_read = Sentence::with_agent_audit(
            shots.clone(),
            guards.clone(),
            Vec::new(),
            vec![ReadReading::new(GuardSite::T3Archive, 0.95, Some(0), false)],
            Some(100),
            Some(10),
        );
        assert_eq!(
            judge(&unauthorized_read, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::UnauthorizedRead)
        );

        let over_budget = Sentence::with_agent_audit(
            shots.clone(),
            guards.clone(),
            vec![AgentAction {
                kind: AgentActionKind::Read,
                scope: vec!["telemetry/read".to_string()],
                principal: "agent-01".to_string(),
                on_behalf_of: "citizen-board".to_string(),
                contract_id: "contract-2026".to_string(),
                authorized_scopes: vec!["telemetry/read".to_string()],
                can_halt: true,
                token_cost: 150,
            }],
            Vec::new(),
            Some(100),
            Some(10),
        );
        assert_eq!(
            judge(&over_budget, TAU),
            GrammarVerdict::Unparsable(UnparsableReason::BudgetExhausted)
        );

        let permitted_agent = Sentence::with_agent_audit(
            shots,
            guards,
            vec![
                AgentAction {
                    kind: AgentActionKind::Read,
                    scope: vec!["telemetry/read".to_string()],
                    principal: "agent-01".to_string(),
                    on_behalf_of: "citizen-board".to_string(),
                    contract_id: "contract-2026".to_string(),
                    authorized_scopes: vec![
                        "telemetry/read".to_string(),
                        "reserve/vault".to_string(),
                    ],
                    can_halt: true,
                    token_cost: 15,
                },
                AgentAction {
                    kind: AgentActionKind::Write,
                    scope: vec!["reserve/vault".to_string()],
                    principal: "agent-01".to_string(),
                    on_behalf_of: "citizen-board".to_string(),
                    contract_id: "contract-2026".to_string(),
                    authorized_scopes: vec![
                        "telemetry/read".to_string(),
                        "reserve/vault".to_string(),
                    ],
                    can_halt: true,
                    token_cost: 20,
                },
                AgentAction {
                    kind: AgentActionKind::Halt,
                    scope: Vec::new(),
                    principal: "agent-01".to_string(),
                    on_behalf_of: "citizen-board".to_string(),
                    contract_id: "contract-2026".to_string(),
                    authorized_scopes: Vec::new(),
                    can_halt: true,
                    token_cost: 5,
                },
            ],
            vec![ReadReading::new(GuardSite::T3Archive, 0.95, Some(0), true)],
            Some(100),
            Some(10),
        );
        assert_eq!(judge(&permitted_agent, TAU), GrammarVerdict::Interior);
    }
}
