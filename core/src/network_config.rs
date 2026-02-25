use serde::{Deserialize, Serialize};

use crate::simulation::SorobanResources;

// ── Protocol cost parameters ──────────────────────────────────────────────────

/// Network-level cost parameters that govern how resource consumption maps to
/// fees (stroops).  Different protocol versions use different rates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConfig {
    /// Human-readable label for this configuration.
    pub name: String,
    /// Protocol version number (e.g. 21, 22).
    pub protocol_version: u32,

    // ── Fee rates ─────────────────────────────────────────────────────────
    /// CPU instructions per fee unit (higher = cheaper per instruction).
    pub cpu_insns_per_fee_unit: u64,
    /// Memory bytes per fee unit.
    pub mem_bytes_per_fee_unit: u64,
    /// Ledger I/O bytes per fee unit.
    pub ledger_bytes_per_fee_unit: u64,
    /// Transaction size bytes per fee unit.
    pub tx_size_bytes_per_fee_unit: u64,

    // ── Resource limits (per transaction) ─────────────────────────────────
    /// Maximum CPU instructions a single transaction may consume.
    pub tx_max_instructions: u64,
    /// Maximum memory bytes a single transaction may consume.
    pub tx_max_memory_bytes: u64,
    /// Maximum ledger read bytes per transaction.
    pub tx_max_read_bytes: u64,
    /// Maximum ledger write bytes per transaction.
    pub tx_max_write_bytes: u64,
    /// Maximum transaction envelope size in bytes.
    pub tx_max_size_bytes: u64,
}

impl NetworkConfig {
    /// Calculate the total fee (stroops) for a given resource footprint under
    /// this configuration's cost rates.
    pub fn calculate_cost(&self, resources: &SorobanResources) -> u64 {
        let cpu_fee = resources.cpu_instructions / self.cpu_insns_per_fee_unit;
        let mem_fee = resources.ram_bytes / self.mem_bytes_per_fee_unit;
        let ledger_fee = (resources.ledger_read_bytes + resources.ledger_write_bytes)
            / self.ledger_bytes_per_fee_unit;
        let size_fee = resources.transaction_size_bytes / self.tx_size_bytes_per_fee_unit;
        cpu_fee + mem_fee + ledger_fee + size_fee
    }

    /// Check which resource limits would be exceeded under this configuration.
    pub fn check_limits(&self, resources: &SorobanResources) -> Vec<LimitExceeded> {
        let mut exceeded = Vec::new();
        if resources.cpu_instructions > self.tx_max_instructions {
            exceeded.push(LimitExceeded {
                resource: "cpu_instructions".to_string(),
                used: resources.cpu_instructions,
                limit: self.tx_max_instructions,
            });
        }
        if resources.ram_bytes > self.tx_max_memory_bytes {
            exceeded.push(LimitExceeded {
                resource: "ram_bytes".to_string(),
                used: resources.ram_bytes,
                limit: self.tx_max_memory_bytes,
            });
        }
        if resources.ledger_read_bytes > self.tx_max_read_bytes {
            exceeded.push(LimitExceeded {
                resource: "ledger_read_bytes".to_string(),
                used: resources.ledger_read_bytes,
                limit: self.tx_max_read_bytes,
            });
        }
        if resources.ledger_write_bytes > self.tx_max_write_bytes {
            exceeded.push(LimitExceeded {
                resource: "ledger_write_bytes".to_string(),
                used: resources.ledger_write_bytes,
                limit: self.tx_max_write_bytes,
            });
        }
        if resources.transaction_size_bytes > self.tx_max_size_bytes {
            exceeded.push(LimitExceeded {
                resource: "transaction_size_bytes".to_string(),
                used: resources.transaction_size_bytes,
                limit: self.tx_max_size_bytes,
            });
        }
        exceeded
    }
}

/// A single resource limit that was exceeded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LimitExceeded {
    pub resource: String,
    pub used: u64,
    pub limit: u64,
}

