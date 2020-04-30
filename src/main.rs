use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

const UNDER: &str = "Under";
const OVER: &str = "Over";
const CTE_SCAN: &str = "CTE Scan";

type NodeType = String;
type EstimateDirection = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
struct Explain {
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
struct Plan {
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

impl Plan {
    fn calculate_planner_estimate(&mut self) {
        self.planner_row_estimate_factor = 0.0;

        if self.plan_rows == self.actual_rows {
            return;
        }

        self.planner_row_estimate_direction = UNDER.to_string();
        if self.plan_rows != 0 {
            self.planner_row_estimate_factor = self.actual_rows as f64 / self.plan_rows as f64;
        }

        if self.planner_row_estimate_factor < 1.0 {
            self.planner_row_estimate_factor = 0.0;
            self.planner_row_estimate_direction = OVER.to_string();
            if self.actual_rows != 0 {
                self.planner_row_estimate_factor = self.plan_rows as f64 / self.actual_rows as f64;
            }
        }
    }
}

impl Explain {
    pub fn process_explain(&mut self) {
        self.plan.calculate_planner_estimate();
        self.calculate_actuals();
        self.calculate_maximums();
    }
    pub fn calculate_actuals(&mut self) {
        self.plan.actual_duration = self.plan.actual_total_time;
        self.plan.actual_cost = self.plan.total_cost;

        for child in self.plan.plans.iter() {
            if child.node_type != CTE_SCAN {
                self.plan.actual_duration = self.plan.actual_duration - child.actual_total_time;
                self.plan.actual_cost = self.plan.actual_cost - child.total_cost;
            }
        }

        if self.plan.actual_cost < 0.0 {
            self.plan.actual_cost = 0.0;
        }

        self.total_cost = self.total_cost + self.plan.actual_cost;

        self.plan.actual_duration = self.plan.actual_duration * self.plan.actual_loops as f64;
    }
    pub fn calculate_maximums(&mut self) {
        if self.max_rows < self.plan.actual_rows {
            self.max_rows = self.plan.actual_rows
        }
        if self.max_cost < self.plan.actual_cost {
            self.max_cost = self.plan.actual_cost
        }
        if self.max_duration < self.plan.actual_duration {
            self.max_duration = self.plan.actual_duration
        }
    }
}

// a little smoke test...
fn write_explain_stub() {
    println!("○ Total Cost: {}\n", 4.265_f64);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    println!("args {}", input);
    let explains: Vec<Explain> = serde_json::from_str(input).unwrap();
    for explain in explains.iter() {
        println!("explain {:#?}", explain)
    }
    write_explain_stub()
}
