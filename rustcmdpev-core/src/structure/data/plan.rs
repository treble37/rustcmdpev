//https://github.com/serde-rs/serde/pull/238
use crate::structure::data::actuals::PlanActuals;
use crate::structure::data::analysis_flags::PlanAnalysisFlags;
use crate::structure::data::buffers::PlanBuffers;
use crate::structure::data::estimates::PlanEstimates;
use crate::structure::data::identity::PlanIdentity;
use crate::structure::data::io_timing::PlanIoTiming;
use crate::structure::data::predicates::PlanPredicates;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The Plan struct.
///
/// Storage is grouped into typed sub-structs (`identity`, `predicates`,
/// `buffers`, `io_timing`, `estimates`, `actuals`, `analysis_flags`) so that
/// related fields stay co-located and downstream code can pass cohesive
/// slices of state instead of every field individually.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Plan {
    #[serde(flatten)]
    pub actuals: PlanActuals,
    #[serde(flatten)]
    pub analysis_flags: PlanAnalysisFlags,
    #[serde(flatten)]
    pub identity: PlanIdentity,
    #[serde(flatten)]
    pub predicates: PlanPredicates,
    #[serde(flatten)]
    pub buffers: PlanBuffers,
    #[serde(flatten)]
    pub estimates: PlanEstimates,
    #[serde(flatten)]
    pub io_timing: PlanIoTiming,
    #[serde(default, rename(deserialize = "Plans"))]
    pub plans: Vec<Plan>,
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.identity.node_type)
    }
}

impl Plan {
    pub fn child_count(&self) -> usize {
        self.plans.len()
    }

    pub fn has_children(&self) -> bool {
        !self.plans.is_empty()
    }

    pub fn is_leaf(&self) -> bool {
        self.plans.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_postgres_fields_into_grouped_substructs() {
        let json = r#"{
            "Node Type": "Index Scan",
            "Schema": "public",
            "Relation Name": "coaches",
            "Index Name": "coaches_pkey",
            "Index Cond": "(id = 1)",
            "Filter": "active",
            "Output": ["a", "b"],
            "Heap Fetches": 7,
            "Shared Hit Blocks": 3,
            "I/O Read Time": 1.5,
            "I/O Write Time": 2.5,
            "Plan Rows": 9,
            "Total Cost": 4.5,
            "Actual Rows": 9,
            "Actual Loops": 1
        }"#;

        let plan: Plan = serde_json::from_str(json).expect("deserialize");

        assert_eq!(plan.identity.node_type, "Index Scan");
        assert_eq!(plan.identity.schema, "public");
        assert_eq!(plan.identity.relation_name, "coaches");
        assert_eq!(plan.identity.index_name, "coaches_pkey");
        assert_eq!(plan.predicates.index_condition, "(id = 1)");
        assert_eq!(plan.predicates.filter, "active");
        assert_eq!(plan.predicates.output, vec!["a", "b"]);
        assert_eq!(plan.buffers.heap_fetches, 7);
        assert_eq!(plan.buffers.shared_hit_blocks, 3);
        assert_eq!(plan.io_timing.io_read_time, 1.5);
        assert_eq!(plan.io_timing.io_write_time, 2.5);
        assert_eq!(plan.estimates.plan_rows, 9);
        assert_eq!(plan.estimates.total_cost, 4.5);
        assert_eq!(plan.actuals.actual_rows, 9);
    }

    #[test]
    fn display_uses_grouped_identity_node_type() {
        let mut plan = Plan::default();
        plan.identity.node_type = "Hash Join".to_string();
        assert_eq!(format!("{plan}"), "Hash Join");
    }
}
