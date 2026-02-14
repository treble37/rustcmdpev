use serde::{Deserialize, Serialize};

/// Identifying and classification fields for a plan node.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlanIdentity {
    #[serde(default, rename(deserialize = "Node Type"))]
    pub node_type: String,
    #[serde(default, rename(deserialize = "Parent Relationship"))]
    pub parent_relationship: String,
    #[serde(default, rename(deserialize = "Join Type"))]
    pub join_type: String,
    #[serde(default, rename(deserialize = "Alias"))]
    pub alias: String,
    #[serde(default, rename(deserialize = "Schema"))]
    pub schema: String,
    #[serde(default, rename(deserialize = "Relation Name"))]
    pub relation_name: String,
    #[serde(default, rename(deserialize = "Index Name"))]
    pub index_name: String,
    #[serde(default, rename(deserialize = "CTE Name"))]
    pub cte_name: String,
    #[serde(default, rename(deserialize = "Strategy"))]
    pub strategy: String,
    #[serde(default, rename(deserialize = "Scan Direction"))]
    pub scan_direction: String,
}
