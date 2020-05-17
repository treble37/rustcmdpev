use serde::{Deserialize, Serialize};
use std::fmt;

const UNDER: &str = "Under";
const OVER: &str = "Over";
const CTE_SCAN: &str = "CTE Scan";

type NodeType = String;
type EstimateDirection = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Explain {
    //TODO: add Triggers back, add default for plan?
    #[serde(default, rename(deserialize = "Plan"))]
    pub plan: Plan,
    #[serde(default, rename(deserialize = "Planning Time"))]
    pub planning_time: f64,
    #[serde(default, rename(deserialize = "Execution Time"))]
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
    #[serde(default, rename(deserialize = "Actual Cost"))]
    pub actual_cost: f64,
    #[serde(default, rename(deserialize = "Actual Duration"))]
    pub actual_duration: f64,
    #[serde(default, rename(deserialize = "Actual Loops"))]
    pub actual_loops: u64,
    #[serde(default, rename(deserialize = "Actual Rows"))]
    pub actual_rows: u64,
    #[serde(default, rename(deserialize = "Actual Startup Time"))]
    pub actual_startup_time: f64,
    #[serde(default, rename(deserialize = "Actual Total Time"))]
    pub actual_total_time: f64,
    #[serde(default, rename(deserialize = "Alias"))]
    pub alias: String,
    #[serde(default, rename(deserialize = "Costliest"))]
    pub costliest: bool,
    #[serde(default, rename(deserialize = "CTE Name"))]
    pub cte_name: String,
    #[serde(default, rename(deserialize = "Filter"))]
    pub filter: String,
    #[serde(default, rename(deserialize = "Group Key"))]
    pub group_key: Vec<String>,
    #[serde(default, rename(deserialize = "Hash Condition"))]
    pub hash_condition: String,
    #[serde(default, rename(deserialize = "Heap Fetches"))]
    pub heap_fetches: u64,
    #[serde(default, rename(deserialize = "Index Condition"))]
    pub index_condition: String,
    #[serde(default, rename(deserialize = "Index Name"))]
    pub index_name: String,
    #[serde(default, rename(deserialize = "I/O Read Time"))]
    pub io_read_time: f64,
    #[serde(default, rename(deserialize = "I/O Write Time"))]
    pub io_write_time: f64,
    #[serde(default, rename(deserialize = "Join Type"))]
    pub join_type: String,
    #[serde(default)]
    pub largest: bool,
    #[serde(default, rename(deserialize = "Local Dirtied Blocks"))]
    pub local_dirtied_blocks: u64,
    #[serde(default, rename(deserialize = "Local Hit Blocks"))]
    pub local_hit_blocks: u64,
    #[serde(default, rename(deserialize = "Local Read Blocks"))]
    pub local_read_blocks: u64,
    #[serde(default, rename(deserialize = "Local Written Blocks"))]
    pub local_written_blocks: u64,
    #[serde(default, rename(deserialize = "Node Type"))]
    pub node_type: NodeType,
    #[serde(default, rename(deserialize = "Output"))]
    pub output: Vec<String>,
    #[serde(default, rename(deserialize = "Parent Relationship"))]
    pub parent_relationship: String,
    #[serde(default)]
    pub planner_row_estimate_direction: EstimateDirection,
    #[serde(default)]
    pub planner_row_estimate_factor: f64,
    #[serde(default, rename(deserialize = "Plan Rows"))]
    pub plan_rows: u64,
    #[serde(default, rename(deserialize = "Plan Width"))]
    pub plan_width: u64,
    #[serde(default, rename(deserialize = "Relation Name"))]
    pub relation_name: String,
    #[serde(default, rename(deserialize = "Rows Removed By Filter"))]
    pub rows_removed_by_filter: u64,
    #[serde(default, rename(deserialize = "Rows Removed By Index Recheck"))]
    pub rows_removed_by_index_recheck: u64,
    #[serde(default, rename(deserialize = "Scan Direction"))]
    pub scan_direction: String,
    #[serde(default, rename(deserialize = "Schema"))]
    pub schema: String,
    #[serde(default, rename(deserialize = "Shared Dirtied Blocks"))]
    pub shared_dirtied_blocks: u64,
    #[serde(default, rename(deserialize = "Shared Hit Blocks"))]
    pub shared_hit_blocks: u64,
    #[serde(default, rename(deserialize = "Shared Read Blocks"))]
    pub shared_read_blocks: u64,
    #[serde(default, rename(deserialize = "Shared Written Blocks"))]
    pub shared_written_blocks: u64,
    #[serde(default)]
    pub slowest: bool,
    #[serde(default, rename(deserialize = "Startup Cost"))]
    pub startup_cost: f64,
    #[serde(default, rename(deserialize = "Strategy"))]
    pub strategy: String,
    #[serde(default, rename(deserialize = "Temp Read Blocks"))]
    pub temp_read_blocks: u64,
    #[serde(default, rename(deserialize = "Temp Written Blocks"))]
    pub temp_written_blocks: u64,
    #[serde(default, rename(deserialize = "Total Cost"))]
    pub total_cost: f64,
    #[serde(default, rename(deserialize = "Plans"))]
    pub plans: Vec<Plan>,
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

pub fn process_explain_child_plans(explain: Explain, plans: Vec<Plan>) -> (Explain, Vec<Plan>) {
    //need to figure out how to deal with recursive process_plan
    let mut new_explain: Explain = explain;
    let mut new_plans: Vec<Plan> = plans;
    for child_plan in new_plans.iter_mut() {
        *child_plan = calculate_planner_estimate(child_plan.clone());
        let (e, p) = calculate_actuals(new_explain.clone(),  child_plan.clone());
        new_explain = e;
        *child_plan = p;
        new_explain = calculate_maximums(new_explain, child_plan.clone());
    }
    (new_explain, new_plans)
}
pub fn process_all(explain: Explain) -> Explain {
    let mut new_explain: Explain = explain;
    new_explain = process_explain(new_explain.clone());
    new_explain
}
