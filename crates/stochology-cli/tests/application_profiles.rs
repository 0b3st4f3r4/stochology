//! Shaped applications of Stochology (`ProtocolProfile`) beyond the worked example in
//! `examples/clay/`.
//!
//! Each dataset is **simulated** (`SplitMix64`) and supports no claim about any real system
//! (`AGENTS.md` §II.C). The three applications here exercise complementary clay:
//! - `examples/fleet/`           — cold-chain dispatch, **passive** agent, English, waives `PHI4_CHOULDECHOVA`;
//! - `examples/credit/`          — loan screening, **performative** agent, audits Chouldechova (not waived);
//! - `examples/process_tracing/` — 5th Methodological Family (Process Tracing & Keynes's Evidential Weight $V$),
//!   Portuguese $\Phi_5$ vocabulary, **performative** agent, seed 73.
//!
//! Regenerate with:
//! `cargo test -p stochology-cli --test application_profiles -- --ignored --nocapture`

use std::fs;
use std::path::{Path, PathBuf};

use stochology_cli::{
    compute_empirical_dataset_audit_with_profile, resolve_workspace_root,
    verify_card_against_empirical_data_with_profile,
};
use stochology_core::card::ProtocolProfile;
use stochology_core::{EpistemologicalCard, Rng, sha256_hex};

fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Everything a shaped application needs: directory, column mapping, generative process, and prose.
struct AppConfig {
    dir: &'static str,
    card_path: &'static str,
    csv_path: &'static str,
    seed: u64,
    header: &'static str,
    /// Number of clusters and the (min, max) row span per cluster.
    clusters: usize,
    rows_min: u64,
    rows_span: u64,
    cluster_sd: f64,
    intercept: f64,
    z_coef: f64,
    u_coef: f64,
    noise_sd: f64,
    x_offset: f64,
    card_template: &'static str,
    profile_section: &'static str,
    unit_plural: &'static str,
    cluster_noun: &'static str,
    /// Whether the card's `who_pays_for_error` already carries a Chouldechova audit (no waiver).
    p_star: f64,
}

fn gen_csv(cfg: &AppConfig, root: &Path) -> String {
    let mut rng = Rng::new(cfg.seed);
    let mut csv = String::from(cfg.header);
    for c in 0..cfg.clusters {
        let rows = (cfg.rows_min + (rng.next_u64() % cfg.rows_span)) as usize;
        let u = rng.normal(0.0, cfg.cluster_sd);
        for _r in 0..rows {
            let z = rng.standard_normal();
            let p = logistic(cfg.intercept + u + cfg.z_coef * z);
            let y = u8::from(rng.bernoulli(p));
            let base_x = cfg.u_coef * u + cfg.z_coef * z + rng.normal(0.0, cfg.noise_sd);
            let x = if cfg.x_offset == 0.0 {
                base_x
            } else {
                base_x + cfg.x_offset
            };
            csv.push_str(&format!("C{c:02},{p:.4},{y},{x:.2}\n"));
        }
    }
    fs::write(root.join(cfg.csv_path), &csv).expect("write csv");
    csv
}

fn write_manifest(cfg: &AppConfig, root: &Path, csv: &str) -> String {
    let sha = sha256_hex(csv.as_bytes());
    fs::write(
        root.join(cfg.dir).join("SHA256SUMS"),
        format!("{sha}  {}\n", cfg.csv_path),
    )
    .expect("write manifest");
    sha
}

fn parse_profile(section: &str) -> ProtocolProfile {
    ProtocolProfile::from_card_yaml(section)
        .expect("profile must parse")
        .expect("profile section must be present")
}

