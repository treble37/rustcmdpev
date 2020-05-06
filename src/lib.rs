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
    pub plan: Plan,
    #[serde(default)]
    pub planning_time: f64,
    #[serde(default)]
    pub execution_time: f64,
    #[serde(default)]
    pub total_cost: f64,
    #[serde(default)]
    pub max_rows: u64,
    #[serde(default)]
    pub max_cost: f64,
    #[serde(default)]
    pub max_duration: f64,
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

impl Default for Plan {
    fn default() -> Plan {
        Plan {
            actual_cost: 0.0,
            actual_duration: 0.0,
            actual_loops: 0,
            actual_rows: 0,
            actual_startup_time: 0.0,
            actual_total_time: 0.0,
            alias: String::from(""),
            costliest: false,
            cte_name: String::from(""),
            filter: String::from(""),
            group_key: Vec::new(),
            hash_condition: String::from(""),
            heap_fetches: 0,
            index_condition: String::from(""),
            index_name: String::from(""),
            io_read_time: 0.0,
            io_write_time: 0.0,
            join_type: String::from(""),
            largest: false,
            local_dirtied_blocks: 0,
            local_hit_blocks: 0,
            local_read_blocks: 0,
            local_written_blocks: 0,
            node_type: String::from(""),
            output: Vec::new(),
            parent_relationship: String::from(""),
            planner_row_estimate_direction: String::from(""),
            planner_row_estimate_factor: 0.0,
            plan_rows: 0,
            plan_width: 0,
            relation_name: String::from(""),
            rows_removed_by_filter: 0,
            rows_removed_by_index_recheck: 0,
            scan_direction: String::from(""),
            schema: String::from(""),
            shared_dirtied_blocks: 0,
            shared_hit_blocks: 0,
            shared_read_blocks: 0,
            shared_written_blocks: 0,
            slowest: false,
            startup_cost: 0.0,
            strategy: String::from(""),
            temp_read_blocks: 0,
            temp_written_blocks: 0,
            total_cost: 0.0,
            plans: Vec::new(),
        }
    }
}

impl Default for Explain {
    fn default() -> Explain {
        Explain {
            plan: Plan {
                ..Default::default()
            },
            planning_time: 0.0,
            execution_time: 0.0,
            total_cost: 0.0,
            max_rows: 0,
            max_cost: 0.0,
            max_duration: 0.0,
        }
    }
}

pub fn calculate_planner_estimate(plan: &mut Plan) {
    plan.planner_row_estimate_factor = 0.0;

    if plan.plan_rows == plan.actual_rows {
        return;
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
}

pub fn calculate_actuals(explain: &mut Explain, plan: &mut Plan) {
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
}

pub fn calculate_maximums(explain: &mut Explain, plan: Plan) {
    if explain.max_rows < plan.actual_rows {
        explain.max_rows = plan.actual_rows
    }
    if explain.max_cost < plan.actual_cost {
        explain.max_cost = plan.actual_cost
    }
    if explain.max_duration < plan.actual_duration {
        explain.max_duration = plan.actual_duration
    }
}

pub fn process_explain(explain: &mut Explain) {
    let mut plan: Plan = explain.plan.clone();
    calculate_planner_estimate(&mut plan);
    calculate_actuals(explain, &mut plan);
    explain.plan = plan.clone();
    calculate_maximums(explain, plan.clone());
    //need to figure out how to deal with recursive process_plan
    for mut child_plan in plan.plans {
        calculate_planner_estimate(&mut child_plan);
        calculate_actuals(explain,  &mut child_plan);
        calculate_maximums(explain, child_plan);
    }
}
