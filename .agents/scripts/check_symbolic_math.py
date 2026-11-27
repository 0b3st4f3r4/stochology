#!/usr/bin/env python3
"""
Standalone symbolic and numerical verification suite for core Stochology claims.

Runs SymPy symbolic proofs and NumPy numerical checks over 13 non-negotiable
identities, inequalities, set boundaries, and regime bounds referenced across
the manuscript. No manuscript file is modified.

Usage:
  python3 .agents/scripts/check_symbolic_math.py [--markdown] [--json] [--out FILE]
"""

from __future__ import annotations

import argparse
from dataclasses import asdict, dataclass
import json
import math
import os
from pathlib import Path
import sys
from typing import Callable

import numpy as np
import sympy as sp

ROOT = Path(os.environ.get("STOCHOLOGY_REPO_ROOT") or Path(__file__).resolve().parents[2]).resolve()


@dataclass
class CheckResult:
    id: str
    chapter_ref: str
    name: str
    passed: bool
    symbolic_note: str
    numeric_note: str
    error: str = ""


def _check_01_pinsker_fluency_cap() -> CheckResult:
    """Ch. 95 §1: Pinsker fixed-budget power ceiling as KL(Q_s || P_0) -> 0+."""
    m, d_kl = sp.symbols("m d_kl", positive=True)
    pinsker_bound = sp.sqrt(m * d_kl / 2)
    bretagnolle_huber = sp.sqrt(1 - sp.exp(-m * d_kl))

    lim_pinsker = sp.limit(pinsker_bound, d_kl, 0, "+")
    lim_bh = sp.limit(bretagnolle_huber, d_kl, 0, "+")
    deriv_pinsker = sp.simplify(sp.diff(pinsker_bound, d_kl))
    assert lim_pinsker == 0 and lim_bh == 0
    assert sp.ask(sp.Q.positive(deriv_pinsker)) is True

    p0 = 0.05
    max_violation = 0.0
    for eps in [0.005, 0.01, 0.02, 0.04]:
        qs = p0 + eps
        kl = qs * math.log(qs / p0) + (1 - qs) * math.log((1 - qs) / (1 - p0))
        for m_cues in [10, 25, 50]:
            k = np.arange(m_cues + 1)
            comb = np.array([math.comb(m_cues, int(i)) for i in k], dtype=float)
            pmf_p = comb * (p0**k) * ((1 - p0) ** (m_cues - k))
            pmf_q = comb * (qs**k) * ((1 - qs) ** (m_cues - k))
            tv_m = 0.5 * float(np.sum(np.abs(pmf_q - pmf_p)))
            bound = min(1.0, math.sqrt(0.5 * m_cues * kl))
            max_violation = max(max_violation, tv_m - bound)
            assert tv_m <= bound + 1e-12

    return CheckResult(
        id="M01",
        chapter_ref="Ch. 28, Ch. 95 §1",
        name="Pinsker fixed-budget fluency power cap",
        passed=True,
        symbolic_note="lim_{D_KL->0+} sqrt(m*D_KL/2) = 0; Bretagnolle-Huber lim = 0; d/dD_KL > 0",
        numeric_note=f"Exact Binomial(m,p) TV <= sqrt(m*KL/2) across grid (max gap={max_violation:.2e})",
    )


def _check_02_wald_sequential_latency() -> CheckResult:
    """Ch. 95 §1: Sequential expected stopping time lower bound under Q_s (i.i.d. cues)."""
    alpha_t, beta_t, d_kl = sp.symbols("alpha_t beta_t d_kl", positive=True)
    num = (1 - beta_t) * sp.log((1 - beta_t) / alpha_t) + beta_t * sp.log(beta_t / (1 - alpha_t))
    wald_lb = num / d_kl

    deriv_kl = sp.simplify(sp.diff(wald_lb, d_kl))
    assert sp.simplify(deriv_kl + num / (d_kl**2)) == 0

    test_pairs = [(0.05, 0.10), (0.01, 0.20), (0.05, 0.05)]
    min_ratio = float("inf")
    for a, b in test_pairs:
        d_bin = (1 - b) * math.log((1 - b) / a) + b * math.log(b / (1 - a))
        assert d_bin > 0.0
        prev_e_n = 0.0
        for mu in [0.5, 0.25, 0.1]:
            kl = 0.5 * (mu**2)
            lb = d_bin / kl
            assert lb > prev_e_n
            prev_e_n = lb
            min_ratio = min(min_ratio, lb * kl)

    return CheckResult(
        id="M02",
        chapter_ref="Ch. 95 §1",
        name="Sequential fluency latency lower bound (Wald/Kullback-Leibler/Kaufmann)",
        passed=True,
        symbolic_note="E_Qs[N] >= d_bin(1-beta_test||alpha_test)/D_KL with d/dD_KL = -num/D_KL^2 < 0",
        numeric_note=f"Binary KL numerator > 0 for all (alpha_test, beta_test); min E[N]*KL = {min_ratio:.4f}",
    )


