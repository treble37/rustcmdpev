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
        let plan = self
            .plan
            .ok_or(VisualizeError::MissingPlan)?
            .into_domain(schema_profile)?;

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
    pub fn into_domain(
        self,
        schema_profile: PostgresSchemaProfile,
    ) -> Result<Plan, VisualizeError> {
        let plans = self
            .plans
            .into_iter()
            .map(|plan| plan.into_domain(schema_profile))
            .collect::<Result<Vec<_>, _>>()?;

        let resolved_io = self.io_timing.resolve(schema_profile);

        Ok(Plan {
            actuals: self.actuals,
            analysis_flags: PlanAnalysisFlags::default(),
            identity: self.identity,
            predicates: self.predicates,
            buffers: self.buffers,
            estimates: self.estimates,
            io_timing: resolved_io,
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
            PostgresSchemaProfile::ModernIoTiming | PostgresSchemaProfile::Unknown => {
                PlanIoTiming {
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
                }
            }
        }
    }
}

pub fn extract_major_version(version: &str) -> Option<u32> {
    version
        .split(|c: char| !c.is_ascii_digit())
        .find(|part| !part.is_empty())
        .and_then(|part| part.parse::<u32>().ok())
}

/// Resolve a schema profile from an explicit hint (e.g. `"--postgres-version 12"`).
pub fn schema_profile_from_hint(hint: &str) -> Option<PostgresSchemaProfile> {
    extract_major_version(hint).map(|major| {
        if major < 13 {
            PostgresSchemaProfile::LegacyIoTiming
        } else {
            PostgresSchemaProfile::ModernIoTiming
        }
    })
}
