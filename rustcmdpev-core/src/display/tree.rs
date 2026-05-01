//! Tree-drawing primitives for plan rendering.
//!
//! This module owns the unicode glyphs and joint-selection logic used to draw
//! plan trees. Rendering code combines these with formatting and color helpers
//! from the sibling `format` and `colors` modules.

use crate::constants::{
    TREE_BRANCH, TREE_ELBOW, TREE_OUTPUT_BRANCH, TREE_OUTPUT_CHILD, TREE_OUTPUT_CONTINUATION,
    TREE_OUTPUT_PADDING, TREE_VERTICAL,
};
use crate::structure::data::plan::Plan;

/// Choose the joint glyph for a node based on whether it has multiple children
/// or is the last child of its parent.
pub fn node_joint(child_count: usize, last_child: bool) -> &'static str {
    if child_count > 1 || last_child {
        TREE_ELBOW
    } else {
        TREE_BRANCH
    }
}

/// Continuation glyphs to extend the prefix as rendering descends.
pub fn prefix_continuation(child_count: usize, last_child: bool) -> &'static str {
    if child_count > 1 || last_child {
        "  "
    } else {
        TREE_VERTICAL
    }
}

/// Glyph used at the leftmost column of an output line for a child plan.
pub fn output_terminator(index: usize, plan: &Plan) -> &'static str {
    if index == 0 {
        if plan.plans.is_empty() {
            TREE_OUTPUT_CHILD
        } else {
            TREE_OUTPUT_BRANCH
        }
    } else if plan.plans.is_empty() {
        TREE_OUTPUT_PADDING
    } else {
        TREE_OUTPUT_CONTINUATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_joint_uses_elbow_for_multi_child_or_last() {
        assert_eq!(node_joint(2, false), TREE_ELBOW);
        assert_eq!(node_joint(1, true), TREE_ELBOW);
    }

    #[test]
    fn node_joint_uses_branch_for_single_non_last() {
        assert_eq!(node_joint(1, false), TREE_BRANCH);
        assert_eq!(node_joint(0, false), TREE_BRANCH);
    }

    #[test]
    fn prefix_continuation_collapses_for_multi_child_or_last() {
        assert_eq!(prefix_continuation(2, false), "  ");
        assert_eq!(prefix_continuation(1, true), "  ");
    }

    #[test]
    fn prefix_continuation_keeps_vertical_for_single_non_last() {
        assert_eq!(prefix_continuation(1, false), TREE_VERTICAL);
    }

    #[test]
    fn output_terminator_handles_leaf_first_line() {
        let plan = Plan::default();
        assert_eq!(output_terminator(0, &plan), TREE_OUTPUT_CHILD);
    }

    #[test]
    fn output_terminator_handles_branch_first_line() {
        let mut plan = Plan::default();
        plan.plans.push(Plan::default());
        assert_eq!(output_terminator(0, &plan), TREE_OUTPUT_BRANCH);
    }

    #[test]
    fn output_terminator_handles_leaf_continuation() {
        let plan = Plan::default();
        assert_eq!(output_terminator(1, &plan), TREE_OUTPUT_PADDING);
    }

    #[test]
    fn output_terminator_handles_branch_continuation() {
        let mut plan = Plan::default();
        plan.plans.push(Plan::default());
        assert_eq!(output_terminator(1, &plan), TREE_OUTPUT_CONTINUATION);
    }
}