def _check_03_adroitness_latency_floor() -> CheckResult:
    """Ch. 49 §§8.2.2, 8.6.1: Physical time tau_detect, discrete floor a*_skill & joint cutoff."""
    q0, delta, a, dt_shift = sp.symbols("q0 delta a dt_shift", positive=True)
    tau_cont = dt_shift * q0 / ((1 - delta) * a)
    d_tau_da = sp.simplify(sp.diff(tau_cont, a))
    d_tau_ddelta = sp.simplify(sp.diff(tau_cont, delta))

    sub_vals = {q0: 2, delta: sp.Rational(1, 4), a: sp.Rational(3, 5), dt_shift: 1}
    assert float(d_tau_da.subs(sub_vals)) < 0
    assert float(d_tau_ddelta.subs(sub_vals)) > 0

    k_shifts = sp.symbols("k_shifts", positive=True, integer=True)
    a_star = q0 / k_shifts
    joint_lhs = sp.simplify((q0 / ((1 - delta) * a)).subs(a, a_star / (1 - delta)))
    assert sp.simplify(joint_lhs - k_shifts) == 0

    # Discrete-ceiling non-exact doubling counterexample & joint vs marginal cutoff counterexample
    assert math.ceil(0.4) == 1 and math.ceil(0.8) == 1
    assert math.ceil(2.4) == 3 and math.ceil(4.8) == 5 != 6
    delta_star, a_star_num = 0.6, 0.4  # q0=4, floor(T_rem / dt_shift)=10
    assert 0.5 < delta_star and 0.5 > a_star_num and math.ceil(4.0 / ((1 - 0.5) * 0.5)) == 16 > 10

    # Ch. 49/93/95/97 & App. A #19 / App. G: finite-window H_mix vs late onset vs e-CUSUM
    H_mix, alpha_test, e_sig, m_steps = 5, 0.05, 2.0, 7
    E_trip = 1.0 / alpha_test
    w_min = 1.0 / H_mix
    a_star_wmin = math.log(E_trip / w_min) / (m_steps * math.log(e_sig))
    assert 0.0 < a_star_wmin <= 1.0
    # (i) In-window onset (t0 = 3 <= H_mix) trips within m_steps under a*_skill(w_min), and w_min=1 forces H_mix=1
    seq_in = [0.5] * 2 + [e_sig ** a_star_wmin] * m_steps
    E_in = sum((1.0 / H_mix) * math.prod(seq_in[k - 1 :]) for k in range(1, min(len(seq_in), H_mix) + 1))
    assert E_in >= E_trip - 1e-12
    assert [h for h in range(1, 6) if h * 1.0 <= 1.0] == [1]
    # (ii) Late onset (t0 = 21 > H_mix) fails H_mix mixture even at a_skill=1, and periodic resets violate Ville
    seq_late = [0.5] * 20 + [e_sig] * m_steps
    E_late = sum((1.0 / H_mix) * math.prod(seq_late[k - 1 :]) for k in range(1, min(len(seq_late), H_mix) + 1))
    assert E_late < E_trip and (1.0 - (1.0 - alpha_test) ** 20) > 0.64 > alpha_test
    # (iii) Indefinite-horizon stationary e-CUSUM recovers late-onset trip under a*_skill,CUSUM
    a_star_cusum = math.log(E_trip) / (m_steps * math.log(e_sig))
    seq_cusum = [0.5] * 20 + [e_sig ** a_star_cusum] * m_steps
    E_cusum = max(math.prod(seq_cusum[k - 1 :]) for k in range(1, len(seq_cusum) + 1))
    assert 0.0 < a_star_cusum < a_star_wmin and E_cusum >= E_trip - 1e-12

    return CheckResult(
        id="M03",
        chapter_ref="Ch. 47, Ch. 49 §§8.2.2, 8.6.1; Ch. 75",
        name="Adroitness physical latency decay, floor a*_skill & joint threshold",
        passed=True,
        symbolic_note="d(tau)/da < 0, d(tau)/d(delta_status) > 0; (1-delta)*a >= a*_skill solves discrete clearance",
        numeric_note="Verified ceiling non-doubling (ceil(4.8)=5 != 6) & joint failure at marginal pass (16 > 10)",
    )


