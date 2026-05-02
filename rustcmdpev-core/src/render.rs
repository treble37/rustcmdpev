use std::fmt::Write;

use crate::constants::DESCRIPTIONS;
use crate::display::colors::{themed_format, Theme};
use crate::display::format::{
    duration_to_string_themed, format_details, format_percent, format_tags,
};
use crate::display::tree::{
    styled_node_joint, styled_output_terminator, styled_prefix_continuation, TreeStyle,
};
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;
use crate::summary::PlanSummary;

/// Render verbosity for a plan.
///
/// `Default` matches the historical baseline. `Condensed` collapses per-node
/// description prose and per-node output expressions; it is intended for
/// dense scrolled views. `Verbose` adds buffer/IO timing extras after the
/// per-node duration block.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum RenderMode {
    #[default]
    Default,
    Condensed,
    Verbose,
}

impl RenderMode {
    pub fn parse(name: &str) -> Option<RenderMode> {
        match name {
            "default" | "normal" => Some(RenderMode::Default),
            "condensed" | "compact" => Some(RenderMode::Condensed),
            "verbose" | "full" => Some(RenderMode::Verbose),
            _ => None,
        }
    }
}

/// Detail level of the header summary block.
///
/// `Compact` preserves the gocmdpev-parity three-line header (cost, planning,
/// execution). `Detailed` adds aggregate Total Loops, shared/local/temp buffer
/// counts, and total I/O timing — useful for tuning without inspecting every
/// node.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum SummaryStyle {
    #[default]
    Compact,
    Detailed,
}

impl SummaryStyle {
    pub fn parse(name: &str) -> Option<SummaryStyle> {
        match name {
            "compact" | "minimal" => Some(SummaryStyle::Compact),
            "detailed" | "full" => Some(SummaryStyle::Detailed),
            _ => None,
        }
    }
}

/// Options controlling pretty-plan rendering.
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub width: usize,
    pub theme: Theme,
    pub mode: RenderMode,
    pub summary: SummaryStyle,
    pub tree_style: TreeStyle,
}

impl RenderOptions {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            theme: Theme::default(),
            mode: RenderMode::default(),
            summary: SummaryStyle::default(),
            tree_style: TreeStyle::default(),
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_mode(mut self, mode: RenderMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_summary(mut self, summary: SummaryStyle) -> Self {
        self.summary = summary;
        self
    }

    pub fn with_tree_style(mut self, tree_style: TreeStyle) -> Self {
        self.tree_style = tree_style;
        self
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self::new(60)
    }
}

/// Mutable context threaded through the recursive render. Holds the per-render
/// state that does not vary as the traversal descends so call sites stay focused
/// on the per-node arguments they actually care about.
struct RenderContext<'a> {
    buffer: &'a mut String,
    explain: &'a Explain,
    options: RenderOptions,
}

impl RenderContext<'_> {
    fn paint<S: AsRef<str>>(&self, text: S, role: &str) -> colored::ColoredString {
        themed_format(text, role, self.options.theme)
    }
}

/// Per-node positional arguments grouped to keep recursion signatures small.
struct NodePosition {
    prefix: String,
    last_child: bool,
}