/// Generates CSV + manifest, fills the card's empirical numbers and pre-registration seal, and
/// writes the final card YAML (serialized body + hand-written `protocol_profile` section).
fn regenerate(cfg: &AppConfig, root: &Path) {
    fs::create_dir_all(root.join(cfg.dir)).expect("create example dir");
    let csv = gen_csv(cfg, root);
    let sha = write_manifest(cfg, root, &csv);

    let mut card = EpistemologicalCard::from_yaml(cfg.card_template).expect("template must parse");
    let profile = parse_profile(cfg.profile_section);

    card.data_and_audit_contract.dataset_sha256 = sha.clone();
    let emp = compute_empirical_dataset_audit_with_profile(root, &card, &profile)
        .expect("empirical audit must run");

    // Fill every empirical quantity the card must declare (tolerances are tight).
    let um = &mut card.epistemological_header.unit_and_margin;
    um.nominal_sample_size_n = emp.n_nominal;
    um.effective_sample_size_n_eff = emp.n_eff_exceedance_90;
    let mult = emp.m_tilde / emp.m_bar.max(1e-9);
    let cv = (mult - 1.0).max(0.0).sqrt();
    um.weighted_cluster_mean_m_tilde = format!(
        "m_tilde = m_bar * (1 + CV^2) = {:.2} * {:.3} = {:.2} (Moulton-Campbell, CV = {:.3})",
        emp.m_bar, mult, emp.m_tilde, cv
    );
    um.threshold_exceedance_icc = format!(
        "rho_I(0.90) = {:.4} -> DEFF_exceedance = {:.2}",
        emp.rho_exceedance_90, emp.deff_exceedance_90
    );
    let p = emp.event_probability_p;
    let moe = 1.96 * (p * (1.0 - p) / (emp.n_eff_exceedance_90 as f64)).sqrt() * 100.0;
    um.unit_margin_of_error = format!(
        "+/- {moe:.1} percentage points over {} effective exceedance-adjusted {} ({} {})",
        emp.n_eff_exceedance_90, cfg.unit_plural, emp.num_clusters, cfg.cluster_noun
    );

    let js = &emp.jensen_summary;
    card.epistemological_header.nonlinearity = format!(
        "f(E[X])={:.2}, E[f(X)]={:.2}, Jensen gap Delta_J={:+.2}, median={:.2}, q05={:.2}, q95={:.2}, CRPS={:.2}",
        js.f_at_mean,
        js.expected_f,
        js.jensen_gap,
        js.median_f,
        js.q05_f,
        js.q95_f,
        js.empirical_crps
    );

    card.decision_rule_and_scope.event_probability_p = p;
    card.decision_rule_and_scope.declared_act = if p > cfg.p_star {
        "protect".to_string()
    } else {
        "do_not_protect".to_string()
    };
    card.uncertainty_budget_and_authorship
        .dominant_event
        .probability_p = p;

    let mc = &mut card.data_and_audit_contract.murphy_corp_decomposition;
    mc.total_score_brier_or_crps = emp.corp_pav.brier_total;
    mc.miscalibration_rel_mcb = emp.corp_pav.rel_miscalibration;
    mc.resolution_res_dsc = emp.corp_pav.res_resolution;
    mc.base_uncertainty_unc0 = emp.corp_pav.unc0_uncertainty;

    let e_t: f64 = emp.ville_e_values.iter().product();
    let inv_alpha = profile.e_process_min_inverse_alpha;
    card.data_and_audit_contract.e_process_multiplicity = format!(
        "Cumulative Ville e-process E_{} = {e_t:.1} (threshold 1/alpha = {inv_alpha:.1} for alpha = {:.3})",
        emp.ville_e_values.len(),
        1.0 / inv_alpha
    );
    card.data_and_audit_contract.preregistration_hash = profile.preregistration_hash_for(&card);

    // Serialize the (now correct) card body and re-attach the hand-written profile section.
    let body = card.to_yaml();
    let final_yaml = format!("{body}\n{}", cfg.profile_section);
    fs::write(root.join(cfg.card_path), &final_yaml).expect("write card");

    // Confirm end-to-end: contract audit + empirical severe test + SIGA roadmap.
    let (emp2, siga) = verify_card_against_empirical_data_with_profile(root, &card, &profile)
        .expect("generated card must pass end-to-end");
    assert_eq!(emp2.dataset_sha256, sha);
    assert!(siga.approved_end_to_end);

    println!(
        "[{}] n={} clusters={} m_tilde={:.2} rho_I={:.4} DEFF={:.2} n_eff={}",
        cfg.dir,
        emp.n_nominal,
        emp.num_clusters,
        emp.m_tilde,
        emp.rho_exceedance_90,
        emp.deff_exceedance_90,
        emp.n_eff_exceedance_90
    );
    println!(
        "[{}] p={:.5} CORP BS={:.5} REL={:.5} RES={:.5} UNC0={:.5}",
        cfg.dir,
        emp.event_probability_p,
        emp.corp_pav.brier_total,
        emp.corp_pav.rel_miscalibration,
        emp.corp_pav.res_resolution,
        emp.corp_pav.unc0_uncertainty
    );
    println!(
        "[{}] E_t={e_t:.1} null_p={:.4} sha={}",
        cfg.dir, siga.null_p_value, sha
    );
    println!(
        "[{}] preregistration_hash={}",
        cfg.dir,
        profile.preregistration_hash_for(&card)
    );
}

