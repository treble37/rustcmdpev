use serde::Deserialize;

use crate::structure::data::actuals::PlanActuals;
use crate::structure::data::analysis_flags::PlanAnalysisFlags;
use crate::structure::data::buffers::PlanBuffers;
use crate::structure::data::estimates::PlanEstimates;
use crate::structure::data::explain::Explain;
use crate::structure::data::identity::PlanIdentity;
use crate::structure::data::io_timing::PlanIoTiming;
use crate::structure::data::plan::Plan;
use crate::structure::data::predicates::PlanPredicates;
use crate::VisualizeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgresSchemaProfile {
    Unknown,
    LegacyIoTiming,
    ModernIoTiming,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawExplain {
    #[serde(flatten)]
    pub metadata: RawExplainMetadata,
    #[serde(default, rename = "Plan")]
    pub plan: Option<RawPlan>,
}

impl RawExplain {
    pub fn schema_profile(&self) -> PostgresSchemaProfile {
        match self
            .metadata
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
            postgres_version: self.metadata.postgres_version,
            planning_time: self.metadata.planning_time,
            execution_time: self.metadata.execution_time,
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawExplainMetadata {
    #[serde(default, rename = "Planning Time")]
    pub planning_time: f64,
    #[serde(default, rename = "Execution Time")]
    pub execution_time: f64,
    #[serde(default, rename = "PostgreSQL Version", alias = "Postgres Version")]
    pub postgres_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawPlan {
    #[serde(flatten)]
    pub actuals: PlanActuals,
    #[serde(flatten)]
    pub identity: PlanIdentity,
    #[serde(flatten)]
    pub predicates: PlanPredicates,
    #[serde(flatten)]
    pub buffers: PlanBuffers,
    #[serde(flatten)]
    pub estimates: PlanEstimates,
    #[serde(flatten)]
    pub io_timing: RawPlanIoTiming,
    #[serde(default, rename = "Plans")]
    pub plans: Vec<RawPlan>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawPlanIoTiming {
    #[serde(default, flatten)]
    pub canonical: PlanIoTiming,
    #[serde(default, rename = "IO Read Time")]
    pub legacy_io_read_time: Option<f64>,
    #[serde(default, rename = "IO Write Time")]
    pub legacy_io_write_time: Option<f64>,
}

impl RawPlan {
    pub fn into_domain(self, schema_profile: PostgresSchemaProfile) -> Result<Plan, VisualizeError> {
        let plans = self
            .plans
            .into_iter()
            .map(|plan| plan.into_domain(schema_profile))
            .collect::<Result<Vec<_>, _>>()?;

        let resolved_io = self.io_timing.resolve(schema_profile);

        Ok(Plan {
            actuals: self.actuals,
            analysis_flags: PlanAnalysisFlags::default(),
            alias: self.identity.alias,
            cte_name: self.identity.cte_name,
            filter: self.predicates.filter,
            group_key: self.predicates.group_key,
            hash_condition: self.predicates.hash_condition,
            heap_fetches: self.buffers.heap_fetches,
            index_condition: self.predicates.index_condition,
            index_name: self.identity.index_name,
            io_read_time: resolved_io.io_read_time,
            io_write_time: resolved_io.io_write_time,
            join_type: self.identity.join_type,
            local_dirtied_blocks: self.buffers.local_dirtied_blocks,
            local_hit_blocks: self.buffers.local_hit_blocks,
            local_read_blocks: self.buffers.local_read_blocks,
            local_written_blocks: self.buffers.local_written_blocks,
            node_type: self.identity.node_type,
            output: self.predicates.output,
            parent_relationship: self.identity.parent_relationship,
            estimates: self.estimates,
            relation_name: self.identity.relation_name,
            rows_removed_by_filter: self.predicates.rows_removed_by_filter,
            rows_removed_by_index_recheck: self.predicates.rows_removed_by_index_recheck,
            scan_direction: self.identity.scan_direction,
            schema: self.identity.schema,
            shared_dirtied_blocks: self.buffers.shared_dirtied_blocks,
            shared_hit_blocks: self.buffers.shared_hit_blocks,
            shared_read_blocks: self.buffers.shared_read_blocks,
            shared_written_blocks: self.buffers.shared_written_blocks,
            strategy: self.identity.strategy,
            temp_read_blocks: self.buffers.temp_read_blocks,
            temp_written_blocks: self.buffers.temp_written_blocks,
            plans,
        })
    }
}

impl RawPlanIoTiming {
    pub fn resolve(self, schema_profile: PostgresSchemaProfile) -> PlanIoTiming {
        match schema_profile {
            PostgresSchemaProfile::LegacyIoTiming => PlanIoTiming {
                io_read_time: self
                    .legacy_io_read_time
                    .or(Some(self.canonical.io_read_time))
                    .unwrap_or_default(),
                io_write_time: self
                    .legacy_io_write_time
                    .or(Some(self.canonical.io_write_time))
                    .unwrap_or_default(),
            },
            PostgresSchemaProfile::ModernIoTiming | PostgresSchemaProfile::Unknown => PlanIoTiming {
                io_read_time: if self.canonical.io_read_time == 0.0 {
                    self.legacy_io_read_time.unwrap_or_default()
                } else {
                    self.canonical.io_read_time
                },
                io_write_time: if self.canonical.io_write_time == 0.0 {
                    self.legacy_io_write_time.unwrap_or_default()
                } else {
                    self.canonical.io_write_time
                },
            },
        }
    }
}

fn extract_major_version(version: &str) -> Option<u32> {
    version
        .split(|c: char| !c.is_ascii_digit())
        .find(|part| !part.is_empty())
        .and_then(|part| part.parse::<u32>().ok())
}
