//https://github.com/serde-rs/serde/pull/238
use crate::structure::data::actuals::PlanActuals;
use crate::structure::data::analysis_flags::PlanAnalysisFlags;
use crate::structure::data::buffers::PlanBuffers;
use crate::structure::data::estimates::PlanEstimates;
use crate::structure::data::identity::PlanIdentity;
use crate::structure::data::io_timing::PlanIoTiming;
use crate::structure::data::predicates::PlanPredicates;
use serde::{Deserialize, Serialize};
use std::fmt;

type NodeType = String;

/// The Plan struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    #[serde(flatten)]
    pub actuals: PlanActuals,
    #[serde(flatten)]
    pub analysis_flags: PlanAnalysisFlags,
    #[serde(default, rename(deserialize = "Alias"))]
    pub alias: String,
    #[serde(default, rename(deserialize = "CTE Name"))]
    pub cte_name: String,
    #[serde(default, rename(deserialize = "Filter"))]
    pub filter: String,
    #[serde(default, rename(deserialize = "Group Key"))]
    pub group_key: Vec<String>,
    #[serde(default, rename(deserialize = "Hash Cond"))]
    pub hash_condition: String,
    #[serde(default, rename(deserialize = "Heap Fetches"))]
    pub heap_fetches: u64,
    #[serde(default, rename(deserialize = "Index Cond"))]
    pub index_condition: String,
    #[serde(default, rename(deserialize = "Index Name"))]
    pub index_name: String,
    #[serde(default, rename(deserialize = "I/O Read Time"))]
    pub io_read_time: f64,
    #[serde(default, rename(deserialize = "I/O Write Time"))]
    pub io_write_time: f64,
    #[serde(default, rename(deserialize = "Join Type"))]
    pub join_type: String,
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
    #[serde(flatten)]
    pub estimates: PlanEstimates,
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
    #[serde(default, rename(deserialize = "Strategy"))]
    pub strategy: String,
    #[serde(default, rename(deserialize = "Temp Read Blocks"))]
    pub temp_read_blocks: u64,
    #[serde(default, rename(deserialize = "Temp Written Blocks"))]
    pub temp_written_blocks: u64,
    #[serde(default, rename(deserialize = "Plans"))]
    pub plans: Vec<Plan>,
}
impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.node_type)
    }
}

impl Default for Plan {
    fn default() -> Plan {
        Plan {
            actuals: PlanActuals::default(),
            analysis_flags: PlanAnalysisFlags::default(),
            alias: String::from(""),
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
            local_dirtied_blocks: 0,
            local_hit_blocks: 0,
            local_read_blocks: 0,
            local_written_blocks: 0,
            node_type: String::from(""),
            output: Vec::new(),
            parent_relationship: String::from(""),
            estimates: PlanEstimates::default(),
            relation_name: String::from(""),
            rows_removed_by_filter: 0,
            rows_removed_by_index_recheck: 0,
            scan_direction: String::from(""),
            schema: String::from(""),
            shared_dirtied_blocks: 0,
            shared_hit_blocks: 0,
            shared_read_blocks: 0,
            shared_written_blocks: 0,
            strategy: String::from(""),
            temp_read_blocks: 0,
            temp_written_blocks: 0,
            plans: Vec::new(),
        }
    }
}

impl Plan {
    pub fn child_count(&self) -> usize {
        self.plans.len()
    }

    pub fn has_children(&self) -> bool {
        !self.plans.is_empty()
    }

    pub fn is_leaf(&self) -> bool {
        self.plans.is_empty()
    }

    pub fn analysis_flags(&self) -> PlanAnalysisFlags {
        self.analysis_flags.clone()
    }

    pub fn set_analysis_flags(&mut self, analysis_flags: PlanAnalysisFlags) {
        self.analysis_flags = analysis_flags;
    }

    pub fn actuals(&self) -> PlanActuals {
        self.actuals.clone()
    }

    pub fn set_actuals(&mut self, actuals: PlanActuals) {
        self.actuals = actuals;
    }

