use serde::{Deserialize, Serialize};
use std::fmt;

const UNDER: &str = "Under";
const OVER: &str = "Over";
const CTE_SCAN: &str = "CTE Scan";

type NodeType = String;
type EstimateDirection = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Explain {
    //TODO: add Triggers back, add default for plan?
    plan: Plan,
    #[serde(default)]
    planning_time: f64,
    #[serde(default)]
    execution_time: f64,
    #[serde(default)]
    total_cost: f64,
    #[serde(default)]
    max_rows: u64,
    #[serde(default)]
    max_cost: f64,
    #[serde(default)]
    max_duration: f64,
}

//https://github.com/serde-rs/serde/pull/238
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Plan {
    #[serde(default)]
    actual_cost: f64,
    #[serde(default)]
    actual_duration: f64,
    #[serde(default)]
    actual_loops: u64,
    #[serde(default)]
    actual_rows: u64,
    #[serde(default)]
    actual_startup_time: f64,
    #[serde(default)]
    actual_total_time: f64,
    #[serde(default)]
    alias: String,
    #[serde(default)]
    costliest: bool,
    #[serde(default)]
    cte_name: String,
    #[serde(default)]
    filter: String,
    #[serde(default)]
    group_key: Vec<String>,
    #[serde(default)]
    hash_condition: String,
    #[serde(default)]
    heap_fetches: u64,
    #[serde(default)]
    index_condition: String,
    #[serde(default)]
    index_name: String,
    #[serde(default)]
    io_read_time: f64,
    #[serde(default)]
    io_write_time: f64,
    #[serde(default)]
    join_type: String,
    #[serde(default)]
    largest: bool,
    #[serde(default)]
    local_dirtied_blocks: u64,
    #[serde(default)]
    local_hit_blocks: u64,
    #[serde(default)]
    local_read_blocks: u64,
    #[serde(default)]
    local_written_blocks: u64,
    #[serde(default)]
    node_type: NodeType,
    #[serde(default)]
    output: Vec<String>,
    #[serde(default)]
    parent_relationship: String,
    #[serde(default)]
    planner_row_estimate_direction: EstimateDirection,
    #[serde(default)]
    planner_row_estimate_factor: f64,
    #[serde(default)]
    plan_rows: u64,
    #[serde(default)]
    plan_width: u64,
    #[serde(default)]
    relation_name: String,
    #[serde(default)]
    rows_removed_by_filter: u64,
    #[serde(default)]
    rows_removed_by_index_recheck: u64,
    #[serde(default)]
    scan_direction: String,
    #[serde(default)]
    schema: String,
    #[serde(default)]
    shared_dirtied_blocks: u64,
    #[serde(default)]
    shared_hit_blocks: u64,
    #[serde(default)]
    shared_read_blocks: u64,
    #[serde(default)]
    shared_written_blocks: u64,
    #[serde(default)]
    slowest: bool,
    #[serde(default)]
    startup_cost: f64,
    #[serde(default)]
    strategy: String,
    #[serde(default)]
    temp_read_blocks: u64,
    #[serde(default)]
    temp_written_blocks: u64,
    #[serde(default)]
    total_cost: f64,
    #[serde(default)]
    plans: Vec<Plan>,
}

impl fmt::Display for Plan {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //write!(f, "{}", self.0)
        write!(f, "{}", self)
    }
}

pub fn calculate_planner_estimate(mut plan: Plan) -> Plan {
    plan.planner_row_estimate_factor = 0.0;

    if plan.plan_rows == plan.actual_rows {
        return plan;
    }

    plan.planner_row_estimate_direction = UNDER.to_string();
    if plan.plan_rows != 0 {
        plan.planner_row_estimate_factor = plan.actual_rows as f64 / plan.plan_rows as f64;
    }

    if plan.planner_row_estimate_factor < 1.0 {
        plan.planner_row_estimate_factor = 0.0;
        plan.planner_row_estimate_direction = OVER.to_string();
        if plan.actual_rows != 0 {
            plan.planner_row_estimate_factor = plan.plan_rows as f64 / plan.actual_rows as f64;
        }
    }
    plan
}

pub fn calculate_actuals(mut explain: Explain, mut plan: Plan) -> (Explain, Plan) {
    plan.actual_duration = plan.actual_total_time;
    plan.actual_cost = plan.total_cost;

    for mut child_plan in &mut plan.plans {
        if child_plan.node_type != CTE_SCAN {
            child_plan.actual_duration = child_plan.actual_duration - child_plan.actual_total_time;
            child_plan.actual_cost = child_plan.actual_cost - child_plan.total_cost;
        }
    }

    if plan.actual_cost < 0.0 {
        plan.actual_cost = 0.0;
    }

    explain.total_cost = explain.total_cost + plan.actual_cost;

    plan.actual_duration = plan.actual_duration * plan.actual_loops as f64;
    (explain, plan)
}

pub fn calculate_maximums(mut explain: Explain, plan: Plan) -> Explain {
    if explain.max_rows < plan.actual_rows {
        explain.max_rows = plan.actual_rows
    }
    if explain.max_cost < plan.actual_cost {
        explain.max_cost = plan.actual_cost
    }
    if explain.max_duration < plan.actual_duration {
        explain.max_duration = plan.actual_duration
    }
    explain
}

pub fn process_explain(mut explain: Explain) -> Explain {
    let plan: Plan = calculate_planner_estimate(explain.plan.clone());
    let (e, plan) = calculate_actuals(explain.clone(), plan);
    explain = calculate_maximums(e.clone(), plan);
    //need to figure out how to deal with recursive process_plan
    for mut child_plan in e.plan.plans {
        child_plan = calculate_planner_estimate(child_plan.clone());
        let (explain2, child_plan) = calculate_actuals(explain.clone(), child_plan.clone());
        explain = calculate_maximums(explain2, child_plan);
    }
    explain
}