const FLEET_TEMPLATE: &str = r#"id: "cold_chain_spoilage_dispatch"
layer: "L2"
uncertainty: "epistemic_parameter"

decision_maker_question:
  who_decides: "Cold-Chain Logistics Dispatch Supervisor"
  altered_action: "Dispatch a refrigerated truck instead of the standard van for a route when the calibrated spoilage probability exceeds p*"
  question: "Does the calibrated spoilage probability for this route's delivery exceed the cost-loss threshold p* = 0.25000?"

epistemological_header:
  generating_process: "Simulated logistic spoilage process with route random effects u_r ~ N(0, 0.5^2) and per-trip ambient temperature pressure z ~ N(0, 1) (SplitMix64, seed 40)"
  nonlinearity: "f(E[X])=0.00, E[f(X)]=0.00, Jensen gap Delta_J=+0.00, median=0.00, q05=0.00, q95=0.00, CRPS=0.00"
  unit_and_margin:
    independent_unit: "route across daily cold-chain deliveries (cluster of days per route)"
    nominal_sample_size_n: 0
    weighted_cluster_mean_m_tilde: "m_tilde = 0.00 (Moulton-Campbell)"
    threshold_exceedance_icc: "rho_I(0.90) = 0.00"
    effective_sample_size_n_eff: 0
    unit_margin_of_error: "+/- 0.0 percentage points"
  noise_physics:
    stochastic_regime: "Bernoulli spoilage with logistic link and between-route heterogeneity (simulated)"
    absorbing_states: "Cold-chain break: spoiled goods must be destroyed and the delivery rescheduled at full cost"
    bridge_L3_L1_performativity:
      agent_type: "passive"
      wasserstein_sensitivity_epsilon: 0.0
      rrm_convergence_condition: "eps * beta / gamma = 0.00 * 1.00 / 1.00 = 0.00 < 1 (strict contraction)"
      stability_optimality_divergence: "passive agent: no Decision -> World feedback, theta_PS and theta_PO coincide"
  causal_ladder:
    pearl_rung: "2-Act"
    identification_strategy: "Simulated intervention on truck dispatch; Markov blanket of spoilage mapped (route risk, ambient temperature) without collider conditioning"
    untestable_assumption: "No unobserved surge shifting ambient temperature and dispatch at once (Lambda* <= 1.25)"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: 0.50
    parameters: 0.30
    covariance: 0.20
    structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) for route-level logit discrepancy per Kennedy-O'Hagan"
  dominant_event:
    description: "Spoilage of a cold-chain delivery on a given route-day"
    probability_p: 0.0
    base_rate_floor_epsilon: 0.001
    cost_share: 0.85
  info_gap_robustness:
    critical_requirement_rc: "Fewer than 1 spoiled delivery per route per 20 route-days"
    robustness_horizon_alpha_hat: "alpha_hat = 2.20 standard deviations of route risk tolerated without violating r_c"
    opportuneness_horizon_beta_hat: "beta_hat = 0.80 deviations for windfall shelf-life extension r_w"
    global_maximin_safeguard: "Wald global Maximin floor: at least one standby refrigerated truck on call, with reversible recall at the next dispatch window"
  field_authorship:
    null_model: "cold_chain_operations_modeling_group"
    decision_rule: "logistics_dispatch_planning"
    adversarial_reviewer: "independent_food_safety_auditor"