def _check_04_regime3_survival_scaling() -> CheckResult:
    """Ch. 83 §3: Non-ergodic Poisson ruin scaling under throughput multiplier M."""
    lam0, p_ruin, W, M = sp.symbols("lam0 p_ruin W M", positive=True)
    S1 = sp.exp(-lam0 * p_ruin * W)
    SM = sp.exp(-M * lam0 * p_ruin * W)
    assert sp.simplify(SM - S1**M) == 0
    assert sp.limit(SM, M, sp.oo) == 0
    ruin_M = 1 - SM
    assert sp.ask(sp.Q.positive(sp.diff(ruin_M, M))) is True

    s1_num = math.exp(-0.02 * 0.1 * 50.0)
    s10_num = s1_num**10
    assert abs(s10_num - math.exp(-10 * 0.02 * 0.1 * 50.0)) < 1e-14

    return CheckResult(
        id="M04",
        chapter_ref="Ch. 41 §5, Ch. 83 §3",
        name="Regime III Poisson survival scaling S(W; M) = S(W; 1)^M",
        passed=True,
        symbolic_note="S(W;M) - S(W;1)^M = 0; lim_{M->oo} S(W;M) = 0; d(1-S)/dM > 0",
        numeric_note=f"At lambda0*p_ruin*W=0.1: S(W;1)={s1_num:.4f} -> S(W;10)={s10_num:.4f}",
    )


def _check_05_meng_identity() -> CheckResult:
    """Ch. 30 / App. A: Meng (2018) finite-population Data Defect Index identity."""
    y = sp.symbols("y1:5", real=True)
    r = [1, 1, 0, 0]
    N = 4
    n = sum(r)
    y_bar_N = sum(y) / N
    y_bar_n = sum(r[i] * y[i] for i in range(N)) / n
    r_bar = sp.Rational(n, N)
    cov_ry = sum((r[i] - r_bar) * (y[i] - y_bar_N) for i in range(N)) / N
    var_r = r_bar * (1 - r_bar)
    var_y = sum((y[i] - y_bar_N) ** 2 for i in range(N)) / N
    rho_ry = cov_ry / sp.sqrt(var_r * var_y)
    sigma_y = sp.sqrt(var_y)
    rhs = rho_ry * sp.sqrt(sp.Rational(N - n, n)) * sigma_y
    assert sp.simplify((y_bar_n - y_bar_N) ** 2 - rhs**2) == 0

    rng = np.random.default_rng(42)
    max_err = 0.0
    for _ in range(25):
        N_num = 200
        probs = rng.uniform(0.1, 0.9, size=N_num)
        R = (rng.random(N_num) < probs).astype(float)
        if R.sum() in (0, N_num):
            continue
        Y = 2.0 * probs + rng.normal(0.0, 0.5, size=N_num)
        n_num = float(R.sum())
        err_lhs = float(np.sum(R * Y) / n_num - np.mean(Y))
        rho = float(np.corrcoef(R, Y)[0, 1])
        sigma = float(np.std(Y, ddof=0))
        err_rhs = rho * math.sqrt((N_num - n_num) / n_num) * sigma
        max_err = max(max_err, abs(err_lhs - err_rhs))
        assert abs(err_lhs - err_rhs) < 1e-12

    return CheckResult(
        id="M05",
        chapter_ref="Ch. 30, App. A",
        name="Meng finite-population Data Defect identity",
        passed=True,
        symbolic_note="Exact squared-identity equality verified symbolically over N=4",
        numeric_note=f"Max |LHS - RHS| across 25 biased samples (N=200): {max_err:.2e}",
    )


