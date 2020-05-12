use serde::{Deserialize, Serialize};
use std::fmt;

const UNDER: &str = "Under";
const OVER: &str = "Over";
const CTE_SCAN: &str = "CTE Scan";

type NodeType = String;
type EstimateDirection = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
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
    #[serde(default, rename(deserialize = "Alias"))]
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
    #[serde(default, rename(deserialize = "Total Cost"))]
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

// https://www.forrestthewoods.com/blog/should-small-rust-structs-be-passed-by-copy-or-by-borrow/
pub fn calculate_planner_estimate(plan: Plan) -> Plan {
    let mut new_plan: Plan = plan;
    new_plan.planner_row_estimate_factor = 0.0;

    if new_plan.plan_rows == new_plan.actual_rows {
        return new_plan;
    }

    new_plan.planner_row_estimate_direction = UNDER.to_string();
    if new_plan.plan_rows != 0 {
        new_plan.planner_row_estimate_factor =
            new_plan.actual_rows as f64 / new_plan.plan_rows as f64;
    }

    if new_plan.planner_row_estimate_factor < 1.0 {
        new_plan.planner_row_estimate_factor = 0.0;
        new_plan.planner_row_estimate_direction = OVER.to_string();
        if new_plan.actual_rows != 0 {
            new_plan.planner_row_estimate_factor =
                new_plan.plan_rows as f64 / new_plan.actual_rows as f64;
        }
    }
    new_plan
}

pub fn calculate_actuals(explain: Explain, plan: Plan) -> (Explain, Plan) {
    let mut new_plan: Plan = plan;
    let mut new_explain: Explain = explain;
    new_plan.actual_duration = new_plan.actual_total_time;
    new_plan.actual_cost = new_plan.total_cost;
    for mut child_plan in new_plan.plans.iter_mut() {
        if child_plan.node_type != CTE_SCAN {
            child_plan.actual_duration = child_plan.actual_duration - child_plan.actual_total_time;
            child_plan.actual_cost = child_plan.actual_cost - child_plan.total_cost;
        }
    }

    if new_plan.actual_cost < 0.0 {
        new_plan.actual_cost = 0.0;
    }

    new_explain.total_cost = new_explain.total_cost + new_plan.actual_cost;
    println!("{:?}", new_plan.total_cost);
    new_plan.actual_duration = new_plan.actual_duration * new_plan.actual_loops as f64;
    (new_explain, new_plan)
}

pub fn calculate_maximums(explain: Explain, plan: Plan) -> Explain {
    let mut new_explain: Explain = explain;
    if new_explain.max_rows < plan.actual_rows {
        new_explain.max_rows = plan.actual_rows
    }
    if new_explain.max_cost < plan.actual_cost {
        new_explain.max_cost = plan.actual_cost
    }
    if new_explain.max_duration < plan.actual_duration {
        new_explain.max_duration = plan.actual_duration
    }
    new_explain
}

pub fn process_explain(explain: Explain) -> Explain {
    let mut new_explain: Explain = explain;
    new_explain.plan = calculate_planner_estimate(new_explain.plan);
    let (e, p) = calculate_actuals(new_explain.clone(), new_explain.clone().plan);
    new_explain = e.clone();
    new_explain.plan = p.clone();
    new_explain = calculate_maximums(new_explain.clone(), new_explain.plan);
    new_explain
}
/*
pub fn process_explain_child_plans(explain: Explain, plan: Plan) -> (Explain, Plan) {
    //need to figure out how to deal with recursive process_plan
    let mut new_explain: Explain = explain;
    for mut child_plan in plan.plans.into_iter() {
        child_plan = calculate_planner_estimate(child_plan);
        let (e, p) = calculate_actuals(new_explain.clone(),  child_plan.clone());
        new_explain = e;
        child_plan = p;
        new_explain = calculate_maximums(new_explain, child_plan);
    }
    new_explain
}*/
pub fn process_all(explain: Explain) -> Explain {
    let mut new_explain: Explain = explain;
    new_explain = process_explain(new_explain.clone());
    new_explain
}
