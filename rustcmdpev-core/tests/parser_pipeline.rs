use rustcmdpev_core::parser::{build_domain_explain, parse_raw_explains, validate_plan_tree};
use rustcmdpev_core::render::{render_explain, RenderOptions};
use rustcmdpev_core::structure::data::io_timing::PlanIoTiming;
use rustcmdpev_core::structure::raw::{PostgresSchemaProfile, RawPlanIoTiming};
use rustcmdpev_core::structure::tree::PlanTree;
use rustcmdpev_core::VisualizeError;

#[test]
fn parser_pipeline_flows_from_raw_json_to_domain_to_validated_tree() {
    let input = r#"
        [
          {
            "PostgreSQL Version": "16.4",
            "Plan": {
              "Node Type": "Nested Loop",
              "Startup Cost": 1.25,
              "Total Cost": 2.50,
              "Actual Startup Time": 0.10,
              "Actual Total Time": 0.20,
              "Actual Rows": 1,
              "Actual Loops": 1,
              "Plans": [
                {
                  "Node Type": "Seq Scan",
                  "Startup Cost": 0.00,
                  "Total Cost": 1.00,
                  "Actual Startup Time": 0.01,
                  "Actual Total Time": 0.03,
                  "Actual Rows": 1,
                  "Actual Loops": 1
                }
              ]
            },
            "Planning Time": 0.20,
            "Execution Time": 0.50
          }
        ]
    "#;

    let raw = parse_raw_explains(input).expect("expected raw explain parse");
    assert_eq!(raw.len(), 1);
    assert_eq!(raw[0].schema_profile(), PostgresSchemaProfile::ModernIoTiming);

    let domain = build_domain_explain(raw.into_iter().next().expect("expected first explain"))
        .expect("expected domain explain");
    assert_eq!(domain.plan.node_type, "Nested Loop");
    assert_eq!(domain.postgres_version.as_deref(), Some("16.4"));

    let validated = validate_plan_tree(domain).expect("expected validated tree");
    assert_eq!(validated.plan.plans.len(), 1);
}

#[test]
fn schema_aware_parser_accepts_legacy_io_timing_aliases() {
    let input = r#"
        [
          {
            "Postgres Version": "12.14",
            "Plan": {
              "Node Type": "Seq Scan",
              "Startup Cost": 0.00,
              "Total Cost": 4.00,
              "Actual Startup Time": 0.05,
              "Actual Total Time": 0.10,
              "Actual Rows": 3,
              "Actual Loops": 1,
              "IO Read Time": 1.25,
              "IO Write Time": 0.75
            }
          }
        ]
    "#;

    let raw = parse_raw_explains(input).expect("expected raw explain parse");
    let explain = build_domain_explain(raw.into_iter().next().expect("expected first explain"))
        .expect("expected domain explain");

    assert_eq!(explain.plan.io_read_time, 1.25);
    assert_eq!(explain.plan.io_write_time, 0.75);
    assert_eq!(explain.postgres_version.as_deref(), Some("12.14"));
}

#[test]
fn schema_aware_parser_prefers_canonical_io_fields_for_modern_versions() {
    let io_timing = RawPlanIoTiming {
        canonical: PlanIoTiming {
            io_read_time: 2.5,
            io_write_time: 3.5,
        },
        legacy_io_read_time: Some(9.5),
        legacy_io_write_time: Some(10.5),
    };

    let resolved = io_timing.resolve(PostgresSchemaProfile::ModernIoTiming);

    assert_eq!(resolved.io_read_time, 2.5);
    assert_eq!(resolved.io_write_time, 3.5);
}

#[test]
fn schema_aware_parser_falls_back_to_legacy_io_fields_for_legacy_versions() {
    let io_timing = RawPlanIoTiming {
        canonical: PlanIoTiming {
            io_read_time: 0.0,
            io_write_time: 0.0,
        },
        legacy_io_read_time: Some(7.25),
        legacy_io_write_time: Some(8.25),
    };

    let resolved = io_timing.resolve(PostgresSchemaProfile::LegacyIoTiming);

    assert_eq!(resolved.io_read_time, 7.25);
    assert_eq!(resolved.io_write_time, 8.25);
}

