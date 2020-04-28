use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

type NodeType = String;
type EstimateDirection = String;

#[derive(Serialize, Deserialize)]
struct Plan {
    actual_cost: f64,
    actual_duration: f64,
    actual_loops: u64,
    actual_rows: u64,
    actual_startup_time: f64,
    actual_total_time: f64,
    alias: String,
    costliest: bool,
    cte_name: String,
    filter: String,
    group_key: Vec<String>,
    hash_condition: String,
    heap_fetches: u64,
    index_condition: String,
    index_name: String,
    io_read_time: f64,
    io_write_time: f64,
    join_type: String,
    largest: bool,
    local_dirtied_blocks: u64,
    local_hit_blocks: u64,
    local_read_blocks: u64,
    local_written_blocks: u64,
    node_type: NodeType,
    output: Vec<String>,
    parent_relationship: String,
    planner_row_estimate_direction: EstimateDirection,
    planner_row_estimate_factor: f64,
    plan_rows: u64,
    plan_width: u64,
    relation_name: String,
    rows_removed_by_filter: u64,
    rows_removed_by_index_recheck: u64,
    scan_direction: String,
    schema: String,
    shared_dirtied_blocks: u64,
    shared_hit_blocks: u64,
    shared_read_blocks: u64,
    shared_written_blocks: u64,
    slowest: bool,
    startup_cost: f64,
    strategy: String,
    temp_read_blocks: u64,
    temp_written_blocks: u64,
    total_cost: f64,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    let plans: Vec<Plan> = serde_json::from_str(input).unwrap();
    println!("input {}", plans[0]);
}