data_and_audit_contract:
  dataset_path: "examples/fleet/cold_chain_spoilage.csv"
  dataset_sha256: "0000000000000000000000000000000000000000000000000000000000000000"
  missingness_mechanism: "MCAR (simulated dataset with no missing route-days; Meng correlation rho_R_Y = 0; Manski bounds closed)"
  seed_and_environment: "seed=40; rustc=1.97; simulated dataset"
  null_model: "crates/stochology-cli#evaluate_empirical_card_severe_test_with_profile"
  preregistration_hash: "0000000000000000000000000000000000000000000000000000000000000000"
  attempt_counter_K: 1
  e_process_multiplicity: "Cumulative Ville e-process E_30 = 0.0 (threshold 1/alpha = 40.0 for alpha = 0.025)"
  murphy_corp_decomposition:
    total_score_brier_or_crps: 0.0
    miscalibration_REL_MCB: 0.0
    resolution_RES_DSC: 0.0
    base_uncertainty_UNC0: 0.0
  findings:
    registered: 2
    used: 1
    unused:
      - "Day-of-week spoilage effect discarded: no incremental RES_CORP on the development half"

decision_rule_and_scope:
  event_probability_p: 0.0
  base_rate_floor_epsilon: 0.001
  protection_cost_C: 150.0
  event_loss_L: 1200.0
  mitigation_efficacy_eta: 0.5
  critical_threshold_p_star: 0.25000
  declared_act: "protect"
  scope_statement: "Worked example on simulated route-days; does not apply to pharmaceutical air freight or to any real cold-chain network"
  who_pays_for_error: "Chouldechova instrument waived in protocol_profile: the action is allocated per route-day by forecast spoilage risk, not across customer groups with distinct base rates"
  exit_clause: "Suspend automated refrigerated-truck dispatch if REL_CORP > 0.020 or RES_CORP < 0.020 over 14 consecutive route-days"
"#;

const FLEET_PROFILE: &str = r#"protocol_profile:
  name: "cold_chain_spoilage_dispatch"
  rationale: "Route-level cold-chain dispatch asked in English; stricter anytime-valid evidence (1/alpha = 40); passive agent with no Decision -> World feedback; dataset kept outside data/ under its own manifest"
  seed: 40
  e_process_min_inverse_alpha: 40.0
  dataset_root: "examples/fleet/"
  dataset_manifest: "examples/fleet/SHA256SUMS"
  dataset_columns:
    cluster: "route"
    forecast: "spoilage_prob"
    outcome: "spoiled"
    value: "loss_cost"
    jensen_loss: "square"
  severe_test:
    permutation_replications: 500
  waivers:
    PHI4_CHOULDECHOVA: "The action is allocated per route-day by forecast spoilage risk; no protective action is split across customer groups with distinct base rates"
"#;

const CREDIT_TEMPLATE: &str = r#"id: "loan_default_screening"
layer: "L2"
uncertainty: "epistemic_parameter"

decision_maker_question:
  who_decides: "Retail Credit Risk Committee"
  altered_action: "Route the loan application to manual underwriting review when the calibrated default probability exceeds p*"
  question: "Does the calibrated default probability for this loan application exceed the cost-loss threshold p* = 0.20000?"

epistemological_header:
  generating_process: "Simulated logistic default process with branch random effects u_b ~ N(0, 0.5^2) and per-application credit score signal z ~ N(0, 1) (SplitMix64, seed 41)"
  nonlinearity: "f(E[X])=0.00, E[f(X)]=0.00, Jensen gap Delta_J=+0.00, median=0.00, q05=0.00, q95=0.00, CRPS=0.00"
  unit_and_margin:
    independent_unit: "branch across daily loan applications (cluster of applications per branch)"
    nominal_sample_size_n: 0
    weighted_cluster_mean_m_tilde: "m_tilde = 0.00 (Moulton-Campbell)"
    threshold_exceedance_icc: "rho_I(0.90) = 0.00"
    effective_sample_size_n_eff: 0
    unit_margin_of_error: "+/- 0.0 percentage points"
  noise_physics:
    stochastic_regime: "Bernoulli default with logistic link and between-branch heterogeneity (simulated)"
    absorbing_states: "Default cascade: a defaulted loan consumes the branch's loss reserve and tightens later credit"
    bridge_L3_L1_performativity:
      agent_type: "performative"
      wasserstein_sensitivity_epsilon: 0.10
      rrm_convergence_condition: "eps * beta / gamma = 0.10 * 1.50 / 1.00 = 0.15 < 1 (strict contraction)"
      stability_optimality_divergence: "theta_PS = 0.210 vs Stackelberg theta_PO = 0.203 (screening shifts the default rate it forecasts)"
  causal_ladder:
    pearl_rung: "2-Act"
    identification_strategy: "Simulated intervention on credit screening; Markov blanket of default mapped (branch risk, credit score) without collider conditioning"
    untestable_assumption: "No unobserved economic shock shifting branch risk and screening at once (Lambda* <= 1.25)"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: 0.50
    parameters: 0.30
    covariance: 0.20
    structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) for branch-level logit discrepancy per Kennedy-O'Hagan"
  dominant_event:
    description: "Default of a retail loan application at a given branch"
    probability_p: 0.0
    base_rate_floor_epsilon: 0.001
    cost_share: 0.80
  info_gap_robustness:
    critical_requirement_rc: "Fewer than 1 undetected default per branch per 40 applications"
    robustness_horizon_alpha_hat: "alpha_hat = 2.40 standard deviations of branch risk tolerated without violating r_c"
    opportuneness_horizon_beta_hat: "beta_hat = 0.90 deviations for windfall portfolio recovery r_w"
    global_maximin_safeguard: "Wald global Maximin floor: every auto-approval retains a reversible human review override at the next underwriting cycle"
  field_authorship:
    null_model: "retail_credit_modeling_group"
    decision_rule: "underwriting_risk_committee"
    adversarial_reviewer: "independent_fair_lending_auditor"