// ── Pre-set protocol configurations ───────────────────────────────────────────

/// Protocol 21 — Current Testnet (baseline).
///
/// Cost rates match the hardcoded values previously in
/// `SimulationEngine::calculate_cost`.
pub fn protocol_21() -> NetworkConfig {
    NetworkConfig {
        name: "Protocol 21 (Current Testnet)".to_string(),
        protocol_version: 21,
        cpu_insns_per_fee_unit: 10_000,
        mem_bytes_per_fee_unit: 1_024,
        ledger_bytes_per_fee_unit: 1_024,
        tx_size_bytes_per_fee_unit: 1_024,
        tx_max_instructions: 100_000_000,
        tx_max_memory_bytes: 40 * 1024 * 1024, // 40 MiB
        tx_max_read_bytes: 200 * 1024,         // 200 KiB
        tx_max_write_bytes: 65_536,            // 64 KiB
        tx_max_size_bytes: 71_680,             // 70 KiB
    }
}

/// Protocol 22 — Upcoming / Next.
///
/// Models an anticipated upgrade with higher CPU/memory budgets but tighter
/// per-unit costs to incentivise efficient contracts.
pub fn protocol_22() -> NetworkConfig {
    NetworkConfig {
        name: "Protocol 22 (Upcoming/Next)".to_string(),
        protocol_version: 22,
        // Slightly cheaper CPU (larger divisor → lower per-unit fee).
        cpu_insns_per_fee_unit: 12_500,
        // Memory cost stays the same.
        mem_bytes_per_fee_unit: 1_024,
        // Ledger I/O gets more expensive (smaller divisor → higher fee).
        ledger_bytes_per_fee_unit: 768,
        tx_size_bytes_per_fee_unit: 1_024,
        // Higher CPU budget — allows more complex contracts.
        tx_max_instructions: 200_000_000,
        tx_max_memory_bytes: 64 * 1024 * 1024, // 64 MiB
        tx_max_read_bytes: 200 * 1024,
        tx_max_write_bytes: 131_072, // 128 KiB
        tx_max_size_bytes: 71_680,
    }
}

/// Custom / Private Network — sensible defaults that can be overridden via
/// the API request body.
pub fn custom_private() -> NetworkConfig {
    NetworkConfig {
        name: "Custom Private Network".to_string(),
        protocol_version: 21,
        cpu_insns_per_fee_unit: 10_000,
        mem_bytes_per_fee_unit: 1_024,
        ledger_bytes_per_fee_unit: 1_024,
        tx_size_bytes_per_fee_unit: 1_024,
        tx_max_instructions: 500_000_000,       // generous
        tx_max_memory_bytes: 128 * 1024 * 1024, // 128 MiB
        tx_max_read_bytes: 1024 * 1024,         // 1 MiB
        tx_max_write_bytes: 512 * 1024,         // 512 KiB
        tx_max_size_bytes: 256 * 1024,          // 256 KiB
    }
}

/// Resolve a preset name to the corresponding `NetworkConfig`.
///
/// Recognised names (case-insensitive):
/// - `"protocol_21"` / `"p21"` / `"current"`
/// - `"protocol_22"` / `"p22"` / `"next"` / `"upcoming"`
/// - `"custom"` / `"private"`
pub fn resolve_preset(name: &str) -> Option<NetworkConfig> {
    match name.to_lowercase().as_str() {
        "protocol_21" | "p21" | "current" => Some(protocol_21()),
        "protocol_22" | "p22" | "next" | "upcoming" => Some(protocol_22()),
        "custom" | "private" => Some(custom_private()),
        _ => None,
    }
}

// ── Impact comparison ─────────────────────────────────────────────────────────

