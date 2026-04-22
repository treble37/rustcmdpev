use crate::structure::data::explain::Explain;
use crate::structure::raw::{PostgresSchemaProfile, RawExplain};
use crate::structure::tree::PlanTree;
use crate::VisualizeError;

pub fn parse_raw_explains(input: &str) -> Result<Vec<RawExplain>, VisualizeError> {
    serde_json::from_str(input).map_err(VisualizeError::InvalidJson)
}

pub fn build_domain_explain(raw: RawExplain) -> Result<Explain, VisualizeError> {
    raw.into_domain()
}

pub fn validate_plan_tree(explain: Explain) -> Result<Explain, VisualizeError> {
    let tree = PlanTree::new(explain.plan.clone())?;
    Ok(Explain {
        plan: tree.into_root(),
        ..explain
    })
}

pub fn parse_explain_document(input: &str) -> Result<Explain, VisualizeError> {
    let raw_explain = parse_raw_explains(input)?
        .into_iter()
        .next()
        .ok_or(VisualizeError::EmptyExplainArray)?;
    let mut explain = build_domain_explain(raw_explain.clone())?;

    if raw_explain.schema_profile() == PostgresSchemaProfile::Unknown {
        explain.postgres_version = None;
    }

    validate_plan_tree(explain)
}
