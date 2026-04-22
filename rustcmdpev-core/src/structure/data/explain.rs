use serde::{Deserialize, Serialize};

/// The Explain struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Explain {
    #[serde(default, rename(deserialize = "Plan"))]
    pub plan: crate::structure::data::plan::Plan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_version: Option<String>,
    #[serde(default, rename(deserialize = "Planning Time"))]
    pub planning_time: f64,
    #[serde(default, rename(deserialize = "Execution Time"))]
    pub execution_time: f64,
    #[serde(default, rename(deserialize = "Total Cost"))]
    pub total_cost: f64,
    #[serde(default, rename(deserialize = "Max Rows"))]
    pub max_rows: u64,
    #[serde(default, rename(deserialize = "Max Cost"))]
    pub max_cost: f64,
    #[serde(default, rename(deserialize = "Max Duration"))]
    pub max_duration: f64,
}

impl Default for Explain {
    fn default() -> Explain {
        Explain {
            plan: crate::structure::data::plan::Plan {
                ..Default::default()
            },
            postgres_version: None,
            planning_time: 0.0,
            execution_time: 0.0,
            total_cost: 0.0,
            max_rows: 0,
            max_cost: 0.0,
            max_duration: 0.0,
        }
    }
}
