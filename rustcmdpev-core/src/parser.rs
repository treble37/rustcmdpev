use crate::structure::data::explain::Explain;
use crate::structure::raw::{schema_profile_from_hint, PostgresSchemaProfile, RawExplain, RawPlan};
use crate::structure::tree::PlanTree;
use crate::VisualizeError;

/// Parser-level configuration.
///
/// Currently the only knob is the optional `postgres_version_hint`, which lets
/// callers steer schema-profile selection when the JSON payload itself does
/// not carry a `PostgreSQL Version` field. The hint is parsed for a leading
/// integer (e.g. `"12"`, `"PostgreSQL 13.4"`) and ignored if no major version
/// can be extracted.
#[derive(Debug, Clone, Default)]
pub struct ParseOptions {
    pub postgres_version_hint: Option<String>,
}

impl ParseOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_postgres_version_hint(mut self, hint: impl Into<String>) -> Self {
        self.postgres_version_hint = Some(hint.into());
        self
    }

    pub fn schema_profile_hint(&self) -> Option<PostgresSchemaProfile> {
        self.postgres_version_hint
            .as_deref()
            .and_then(schema_profile_from_hint)
    }
}

/// Parse the top-level PostgreSQL EXPLAIN JSON array into raw serde models.
pub fn parse_raw_explains(input: &str) -> Result<Vec<RawExplain>, VisualizeError> {
    serde_json::from_str(input).map_err(VisualizeError::InvalidJson)
}

/// Convert a raw explain document into the strongly typed domain model.
pub fn build_domain_explain(raw: RawExplain) -> Result<Explain, VisualizeError> {
    raw.into_domain()
}

/// Convert a raw explain into the domain model, respecting an explicit version
/// hint when the JSON metadata does not name a PostgreSQL version.
pub fn build_domain_explain_with(
    raw: RawExplain,
    options: &ParseOptions,
) -> Result<Explain, VisualizeError> {
    let mut effective_profile = raw.schema_profile();
    if effective_profile == PostgresSchemaProfile::Unknown {
        if let Some(hinted) = options.schema_profile_hint() {
            effective_profile = hinted;
        }
    }

    let RawExplain { metadata, plan } = raw;
    let plan = plan
        .ok_or(VisualizeError::MissingPlan)?
        .into_domain(effective_profile)?;

    let postgres_version = metadata
        .postgres_version
        .clone()
        .or_else(|| options.postgres_version_hint.clone());

    Ok(Explain {
        plan,
        postgres_version,
        planning_time: metadata.planning_time,
        execution_time: metadata.execution_time,
        ..Default::default()
    })
}

/// Validate the domain plan as an explicit tree with structural invariants.
pub fn validate_plan_tree(explain: Explain) -> Result<Explain, VisualizeError> {
    let tree = PlanTree::new(explain.plan.clone())?;
    Ok(Explain {
        plan: tree.into_root(),
        ..explain
    })
}

/// Run the full parser pipeline from raw JSON through validated domain explain.
pub fn parse_explain_document(input: &str) -> Result<Explain, VisualizeError> {
    parse_explain_document_with(input, &ParseOptions::default())
}

/// Like [`parse_explain_document`] but accepting a `ParseOptions` value.
pub fn parse_explain_document_with(
    input: &str,
    options: &ParseOptions,
) -> Result<Explain, VisualizeError> {
    let raw_explain = parse_raw_explains(input)?
        .into_iter()
        .next()
        .ok_or(VisualizeError::EmptyExplainArray)?;
    let mut explain = build_domain_explain_with(raw_explain.clone(), options)?;

    if raw_explain.schema_profile() == PostgresSchemaProfile::Unknown
        && options.schema_profile_hint().is_none()
    {
        explain.postgres_version = None;
    }

    validate_plan_tree(explain)
}

// Avoid an "unused" warning if no caller of `RawPlan` needs it from here yet.
#[allow(dead_code)]
fn _ensure_raw_plan_visible(_: &RawPlan) {}

#[cfg(test)]
mod tests {
    use super::*;

    const PG12_PAYLOAD_NO_VERSION: &str = r#"[{"Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1,"IO Read Time":1.5}}]"#;

    #[test]
    fn version_hint_picks_legacy_io_timing_for_pg12() {
        let options = ParseOptions::new().with_postgres_version_hint("12");
        let explain = parse_explain_document_with(PG12_PAYLOAD_NO_VERSION, &options)
            .expect("parse with hint");
        assert!((explain.plan.io_timing.io_read_time - 1.5).abs() < 1e-9);
        assert_eq!(explain.postgres_version.as_deref(), Some("12"));
    }

    #[test]
    fn version_hint_picks_modern_io_timing_for_pg13_plus() {
        let options = ParseOptions::new().with_postgres_version_hint("PostgreSQL 16.1");
        let explain = parse_explain_document_with(PG12_PAYLOAD_NO_VERSION, &options)
            .expect("parse with hint");
        // Legacy `IO Read Time` is preserved on modern profile only when canonical is missing.
        assert!((explain.plan.io_timing.io_read_time - 1.5).abs() < 1e-9);
        assert_eq!(explain.postgres_version.as_deref(), Some("PostgreSQL 16.1"));
    }

    #[test]
    fn metadata_version_in_payload_wins_over_hint() {
        let payload = r#"[{"PostgreSQL Version":"PostgreSQL 12","Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1}}]"#;
        let options = ParseOptions::new().with_postgres_version_hint("PostgreSQL 16.1");
        let explain = parse_explain_document_with(payload, &options).expect("parse");
        assert_eq!(explain.postgres_version.as_deref(), Some("PostgreSQL 12"));
    }

    #[test]
    fn invalid_hint_is_ignored_and_does_not_error() {
        let options = ParseOptions::new().with_postgres_version_hint("not-a-version");
        let explain = parse_explain_document_with(PG12_PAYLOAD_NO_VERSION, &options)
            .expect("parse with bad hint");
        assert!(explain.postgres_version.is_none());
    }

    #[test]
    fn no_hint_preserves_existing_unknown_profile_behavior() {
        let explain = parse_explain_document(PG12_PAYLOAD_NO_VERSION).expect("parse no hint");
        assert!(explain.postgres_version.is_none());
    }
}