def _check_06_additive_deff_composition() -> CheckResult:
    """AGENTS.md §V-9 / App. A: Exact additive DEFF composition vs product bound."""
    d1, d2 = sp.symbols("d1 d2", real=True)
    deff_add = 1 + (d1 - 1) + (d2 - 1)
    deff_prod = d1 * d2
    gap = sp.factor(deff_prod - deff_add)
    assert gap == (d1 - 1) * (d2 - 1)

    for v1, v2 in [(1.0, 1.0), (1.4, 2.1), (3.5, 4.0)]:
        add_v = 1.0 + (v1 - 1.0) + (v2 - 1.0)
        prod_v = v1 * v2
        assert add_v <= prod_v + 1e-15

    return CheckResult(
        id="M06",
        chapter_ref="AGENTS.md §V-9, Ch. 31, App. A",
        name="Additive DEFF composition vs multiplicative upper bound",
        passed=True,
        symbolic_note="DEFF_1 * DEFF_2 - (DEFF_1 + DEFF_2 - 1) = (DEFF_1 - 1)(DEFF_2 - 1) >= 0",
        numeric_note="Verified DEFF_add <= DEFF_prod across sample design effects >= 1",
    )


def _check_07_leontief_vs_product_and_5_6_limit() -> CheckResult:
    """Prologue / Ch. 41 / App. A: Leontief cube vs additive threshold & 5/6 cut."""
    t = sp.Symbol("t", real=True)
    x1, x2, x3 = sp.symbols("x1 x2 x3", real=True)
    v_leontief = (1 - t) ** 3

    vol_simplex_1 = sp.integrate(
        sp.integrate(sp.integrate(1, (x3, 0, 1 - x1 - x2)), (x2, 0, 1 - x1)),
        (x1, 0, 1),
    )
    vol_mean_one_third = 1 - vol_simplex_1
    v_leontief_one_third = v_leontief.subs(t, sp.Rational(1, 3))
    assert vol_mean_one_third == sp.Rational(5, 6)
    assert v_leontief_one_third == sp.Rational(8, 27)
    assert v_leontief_one_third < vol_mean_one_third

    c = sp.Symbol("c", positive=True)
    v_prod_cdf = c * (1 - sp.log(c) + sp.Rational(1, 2) * sp.log(c) ** 2)
    dv = sp.simplify(sp.diff(v_prod_cdf, c))
    assert sp.simplify(dv - sp.Rational(1, 2) * sp.log(c) ** 2) == 0
    for tv in [0.2, 0.3333333333, 0.5, 0.8]:
        cv = tv**3
        surv_prod = 1.0 - cv * (1.0 - math.log(cv) + 0.5 * (math.log(cv) ** 2))
        surv_leo = (1.0 - tv) ** 3
        assert surv_leo < surv_prod

    return CheckResult(
        id="M07",
        chapter_ref="Ch. 00, Ch. 41, App. A",
        name="Leontief cube contraction vs product & 5/6 mean threshold",
        passed=True,
        symbolic_note="Vol(x_bar >= 1/3) = 5/6 > (2/3)^3 = 8/27; [t,1]^3 strictly inside {prod >= t^3}",
        numeric_note="Leontief survival (1-t)^3 strictly below product-threshold survival on (0,1)",
    )


