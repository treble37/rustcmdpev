//! Tree-drawing primitives for plan rendering.
//!
//! This module owns the glyphs and joint-selection logic used to draw plan
//! trees. Rendering code combines these with formatting and color helpers
//! from the sibling `format` and `colors` modules.
//!
//! [`TreeStyle`] groups every glyph the renderer needs into one struct so
//! callers can swap between the historical Unicode look (parity with
//! gocmdpev), a portable ASCII fallback, and a heavier Unicode style without
//! editing constants per call site.

use crate::constants::{
    TREE_BRANCH, TREE_ELBOW, TREE_NODE_CONNECTOR, TREE_OUTPUT_BRANCH, TREE_OUTPUT_CHILD,
    TREE_OUTPUT_CONTINUATION, TREE_OUTPUT_PADDING, TREE_ROOT_MARKER, TREE_VERTICAL,
};
use crate::structure::data::plan::Plan;

/// A complete set of glyphs for drawing a plan tree.
///
/// Each field maps directly onto a position in the rendered output. The
/// `vertical_pad` and `padding` fields keep child indentation aligned even
/// when the elbow/branch glyphs change width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeStyle {
    pub root_marker: &'static str,
    pub vertical: &'static str,
    pub vertical_pad: &'static str,
    pub elbow: &'static str,
    pub branch: &'static str,
    pub node_connector: &'static str,
    pub output_child: &'static str,
    pub output_branch: &'static str,
    pub output_continuation: &'static str,
    pub output_padding: &'static str,
    pub padding: &'static str,
}

impl TreeStyle {
    /// Historical Unicode glyphs matching the gocmdpev parity output.
    pub const fn unicode() -> Self {
        Self {
            root_marker: TREE_ROOT_MARKER,
            vertical: TREE_VERTICAL,
            vertical_pad: " ",
            elbow: TREE_ELBOW,
            branch: TREE_BRANCH,
            node_connector: TREE_NODE_CONNECTOR,
            output_child: TREE_OUTPUT_CHILD,
            output_branch: TREE_OUTPUT_BRANCH,
            output_continuation: TREE_OUTPUT_CONTINUATION,
            output_padding: TREE_OUTPUT_PADDING,
            padding: "  ",
        }
    }

    /// Pure ASCII alternative for terminals or pipelines that mangle Unicode.
    pub const fn ascii() -> Self {
        Self {
            root_marker: "+",
            vertical: "|",
            vertical_pad: " ",
            elbow: "+",
            branch: "+",
            node_connector: "--",
            output_child: ">> ",
            output_branch: "+>  ",
            output_continuation: "|  ",
            output_padding: "   ",
            padding: "  ",
        }
    }

    /// Heavier Unicode box-drawing variant with bolder strokes.
    pub const fn heavy() -> Self {
        Self {
            root_marker: "┳",
            vertical: "┃",
            vertical_pad: " ",
            elbow: "┗",
            branch: "┣",
            node_connector: "━┓",
            output_child: "╚▶ ",
            output_branch: "┣▶  ",
            output_continuation: "┃  ",
            output_padding: "   ",
            padding: "  ",
        }
    }

    pub fn parse(name: &str) -> Option<TreeStyle> {
        match name {
            "unicode" | "default" => Some(TreeStyle::unicode()),
            "ascii" | "plain" => Some(TreeStyle::ascii()),
            "heavy" | "bold" => Some(TreeStyle::heavy()),
            _ => None,
        }
    }
}

impl Default for TreeStyle {
    fn default() -> Self {
        Self::unicode()
    }
}

/// Choose the joint glyph for a node based on whether it has multiple children
/// or is the last child of its parent.
pub fn node_joint(child_count: usize, last_child: bool) -> &'static str {
    styled_node_joint(&TreeStyle::unicode(), child_count, last_child)
}

pub fn styled_node_joint(
    style: &TreeStyle,
    child_count: usize,
    last_child: bool,
) -> &'static str {
    if child_count > 1 || last_child {
        style.elbow
    } else {
        style.branch
    }
}

/// Continuation glyphs to extend the prefix as rendering descends.
pub fn prefix_continuation(child_count: usize, last_child: bool) -> &'static str {
    styled_prefix_continuation(&TreeStyle::unicode(), child_count, last_child)
}

pub fn styled_prefix_continuation(
    style: &TreeStyle,
    child_count: usize,
    last_child: bool,
) -> &'static str {
    if child_count > 1 || last_child {
        style.padding
    } else {
        style.vertical
    }
}

/// Glyph used at the leftmost column of an output line for a child plan.
pub fn output_terminator(index: usize, plan: &Plan) -> &'static str {
    styled_output_terminator(&TreeStyle::unicode(), index, plan)
}

pub fn styled_output_terminator(
    style: &TreeStyle,
    index: usize,
    plan: &Plan,
) -> &'static str {
    if index == 0 {
        if plan.plans.is_empty() {
            style.output_child
        } else {
            style.output_branch
        }
    } else if plan.plans.is_empty() {
        style.output_padding
    } else {
        style.output_continuation
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

    #[test]
    fn ascii_style_has_no_unicode_box_drawing_glyphs() {
        let style = TreeStyle::ascii();
        let payload = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            style.root_marker,
            style.vertical,
            style.elbow,
            style.branch,
            style.node_connector,
            style.output_child,
            style.output_branch,
            style.output_continuation,
            style.output_padding,
            style.padding,
        );
        assert!(payload.is_ascii(), "expected ascii-only payload, got: {payload}");
    }

    #[test]
    fn heavy_style_uses_distinct_glyphs_from_unicode() {
        let unicode = TreeStyle::unicode();
        let heavy = TreeStyle::heavy();
        assert_ne!(unicode.vertical, heavy.vertical);
        assert_ne!(unicode.elbow, heavy.elbow);
        assert_ne!(unicode.root_marker, heavy.root_marker);
    }

    #[test]
    fn parse_returns_named_style_or_none() {
        assert_eq!(TreeStyle::parse("unicode"), Some(TreeStyle::unicode()));
        assert_eq!(TreeStyle::parse("ascii"), Some(TreeStyle::ascii()));
        assert_eq!(TreeStyle::parse("heavy"), Some(TreeStyle::heavy()));
        assert_eq!(TreeStyle::parse("bold"), Some(TreeStyle::heavy()));
        assert_eq!(TreeStyle::parse("rococo"), None);
    }

    #[test]
    fn styled_helpers_match_legacy_helpers_for_unicode_style() {
        let style = TreeStyle::unicode();
        assert_eq!(styled_node_joint(&style, 2, false), node_joint(2, false));
        assert_eq!(
            styled_prefix_continuation(&style, 1, false),
            prefix_continuation(1, false)
        );
        let mut plan = Plan::default();
        plan.plans.push(Plan::default());
        assert_eq!(
            styled_output_terminator(&style, 0, &plan),
            output_terminator(0, &plan)
        );
    }
}
