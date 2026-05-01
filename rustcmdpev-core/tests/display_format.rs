use rustcmdpev_core::display::format::{format_tags, get_terminator};
use rustcmdpev_core::display::tree::{node_joint, output_terminator, prefix_continuation};
use rustcmdpev_core::structure::data::plan::Plan;

#[test]
fn format_tags_uses_shared_bad_estimate_threshold() {
    let mut plan = Plan::default();
    plan.analysis_flags.slowest = true;
    plan.analysis_flags.planner_row_estimate_factor = 100.0;

    let tags = format_tags(plan);

    assert!(tags.contains("slowest"));
    assert!(tags.contains("bad estimate"));
}

#[test]
fn get_terminator_uses_shared_tree_markers() {
    let leaf = Plan::default();
    let mut parent = Plan::default();
    parent.plans.push(Plan::default());

    assert_eq!(get_terminator(0, leaf), "⌡► ");
    assert_eq!(get_terminator(0, parent.clone()), "├►  ");
    assert_eq!(get_terminator(1, parent), "│  ");
}

#[test]
fn tree_module_exposes_terminator_directly() {
    let leaf = Plan::default();
    let mut parent = Plan::default();
    parent.plans.push(Plan::default());

    assert_eq!(output_terminator(0, &leaf), "⌡► ");
    assert_eq!(output_terminator(0, &parent), "├►  ");
    assert_eq!(output_terminator(1, &parent), "│  ");
}

#[test]
fn tree_module_picks_joint_and_continuation() {
    assert_eq!(node_joint(1, false), "├");
    assert_eq!(node_joint(1, true), "└");
    assert_eq!(node_joint(2, false), "└");

    assert_eq!(prefix_continuation(1, false), "│");
    assert_eq!(prefix_continuation(1, true), "  ");
    assert_eq!(prefix_continuation(2, false), "  ");
}