def _check_08_chamfered_cube_volume() -> CheckResult:
    """Ch. 00 / App. A: Chamfered cognitive cube volume {x in [0,1]^3 : x1+x2+x3 <= B}."""
    B = sp.Symbol("B", real=True)
    v1 = B**3 / 6
    v2 = (B**3 - 3 * (B - 1) ** 3) / 6
    v3 = 1 - (3 - B) ** 3 / 6

    assert v1.subs(B, 1) == v2.subs(B, 1) == sp.Rational(1, 6)
    assert sp.diff(v1, B).subs(B, 1) == sp.diff(v2, B).subs(B, 1)
    assert v2.subs(B, 2) == v3.subs(B, 2) == sp.Rational(5, 6)
    assert sp.diff(v2, B).subs(B, 2) == sp.diff(v3, B).subs(B, 2)
    assert v2.subs(B, sp.Rational(3, 2)) == sp.Rational(1, 2)

    rng = np.random.default_rng(7)
    samples = rng.random((200000, 3))
    sums = samples.sum(axis=1)
    for b_val in [0.8, 1.5, 2.2]:
        if b_val <= 1.0:
            exact = float(v1.subs(B, b_val))
        elif b_val <= 2.0:
            exact = float(v2.subs(B, b_val))
        else:
            exact = float(v3.subs(B, b_val))
        mc = float(np.mean(sums <= b_val))
        assert abs(mc - exact) < 0.004

    # Ch. 95 §2 reserved governance box [tau,1]^2 x [B_gov_min/k_gov, 1] inclusion-exclusion check
    tau_s, b_gov_s, k_gov_s, k_epi_s, k_vit_s, d_cut_s = sp.symbols(
        "tau_s b_gov_s k_gov_s k_epi_s k_vit_s d_cut_s", positive=True
    )
    vol_box = ((1 - tau_s) ** 2) * (1 - b_gov_s / k_gov_s)
    vol_single = vol_box - d_cut_s**3 / (6 * k_epi_s * k_vit_s * k_gov_s)
    assert sp.simplify(vol_single.subs(b_gov_s, k_gov_s * tau_s) - ((1 - tau_s) ** 3 - d_cut_s**3 / (6 * k_epi_s * k_vit_s * k_gov_s))) == 0

    # Strict reserve breach example: kappa_a=1, tau=0.2, B_gov_min=0.5, B_cog=2.3 (Delta_cut=0.7 > 0.5)
    ex_box = (1 - 0.2) ** 2 * (1 - 0.5)
    ex_inc_exc = ex_box - (0.7**3) / 6.0 + ((0.7 - 0.5) ** 3) / 6.0
    assert abs(ex_box - 0.32) < 1e-12 and abs(ex_inc_exc - 0.26416666666666666) < 1e-12
    g_pts = np.linspace(0.0, 1.0, 120)
    r_e, r_v, r_g = np.meshgrid(
        0.2 + 0.8 * g_pts, 0.2 + 0.8 * g_pts, 0.5 + 0.5 * g_pts, indexing="ij"
    )
    grid_vol = ex_box * float(np.mean((r_e + r_v + r_g) <= 2.3))
    assert abs(grid_vol - ex_inc_exc) < 0.004

    return CheckResult(
        id="M08",
        chapter_ref="Ch. 00, Ch. 95 §2, App. A",
        name="Chamfered cognitive cube & reserved-governance box volume",
        passed=True,
        symbolic_note="C^1 Irwin-Hall cube + reserved-gov base (1-tau)^2*(1-B_gov_min/k_gov) matching inclusion-exclusion",
        numeric_note=f"200k MC & strict-reserve breach check V={ex_inc_exc:.6f} (grid={grid_vol:.6f})",
    )


def _check_09_perdomo_ps_vs_po() -> CheckResult:
    """Ch. 48 / App. A / redteam-gov: Perdomo performative stability vs optimality."""
    mu0, eps, lam, sigma, theta, theta_prime = sp.symbols(
        "mu0 eps lam sigma theta theta_prime", real=True
    )
    R_dec = sp.Rational(1, 2) * (
        sigma**2 + (mu0 + eps * theta - theta_prime) ** 2 + lam * theta_prime**2
    )
    foc_ps = sp.diff(R_dec, theta_prime).subs(theta_prime, theta)
    theta_ps = sp.solve(foc_ps, theta)[0]
    assert sp.simplify(theta_ps - mu0 / (1 - eps + lam)) == 0

    PR = sp.simplify(R_dec.subs(theta_prime, theta))
    theta_po = sp.solve(sp.diff(PR, theta), theta)[0]
    assert sp.simplify(theta_po - (1 - eps) * mu0 / ((1 - eps) ** 2 + lam)) == 0

    diff_theta = sp.factor(theta_ps - theta_po)
    gap_pr = sp.factor(PR.subs(theta, theta_ps) - PR.subs(theta, theta_po))
    sample_gap = float(gap_pr.subs({mu0: 2, eps: sp.Rational(1, 3), lam: sp.Rational(1, 2), sigma: 1}))
    assert sample_gap > 0 and diff_theta != 0

    return CheckResult(
        id="M09",
        chapter_ref="Ch. 48, Ch. 82, App. A",
        name="Perdomo performative stability vs performative optimality gap",
        passed=True,
        symbolic_note="theta_PS = mu0/(1-eps+lam) != theta_PO = (1-eps)mu0/((1-eps)^2+lam) when eps,lam,mu0 != 0",
        numeric_note=f"Factored risk excess PR(theta_PS)-PR(theta_PO) at test point = {sample_gap:.6f} > 0",
    )


