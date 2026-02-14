use serde::{Deserialize, Serialize};

/// Planner-side estimates for a plan node.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlanEstimates {
    #[serde(default, rename(deserialize = "Startup Cost"))]
    pub startup_cost: f64,
    #[serde(default, rename(deserialize = "Total Cost"))]
    pub total_cost: f64,
    #[serde(default, rename(deserialize = "Plan Rows"))]
    pub plan_rows: u64,
    #[serde(default, rename(deserialize = "Plan Width"))]
    pub plan_width: u64,
}
