//! G9.3 — Property tests for plan invariants.
//!
//! Generates random plan trees and asserts the post-analysis invariants the
//! renderer relies on:
//!   - actual_cost is always non-negative (clamped by analysis).
//!   - parent's max_cost / max_rows / max_duration dominates every node.
//!   - planner_row_estimate_factor is a finite, non-negative number.
//!   - tree shape is preserved (node count + child counts at each depth).

use proptest::prelude::*;
use rustcmdpev_core::analysis;
use rustcmdpev_core::structure::data::actuals::PlanActuals;
use rustcmdpev_core::structure::data::estimates::PlanEstimates;
use rustcmdpev_core::structure::data::explain::Explain;
use rustcmdpev_core::structure::data::plan::Plan;

#[derive(Debug, Clone)]
struct PlanSpec {
    node_type: String,
    plan_rows: u64,
    actual_rows: u64,
    total_cost: f64,
    actual_total_time: f64,
    actual_loops: u64,
    children: Vec<PlanSpec>,
}

impl PlanSpec {
    fn into_plan(self) -> Plan {
        let mut plan = Plan::default();
        plan.identity.node_type = self.node_type;
        plan.estimates = PlanEstimates {
            plan_rows: self.plan_rows,
            total_cost: self.total_cost,
            ..PlanEstimates::default()
        };
        plan.actuals = PlanActuals {
            actual_total_time: self.actual_total_time,
            actual_rows: self.actual_rows,
            actual_loops: self.actual_loops,
            ..PlanActuals::default()
        };
        plan.plans = self.children.into_iter().map(PlanSpec::into_plan).collect();
        plan
    }

    fn count_nodes(&self) -> usize {
        1 + self.children.iter().map(|c| c.count_nodes()).sum::<usize>()
    }
}

fn arb_node_type() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Seq Scan".to_string()),
        Just("Index Scan".to_string()),
        Just("Hash Join".to_string()),
        Just("Merge Join".to_string()),
        Just("Nested Loop".to_string()),
        Just("Sort".to_string()),
        Just("Aggregate".to_string()),
        Just("Limit".to_string()),
        Just("Append".to_string()),
    ]
}

fn arb_plan_spec() -> impl Strategy<Value = PlanSpec> {
    let leaf = (
        arb_node_type(),
        0u64..1_000,
        0u64..1_000,
        0.0f64..10_000.0,
        0.0f64..1_000.0,
        1u64..8,
    )
        .prop_map(
            |(node_type, plan_rows, actual_rows, total_cost, actual_total_time, actual_loops)| {
                PlanSpec {
                    node_type,
                    plan_rows,
                    actual_rows,
                    total_cost,
                    actual_total_time,
                    actual_loops,
                    children: Vec::new(),
                }
            },
        );

    leaf.prop_recursive(4, 32, 3, |inner| {
        (
            arb_node_type(),
            0u64..1_000,
            0u64..1_000,
            0.0f64..10_000.0,
            0.0f64..1_000.0,
            1u64..8,
            prop::collection::vec(inner, 0..3),
        )
            .prop_map(
                |(
                    node_type,
                    plan_rows,
                    actual_rows,
                    total_cost,
                    actual_total_time,
                    actual_loops,
                    children,
                )| {
                    PlanSpec {
                        node_type,
                        plan_rows,
                        actual_rows,
                        total_cost,
                        actual_total_time,
                        actual_loops,
                        children,
                    }
                },
            )
    })
}

fn flatten<'a>(plan: &'a Plan, out: &mut Vec<&'a Plan>) {
    out.push(plan);
    for c in &plan.plans {
        flatten(c, out);
    }
}

fn structure_signature(plan: &Plan) -> Vec<usize> {
    let mut sig = vec![plan.plans.len()];
    for c in &plan.plans {
        sig.extend(structure_signature(c));
    }
    sig
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn analysis_keeps_actual_cost_non_negative(spec in arb_plan_spec()) {
        let expected_count = spec.count_nodes();
        let plan = spec.into_plan();
        let original_signature = structure_signature(&plan);

        let processed = analysis::process_all(Explain {
            plan,
            ..Explain::default()
        });

        let mut nodes = Vec::new();
        flatten(&processed.plan, &mut nodes);
        prop_assert_eq!(nodes.len(), expected_count);
        prop_assert_eq!(structure_signature(&processed.plan), original_signature);

        for node in &nodes {
            prop_assert!(node.actuals.actual_cost >= 0.0);
            prop_assert!(node.actuals.actual_duration.is_finite());
            let factor = node.analysis_flags.planner_row_estimate_factor;
            prop_assert!(factor.is_finite());
            prop_assert!(factor >= 0.0);
        }
    }

    #[test]
    fn maxima_dominate_every_node(spec in arb_plan_spec()) {
        let plan = spec.into_plan();
        let processed = analysis::process_all(Explain {
            plan,
            ..Explain::default()
        });
        let mut nodes = Vec::new();
        flatten(&processed.plan, &mut nodes);
        for node in &nodes {
            prop_assert!(node.actuals.actual_cost <= processed.max_cost + 1e-9);
            prop_assert!(node.actuals.actual_rows <= processed.max_rows);
            prop_assert!(node.actuals.actual_duration <= processed.max_duration + 1e-9);
        }
    }

    #[test]
    fn at_least_one_outlier_flag_when_nodes_present(spec in arb_plan_spec()) {
        let plan = spec.into_plan();
        let processed = analysis::process_all(Explain {
            plan,
            ..Explain::default()
        });
        let mut nodes = Vec::new();
        flatten(&processed.plan, &mut nodes);
        let any_flag = nodes.iter().any(|n| {
            n.analysis_flags.slowest
                || n.analysis_flags.costliest
                || n.analysis_flags.largest
        });
        prop_assert!(any_flag, "expected at least one outlier flag across the tree");
    }
}