data_and_audit_contract:
  dataset_path: "examples/credit/loan_default_screening.csv"
  dataset_sha256: "0000000000000000000000000000000000000000000000000000000000000000"
  missingness_mechanism: "MCAR (simulated dataset with no missing applications; Meng correlation rho_R_Y = 0; Manski bounds closed)"
  seed_and_environment: "seed=41; rustc=1.97; simulated dataset"
  null_model: "crates/stochology-cli#evaluate_empirical_card_severe_test_with_profile"
  preregistration_hash: "0000000000000000000000000000000000000000000000000000000000000000"
  attempt_counter_K: 1
  e_process_multiplicity: "Cumulative Ville e-process E_30 = 0.0 (threshold 1/alpha = 40.0 for alpha = 0.025)"
  murphy_corp_decomposition:
    total_score_brier_or_crps: 0.0
    miscalibration_REL_MCB: 0.0
    resolution_RES_DSC: 0.0
    base_uncertainty_UNC0: 0.0
  findings:
    registered: 2
    used: 1
    unused:
      - "Seasonal spending effect discarded: no incremental RES_CORP on the development half"

decision_rule_and_scope:
  event_probability_p: 0.0
  base_rate_floor_epsilon: 0.001
  protection_cost_C: 800.0
  event_loss_L: 5000.0
  mitigation_efficacy_eta: 0.8
  critical_threshold_p_star: 0.20000
  declared_act: "protect"
  scope_statement: "Worked example on simulated loan applications; does not apply to secured mortgage underwriting or to any real lender"
  who_pays_for_error: "Chouldechova trilemma audit: marginal vs prime applicants (p_A=0.3100 vs p_B=0.1800); equalizing PPV at 0.60 & FNR at 0.30 implies monitored ratio FPR_A/FPR_B = 3.40"
  exit_clause: "Suspend automated credit screening if REL_CORP > 0.020 or RES_CORP < 0.020 over 14 consecutive business days"
"#;

const CREDIT_PROFILE: &str = r#"protocol_profile:
  name: "loan_default_screening"
  rationale: "Application-level credit screening asked in English; stricter anytime-valid evidence (1/alpha = 40); performative agent audited on Bridge L3 -> L1; Chouldechova trilemma audited, not waived; dataset kept outside data/ under its own manifest"
  seed: 41
  e_process_min_inverse_alpha: 40.0
  dataset_root: "examples/credit/"
  dataset_manifest: "examples/credit/SHA256SUMS"
  dataset_columns:
    cluster: "branch"
    forecast: "default_prob"
    outcome: "defaulted"
    value: "loss_amount"
    jensen_loss: "square"
  severe_test:
    permutation_replications: 500
"#;

const PROCESS_TRACING_TEMPLATE: &str = r#"id: "epidemiological_field_process_tracing"
layer: "L2"
uncertainty: "epistemic_parameter"

decision_maker_question:
  who_decides: "Comissão de Investigação Epidemiológica de Campo"
  altered_action: "Emitir alerta de contenção focal e rastreamento laboratorial para o setor neste turno quando a probabilidade calibrada de transmissão ativa exceder p*"
  question: "A probabilidade calibrada de transmissão ativa neste turno de investigação de campo excede o limiar custo-perda p* = 0.20000?"