fn write_summary_block(buffer: &mut String, summary: &PlanSummary, options: RenderOptions) {
    let theme = options.theme;
    writeln!(buffer, "○ Total Cost {}", summary.total_cost).expect("write to string");
    writeln!(
        buffer,
        "○ Planning Time: {}",
        duration_to_string_themed(summary.planning_time, theme)
    )
    .expect("write to string");
    writeln!(
        buffer,
        "○ Execution Time: {}",
        duration_to_string_themed(summary.execution_time, theme)
    )
    .expect("write to string");

    let detailed = options.summary == SummaryStyle::Detailed || options.mode == RenderMode::Verbose;
    if !detailed {
        return;
    }

    writeln!(buffer, "○ Total Loops: {}", summary.total_loops).expect("write to string");
    writeln!(buffer, "○ Total Nodes: {}", summary.node_count).expect("write to string");

    let buffers = &summary.buffers;
    if !buffers.is_empty() {
        writeln!(
            buffer,
            "○ Buffers: shared hit={} read={} written={} dirtied={}",
            buffers.shared_hit_blocks,
            buffers.shared_read_blocks,
            buffers.shared_written_blocks,
            buffers.shared_dirtied_blocks,
        )
        .expect("write to string");
        if buffers.local_hit_blocks
            + buffers.local_read_blocks
            + buffers.local_written_blocks
            + buffers.local_dirtied_blocks
            > 0
        {
            writeln!(
                buffer,
                "○ Buffers: local hit={} read={} written={} dirtied={}",
                buffers.local_hit_blocks,
                buffers.local_read_blocks,
                buffers.local_written_blocks,
                buffers.local_dirtied_blocks,
            )
            .expect("write to string");
        }
        if buffers.temp_read_blocks + buffers.temp_written_blocks > 0 {
            writeln!(
                buffer,
                "○ Buffers: temp read={} written={}",
                buffers.temp_read_blocks, buffers.temp_written_blocks,
            )
            .expect("write to string");
        }
    }

    if summary.total_io_read_time > 0.0 || summary.total_io_write_time > 0.0 {
        writeln!(
            buffer,
            "○ I/O Time: read={} write={}",
            duration_to_string_themed(summary.total_io_read_time, theme),
            duration_to_string_themed(summary.total_io_write_time, theme),
        )
        .expect("write to string");
    }
}

/// Render a processed explain tree into terminal-friendly text.
pub fn render_explain(explain: &Explain, options: RenderOptions) -> String {
    let mut buffer = String::new();
    let theme = options.theme;
    let summary = PlanSummary::from_explain(explain);
    write_summary_block(&mut buffer, &summary, options);
    writeln!(
        &mut buffer,
        "{}",
        themed_format(options.tree_style.root_marker, "output", theme)
    )
    .expect("write to string");
    let mut ctx = RenderContext {
        buffer: &mut buffer,
        explain,
        options,
    };
    let last_child = explain.plan.plans.len() == 1;
    write_plan(
        &mut ctx,
        &explain.plan,
        NodePosition {
            prefix: String::new(),
            last_child,
        },
    );
    buffer
}

