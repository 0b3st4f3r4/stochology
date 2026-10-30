//! **e-BH** false discovery rate control over K e-values (Wang & Ramdas 2022), and the simulated
//! peeking-without-multiplicity defect it blocks (§3.2 Step 6, Prohibition 4).

use stochology_core::Rng;

use super::martingales::EvidenceProcess;

/// e-BH procedure (Wang & Ramdas 2022, *J. R. Stat. Soc. B* 84(3):822–852, informative): with
/// $e_{(1)} \ge \dots \ge e_{(K)}$, reject the $\hat{k}$ largest, where
/// $$\hat{k} = \max\Bigl\{k : e_{(k)} \ge \frac{K}{\alpha k}\Bigr\},$$
/// which controls the false discovery rate at $\alpha$ under **arbitrary** dependence among the e-values.
/// Rejections are the used findings; every other hypothesis is named in `discarded` (Prohibition 4),
/// and $K$ is the attempt counter of the analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct EBhMultiplicityAudit {
    pub alpha_level: f64,
    /// Attempt counter $K$ = number of hypotheses tested.
    pub hypotheses_k: usize,
    pub labels: Vec<String>,
    pub e_values: Vec<f64>,
    /// Number of rejections $\hat{k}$.
    pub rejections_k_hat: usize,
    /// e-value threshold $K / (\alpha \hat{k})$ actually used ($+\infty$ when nothing is rejected).
    pub rejection_threshold: f64,
    pub rejected: Vec<String>,
    pub discarded: Vec<String>,
    /// Benjamini-Yekutieli (2001, *Ann. Stat.* 29(4):1165–1188, informative) step-up on $p_k = \min(1, 1/e_k)$:
    /// the harmonic penalty $H_K = \sum_{i \le K} 1/i$ that BH needs under arbitrary dependence and e-BH avoids.
    pub benjamini_yekutieli_rejected: Vec<String>,
}

impl EBhMultiplicityAudit {
    /// Applies e-BH at level `alpha_level` to the labelled e-values.
    pub fn evaluate(labels: &[String], e_values: &[f64], alpha_level: f64) -> Self {
        assert_eq!(labels.len(), e_values.len());
        assert!(!e_values.is_empty());
        assert!(alpha_level > 0.0 && alpha_level < 1.0);
        let k_total = e_values.len();
        let mut order: Vec<usize> = (0..k_total).collect();
        order.sort_by(|&a, &b| e_values[b].total_cmp(&e_values[a]));
        let k_hat = (1..=k_total)
            .rev()
            .find(|&k| e_values[order[k - 1]] >= k_total as f64 / (alpha_level * k as f64))
            .unwrap_or(0);
        let mut rejected_flags = vec![false; k_total];
        for &i in &order[..k_hat] {
            rejected_flags[i] = true;
        }
        let pick = |want: bool| -> Vec<String> {
            labels
                .iter()
                .zip(&rejected_flags)
                .filter(|&(_, &r)| r == want)
                .map(|(l, _)| l.clone())
                .collect()
        };
        let harmonic: f64 = (1..=k_total).map(|i| 1.0 / i as f64).sum();
        let p_value = |i: usize| (1.0 / e_values[i]).min(1.0);
        let k_by = (1..=k_total)
            .rev()
            .find(|&k| {
                p_value(order[k - 1]) <= alpha_level * k as f64 / (k_total as f64 * harmonic)
            })
            .unwrap_or(0);
        let mut by_flags = vec![false; k_total];
        for &i in &order[..k_by] {
            by_flags[i] = true;
        }
        let benjamini_yekutieli_rejected = labels
            .iter()
            .zip(&by_flags)
            .filter(|&(_, &r)| r)
            .map(|(l, _)| l.clone())
            .collect();
        Self {
            alpha_level,
            hypotheses_k: k_total,
            labels: labels.to_vec(),
            e_values: e_values.to_vec(),
            rejections_k_hat: k_hat,
            rejection_threshold: if k_hat == 0 {
                f64::INFINITY
            } else {
                k_total as f64 / (alpha_level * k_hat as f64)
            },
            rejected: pick(true),
            discarded: pick(false),
            benjamini_yekutieli_rejected,
        }
    }

    /// e-BH over GRAPA betting e-processes (`EvidenceProcess::from_grapa_betting`), one per stream of
    /// centered payoffs $X_i \in [-1, 1]$, each evaluated at the end of its fixed-length stream.
    pub fn from_grapa_streams(
        labels: &[String],
        centered_streams: &[Vec<f64>],
        alpha_level: f64,
    ) -> Result<Self, String> {
        let e_values = centered_streams
            .iter()
            .map(|s| {
                EvidenceProcess::from_grapa_betting(s, alpha_level, 1).map(|e| e.final_e_value)
            })
            .collect::<Result<Vec<f64>, String>>()?;
        Ok(Self::evaluate(labels, &e_values, alpha_level))
    }
}

/// Simulated global null for the `multiplicity` negative proof: $K$ independent Bernoulli($p_0$) streams,
/// all null. The naive rule rejects each hypothesis whose running e-process ever crosses $1/\alpha$
/// (peeking with `attempt_counter_k = 1` per hypothesis); e-BH uses the terminal e-values. Under a global
/// null every rejection is false, so FDR equals the share of replications with at least one rejection.
#[derive(Debug, Clone, PartialEq)]
pub struct PeekingMultiplicitySimulation {
    pub seed: u64,
    pub hypotheses_k: usize,
    pub stream_length: usize,
    pub replications: usize,
    pub alpha_level: f64,
    pub naive_peeking_fdr: f64,
    pub ebh_fdr: f64,
}

impl PeekingMultiplicitySimulation {
    pub fn simulate(
        seed: u64,
        hypotheses_k: usize,
        stream_length: usize,
        replications: usize,
        alpha_level: f64,
        null_rate_p0: f64,
    ) -> Self {
        let mut rng = Rng::new(seed);
        let labels: Vec<String> = (0..hypotheses_k).map(|k| format!("H{k}")).collect();
        let (mut naive_any, mut ebh_any) = (0usize, 0usize);
        for _ in 0..replications {
            let mut naive_rejects = false;
            let mut terminal = Vec::with_capacity(hypotheses_k);
            for _ in 0..hypotheses_k {
                let stream: Vec<f64> = (0..stream_length)
                    .map(|_| f64::from(u8::from(rng.bernoulli(null_rate_p0))) - null_rate_p0)
                    .collect();
                let ep = EvidenceProcess::from_grapa_betting(&stream, alpha_level, 1)
                    .expect("Bernoulli payoffs are bounded");
                naive_rejects |= ep.rejects_null_hypothesis;
                terminal.push(ep.final_e_value);
            }
            naive_any += usize::from(naive_rejects);
            let ebh = EBhMultiplicityAudit::evaluate(&labels, &terminal, alpha_level);
            ebh_any += usize::from(ebh.rejections_k_hat > 0);
        }
        Self {
            seed,
            hypotheses_k,
            stream_length,
            replications,
            alpha_level,
            naive_peeking_fdr: naive_any as f64 / replications as f64,
            ebh_fdr: ebh_any as f64 / replications as f64,
        }
    }

    /// The defect is exposed when the naive rule breaks FDR control and e-BH keeps it.
    pub fn naive_breaks_and_ebh_controls(&self) -> bool {
        self.naive_peeking_fdr > self.alpha_level && self.ebh_fdr <= self.alpha_level
    }
}