def _check_10_chouldechova_identity() -> CheckResult:
    """Ch. 49 / App. A / redteam-gov: Chouldechova calibration-error-rate identity."""
    p, ppv, fpr, fnr = sp.symbols("p ppv fpr fnr", positive=True)
    tpr = 1 - fnr
    odds_false_pos = (1 - ppv) / ppv
    rhs_identity = (p / (1 - p)) * odds_false_pos * (1 - fnr)
    ppv_expr = (tpr * p) / (tpr * p + fpr * (1 - p))
    lhs_minus_rhs = sp.simplify(fpr - rhs_identity.subs(ppv, ppv_expr))
    assert lhs_minus_rhs == 0

    p_a, p_b = 0.2, 0.5
    ppv_v = 0.75
    fnr_v = 0.20
    fpr_a = (p_a / (1 - p_a)) * ((1 - ppv_v) / ppv_v) * (1 - fnr_v)
    fpr_b = (p_b / (1 - p_b)) * ((1 - ppv_v) / ppv_v) * (1 - fnr_v)
    assert fpr_a < fpr_b

    return CheckResult(
        id="M10",
        chapter_ref="Ch. 49, App. A",
        name="Chouldechova calibration & error-rate balance identity",
        passed=True,
        symbolic_note="FPR - [p/(1-p)] * [(1-PPV)/PPV] * (1-FNR) == 0 identically",
        numeric_note=f"At equal PPV={ppv_v}, FNR={fnr_v}: p=0.2 -> FPR={fpr_a:.4f} != p=0.5 -> FPR={fpr_b:.4f}",
    )


def _check_11_brake2_kkt_ceiling() -> CheckResult:
    """Ch. 50 §9.3.2, Ch. 90, Ch. 91: Reflexive Brake 2 covariant KKT hoarding ceiling."""
    R, R_ref, R_floor, k_prec, c_urg = sp.symbols(
        "R R_ref R_floor k_prec c_urg", positive=True
    )
    W_wel = k_prec * sp.log(1 + R / R_ref) - c_urg * (R / R_ref)
    d1 = sp.diff(W_wel, R)
    d2 = sp.simplify(sp.diff(W_wel, R, 2))
    assert sp.ask(sp.Q.negative(d2)) is True
    r_unc = sp.solve(d1, R)[0]
    assert sp.simplify(r_unc - R_ref * (k_prec / c_urg - 1)) == 0

    # Verify Stevens ratio-scale covariance (R -> s*R, R_ref -> s*R_ref leaves W_wel invariant & scales R*)
    s = sp.Symbol("s", positive=True)
    assert sp.simplify(W_wel.subs({R: s * R, R_ref: s * R_ref}) - W_wel) == 0

    # Verify survival-floor lock blocks Ch. 90/91's unscaled max(0, k_prec/c_urg - 1) under-floor collapse
    r_floor_v, r_ref_v, kp_v, cu_v = 500.0, 100.0, 2.0, 0.5
    r_star_ch50 = max(r_floor_v, r_ref_v * (kp_v / cu_v - 1.0))
    r_unscaled = max(0.0, kp_v / cu_v - 1.0)
    assert r_star_ch50 == 500.0 and r_unscaled == 3.0 < r_floor_v
    assert max(r_floor_v, r_ref_v * (0.5 / 1.0 - 1.0)) == r_floor_v

    return CheckResult(
        id="M11",
        chapter_ref="Ch. 50 §9.3.2, Ch. 90, Ch. 91",
        name="Brake 2 covariant KKT hoarding ceiling & survival-floor lock",
        passed=True,
        symbolic_note="W_wel strictly concave; R* = max(R_floor, R_ref*(k_prec/c_urg - 1)) invariant under ratio rescaling",
        numeric_note=f"Floor-locked R*={r_star_ch50:.1f} protects B_req={r_floor_v:.1f} vs unscaled {r_unscaled:.1f}",
    )