#[test]
fn grouped_raw_models_preserve_typed_identity_predicates_and_buffers() {
    let input = r#"
        [
          {
            "Plan": {
              "Node Type": "Index Scan",
              "Relation Name": "coaches",
              "Schema": "public",
              "Alias": "coaches",
              "Index Name": "coaches_pkey",
              "Index Cond": "(coaches.id = 42)",
              "Output": ["coaches.id"],
              "Shared Hit Blocks": 5,
              "Temp Written Blocks": 1,
              "Startup Cost": 0.10,
              "Total Cost": 2.50,
              "Plan Rows": 1,
              "Plan Width": 8,
              "Actual Startup Time": 0.01,
              "Actual Total Time": 0.02,
              "Actual Rows": 1,
              "Actual Loops": 1
            }
          }
        ]
    "#;

    let raw = parse_raw_explains(input).expect("expected raw explain parse");
    let explain = build_domain_explain(raw.into_iter().next().expect("expected first explain"))
        .expect("expected domain explain");

    assert_eq!(explain.plan.node_type, "Index Scan");
    assert_eq!(explain.plan.relation_name, "coaches");
    assert_eq!(explain.plan.index_name, "coaches_pkey");
    assert_eq!(explain.plan.index_condition, "(coaches.id = 42)");
    assert_eq!(explain.plan.output, vec!["coaches.id"]);
    assert_eq!(explain.plan.shared_hit_blocks, 5);
    assert_eq!(explain.plan.temp_written_blocks, 1);
}

#[test]
fn plan_tree_reports_explicit_node_and_depth_semantics() {
    let explain = rustcmdpev_core::parse_and_process(
        r#"[{"Plan":{"Node Type":"Nested Loop","Startup Cost":0.0,"Total Cost":2.0,"Actual Startup Time":0.1,"Actual Total Time":0.3,"Actual Rows":2,"Actual Loops":1,"Plans":[{"Node Type":"Seq Scan","Startup Cost":0.0,"Total Cost":1.0,"Actual Startup Time":0.05,"Actual Total Time":0.1,"Actual Rows":2,"Actual Loops":1}]},"Execution Time":0.3}]"#,
    )
    .expect("expected parsed explain");

    let tree = PlanTree::new(explain.plan.clone()).expect("expected valid plan tree");

    assert_eq!(tree.root().child_count(), 1);
    assert!(tree.root().has_children());
    assert!(!tree.root().is_leaf());
    assert_eq!(tree.stats().node_count, 2);
    assert_eq!(tree.stats().max_depth, 1);
}

#[test]
fn validated_tree_rejects_negative_numeric_invariants() {
    let input = r#"
        [
          {
            "Plan": {
              "Node Type": "Seq Scan",
              "Startup Cost": -1.00,
              "Total Cost": 4.00,
              "Actual Startup Time": 0.05,
              "Actual Total Time": 0.10,
              "Actual Rows": 3,
              "Actual Loops": 1
            }
          }
        ]
    "#;

    let err = rustcmdpev_core::parse_and_process(input).expect_err("expected invariant error");
    match err {
        VisualizeError::InvalidPlan(message) => {
            assert!(message.contains("Startup Cost"));
        }
        other => panic!("expected invalid plan error, got {other:?}"),
    }
}

#[test]
fn validated_tree_rejects_missing_node_types() {
    let input = r#"
        [
          {
            "Plan": {
              "Startup Cost": 0.00,
              "Total Cost": 4.00,
              "Actual Startup Time": 0.05,
              "Actual Total Time": 0.10,
              "Actual Rows": 3,
              "Actual Loops": 1
            }
          }
        ]
    "#;

    let err = rustcmdpev_core::parse_and_process(input).expect_err("expected invariant error");
    match err {
        VisualizeError::InvalidPlan(message) => {
            assert!(message.contains("Node Type"));
        }
        other => panic!("expected invalid plan error, got {other:?}"),
    }
}

#[test]
fn rendering_is_separate_and_returns_tree_text() {
    let explain = rustcmdpev_core::parse_and_process(
        r#"[{"Plan":{"Node Type":"Seq Scan","Startup Cost":0.0,"Total Cost":1.0,"Actual Startup Time":0.1,"Actual Total Time":0.2,"Actual Rows":2,"Actual Loops":1},"Execution Time":0.2}]"#,
    )
    .expect("expected parsed explain");

    let rendered = render_explain(&explain, RenderOptions { width: 80 });

    assert!(rendered.contains("Seq Scan"));
    assert!(rendered.contains("○ Duration:"));
    assert!(rendered.contains("┬"));
}

#[test]
fn library_can_render_without_printing_as_part_of_analysis() {
    let rendered = rustcmdpev_core::render_visualization(
        r#"[{"Plan":{"Node Type":"Seq Scan","Startup Cost":0.0,"Total Cost":1.0,"Actual Startup Time":0.1,"Actual Total Time":0.2,"Actual Rows":2,"Actual Loops":1},"Execution Time":0.2}]"#,
        80,
    )
    .expect("expected rendered output");

    assert!(rendered.contains("Seq Scan"));
    assert!(rendered.contains("Execution Time"));
}
