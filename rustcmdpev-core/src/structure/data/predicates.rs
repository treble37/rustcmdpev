use serde::{Deserialize, Serialize};

/// Predicate and projection details for a plan node.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlanPredicates {
    #[serde(default, rename(deserialize = "Filter"))]
    pub filter: String,
    #[serde(default, rename(deserialize = "Index Cond"))]
    pub index_condition: String,
    #[serde(default, rename(deserialize = "Hash Cond"))]
    pub hash_condition: String,
    #[serde(default, rename(deserialize = "Group Key"))]
    pub group_key: Vec<String>,
    #[serde(default, rename(deserialize = "Output"))]
    pub output: Vec<String>,
    #[serde(default, rename(deserialize = "Rows Removed By Filter"))]
    pub rows_removed_by_filter: u64,
    #[serde(default, rename(deserialize = "Rows Removed By Index Recheck"))]
    pub rows_removed_by_index_recheck: u64,
}
