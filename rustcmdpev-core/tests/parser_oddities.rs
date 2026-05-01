//! G9.1 — Targeted regression tests for parsing oddities and analysis edge
//! cases that real-world EXPLAIN payloads exercise: legacy/modern IO field
//! aliases, zero-rows estimate divisions, CTE-child cost accounting, and
//! version-hint disambiguation.

use rustcmdpev_core::analysis;
use rustcmdpev_core::parser::{parse_explain_document, parse_explain_document_with, ParseOptions};
use rustcmdpev_core::structure::data::actuals::PlanActuals;
use rustcmdpev_core::structure::data::estimates::PlanEstimates;
use rustcmdpev_core::structure::data::explain::Explain;
use rustcmdpev_core::structure::data::plan::Plan;

fn leaf(node_type: &str, total_cost: f64, total_time: f64, rows: u64) -> Plan {
    let mut plan = Plan::default();
    plan.identity.node_type = node_type.to_string();
    plan.estimates = PlanEstimates {
        total_cost,
        ..PlanEstimates::default()
    };
    plan.actuals = PlanActuals {
        actual_total_time: total_time,
        actual_rows: rows,
        actual_loops: 1,
        ..PlanActuals::default()
    };
    plan
}

#[test]
fn legacy_alias_io_read_time_resolves_when_canonical_missing() {
    let payload = r#"[{"PostgreSQL Version":"12.4","Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1,"IO Read Time":3.25}}]"#;
    let explain = parse_explain_document(payload).expect("parse");
    assert!((explain.plan.io_timing.io_read_time - 3.25).abs() < 1e-9);
}

#[test]
fn modern_canonical_io_read_time_wins_over_legacy_when_both_present() {
    let payload = r#"[{"PostgreSQL Version":"PostgreSQL 16.0","Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1,"I/O Read Time":4.0,"IO Read Time":2.5}}]"#;
    let explain = parse_explain_document(payload).expect("parse");
    assert!((explain.plan.io_timing.io_read_time - 4.0).abs() < 1e-9);
}

#[test]
fn version_hint_resolves_legacy_io_when_metadata_silent() {
    let payload = r#"[{"Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1,"IO Read Time":7.5}}]"#;
    let explain = parse_explain_document_with(
        payload,
        &ParseOptions::new().with_postgres_version_hint("11"),
    )
    .expect("parse with hint");
    assert!((explain.plan.io_timing.io_read_time - 7.5).abs() < 1e-9);
    assert_eq!(explain.postgres_version.as_deref(), Some("11"));
}

#[test]
fn calculate_planner_estimate_zero_actual_rows_does_not_divide_by_zero() {
    let mut plan = leaf("Seq Scan", 10.0, 1.0, 0);
    plan.estimates.plan_rows = 1_000;
    plan.actuals.actual_rows = 0;
    analysis::calculate_planner_estimate(&mut plan);
    // With zero actuals and non-zero estimate, factor must remain finite.
    assert!(plan.analysis_flags.planner_row_estimate_factor.is_finite());
    assert_eq!(plan.analysis_flags.planner_row_estimate_direction, "Over");
}

#[test]
fn calculate_planner_estimate_equal_rows_clears_factor() {
    let mut plan = leaf("Seq Scan", 1.0, 1.0, 5);
    plan.estimates.plan_rows = 5;
    plan.actuals.actual_rows = 5;
    analysis::calculate_planner_estimate(&mut plan);
    assert_eq!(plan.analysis_flags.planner_row_estimate_factor, 0.0);
    assert!(plan.analysis_flags.planner_row_estimate_direction.is_empty());
}

#[test]
fn cte_child_does_not_subtract_from_parent_cost() {
    let mut explain = Explain::default();
    let mut parent = leaf("Hash Join", 10.0, 5.0, 50);
    let mut cte_child = leaf("CTE Scan", 4.0, 2.0, 10);
    cte_child.identity.node_type = "CTE Scan".into();
    parent.plans.push(cte_child);

    analysis::calculate_actuals(&mut explain, &mut parent);

    // CTE Scan child is not subtracted from parent's actual_cost/duration.
    assert!((parent.actuals.actual_cost - 10.0).abs() < 1e-6);
    assert!((parent.actuals.actual_duration - 5.0).abs() < 1e-6);
}

#[test]
fn calculate_actuals_clamps_negative_cost_to_zero() {
    let mut explain = Explain::default();
    let mut parent = leaf("Hash Join", 1.0, 1.0, 10);
    parent.plans.push(leaf("Seq Scan", 5.0, 0.5, 5));
    analysis::calculate_actuals(&mut explain, &mut parent);
    assert_eq!(parent.actuals.actual_cost, 0.0);
}

#[test]
fn loops_multiplier_scales_actual_duration() {
    let mut explain = Explain::default();
    let mut plan = leaf("Seq Scan", 1.0, 2.5, 1);
    plan.actuals.actual_loops = 4;
    analysis::calculate_actuals(&mut explain, &mut plan);
    assert!((plan.actuals.actual_duration - 10.0).abs() < 1e-6);
}

#[test]
fn alias_postgres_version_field_is_accepted() {
    let payload = r#"[{"Postgres Version":"PostgreSQL 13.6","Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1}}]"#;
    let explain = parse_explain_document(payload).expect("parse");
    assert_eq!(
        explain.postgres_version.as_deref(),
        Some("PostgreSQL 13.6")
    );
}

#[test]
fn missing_optional_planning_and_execution_times_default_to_zero() {
    let payload = r#"[{"Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1}}]"#;
    let explain = parse_explain_document(payload).expect("parse");
    assert_eq!(explain.planning_time, 0.0);
    assert_eq!(explain.execution_time, 0.0);
}
