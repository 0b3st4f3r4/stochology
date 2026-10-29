//! Stock-Flow Consistent (SFC) 4-Sector Transaction-Flow Matrix (`SfcFlowMatrix`, `SfcCellSummary`)
//! and preregistered constants for the Coupled Public-Private Society synthesis.

/// Assumed: Pre-registration SHA-256 seal frozen in `data/preregistration/coupled_society_preregistration.yaml`.
pub const COUPLED_SOCIETY_PREREGISTRATION_HASH: &str =
    "82e6cf8431de6e6a7fbad2417d7dacaf49cc1723c1759252079e0c850815008a";

/// Assumed: Number of households $N_H = 120$ in the closed SFC economy.
pub const SOCIETY_HOUSEHOLDS_N_H: usize = 120;

/// Assumed: Number of firms $N_F = 30$ in the closed SFC economy.
pub const SOCIETY_FIRMS_N_F: usize = 30;

/// Assumed: Simulation horizon $T = 60$ epochs (shifts/cycles).
pub const SOCIETY_HORIZON_EPOCHS_T: usize = 60;

/// Assumed: Supply-chain Pareto shock onset epoch $t_0 = 10$.
pub const SOCIETY_SHOCK_EPOCH_T0: usize = 10;

/// Assumed: Heavy-tailed Pareto shock exponent $\alpha = 2.25$.
pub const SOCIETY_PARETO_SHOCK_ALPHA: f64 = 2.25;

/// Assumed: Pareto shock minimum scale $x_m = 0.18$.
pub const SOCIETY_PARETO_SHOCK_SCALE_XM: f64 = 0.18;

/// Assumed: Fraction of firms directly hit by the supply-chain shock $f_{\text{shock}} = 0.25$.
pub const SOCIETY_SHOCKED_FIRM_FRACTION: f64 = 0.25;

/// Assumed: Bitemporal reporting lag $\Delta t_{\text{disp}} = 2$ epochs ($t_{\text{disp}} = t_{\text{ev}} + 2$).
pub const SOCIETY_BITEMPORAL_LAG_DELTA_T: usize = 2;

/// Assumed: Target administrative `MNAR` selection correlation $\rho_{R,Y} = -0.15$.
pub const SOCIETY_MNAR_CORR_RHO_RY: f64 = -0.15;

/// Assumed: Observed administrative filing fraction $f_{\text{obs}} = 0.75$.
pub const SOCIETY_MNAR_OBS_FRACTION: f64 = 0.75;

/// Assumed: Household Constant Relative Risk Aversion coefficient $\gamma = 2.0$ (Carroll 1997).
pub const SOCIETY_CRRA_GAMMA: f64 = 2.0;

/// Informative: Bester (1985, *AER*) safe project success probability $p_G = 0.92$ (Assumed numerical calibration).
pub const BESTER_SUCCESS_PROB_GOOD: f64 = 0.92;

/// Informative: Bester (1985, *AER*) risky project success probability $p_B = 0.65$ (Assumed numerical calibration).
pub const BESTER_SUCCESS_PROB_BAD: f64 = 0.65;

/// Informative: Bester (1985, *AER*) bank collateral valuation recovery factor $\kappa = 0.80$ (20% repossession deadweight loss).
pub const BESTER_COLLATERAL_RECOVERY_KAPPA: f64 = 0.80;

/// Assumed: Preregistered grid of safe borrower shares $\lambda_{\text{good}} \in \{0.50, 0.88\}$.
pub const BESTER_GOOD_FRACTION_GRID: [f64; 2] = [0.50, 0.88];

/// Assumed: Preregistered VCG cohort size grid $M \in \{4, 8, 16, 32\}$.
pub const VCG_COHORT_SIZE_GRID_M: [usize; 4] = [4, 8, 16, 32];

/// Assumed: Per-capita public-good provision cost $c = 0.85$ in the Clarke VCG mechanism.
pub const VCG_PUBLIC_GOOD_COST_PER_CAPITA: f64 = 0.85;

/// Assumed: Critical cost-loss vulnerability threshold $p^* = \frac{C}{\eta L} = 0.12000$.
pub const SOCIETY_CRITICAL_THRESHOLD_P_STAR: f64 = 0.12000;