/// Side-by-side comparison of a transaction's cost under two protocol configs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolImpact {
    pub baseline: ProtocolCostSnapshot,
    pub shadow: ProtocolCostSnapshot,
    /// Signed difference: `shadow.cost_stroops - baseline.cost_stroops`.
    /// Positive means the shadow config is *more* expensive.
    pub cost_difference_stroops: i64,
    /// Percentage change: `(shadow - baseline) / baseline * 100`.
    pub cost_change_pct: f64,
}

/// Cost snapshot under a single protocol configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolCostSnapshot {
    pub config_name: String,
    pub protocol_version: u32,
    pub cost_stroops: u64,
    pub limits_exceeded: Vec<LimitExceeded>,
}

/// Compare a resource footprint across two configurations and produce an
/// impact report.
pub fn compare(
    resources: &SorobanResources,
    baseline: &NetworkConfig,
    shadow: &NetworkConfig,
) -> ProtocolImpact {
    let baseline_cost = baseline.calculate_cost(resources);
    let shadow_cost = shadow.calculate_cost(resources);

    let diff = shadow_cost as i64 - baseline_cost as i64;
    let pct = if baseline_cost > 0 {
        (diff as f64 / baseline_cost as f64) * 100.0
    } else {
        0.0
    };

    ProtocolImpact {
        baseline: ProtocolCostSnapshot {
            config_name: baseline.name.clone(),
            protocol_version: baseline.protocol_version,
            cost_stroops: baseline_cost,
            limits_exceeded: baseline.check_limits(resources),
        },
        shadow: ProtocolCostSnapshot {
            config_name: shadow.name.clone(),
            protocol_version: shadow.protocol_version,
            cost_stroops: shadow_cost,
            limits_exceeded: shadow.check_limits(resources),
        },
        cost_difference_stroops: diff,
        cost_change_pct: pct,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_resources() -> SorobanResources {
        SorobanResources {
            cpu_instructions: 1_000_000,
            ram_bytes: 2_048,
            ledger_read_bytes: 512,
            ledger_write_bytes: 256,
            transaction_size_bytes: 1_024,
        }
    }

    #[test]
    fn test_protocol_21_cost_matches_legacy() {
        let r = sample_resources();
        let cfg = protocol_21();
        // Legacy formula: cpu/10000 + ram/1024 + (read+write)/1024
        let legacy = r.cpu_instructions / 10_000
            + r.ram_bytes / 1_024
            + (r.ledger_read_bytes + r.ledger_write_bytes) / 1_024
            + r.transaction_size_bytes / 1_024;
        assert_eq!(cfg.calculate_cost(&r), legacy);
    }

    #[test]
    fn test_protocol_22_cheaper_cpu() {
        let r = sample_resources();
        let p21 = protocol_21();
        let p22 = protocol_22();
        // P22 has a higher cpu_insns_per_fee_unit, so CPU portion is cheaper.
        let p21_cpu = r.cpu_instructions / p21.cpu_insns_per_fee_unit;
        let p22_cpu = r.cpu_instructions / p22.cpu_insns_per_fee_unit;
        assert!(p22_cpu < p21_cpu, "P22 should have cheaper CPU fees");
    }

    #[test]
    fn test_protocol_22_more_expensive_ledger() {
        let r = sample_resources();
        let p21 = protocol_21();
        let p22 = protocol_22();
        let p21_ledger =
            (r.ledger_read_bytes + r.ledger_write_bytes) / p21.ledger_bytes_per_fee_unit;
        let p22_ledger =
            (r.ledger_read_bytes + r.ledger_write_bytes) / p22.ledger_bytes_per_fee_unit;
        assert!(
            p22_ledger >= p21_ledger,
            "P22 should have same or higher ledger fees"
        );
    }

    #[test]
    fn test_compare_produces_correct_diff() {
        let r = sample_resources();
        let impact = compare(&r, &protocol_21(), &protocol_22());
        let expected_diff = impact.shadow.cost_stroops as i64 - impact.baseline.cost_stroops as i64;
        assert_eq!(impact.cost_difference_stroops, expected_diff);
    }

    #[test]
    fn test_compare_percentage() {
        let r = sample_resources();
        let impact = compare(&r, &protocol_21(), &protocol_22());
        let expected_pct =
            (impact.cost_difference_stroops as f64 / impact.baseline.cost_stroops as f64) * 100.0;
        assert!((impact.cost_change_pct - expected_pct).abs() < 0.001);
    }

    #[test]
    fn test_check_limits_within_budget() {
        let r = sample_resources();
        assert!(protocol_21().check_limits(&r).is_empty());
        assert!(protocol_22().check_limits(&r).is_empty());
        assert!(custom_private().check_limits(&r).is_empty());
    }

    #[test]
    fn test_check_limits_exceeded() {
        let r = SorobanResources {
            cpu_instructions: 500_000_000, // exceeds P21 limit of 100M
            ram_bytes: 2_048,
            ledger_read_bytes: 512,
            ledger_write_bytes: 512,
            transaction_size_bytes: 1_024,
        };
        let exceeded = protocol_21().check_limits(&r);
        assert_eq!(exceeded.len(), 1);
        assert_eq!(exceeded[0].resource, "cpu_instructions");
        assert_eq!(exceeded[0].used, 500_000_000);
        assert_eq!(exceeded[0].limit, 100_000_000);
    }

    #[test]
    fn test_resolve_preset_case_insensitive() {
        assert!(resolve_preset("protocol_21").is_some());
        assert!(resolve_preset("P21").is_some());
        assert!(resolve_preset("CURRENT").is_some());
        assert!(resolve_preset("protocol_22").is_some());
        assert!(resolve_preset("Next").is_some());
        assert!(resolve_preset("custom").is_some());
        assert!(resolve_preset("unknown").is_none());
    }

    #[test]
    fn test_resolve_preset_returns_correct_version() {
        let p21 = resolve_preset("p21").unwrap();
        assert_eq!(p21.protocol_version, 21);
        let p22 = resolve_preset("p22").unwrap();
        assert_eq!(p22.protocol_version, 22);
    }

    #[test]
    fn test_custom_private_generous_limits() {
        let cfg = custom_private();
        assert!(cfg.tx_max_instructions > protocol_21().tx_max_instructions);
        assert!(cfg.tx_max_memory_bytes > protocol_21().tx_max_memory_bytes);
    }

    #[test]
    fn test_network_config_serialization() {
        let cfg = protocol_21();
        let json = serde_json::to_string(&cfg).unwrap();
        let deserialized: NetworkConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg, deserialized);
    }

    #[test]
    fn test_protocol_impact_serialization() {
        let r = sample_resources();
        let impact = compare(&r, &protocol_21(), &protocol_22());
        let json = serde_json::to_string(&impact).unwrap();
        let deserialized: ProtocolImpact = serde_json::from_str(&json).unwrap();
        assert_eq!(impact.baseline, deserialized.baseline);
        assert_eq!(impact.shadow, deserialized.shadow);
        assert_eq!(
            impact.cost_difference_stroops,
            deserialized.cost_difference_stroops
        );
        // f64 round-trip through JSON may introduce tiny precision differences.
        assert!((impact.cost_change_pct - deserialized.cost_change_pct).abs() < 1e-10);
    }

    #[test]
    fn test_zero_resources_zero_cost() {
        let r = SorobanResources::default();
        assert_eq!(protocol_21().calculate_cost(&r), 0);
        assert_eq!(protocol_22().calculate_cost(&r), 0);
    }

    #[test]
    fn test_compare_identical_configs() {
        let r = sample_resources();
        let impact = compare(&r, &protocol_21(), &protocol_21());
        assert_eq!(impact.cost_difference_stroops, 0);
        assert!((impact.cost_change_pct - 0.0).abs() < 0.001);
    }
}
