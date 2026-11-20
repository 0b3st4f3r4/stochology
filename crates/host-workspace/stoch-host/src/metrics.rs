//! Contention-free host observability using atomic counters (`AtomicU64`) and OpenMetrics 1.0 rendering.
//!
//! No counter or reader acquires any execution-kernel lock, guaranteeing telemetry polling cannot
//! induce timing stalls or denial-of-service in `Regime III-B`.

use core::sync::atomic::{AtomicU64, Ordering};

/// Contention-free host metrics registry backed exclusively by `AtomicU64`.
#[derive(Debug, Default)]
pub struct LockFreeMetrics {
    /// Total durable `redb` two-phase checkpoints completed.
    pub redb_checkpoints_total: AtomicU64,
    /// Number of currently healthy active peers / anchors.
    pub anchors_active: AtomicU64,
    /// Total configured peers / anchors in the cluster.
    pub anchors_configured: AtomicU64,
    /// Current solar conductance scale emitted by the watchdog (`0..=1_000_000`).
    pub solar_kappa_scale_millionths: AtomicU64,
    /// Total `R0` storage rollbacks executed.
    pub r0_rollbacks_total: AtomicU64,
    /// Total post-signature verification mismatches detected across the HSM FFI seam.
    pub hsm_fault_injections_total: AtomicU64,
    /// Total rejected capability-firewall invocations.
    pub ocap_violations_total: AtomicU64,
    /// Observed fleet brownout fraction scaled in millionths (`0..=1_000_000`).
    pub fleet_brownout_ratio_millionths: AtomicU64,
    /// Total local autonomous `Regime III-B` halts executed.
    pub local_halt_triggers_total: AtomicU64,
}

/// Consistent point-in-time copy of host metric counters for off-shift reporting.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MetricsSnapshot {
    /// Durable `redb` checkpoints recorded.
    pub redb_checkpoints_total: u64,
    /// Active peers observed.
    pub anchors_active: u64,
    /// Total configured peers.
    pub anchors_configured: u64,
    /// Current solar conductance scale in millionths.
    pub solar_kappa_scale_millionths: u64,
    /// Total `R0` rollbacks.
    pub r0_rollbacks_total: u64,
    /// Total post-signature HSM faults caught.
    pub hsm_fault_injections_total: u64,
    /// Total object-capability firewall violations rejected.
    pub ocap_violations_total: u64,
    /// Fleet brownout ratio in millionths.
    pub fleet_brownout_ratio_millionths: u64,
    /// Local autonomous survival halt triggers.
    pub local_halt_triggers_total: u64,
}

impl LockFreeMetrics {
    /// Constructs a zeroed lock-free metrics registry (nominal solar scale = `1_000_000`).
    #[must_use]
    pub const fn new() -> Self {
        Self {
            redb_checkpoints_total: AtomicU64::new(0),
            anchors_active: AtomicU64::new(0),
            anchors_configured: AtomicU64::new(0),
            solar_kappa_scale_millionths: AtomicU64::new(1_000_000),
            r0_rollbacks_total: AtomicU64::new(0),
            hsm_fault_injections_total: AtomicU64::new(0),
            ocap_violations_total: AtomicU64::new(0),
            fleet_brownout_ratio_millionths: AtomicU64::new(0),
            local_halt_triggers_total: AtomicU64::new(0),
        }
    }

    /// Atomically reads all metric gauges and counters without locking.
    #[must_use]
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            redb_checkpoints_total: self.redb_checkpoints_total.load(Ordering::Relaxed),
            anchors_active: self.anchors_active.load(Ordering::Relaxed),
            anchors_configured: self.anchors_configured.load(Ordering::Relaxed),
            solar_kappa_scale_millionths: self
                .solar_kappa_scale_millionths
                .load(Ordering::Relaxed),
            r0_rollbacks_total: self.r0_rollbacks_total.load(Ordering::Relaxed),
            hsm_fault_injections_total: self.hsm_fault_injections_total.load(Ordering::Relaxed),
            ocap_violations_total: self.ocap_violations_total.load(Ordering::Relaxed),
            fleet_brownout_ratio_millionths: self
                .fleet_brownout_ratio_millionths
                .load(Ordering::Relaxed),
            local_halt_triggers_total: self.local_halt_triggers_total.load(Ordering::Relaxed),
        }
    }

    /// Formats metrics in OpenMetrics 1.0 text exposition format without blocking execution.
    #[must_use]
    pub fn render_openmetrics(&self) -> String {
        let s = self.snapshot();
        format!(
            concat!(
                "# TYPE stoch_redb_checkpoints_total counter\n",
                "stoch_redb_checkpoints_total {}\n",
                "# TYPE stoch_anchors_active gauge\n",
                "stoch_anchors_active {}\n",
                "# TYPE stoch_anchors_configured gauge\n",
                "stoch_anchors_configured {}\n",
                "# TYPE stoch_solar_kappa_scale_millionths gauge\n",
                "stoch_solar_kappa_scale_millionths {}\n",
                "# TYPE stoch_r0_rollbacks_total counter\n",
                "stoch_r0_rollbacks_total {}\n",
                "# TYPE stoch_hsm_fault_injections_total counter\n",
                "stoch_hsm_fault_injections_total {}\n",
                "# TYPE stoch_ocap_violations_total counter\n",
                "stoch_ocap_violations_total {}\n",
                "# TYPE stoch_fleet_brownout_ratio_millionths gauge\n",
                "stoch_fleet_brownout_ratio_millionths {}\n",
                "# TYPE stoch_local_halt_triggers_total counter\n",
                "stoch_local_halt_triggers_total {}\n",
                "# EOF\n",
            ),
            s.redb_checkpoints_total,
            s.anchors_active,
            s.anchors_configured,
            s.solar_kappa_scale_millionths,
            s.r0_rollbacks_total,
            s.hsm_fault_injections_total,
            s.ocap_violations_total,
            s.fleet_brownout_ratio_millionths,
            s.local_halt_triggers_total,
        )
    }
}
