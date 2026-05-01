//! G9.4 — End-to-end coverage that the published `testing` helpers compose
//! to support common assertion patterns: build a plan with the builders,
//! render it, strip color, and verify expected/forbidden tokens.

use rustcmdpev_core::display::colors::Theme;
use rustcmdpev_core::render::{render_explain, RenderOptions};
use rustcmdpev_core::testing::{
    assert_contains_all, assert_contains_none, collect_nodes, explain_for, node_count, strip_ansi,
    with_children, LeafBuilder,
};

#[test]
fn fixture_helpers_compose_to_render_a_three_node_join() {
    let join = with_children(
        LeafBuilder::new("Hash Join").cost(5.0).time(2.0).build(),
        [
            LeafBuilder::new("Seq Scan")
                .relation("public", "users")
                .cost(2.0)
                .time(0.5)
                .build(),
            LeafBuilder::new("Index Scan")
                .relation("public", "events")
                .index("events_user_id_idx")
                .cost(1.0)
                .time(0.5)
                .build(),
        ],
    );
    assert_eq!(node_count(&join), 3);

    let explain = explain_for(join, 2.0, 5.0);
    let rendered = render_explain(
        &explain,
        RenderOptions::new(80).with_theme(Theme::NoColor),
    );

    assert_contains_all(
        &rendered,
        &[
            "Hash Join",
            "Seq Scan",
            "Index Scan",
            "users",
            "events",
            "events_user_id_idx",
        ],
    );
    assert_contains_none(&rendered, &["\u{1b}["]);
}

#[test]
fn strip_ansi_lets_callers_compare_against_colored_output() {
    let plan = LeafBuilder::new("Seq Scan").build();
    let rendered = render_explain(
        &explain_for(plan, 1.0, 1.0),
        RenderOptions::new(60), // default colored theme
    );
    let stripped = strip_ansi(&rendered);
    assert!(!stripped.contains('\u{1b}'));
    assert!(stripped.contains("Seq Scan"));
}

#[test]
fn collect_nodes_walks_in_depth_first_parent_first_order() {
    let plan = with_children(
        LeafBuilder::new("A").build(),
        [
            with_children(
                LeafBuilder::new("B").build(),
                [LeafBuilder::new("C").build()],
            ),
            LeafBuilder::new("D").build(),
        ],
    );
    let order: Vec<_> = collect_nodes(&plan)
        .iter()
        .map(|p| p.identity.node_type.as_str())
        .collect();
    assert_eq!(order, vec!["A", "B", "C", "D"]);
}