fn write_plan(ctx: &mut RenderContext<'_>, plan: &Plan, position: NodePosition) {
    let explain = ctx.explain;
    let width = ctx.options.width;
    let mode = ctx.options.mode;
    let style = ctx.options.tree_style;
    let NodePosition { prefix, last_child } = position;
    let mut source_prefix = prefix;

    writeln!(
        ctx.buffer,
        "{}{}",
        ctx.paint(&source_prefix, "prefix"),
        ctx.paint(style.vertical, "prefix")
    )
    .expect("write to string");

    let joint = styled_node_joint(&style, plan.plans.len(), last_child);

    writeln!(
        ctx.buffer,
        "{}{} {}{} {}",
        ctx.paint(&source_prefix, "prefix"),
        ctx.paint(format!("{joint}{}", style.node_connector), "prefix"),
        ctx.paint(&plan.identity.node_type, "bold"),
        ctx.paint(format_details(plan), "muted"),
        ctx.paint(format_tags(plan), "tag")
    )
    .expect("write to string");

    let continuation = styled_prefix_continuation(&style, plan.plans.len(), last_child);
    if continuation == style.vertical {
        source_prefix.push_str(style.vertical);
        source_prefix.push_str(style.vertical_pad);
    } else {
        source_prefix.push_str(continuation);
    }

    let mut current_prefix = String::with_capacity(source_prefix.len() + 4);
    current_prefix.push_str(&source_prefix);
    current_prefix.push_str(style.vertical);
    current_prefix.push_str(style.vertical_pad);
    let cols = width.saturating_sub(current_prefix.len());

    if mode != RenderMode::Condensed {
        for line in textwrap::fill(DESCRIPTIONS[plan.identity.node_type.as_str()], cols).split('\n')
        {
            writeln!(
                ctx.buffer,
                "{}{}",
                ctx.paint(&current_prefix, "prefix"),
                ctx.paint(line, "muted")
            )
            .expect("write to string");
        }
    }

    writeln!(
        ctx.buffer,
        "{}○ Duration: {} {}",
        ctx.paint(&current_prefix, "prefix"),
        duration_to_string_themed(plan.actuals.actual_duration, ctx.options.theme),
        format_percent(
            (plan.actuals.actual_duration / explain.execution_time) * 100.0,
            1
        )
    )
    .expect("write to string");
    writeln!(
        ctx.buffer,
        "{}○ Cost: {} {}",
        ctx.paint(&current_prefix, "prefix"),
        duration_to_string_themed(plan.actuals.actual_cost, ctx.options.theme),
        format_percent((plan.actuals.actual_cost / explain.total_cost) * 100.0, 1)
    )
    .expect("write to string");
    writeln!(
        ctx.buffer,
        "{}○ Rows: {}",
        ctx.paint(&current_prefix, "prefix"),
        plan.actuals.actual_rows
    )
    .expect("write to string");

    if mode == RenderMode::Verbose {
        writeln!(
            ctx.buffer,
            "{}○ Loops: {}",
            ctx.paint(&current_prefix, "prefix"),
            plan.actuals.actual_loops
        )
        .expect("write to string");
        let buffers = &plan.buffers;
        let buffer_total = buffers.shared_hit_blocks
            + buffers.shared_read_blocks
            + buffers.shared_written_blocks
            + buffers.shared_dirtied_blocks
            + buffers.local_hit_blocks
            + buffers.local_read_blocks
            + buffers.local_written_blocks
            + buffers.local_dirtied_blocks
            + buffers.temp_read_blocks
            + buffers.temp_written_blocks;
        if buffer_total > 0 {
            writeln!(
                ctx.buffer,
                "{}○ Buffers: shared hit={} read={} written={} dirtied={}",
                ctx.paint(&current_prefix, "prefix"),
                buffers.shared_hit_blocks,
                buffers.shared_read_blocks,
                buffers.shared_written_blocks,
                buffers.shared_dirtied_blocks,
            )
            .expect("write to string");
        }
        if plan.io_timing.io_read_time > 0.0 || plan.io_timing.io_write_time > 0.0 {
            writeln!(
                ctx.buffer,
                "{}○ I/O: read={} write={}",
                ctx.paint(&current_prefix, "prefix"),
                duration_to_string_themed(plan.io_timing.io_read_time, ctx.options.theme),
                duration_to_string_themed(plan.io_timing.io_write_time, ctx.options.theme),
            )
            .expect("write to string");
        }
    }

    current_prefix.push_str("  ");

    if !plan.identity.join_type.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("join", "muted"),
            ctx.paint(&plan.identity.join_type, "muted")
        )
        .expect("write to string");
    }

    if !plan.identity.relation_name.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {} {}",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("on", "muted"),
            ctx.paint(&plan.identity.schema, "muted"),
            ctx.paint(&plan.identity.relation_name, "muted")
        )
        .expect("write to string");
    }

    if !plan.identity.index_name.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("using", "muted"),
            plan.identity.index_name
        )
        .expect("write to string");
    }

    if !plan.predicates.index_condition.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("condition", "muted"),
            plan.predicates.index_condition
        )
        .expect("write to string");
    }

    if !plan.predicates.filter.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {} [-{} rows]",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("filter", "muted"),
            plan.predicates.filter,
            ctx.paint(plan.predicates.rows_removed_by_filter.to_string(), "muted")
        )
        .expect("write to string");
    }

    if !plan.predicates.hash_condition.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("on", "muted"),
            plan.predicates.hash_condition
        )
        .expect("write to string");
    }

    if !plan.identity.cte_name.is_empty() {
        writeln!(
            ctx.buffer,
            "{}CTE {}",
            ctx.paint(&current_prefix, "prefix"),
            plan.identity.cte_name
        )
        .expect("write to string");
    }

    if plan.analysis_flags.planner_row_estimate_factor != 0.0 {
        writeln!(
            ctx.buffer,
            "{}{} {}estimated {} {:.2}x",
            ctx.paint(&current_prefix, "prefix"),
            ctx.paint("rows", "muted"),
            plan.analysis_flags.planner_row_estimate_direction,
            ctx.paint("by", "muted"),
            plan.analysis_flags.planner_row_estimate_factor
        )
        .expect("write to string");
    }

    if mode != RenderMode::Condensed && !plan.predicates.output.is_empty() {
        let joined_output = plan.predicates.output.join(" + ");
        let wrapped_output = textwrap::fill(&joined_output, cols);
        for (index, line) in wrapped_output.split('\n').enumerate() {
            writeln!(
                ctx.buffer,
                "{}{}{}",
                ctx.paint(&source_prefix, "prefix"),
                ctx.paint(styled_output_terminator(&style, index, plan), "output"),
                ctx.paint(line, "output")
            )
            .expect("write to string");
        }
    }

    let last_index = plan.plans.len().saturating_sub(1);
    let child_count = plan.plans.len();
    for (index, child_plan) in plan.plans.iter().enumerate() {
        let prefix = if index == child_count - 1 {
            std::mem::take(&mut source_prefix)
        } else {
            source_prefix.clone()
        };
        write_plan(
            ctx,
            child_plan,
            NodePosition {
                prefix,
                last_child: index == last_index,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::data::actuals::PlanActuals;
    use crate::structure::data::buffers::PlanBuffers;
    use crate::structure::data::estimates::PlanEstimates;

    fn child_plan(node_type: &str) -> Plan {
        let mut plan = Plan::default();
        plan.identity.node_type = node_type.to_string();
        plan.actuals = PlanActuals {
            actual_total_time: 1.0,
            actual_duration: 1.0,
            actual_rows: 5,
            actual_loops: 1,
            ..PlanActuals::default()
        };
        plan.estimates = PlanEstimates {
            total_cost: 1.0,
            ..PlanEstimates::default()
        };
        plan
    }

    fn sample_explain() -> Explain {
        let mut root = child_plan("Hash Join");
        root.plans.push(child_plan("Seq Scan"));
        root.plans.push(child_plan("Index Scan"));
        Explain {
            execution_time: 5.0,
            total_cost: 4.0,
            plan: root,
            ..Explain::default()
        }
    }

    #[test]
    fn render_context_threads_explain_and_options_into_recursion() {
        let explain = sample_explain();

        let rendered = render_explain(&explain, RenderOptions::new(80));

        assert!(rendered.contains("Hash Join"));
        assert!(rendered.contains("Seq Scan"));
        assert!(rendered.contains("Index Scan"));
        assert!(rendered.contains("○ Total Cost"));
    }

    #[test]
    fn render_mode_parse_accepts_aliases() {
        assert_eq!(RenderMode::parse("default"), Some(RenderMode::Default));
        assert_eq!(RenderMode::parse("normal"), Some(RenderMode::Default));
        assert_eq!(RenderMode::parse("condensed"), Some(RenderMode::Condensed));
        assert_eq!(RenderMode::parse("compact"), Some(RenderMode::Condensed));
        assert_eq!(RenderMode::parse("verbose"), Some(RenderMode::Verbose));
        assert_eq!(RenderMode::parse("full"), Some(RenderMode::Verbose));
        assert_eq!(RenderMode::parse("loud"), None);
    }

    #[test]
    fn condensed_mode_omits_per_node_descriptions() {
        let explain = sample_explain();

        let default = render_explain(&explain, RenderOptions::new(80));
        let condensed = render_explain(
            &explain,
            RenderOptions::new(80).with_mode(RenderMode::Condensed),
        );

        assert!(default.contains("Joins to record sets"));
        assert!(!condensed.contains("Joins to record sets"));
    }

    #[test]
    fn verbose_mode_includes_loops_and_buffers() {
        let mut explain = sample_explain();
        explain.plan.buffers = PlanBuffers {
            shared_hit_blocks: 4,
            shared_read_blocks: 2,
            ..PlanBuffers::default()
        };

        let default = render_explain(&explain, RenderOptions::new(80));
        let verbose = render_explain(
            &explain,
            RenderOptions::new(80).with_mode(RenderMode::Verbose),
        );

        assert!(!default.contains("○ Loops:"));
        assert!(!default.contains("○ Buffers:"));
        assert!(verbose.contains("○ Loops:"));
        assert!(verbose.contains("○ Buffers:"));
        assert!(verbose.contains("hit=4"));
    }

    #[test]
    fn no_color_theme_strips_ansi_escapes() {
        let explain = sample_explain();
        let options = RenderOptions::new(80).with_theme(Theme::NoColor);

        let rendered = render_explain(&explain, options);

        assert!(!rendered.contains("\u{1b}["));
        assert!(rendered.contains("Hash Join"));
    }

    #[test]
    fn compact_summary_keeps_three_line_header() {
        let explain = sample_explain();
        let rendered = render_explain(&explain, RenderOptions::new(80).with_theme(Theme::NoColor));
        assert!(rendered.contains("○ Total Cost"));
        assert!(rendered.contains("○ Planning Time"));
        assert!(rendered.contains("○ Execution Time"));
        assert!(!rendered.contains("○ Total Loops"));
        assert!(!rendered.contains("○ Total Nodes"));
    }

    #[test]
    fn detailed_summary_emits_loops_nodes_buffers_and_io() {
        let mut explain = sample_explain();
        explain.plan.buffers = PlanBuffers {
            shared_hit_blocks: 11,
            shared_read_blocks: 2,
            ..PlanBuffers::default()
        };
        explain.plan.io_timing.io_read_time = 0.5;
        let options = RenderOptions::new(80)
            .with_theme(Theme::NoColor)
            .with_summary(SummaryStyle::Detailed);

        let rendered = render_explain(&explain, options);

        assert!(rendered.contains("○ Total Loops:"));
        assert!(rendered.contains("○ Total Nodes: 3"));
        assert!(rendered.contains("○ Buffers: shared hit=11"));
        assert!(rendered.contains("○ I/O Time:"));
    }

    #[test]
    fn verbose_render_mode_implies_detailed_summary() {
        let mut explain = sample_explain();
        explain.plan.buffers.shared_hit_blocks = 1;
        let options = RenderOptions::new(80)
            .with_theme(Theme::NoColor)
            .with_mode(RenderMode::Verbose);

        let rendered = render_explain(&explain, options);

        assert!(rendered.contains("○ Total Loops:"));
        assert!(rendered.contains("○ Total Nodes:"));
    }

    #[test]
    fn summary_style_parse_accepts_aliases() {
        assert_eq!(SummaryStyle::parse("compact"), Some(SummaryStyle::Compact));
        assert_eq!(SummaryStyle::parse("minimal"), Some(SummaryStyle::Compact));
        assert_eq!(
            SummaryStyle::parse("detailed"),
            Some(SummaryStyle::Detailed)
        );
        assert_eq!(SummaryStyle::parse("full"), Some(SummaryStyle::Detailed));
        assert_eq!(SummaryStyle::parse("crazy"), None);
    }

    #[test]
    fn ascii_tree_style_renders_pure_ascii_tree_glyphs() {
        let explain = sample_explain();
        let options = RenderOptions::new(80)
            .with_theme(Theme::NoColor)
            .with_tree_style(TreeStyle::ascii());

        let rendered = render_explain(&explain, options);

        assert!(!rendered.contains('│'));
        assert!(!rendered.contains('└'));
        assert!(!rendered.contains('├'));
        assert!(rendered.contains('|'));
        assert!(rendered.contains("+--"));
        assert!(rendered.contains("Hash Join"));
    }

    #[test]
    fn heavy_tree_style_uses_heavy_glyphs() {
        let explain = sample_explain();
        let options = RenderOptions::new(80)
            .with_theme(Theme::NoColor)
            .with_tree_style(TreeStyle::heavy());

        let rendered = render_explain(&explain, options);

        assert!(rendered.contains('┃'));
        assert!(rendered.contains('┗') || rendered.contains('┣'));
        assert!(rendered.contains("Hash Join"));
    }

    #[test]
    fn unicode_tree_style_remains_default() {
        let explain = sample_explain();
        let default = render_explain(&explain, RenderOptions::new(80).with_theme(Theme::NoColor));
        let unicode = render_explain(
            &explain,
            RenderOptions::new(80)
                .with_theme(Theme::NoColor)
                .with_tree_style(TreeStyle::unicode()),
        );
        assert_eq!(default, unicode);
    }
}