epistemological_header:
  generating_process: "Simulated logistic outbreak tracing process with sector random effects u_s ~ N(0, 0.5^2) and per-shift diagnostic trace signal z ~ N(0, 1) (SplitMix64, seed 73)"
  nonlinearity: "f(E[X])=0.00, E[f(X)]=0.00, Jensen gap Delta_J=+0.00, median=0.00, q05=0.00, q95=0.00, CRPS=0.00"
  unit_and_margin:
    independent_unit: "setor sanitário ao longo dos turnos de investigação de campo (cluster of field shifts per sector)"
    nominal_sample_size_n: 0
    weighted_cluster_mean_m_tilde: "m_tilde = 0.00 (Moulton-Campbell)"
    threshold_exceedance_icc: "rho_I(0.90) = 0.00"
    effective_sample_size_n_eff: 0
    unit_margin_of_error: "+/- 0.0 percentage points"
  noise_physics:
    stochastic_regime: "Bernoulli outbreak confirmation with logistic link and between-sector heterogeneity (simulated)"
    absorbing_states: "Uncontained community cluster: secondary transmission spreads across adjacent sectors once early containment window closes"
    bridge_L3_L1_performativity:
      agent_type: "performative"
      wasserstein_sensitivity_epsilon: 0.12
      rrm_convergence_condition: "eps * beta / gamma = 0.12 * 1.25 / 1.00 = 0.15 < 1 (strict contraction)"
      stability_optimality_divergence: "theta_PS = 0.214 vs Stackelberg theta_PO = 0.206 (focal containment shifts the transmission rate it forecasts)"
  causal_ladder:
    pearl_rung: "2-Act"
    identification_strategy: "Simulated process tracing across Hoop Test and Smoking Gun diagnostic cues; Markov blanket of transmission mapped (sector risk, diagnostic trace) without collider conditioning"
    untestable_assumption: "No unobserved reporting blackout shifting sector risk and field tracing at once (Lambda* <= 1.25)"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: 0.50
    parameters: 0.30
    covariance: 0.20
    structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) for sector-level logit discrepancy per Kennedy-O'Hagan"
  dominant_event:
    description: "Active outbreak transmission confirmed in a sector during a field investigation shift"
    probability_p: 0.0
    base_rate_floor_epsilon: 0.001
    cost_share: 0.85
  info_gap_robustness:
    critical_requirement_rc: "Fewer than 1 uncontained secondary cluster per sector per 30 investigation shifts"
    robustness_horizon_alpha_hat: "alpha_hat = 2.30 standard deviations of sector risk tolerated without violating r_c"
    opportuneness_horizon_beta_hat: "beta_hat = 0.85 deviations for windfall early clearance r_w"
    global_maximin_safeguard: "Wald global Maximin floor: at least one mobile rapid-response field team on standby, with reversible recall at the next shift briefing"
  field_authorship:
    null_model: "epidemiological_surveillance_modeling_group"
    decision_rule: "field_outbreak_response_committee"
    adversarial_reviewer: "independent_public_health_auditor"

data_and_audit_contract:
  dataset_path: "examples/process_tracing/epidemiological_field_archive.csv"
  dataset_sha256: "0000000000000000000000000000000000000000000000000000000000000000"
  missingness_mechanism: "MCAR (simulated dataset with no missing shifts; Meng correlation rho_R_Y = 0; Manski bounds closed; Keynes evidential weight V audited)"
  seed_and_environment: "seed=73; rustc=1.97; simulated dataset"
  null_model: "crates/stochology-cli#evaluate_empirical_card_severe_test_with_profile"
  preregistration_hash: "0000000000000000000000000000000000000000000000000000000000000000"
  attempt_counter_K: 1
  e_process_multiplicity: "Cumulative Ville e-process E_30 = 0.0 (threshold 1/alpha = 40.0 for alpha = 0.025)"
  murphy_corp_decomposition:
    total_score_brier_or_crps: 0.0
    miscalibration_REL_MCB: 0.0
    resolution_RES_DSC: 0.0
    base_uncertainty_UNC0: 0.0
  findings:
    registered: 2
    used: 1
    unused:
      - "Straw-in-the-wind ambient humidity cue discarded: low Keynes evidential weight V and no incremental RES_CORP on the development half"