def _check_12_gerstgrasser_basel_and_guard_sites() -> CheckResult:
    """Ch. 75 §3 & AGENTS.md §V-10: Basel pi^2/6 fixed-X_0 OLS ceiling & guard-site root P=3 => n=7."""
    k, P = sp.symbols("k P", positive=True)
    basel = sp.summation(1 / k**2, (k, 1, sp.oo))
    assert sp.simplify(basel - sp.pi**2 / 6) == 0

    roots = sp.solve(P * (P - 1) + 1 - 7, P)
    pos_roots = [r for r in roots if r > 0]
    assert pos_roots == [3]
    assert sp.binomial(7, 3) == 35 == 7 * 5

    partial_1000 = float(np.sum(1.0 / (np.arange(1, 1001, dtype=float) ** 2)))
    assert abs(partial_1000 - (math.pi**2 / 6.0)) < 0.0011

    # Verify Gerstgrasser fixed-covariate X_0 Wishart scaling E[||w_n - w*||^2] = (sigma^2 d/(T-d-1)) sum_{i=1}^n i^-2
    T_n, d_n, sigma, n_rounds = 15, 4, 1.0, 6
    theory_accum = (sigma**2 * d_n / (T_n - d_n - 1)) * float(
        np.sum(1.0 / (np.arange(1, n_rounds + 1, dtype=float) ** 2))
    )
    rng = np.random.default_rng(17)
    w_true = np.zeros(d_n)
    errs = []
    for _ in range(1500):
        X0 = rng.normal(size=(T_n, d_n))
        inv_xt_x = np.linalg.inv(X0.T @ X0)
        pinv = inv_xt_x @ X0.T
        y_sum = np.zeros(T_n)
        w_cur = np.zeros(d_n)
        for i in range(1, n_rounds + 1):
            y_i = X0 @ w_cur + rng.normal(scale=sigma, size=T_n)
            y_sum += y_i
            w_cur = pinv @ (y_sum / float(i))
        errs.append(float(np.sum((w_cur - w_true) ** 2)))
    mc_accum = float(np.mean(errs))
    assert abs(mc_accum - theory_accum) < 0.04

    return CheckResult(
        id="M12",
        chapter_ref="Ch. 75 §3, App. D, AGENTS.md §V-10",
        name="Gerstgrasser fixed-X0 Wishart pi^2/6 ceiling & guard-site n(3)=7, C(7,3)=35",
        passed=True,
        symbolic_note="sum_{k>=1} k^-2 = pi^2/6; fixed-X0 d/(T-d-1) scaling; P(P-1)+1=7 => P=3, C(7,3)=35",
        numeric_note=f"Fixed-X0 MC={mc_accum:.4f} vs theory={theory_accum:.4f} (T=15, d=4, n=6); Basel partial={partial_1000:.6f}",
    )


def _check_13_oracle_disjointness_and_correlated_survival() -> CheckResult:
    """Ch. 56 §5 & Ch. 83 §3: Oracle set disjointness & correlated vs saturated survival."""
    # Symbolic finite-set check: V_gauntlet <= Omega_spec and Omega_unspec = Omega_harm \ Omega_spec
    # => V_gauntlet intersect Omega_unspec = empty even when Omega_unspec is empty
    omega_spec = sp.FiniteSet(1, 2, 3)
    for omega_harm in (sp.FiniteSet(1, 2, 3), sp.FiniteSet(1, 2, 3, 4, 5)):
        omega_unspec = omega_harm - omega_spec
        v_gauntlet = sp.FiniteSet(1, 2)
        assert v_gauntlet.is_subset(omega_spec)
        assert sp.Intersection(v_gauntlet, omega_unspec) == sp.EmptySet
        if len(omega_unspec) > 0:
            # Uniform distribution over Omega_harm with positive probability on Omega_unspec
            p_unspec = len(omega_unspec) / len(omega_harm)
            p_joint = len(sp.Intersection(v_gauntlet, omega_unspec)) / len(omega_harm)
            assert p_unspec > 0 and (p_joint / p_unspec) == 0.0

    # Common-cause correlation vs review saturation around Poisson S(W; M) = S(W; 1)^M
    lam_s, p_s, p0_s, W_s, M_s = sp.symbols("lam_s p_s p0_s W_s M_s", positive=True)
    assert sp.solve(sp.Eq(sp.exp(-M_s * lam_s * p_s * W_s), sp.exp(-lam_s * p0_s * W_s)), p_s) == [p0_s / M_s]
    lam0, p_ruin, W, M = 0.05, 0.02, 100.0, 10
    s_base = math.exp(-lam0 * p_ruin * W)
    s_indep = math.exp(-M * lam0 * p_ruin * W)
    s_common = math.exp(-M * lam0 * W) + (1.0 - p_ruin) * (1.0 - math.exp(-M * lam0 * W))
    p_sat = min(0.99, p_ruin * (1.0 + 0.5 * (M - 1)))
    s_endo = math.exp(-M * lam0 * p_sat * W)
    assert s_common > s_base > s_indep > s_endo

    return CheckResult(
        id="M13",
        chapter_ref="Ch. 56 §5, Ch. 83 §3, App. A #19a",
        name="Oracle set disjointness & correlated/saturated survival bounds",
        passed=True,
        symbolic_note="Intersection(V_gauntlet, Omega_unspec) = EmptySet; P(H in V | H in Omega_unspec)=0; S(W;M)=S_0(W;1) <=> p_ruin=p_0/M",
        numeric_note=f"Survival ordering S_common={s_common:.4f} > S_base={s_base:.4f} > S_indep={s_indep:.4f} > S_endo={s_endo:.4f}",
    )