/// Stock-Flow Consistent (SFC) 4-Sector Transaction-Flow Matrix at a single epoch $t$
/// (`Informative:` Godley & Lavoie 2007, *Monetary Economics*, Table 2.2; Kalecki 1954, *Theory of Economic Dynamics*).
#[derive(Debug, Clone, PartialEq)]
pub struct SfcFlowMatrix {
    /// 7 rows by 5 sectoral columns of signed nominal flows.
    pub entries: [[f64; 5]; 7],
    /// Maximum absolute row-sum residual $\max_r |\sum_c M_{r,c}|$.
    pub max_abs_row_sum_error: f64,
    /// Maximum absolute column-sum residual $\max_c |\sum_r M_{r,c}|$.
    pub max_abs_col_sum_error: f64,
    /// Kalecki-Levy profit identity residual $|\Pi_t - (I_t + \mathrm{Deficit}_{G,t} - S_{H,t} - S_{B,t}^{\text{ret}})|$.
    pub kalecki_levy_profit_error: f64,
    /// Godley Three-Balances identity residual $|(S_{\text{priv},t} - I_t) - \mathrm{Deficit}_{G,t}|$ in the closed economy ($\mathrm{CAB}_t \equiv 0$).
    pub godley_three_balances_error: f64,
}

impl SfcFlowMatrix {
    /// Constructs and audits the 4-sector (5-column) SFC flow matrix from micro-aggregated epoch flows.
    pub fn from_epoch_flows(
        consumption_c: f64,
        investment_i: f64,
        gov_deficit: f64,
        wages_w: f64,
        deposit_interest_hd: f64,
        loan_interest_fl: f64,
        micro_firm_profits_sum: f64,
    ) -> Self {
        let bank_retained_sb = loan_interest_fl - deposit_interest_hd;
        let household_saving_sh = wages_w + gov_deficit + deposit_interest_hd - consumption_c;
        let entries = [
            // Row 0: Consumption C_t
            [-consumption_c, consumption_c, 0.0, 0.0, 0.0],
            // Row 1: Investment I_t
            [0.0, investment_i, -investment_i, 0.0, 0.0],
            // Row 2: Net Fiscal Stimulus / Transfers Deficit_{G,t}
            [gov_deficit, 0.0, 0.0, 0.0, -gov_deficit],
            // Row 3: Wages W_t
            [wages_w, -wages_w, 0.0, 0.0, 0.0],
            // Row 4: Interest flows (deposit interest to H, loan interest from F_curr, spread to Bank)
            [
                deposit_interest_hd,
                -loan_interest_fl,
                0.0,
                bank_retained_sb,
                0.0,
            ],
            // Row 5: Corporate Profits Pi_t transferred from F_curr to F_cap
            [
                0.0,
                -micro_firm_profits_sum,
                micro_firm_profits_sum,
                0.0,
                0.0,
            ],
            // Row 6: Flow of Funds / Sectoral Net Lending (-S_H, I - Pi, -S_B^ret, +Deficit_G)
            [
                -household_saving_sh,
                0.0,
                investment_i - micro_firm_profits_sum,
                -bank_retained_sb,
                gov_deficit,
            ],
        ];

        let mut max_row_err = 0.0_f64;
        for row in &entries {
            let s: f64 = row.iter().sum();
            max_row_err = max_row_err.max(s.abs());
        }
        let mut max_col_err = 0.0_f64;
        for col in 0..5 {
            let s: f64 = entries.iter().map(|r| r[col]).sum();
            max_col_err = max_col_err.max(s.abs());
        }

        let kalecki_rhs = investment_i + gov_deficit - household_saving_sh - bank_retained_sb;
        let kalecki_err = (micro_firm_profits_sum - kalecki_rhs).abs();

        let private_financial_surplus =
            (household_saving_sh + bank_retained_sb + micro_firm_profits_sum) - investment_i;
        let godley_err = (private_financial_surplus - gov_deficit).abs();

        Self {
            entries,
            max_abs_row_sum_error: max_row_err,
            max_abs_col_sum_error: max_col_err,
            kalecki_levy_profit_error: kalecki_err,
            godley_three_balances_error: godley_err,
        }
    }
}

/// Summary of one cell in the $2 \times 2$ `(policy_shielded, closed_loop)` H1a ablation design.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SfcCellSummary {
    /// Fraction of the $N_F = 30$ firms remaining solvent at horizon $T = 60$.
    pub terminal_firm_survival_rate: f64,
    /// Time-and-household mean CRRA ($\gamma = 2.0$) certainty-equivalent welfare index (penalizing consumption variance via Jensen's Gap $\Delta_J$).
    pub crra_household_welfare: f64,
    /// Mean Jensen Gap $\Delta_J = \mathbb{E}[u(C)] - u(\mathbb{E}[C])$ on concave CRRA utility across the trajectory (`< 0`).
    pub crra_jensen_gap: f64,
    /// Maximum SFC row/column zero-sum residual across all $T = 60$ epochs.
    pub max_sfc_zero_sum_error: f64,
    /// Maximum Kalecki-Levy and Godley Three-Balances residual across all $T = 60$ epochs.
    pub max_kalecki_godley_error: f64,
    /// Average `MNAR` Manski ambiguity width observed by the regulator across the horizon.
    pub mean_manski_ambiguity_width: f64,
}
