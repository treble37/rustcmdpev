use serde::{Deserialize, Serialize};

/// I/O timing metrics for a plan node.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlanIoTiming {
    #[serde(default, rename(deserialize = "I/O Read Time"))]
    pub io_read_time: f64,
    #[serde(default, rename(deserialize = "I/O Write Time"))]
    pub io_write_time: f64,
}