decision_rule_and_scope:
  event_probability_p: 0.0
  base_rate_floor_epsilon: 0.001
  protection_cost_C: 600.0
  event_loss_L: 4000.0
  mitigation_efficacy_eta: 0.75
  critical_threshold_p_star: 0.20000
  declared_act: "protect"
  scope_statement: "Worked example of the 5th Methodological Family (Process Tracing & Evidential Weight V) on simulated field shifts; does not apply to clinical trials or to any real epidemiological surveillance system"
  who_pays_for_error: "Chouldechova trilemma audit: high-density vs low-density sectors (p_A=0.3100 vs p_B=0.1800); equalizing PPV at 0.60 & FNR at 0.30 implies monitored ratio FPR_A/FPR_B = 3.40"
  exit_clause: "Suspend automated focal containment alerts if REL_CORP > 0.020 or RES_CORP < 0.020 over 14 consecutive field shifts"
"#;

const PROCESS_TRACING_PROFILE: &str = r#"protocol_profile:
  name: "epidemiological_field_process_tracing"
  rationale: "Shift-level 5th Methodological Family (Process Tracing & Keynes Evidential Weight V) decision asked in Portuguese; stricter anytime-valid evidence (1/alpha = 40); performative agent and Chouldechova trilemma audited; dataset kept outside data/ under its own manifest"
  seed: 73
  e_process_min_inverse_alpha: 40.0
  resolution:
    fine_unit_terms: "plantão, plantões, turno, turnos, hora"
    coarse_horizon_terms: "anual, mensal, trimestral"
    coarse_aggregation_terms: "média, total, soma, único"
    coarse_phrases: "todos os turnos do ano"
  dataset_root: "examples/process_tracing/"
  dataset_manifest: "examples/process_tracing/SHA256SUMS"
  dataset_columns:
    cluster: "sector"
    forecast: "outbreak_prob"
    outcome: "confirmed"
    value: "case_load"
    jensen_loss: "square"
  severe_test:
    permutation_replications: 500
    sbc_replications: 300
"#;

fn fleet_config() -> AppConfig {
    AppConfig {
        dir: "examples/fleet",
        card_path: "examples/fleet/card_fleet_cold_chain_spoilage.yaml",
        csv_path: "examples/fleet/cold_chain_spoilage.csv",
        seed: 40,
        header: "route,spoilage_prob,spoiled,loss_cost\n",
        clusters: 30,
        rows_min: 20,
        rows_span: 21,
        cluster_sd: 0.5,
        intercept: -1.0,
        z_coef: 1.5,
        u_coef: 4.0,
        noise_sd: 1.2,
        x_offset: 0.0,
        card_template: FLEET_TEMPLATE,
        profile_section: FLEET_PROFILE,
        unit_plural: "route-days",
        cluster_noun: "routes",
        p_star: 0.25,
    }
}

fn credit_config() -> AppConfig {
    AppConfig {
        dir: "examples/credit",
        card_path: "examples/credit/card_credit_loan_default_screening.yaml",
        csv_path: "examples/credit/loan_default_screening.csv",
        seed: 41,
        header: "branch,default_prob,defaulted,loss_amount\n",
        clusters: 25,
        rows_min: 20,
        rows_span: 21,
        cluster_sd: 0.5,
        intercept: -1.0,
        z_coef: 1.5,
        u_coef: 3.0,
        noise_sd: 1.2,
        x_offset: 0.0,
        card_template: CREDIT_TEMPLATE,
        profile_section: CREDIT_PROFILE,
        unit_plural: "applications",
        cluster_noun: "branches",
        p_star: 0.20,
    }
}

fn process_tracing_config() -> AppConfig {
    AppConfig {
        dir: "examples/process_tracing",
        card_path: "examples/process_tracing/card_process_tracing_epidemiological_inquiry.yaml",
        csv_path: "examples/process_tracing/epidemiological_field_archive.csv",
        seed: 73,
        header: "sector,outbreak_prob,confirmed,case_load\n",
        clusters: 28,
        rows_min: 20,
        rows_span: 21,
        cluster_sd: 0.5,
        intercept: -1.4,
        z_coef: 1.8,
        u_coef: 3.2,
        noise_sd: 1.2,
        x_offset: 1.5,
        card_template: PROCESS_TRACING_TEMPLATE,
        profile_section: PROCESS_TRACING_PROFILE,
        unit_plural: "field-shifts",
        cluster_noun: "sectors",
        p_star: 0.20,
    }
}