CHECKS: list[Callable[[], CheckResult]] = [
    _check_01_pinsker_fluency_cap,
    _check_02_wald_sequential_latency,
    _check_03_adroitness_latency_floor,
    _check_04_regime3_survival_scaling,
    _check_05_meng_identity,
    _check_06_additive_deff_composition,
    _check_07_leontief_vs_product_and_5_6_limit,
    _check_08_chamfered_cube_volume,
    _check_09_perdomo_ps_vs_po,
    _check_10_chouldechova_identity,
    _check_11_brake2_kkt_ceiling,
    _check_12_gerstgrasser_basel_and_guard_sites,
    _check_13_oracle_disjointness_and_correlated_survival,
]


def run_all() -> dict:
    results: list[CheckResult] = []
    for fn in CHECKS:
        try:
            res = fn()
        except Exception as exc:  # pragma: no cover
            res = CheckResult(
                id=fn.__name__,
                chapter_ref="?",
                name=fn.__doc__ or fn.__name__,
                passed=False,
                symbolic_note="",
                numeric_note="",
                error=f"{type(exc).__name__}: {exc}",
            )
        results.append(res)

    passed_count = sum(1 for r in results if r.passed)
    failed_count = len(results) - passed_count
    return {
        "summary": {
            "total": len(results),
            "passed": passed_count,
            "failed": failed_count,
            "blocking_count": failed_count,
            "verdict": "PASS" if failed_count == 0 else "FAIL",
        },
        "checks": [asdict(r) for r in results],
    }


def format_markdown(report: dict) -> str:
    s = report["summary"]
    lines = [
        "## Symbolic & Numerical Math Oracle (`.agents/scripts/check_symbolic_math.py`)",
        "",
        f"- **Status:** `{s['verdict']}` ({s['passed']}/{s['total']} proofs passed)",
        "",
        "| ID | Scope | Claim | Status | Symbolic / Numeric Summary |",
        "| :--- | :--- | :--- | :---: | :--- |",
    ]
    for c in report["checks"]:
        status = "`PASS`" if c["passed"] else "`FAIL`"
        detail = c["error"] if not c["passed"] else f"{c['symbolic_note']} ({c['numeric_note']})"
        lines.append(f"| `{c['id']}` | {c['chapter_ref']} | {c['name']} | {status} | {detail} |")
    return "\n".join(lines) + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Verify Stochology mathematical claims via SymPy & NumPy.")
    parser.add_argument("--markdown", action="store_true", help="Emit Markdown summary table.")
    parser.add_argument("--json", action="store_true", help="Emit JSON report.")
    parser.add_argument("--out", type=Path, help="Optional output file path.")
    args = parser.parse_args(argv)

    report = run_all()
    if args.json:
        text = json.dumps(report, indent=2) + "\n"
    else:
        text = format_markdown(report)

    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    sys.stdout.write(text)
    return 0 if report["summary"]["blocking_count"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
