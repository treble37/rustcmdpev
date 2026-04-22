use crate::constants::{
    CTE_SCAN_NODE, DELTA_ERROR_THRESHOLD, OVER_LABEL, UNDER_LABEL,
};
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;

// Small explain plans are cheap to clone, which keeps the recursive traversal
// straightforward and avoids aliasing issues between sibling updates.
pub fn calculate_planner_estimate(plan: Plan) -> Plan {
    let mut new_plan = plan;
    new_plan.analysis_flags.planner_row_estimate_factor = 0.0;

    if new_plan.estimates.plan_rows == new_plan.actuals.actual_rows {
        return new_plan;
    }

    new_plan.analysis_flags.planner_row_estimate_direction = UNDER_LABEL.to_string();
    if new_plan.estimates.plan_rows != 0 {
        new_plan.analysis_flags.planner_row_estimate_factor =
            new_plan.actuals.actual_rows as f64 / new_plan.estimates.plan_rows as f64;
    }

    if new_plan.analysis_flags.planner_row_estimate_factor < 1.0 {
        new_plan.analysis_flags.planner_row_estimate_factor = 0.0;
        new_plan.analysis_flags.planner_row_estimate_direction = OVER_LABEL.to_string();
        if new_plan.actuals.actual_rows != 0 {
            new_plan.analysis_flags.planner_row_estimate_factor =
                new_plan.estimates.plan_rows as f64 / new_plan.actuals.actual_rows as f64;
        }
    }

    new_plan
}

pub fn calculate_actuals(explain: Explain, plan: Plan) -> (Explain, Plan) {
    let mut new_plan = plan;
    let mut new_explain = explain;
    new_plan.actuals.actual_duration = new_plan.actuals.actual_total_time;
    new_plan.actuals.actual_cost = new_plan.estimates.total_cost;

    for child_plan in &new_plan.plans {
        if child_plan.node_type != CTE_SCAN_NODE {
            new_plan.actuals.actual_duration -= child_plan.actuals.actual_total_time;
            new_plan.actuals.actual_cost -= child_plan.estimates.total_cost;
        }
    }

    if new_plan.actuals.actual_cost < 0.0 {
        new_plan.actuals.actual_cost = 0.0;
    }

    new_explain.total_cost += new_plan.actuals.actual_cost;
    new_plan.actuals.actual_duration *= new_plan.actuals.actual_loops as f64;
    (new_explain, new_plan)
}

pub fn calculate_maximums(explain: Explain, plan: Plan) -> Explain {
    let mut new_explain = explain;
    if new_explain.max_rows < plan.actuals.actual_rows {
        new_explain.max_rows = plan.actuals.actual_rows;
    }
    if new_explain.max_cost < plan.actuals.actual_cost {
        new_explain.max_cost = plan.actuals.actual_cost;
    }
    if new_explain.max_duration < plan.actuals.actual_duration {
        new_explain.max_duration = plan.actuals.actual_duration;
    }
    new_explain
}

pub fn calculate_outlier_nodes(explain: Explain, plan: Plan) -> Plan {
    let mut new_plan = plan;
    new_plan.analysis_flags.costliest =
        (new_plan.actuals.actual_cost - explain.max_cost).abs() < DELTA_ERROR_THRESHOLD;
    new_plan.analysis_flags.largest = new_plan.actuals.actual_rows == explain.max_rows;
    new_plan.analysis_flags.slowest =
        (new_plan.actuals.actual_duration - explain.max_duration).abs() < DELTA_ERROR_THRESHOLD;

    for child_plan in &mut new_plan.plans {
        *child_plan = calculate_outlier_nodes(explain.clone(), child_plan.clone());
    }

    new_plan
}

fn process_root(explain: Explain) -> Explain {
    let mut new_explain = explain;
    new_explain.plan = calculate_planner_estimate(new_explain.plan);
    let (updated_explain, updated_plan) = calculate_actuals(new_explain.clone(), new_explain.plan);
    new_explain = updated_explain;
    new_explain.plan = updated_plan;
    calculate_maximums(new_explain.clone(), new_explain.plan)
}

fn process_child_plans(explain: Explain, plans: Vec<Plan>) -> (Explain, Vec<Plan>) {
    let mut new_explain = explain;
    let mut new_plans = plans;

    for child_plan in &mut new_plans {
        *child_plan = calculate_planner_estimate(child_plan.clone());
        let (updated_explain, updated_plan) = calculate_actuals(new_explain.clone(), child_plan.clone());
        new_explain = updated_explain;
        *child_plan = updated_plan;
        new_explain = calculate_maximums(new_explain, child_plan.clone());

        if !child_plan.plans.is_empty() {
            let (updated_explain, child_plans) =
                process_child_plans(new_explain.clone(), child_plan.plans.clone());
            child_plan.plans = child_plans;
            new_explain = updated_explain;
        }
    }

    (new_explain, new_plans)
}

pub fn process_all(explain: Explain) -> Explain {
    let mut new_explain = process_root(explain);

    if !new_explain.plan.plans.is_empty() {
        let (updated_explain, child_plans) =
            process_child_plans(new_explain.clone(), new_explain.plan.plans.clone());
        new_explain = updated_explain;
        new_explain.plan.plans = child_plans;
    }

    new_explain.plan = calculate_outlier_nodes(new_explain.clone(), new_explain.plan);
    new_explain
}
