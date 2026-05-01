use crate::constants::{CTE_SCAN_NODE, DELTA_ERROR_THRESHOLD, OVER_LABEL, UNDER_LABEL};
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;

pub fn calculate_planner_estimate(plan: &mut Plan) {
    plan.analysis_flags.planner_row_estimate_factor = 0.0;

    if plan.estimates.plan_rows == plan.actuals.actual_rows {
        return;
    }

    plan.analysis_flags.planner_row_estimate_direction = UNDER_LABEL.to_string();
    if plan.estimates.plan_rows != 0 {
        plan.analysis_flags.planner_row_estimate_factor =
            plan.actuals.actual_rows as f64 / plan.estimates.plan_rows as f64;
    }

    if plan.analysis_flags.planner_row_estimate_factor < 1.0 {
        plan.analysis_flags.planner_row_estimate_factor = 0.0;
        plan.analysis_flags.planner_row_estimate_direction = OVER_LABEL.to_string();
        if plan.actuals.actual_rows != 0 {
            plan.analysis_flags.planner_row_estimate_factor =
                plan.estimates.plan_rows as f64 / plan.actuals.actual_rows as f64;
        }
    }
}

pub fn calculate_actuals(explain: &mut Explain, plan: &mut Plan) {
    plan.actuals.actual_duration = plan.actuals.actual_total_time;
    plan.actuals.actual_cost = plan.estimates.total_cost;

    for child_plan in &plan.plans {
        if child_plan.identity.node_type != CTE_SCAN_NODE {
            plan.actuals.actual_duration -= child_plan.actuals.actual_total_time;
            plan.actuals.actual_cost -= child_plan.estimates.total_cost;
        }
    }

    if plan.actuals.actual_cost < 0.0 {
        plan.actuals.actual_cost = 0.0;
    }

    explain.total_cost += plan.actuals.actual_cost;
    plan.actuals.actual_duration *= plan.actuals.actual_loops as f64;
}

pub fn calculate_maximums(explain: &mut Explain, plan: &Plan) {
    if explain.max_rows < plan.actuals.actual_rows {
        explain.max_rows = plan.actuals.actual_rows;
    }
    if explain.max_cost < plan.actuals.actual_cost {
        explain.max_cost = plan.actuals.actual_cost;
    }
    if explain.max_duration < plan.actuals.actual_duration {
        explain.max_duration = plan.actuals.actual_duration;
    }
}

/// Outlier comparison thresholds extracted from the analyzed explain so the
/// recursion does not need to re-borrow `Explain` while the plan tree is held
/// mutably.
#[derive(Debug, Clone, Copy)]
struct OutlierMaxima {
    max_cost: f64,
    max_rows: u64,
    max_duration: f64,
}

impl OutlierMaxima {
    fn from(explain: &Explain) -> Self {
        Self {
            max_cost: explain.max_cost,
            max_rows: explain.max_rows,
            max_duration: explain.max_duration,
        }
    }
}

pub fn calculate_outlier_nodes(explain: &Explain, plan: &mut Plan) {
    let maxima = OutlierMaxima::from(explain);
    flag_outliers(maxima, plan);
}

fn flag_outliers(maxima: OutlierMaxima, plan: &mut Plan) {
    plan.analysis_flags.costliest =
        (plan.actuals.actual_cost - maxima.max_cost).abs() < DELTA_ERROR_THRESHOLD;
    plan.analysis_flags.largest = plan.actuals.actual_rows == maxima.max_rows;
    plan.analysis_flags.slowest =
        (plan.actuals.actual_duration - maxima.max_duration).abs() < DELTA_ERROR_THRESHOLD;

    for child_plan in &mut plan.plans {
        flag_outliers(maxima, child_plan);
    }
}

fn process_root(explain: &mut Explain) {
    calculate_planner_estimate(&mut explain.plan);
    let mut plan = std::mem::take(&mut explain.plan);
    calculate_actuals(explain, &mut plan);
    calculate_maximums(explain, &plan);
    explain.plan = plan;
}

