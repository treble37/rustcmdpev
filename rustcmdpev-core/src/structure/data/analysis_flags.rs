use serde::{Deserialize, Serialize};

/// Derived analysis flags and planner estimate diagnostics for a plan node.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlanAnalysisFlags {
    #[serde(default, rename(deserialize = "Costliest"))]
    pub costliest: bool,
    #[serde(default)]
    pub largest: bool,
    #[serde(default)]
    pub slowest: bool,
    #[serde(default)]
    pub planner_row_estimate_direction: String,
    #[serde(default)]
    pub planner_row_estimate_factor: f64,
}
