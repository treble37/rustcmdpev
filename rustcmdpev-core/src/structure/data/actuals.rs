use serde::{Deserialize, Serialize};

/// Runtime actual metrics for a plan node.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlanActuals {
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
}