fn process_child_plans(explain: &mut Explain, plans: &mut [Plan]) {
    for child_plan in plans.iter_mut() {
        calculate_planner_estimate(child_plan);
        calculate_actuals(explain, child_plan);
        calculate_maximums(explain, child_plan);

        if !child_plan.plans.is_empty() {
            let mut nested = std::mem::take(&mut child_plan.plans);
            process_child_plans(explain, &mut nested);
            child_plan.plans = nested;
        }
    }
}

pub fn process_all(mut explain: Explain) -> Explain {
    process_root(&mut explain);

    if !explain.plan.plans.is_empty() {
        let mut child_plans = std::mem::take(&mut explain.plan.plans);
        process_child_plans(&mut explain, &mut child_plans);
        explain.plan.plans = child_plans;
    }

    let maxima = OutlierMaxima::from(&explain);
    flag_outliers(maxima, &mut explain.plan);
    explain
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::data::actuals::PlanActuals;
    use crate::structure::data::estimates::PlanEstimates;

    fn leaf(node_type: &str, total_cost: f64, total_time: f64, rows: u64) -> Plan {
        let mut plan = Plan::default();
        plan.identity.node_type = node_type.to_string();
        plan.estimates = PlanEstimates {
            total_cost,
            ..PlanEstimates::default()
        };
        plan.actuals = PlanActuals {
            actual_total_time: total_time,
            actual_rows: rows,
            actual_loops: 1,
            ..PlanActuals::default()
        };
        plan
    }

    #[test]
    fn calculate_planner_estimate_does_not_clone_plan() {
        let mut plan = leaf("Seq Scan", 10.0, 1.0, 0);
        plan.estimates.plan_rows = 100;
        plan.actuals.actual_rows = 5;
        calculate_planner_estimate(&mut plan);
        assert_eq!(plan.analysis_flags.planner_row_estimate_direction, "Over");
        assert_eq!(plan.analysis_flags.planner_row_estimate_factor, 20.0);
    }

    #[test]
    fn calculate_actuals_subtracts_non_cte_children_in_place() {
        let mut explain = Explain::default();
        let mut plan = leaf("Hash Join", 10.0, 5.0, 100);
        plan.plans.push(leaf("Seq Scan", 4.0, 2.0, 50));
        plan.plans.push(leaf("Seq Scan", 3.0, 1.0, 50));

        calculate_actuals(&mut explain, &mut plan);

        assert!((plan.actuals.actual_cost - 3.0).abs() < 1e-6);
        assert!((plan.actuals.actual_duration - 2.0).abs() < 1e-6);
    }

    #[test]
    fn calculate_outlier_nodes_recurses_via_mutable_reference() {
        let explain = Explain {
            max_cost: 5.0,
            max_rows: 10,
            max_duration: 2.0,
            ..Explain::default()
        };

        let mut plan = leaf("Hash Join", 5.0, 2.0, 10);
        plan.actuals.actual_cost = 5.0;
        plan.actuals.actual_duration = 2.0;
        let mut child = leaf("Seq Scan", 2.0, 1.0, 5);
        child.actuals.actual_cost = 2.0;
        child.actuals.actual_duration = 1.0;
        plan.plans.push(child);

        calculate_outlier_nodes(&explain, &mut plan);

        assert!(plan.analysis_flags.costliest);
        assert!(plan.analysis_flags.largest);
        assert!(plan.analysis_flags.slowest);
        assert!(!plan.plans[0].analysis_flags.costliest);
        assert!(!plan.plans[0].analysis_flags.slowest);
    }

    #[test]
    fn process_all_threads_through_without_per_node_clones() {
        let mut root = leaf("Hash Join", 10.0, 5.0, 100);
        root.plans.push(leaf("Seq Scan", 4.0, 2.0, 50));
        root.plans.push(leaf("Seq Scan", 3.0, 1.0, 50));
        let explain = Explain {
            execution_time: 5.0,
            plan: root,
            ..Explain::default()
        };

        let processed = process_all(explain);

        assert_eq!(processed.plan.plans.len(), 2);
        assert!(processed.max_cost > 0.0);
        assert!(processed.plan.analysis_flags.costliest || processed.plan.analysis_flags.slowest);
    }
}
