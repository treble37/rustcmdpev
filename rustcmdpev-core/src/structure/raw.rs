use serde::Deserialize;

use crate::structure::data::analysis_flags::PlanAnalysisFlags;
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;
use crate::VisualizeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgresSchemaProfile {
    Unknown,
    LegacyIoTiming,
    ModernIoTiming,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawExplain {
    #[serde(default, rename = "Plan")]
    pub plan: Option<RawPlan>,
    #[serde(default, rename = "Planning Time")]
    pub planning_time: f64,
    #[serde(default, rename = "Execution Time")]
    pub execution_time: f64,
    #[serde(default, rename = "PostgreSQL Version", alias = "Postgres Version")]
    pub postgres_version: Option<String>,
}

impl RawExplain {
    pub fn schema_profile(&self) -> PostgresSchemaProfile {
        match self
            .postgres_version
            .as_deref()
            .and_then(extract_major_version)
        {
            Some(major) if major < 13 => PostgresSchemaProfile::LegacyIoTiming,
            Some(_) => PostgresSchemaProfile::ModernIoTiming,
            None => PostgresSchemaProfile::Unknown,
        }
    }

    pub fn into_domain(self) -> Result<Explain, VisualizeError> {
        let schema_profile = self.schema_profile();
        let plan = self.plan.ok_or(VisualizeError::MissingPlan)?.into_domain(schema_profile)?;

        Ok(Explain {
            plan,
            postgres_version: self.postgres_version,
            planning_time: self.planning_time,
            execution_time: self.execution_time,
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawPlan {
    #[serde(default, rename = "Actual Loops")]
    pub actual_loops: u64,
    #[serde(default, rename = "Actual Rows")]
    pub actual_rows: u64,
    #[serde(default, rename = "Actual Startup Time")]
    pub actual_startup_time: f64,
    #[serde(default, rename = "Actual Total Time")]
    pub actual_total_time: f64,
    #[serde(default, rename = "Alias")]
    pub alias: String,
    #[serde(default, rename = "CTE Name")]
    pub cte_name: String,
    #[serde(default, rename = "Filter")]
    pub filter: String,
    #[serde(default, rename = "Group Key")]
    pub group_key: Vec<String>,
    #[serde(default, rename = "Hash Cond")]
    pub hash_condition: String,
    #[serde(default, rename = "Heap Fetches")]
    pub heap_fetches: u64,
    #[serde(default, rename = "Index Cond")]
    pub index_condition: String,
    #[serde(default, rename = "Index Name")]
    pub index_name: String,
    #[serde(default, rename = "I/O Read Time")]
    pub io_read_time: Option<f64>,
    #[serde(default, rename = "I/O Write Time")]
    pub io_write_time: Option<f64>,
    #[serde(default, rename = "IO Read Time")]
    pub legacy_io_read_time: Option<f64>,
    #[serde(default, rename = "IO Write Time")]
    pub legacy_io_write_time: Option<f64>,
    #[serde(default, rename = "Join Type")]
    pub join_type: String,
    #[serde(default, rename = "Local Dirtied Blocks")]
    pub local_dirtied_blocks: u64,
    #[serde(default, rename = "Local Hit Blocks")]
    pub local_hit_blocks: u64,
    #[serde(default, rename = "Local Read Blocks")]
    pub local_read_blocks: u64,
    #[serde(default, rename = "Local Written Blocks")]
    pub local_written_blocks: u64,
    #[serde(default, rename = "Node Type")]
    pub node_type: String,
    #[serde(default, rename = "Output")]
    pub output: Vec<String>,
    #[serde(default, rename = "Parent Relationship")]
    pub parent_relationship: String,
    #[serde(default, rename = "Plan Rows")]
    pub plan_rows: u64,
    #[serde(default, rename = "Plan Width")]
    pub plan_width: u64,
    #[serde(default, rename = "Plans")]
    pub plans: Vec<RawPlan>,
    #[serde(default, rename = "Relation Name")]
    pub relation_name: String,
    #[serde(default, rename = "Rows Removed By Filter")]
    pub rows_removed_by_filter: u64,
    #[serde(default, rename = "Rows Removed By Index Recheck")]
    pub rows_removed_by_index_recheck: u64,
    #[serde(default, rename = "Scan Direction")]
    pub scan_direction: String,
    #[serde(default, rename = "Schema")]
    pub schema: String,
    #[serde(default, rename = "Shared Dirtied Blocks")]
    pub shared_dirtied_blocks: u64,
    #[serde(default, rename = "Shared Hit Blocks")]
    pub shared_hit_blocks: u64,
    #[serde(default, rename = "Shared Read Blocks")]
    pub shared_read_blocks: u64,
    #[serde(default, rename = "Shared Written Blocks")]
    pub shared_written_blocks: u64,
    #[serde(default, rename = "Startup Cost")]
    pub startup_cost: f64,
    #[serde(default, rename = "Strategy")]
    pub strategy: String,
    #[serde(default, rename = "Temp Read Blocks")]
    pub temp_read_blocks: u64,
    #[serde(default, rename = "Temp Written Blocks")]
    pub temp_written_blocks: u64,
    #[serde(default, rename = "Total Cost")]
    pub total_cost: f64,
}

impl RawPlan {
    pub fn into_domain(self, schema_profile: PostgresSchemaProfile) -> Result<Plan, VisualizeError> {
        let plans = self
            .plans
            .into_iter()
            .map(|plan| plan.into_domain(schema_profile))
            .collect::<Result<Vec<_>, _>>()?;

        let (io_read_time, io_write_time) = match schema_profile {
            PostgresSchemaProfile::LegacyIoTiming => (
                self.legacy_io_read_time.or(self.io_read_time).unwrap_or_default(),
                self.legacy_io_write_time.or(self.io_write_time).unwrap_or_default(),
            ),
            PostgresSchemaProfile::ModernIoTiming | PostgresSchemaProfile::Unknown => (
                self.io_read_time.or(self.legacy_io_read_time).unwrap_or_default(),
                self.io_write_time.or(self.legacy_io_write_time).unwrap_or_default(),
            ),
        };

        Ok(Plan {
            actuals: crate::structure::data::actuals::PlanActuals {
                actual_loops: self.actual_loops,
                actual_rows: self.actual_rows,
                actual_startup_time: self.actual_startup_time,
                actual_total_time: self.actual_total_time,
                ..Default::default()
            },
            analysis_flags: PlanAnalysisFlags::default(),
            alias: self.alias,
            cte_name: self.cte_name,
            filter: self.filter,
            group_key: self.group_key,
            hash_condition: self.hash_condition,
            heap_fetches: self.heap_fetches,
            index_condition: self.index_condition,
            index_name: self.index_name,
            io_read_time,
            io_write_time,
            join_type: self.join_type,
            local_dirtied_blocks: self.local_dirtied_blocks,
            local_hit_blocks: self.local_hit_blocks,
            local_read_blocks: self.local_read_blocks,
            local_written_blocks: self.local_written_blocks,
            node_type: self.node_type,
            output: self.output,
            parent_relationship: self.parent_relationship,
            estimates: crate::structure::data::estimates::PlanEstimates {
                startup_cost: self.startup_cost,
                total_cost: self.total_cost,
                plan_rows: self.plan_rows,
                plan_width: self.plan_width,
            },
            relation_name: self.relation_name,
            rows_removed_by_filter: self.rows_removed_by_filter,
            rows_removed_by_index_recheck: self.rows_removed_by_index_recheck,
            scan_direction: self.scan_direction,
            schema: self.schema,
            shared_dirtied_blocks: self.shared_dirtied_blocks,
            shared_hit_blocks: self.shared_hit_blocks,
            shared_read_blocks: self.shared_read_blocks,
            shared_written_blocks: self.shared_written_blocks,
            strategy: self.strategy,
            temp_read_blocks: self.temp_read_blocks,
            temp_written_blocks: self.temp_written_blocks,
            plans,
        })
    }
}

fn extract_major_version(version: &str) -> Option<u32> {
    version
        .split(|c: char| !c.is_ascii_digit())
        .find(|part| !part.is_empty())
        .and_then(|part| part.parse::<u32>().ok())
}