    pub fn estimates(&self) -> PlanEstimates {
        self.estimates.clone()
    }

    pub fn set_estimates(&mut self, estimates: PlanEstimates) {
        self.estimates = estimates;
    }

    pub fn identity(&self) -> PlanIdentity {
        PlanIdentity {
            node_type: self.node_type.clone(),
            parent_relationship: self.parent_relationship.clone(),
            join_type: self.join_type.clone(),
            alias: self.alias.clone(),
            schema: self.schema.clone(),
            relation_name: self.relation_name.clone(),
            index_name: self.index_name.clone(),
            cte_name: self.cte_name.clone(),
            strategy: self.strategy.clone(),
            scan_direction: self.scan_direction.clone(),
        }
    }

    pub fn set_identity(&mut self, identity: PlanIdentity) {
        self.node_type = identity.node_type;
        self.parent_relationship = identity.parent_relationship;
        self.join_type = identity.join_type;
        self.alias = identity.alias;
        self.schema = identity.schema;
        self.relation_name = identity.relation_name;
        self.index_name = identity.index_name;
        self.cte_name = identity.cte_name;
        self.strategy = identity.strategy;
        self.scan_direction = identity.scan_direction;
    }

    pub fn predicates(&self) -> PlanPredicates {
        PlanPredicates {
            filter: self.filter.clone(),
            index_condition: self.index_condition.clone(),
            hash_condition: self.hash_condition.clone(),
            group_key: self.group_key.clone(),
            output: self.output.clone(),
            rows_removed_by_filter: self.rows_removed_by_filter,
            rows_removed_by_index_recheck: self.rows_removed_by_index_recheck,
        }
    }

    pub fn set_predicates(&mut self, predicates: PlanPredicates) {
        self.filter = predicates.filter;
        self.index_condition = predicates.index_condition;
        self.hash_condition = predicates.hash_condition;
        self.group_key = predicates.group_key;
        self.output = predicates.output;
        self.rows_removed_by_filter = predicates.rows_removed_by_filter;
        self.rows_removed_by_index_recheck = predicates.rows_removed_by_index_recheck;
    }

    pub fn buffers(&self) -> PlanBuffers {
        PlanBuffers {
            heap_fetches: self.heap_fetches,
            shared_dirtied_blocks: self.shared_dirtied_blocks,
            shared_hit_blocks: self.shared_hit_blocks,
            shared_read_blocks: self.shared_read_blocks,
            shared_written_blocks: self.shared_written_blocks,
            local_dirtied_blocks: self.local_dirtied_blocks,
            local_hit_blocks: self.local_hit_blocks,
            local_read_blocks: self.local_read_blocks,
            local_written_blocks: self.local_written_blocks,
            temp_read_blocks: self.temp_read_blocks,
            temp_written_blocks: self.temp_written_blocks,
        }
    }

    pub fn set_buffers(&mut self, buffers: PlanBuffers) {
        self.heap_fetches = buffers.heap_fetches;
        self.shared_dirtied_blocks = buffers.shared_dirtied_blocks;
        self.shared_hit_blocks = buffers.shared_hit_blocks;
        self.shared_read_blocks = buffers.shared_read_blocks;
        self.shared_written_blocks = buffers.shared_written_blocks;
        self.local_dirtied_blocks = buffers.local_dirtied_blocks;
        self.local_hit_blocks = buffers.local_hit_blocks;
        self.local_read_blocks = buffers.local_read_blocks;
        self.local_written_blocks = buffers.local_written_blocks;
        self.temp_read_blocks = buffers.temp_read_blocks;
        self.temp_written_blocks = buffers.temp_written_blocks;
    }

    pub fn io_timing(&self) -> PlanIoTiming {
        PlanIoTiming {
            io_read_time: self.io_read_time,
            io_write_time: self.io_write_time,
        }
    }

    pub fn set_io_timing(&mut self, io_timing: PlanIoTiming) {
        self.io_read_time = io_timing.io_read_time;
        self.io_write_time = io_timing.io_write_time;
    }
}