#[test]
#[ignore]
fn regenerate_application_profiles() {
    let root = resolve_workspace_root();
    for cfg in [fleet_config(), credit_config(), process_tracing_config()] {
        regenerate(&cfg, &root);
    }
}

fn load_app(cfg: &AppConfig, root: &Path) -> (String, EpistemologicalCard, ProtocolProfile) {
    let yaml = fs::read_to_string(root.join(cfg.card_path)).expect("card must exist");
    let card = EpistemologicalCard::from_yaml(&yaml).expect("card must parse");
    let profile = parse_profile(cfg.profile_section);
    (yaml, card, profile)
}

#[test]
fn application_profiles_pass_end_to_end() {
    let root = resolve_workspace_root();
    for cfg in [fleet_config(), credit_config(), process_tracing_config()] {
        let (_, card, profile) = load_app(&cfg, &root);
        let (_, siga) = verify_card_against_empirical_data_with_profile(&root, &card, &profile)
            .unwrap_or_else(|e| panic!("{} must pass end-to-end: {e}", cfg.dir));
        assert!(siga.approved_end_to_end, "{} SIGA roadmap", cfg.dir);
    }
}

#[test]
fn fleet_waives_chouldechova_and_credit_does_not() {
    let root = resolve_workspace_root();
    let (_, fleet_card, fleet_profile) = load_app(&fleet_config(), &root);
    let report = fleet_card
        .audit_with(&fleet_profile)
        .expect("fleet must audit");
    assert_eq!(report.profile_name, "cold_chain_spoilage_dispatch");
    assert!(
        report
            .applied_waivers
            .iter()
            .any(|w| w.gate_code == "PHI4_CHOULDECHOVA")
    );

    let (_, credit_card, credit_profile) = load_app(&credit_config(), &root);
    let report = credit_card
        .audit_with(&credit_profile)
        .expect("credit must audit");
    assert_eq!(report.profile_name, "loan_default_screening");
    assert!(
        report.applied_waivers.is_empty(),
        "credit audits all instruments"
    );
    assert!(
        credit_card
            .decision_rule_and_scope
            .parsed_chouldechova_metrics()
            .is_some_and(|m| m.is_non_unit_ratio_beyond(1e-3))
    );
}

#[test]
fn each_profile_fails_under_the_canonical_profile() {
    let root = resolve_workspace_root();
    let canonical = ProtocolProfile::canonical();
    for cfg in [fleet_config(), credit_config(), process_tracing_config()] {
        let (_, card, _) = load_app(&cfg, &root);
        let codes: Vec<&str> = card
            .audit_with(&canonical)
            .err()
            .unwrap_or_default()
            .iter()
            .map(|v| v.gate_code)
            .collect();
        for expected in [
            "PHI1_SEED",
            "PHI3_DATASET_PROVENANCE",
            "PHI3_PREREGISTRATION_HASH",
        ] {
            assert!(
                codes.contains(&expected),
                "{:?} missing {expected} in {codes:?}",
                cfg.dir
            );
        }
    }
}

#[test]
fn application_manifest_seals_its_dataset() {
    let root = resolve_workspace_root();
    for cfg in [fleet_config(), credit_config(), process_tracing_config()] {
        let manifest = fs::read_to_string(root.join(cfg.dir).join("SHA256SUMS")).expect("manifest");
        let (sha, rel) = manifest.trim().split_once("  ").expect("manifest line");
        let data = fs::read_to_string(root.join(rel)).expect("csv");
        assert_eq!(sha256_hex(data.as_bytes()), sha, "{} manifest", cfg.dir);
    }
}

#[test]
fn generator_round_trip_is_deterministic() {
    let tmp = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("app_profiles_roundtrip");
    fs::create_dir_all(&tmp).expect("tmp dir");
    let cfg = fleet_config();
    regenerate(&cfg, &tmp);
    let a = fs::read_to_string(tmp.join(cfg.card_path)).expect("card a");
    regenerate(&cfg, &tmp);
    let b = fs::read_to_string(tmp.join(cfg.card_path)).expect("card b");
    assert_eq!(a, b, "regeneration must be byte-for-byte deterministic");
}
