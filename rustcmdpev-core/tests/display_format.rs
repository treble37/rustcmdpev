use rustcmdpev_core::display::format::{format_tags, get_terminator};
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
