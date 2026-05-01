use std::fmt::Write;

use crate::constants::{DESCRIPTIONS, TREE_NODE_CONNECTOR, TREE_ROOT_MARKER, TREE_VERTICAL};
use crate::display::colors::color_format;
use crate::display::format::{
    duration_to_string, format_details, format_percent, format_tags,
};
use crate::display::tree::{node_joint, output_terminator, prefix_continuation};
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;

/// Options controlling pretty-plan rendering.
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub width: usize,
}

/// Mutable context threaded through the recursive render. Holds the per-render
/// state that does not vary as the traversal descends so call sites stay focused
/// on the per-node arguments they actually care about.
struct RenderContext<'a> {
    buffer: &'a mut String,
    explain: &'a Explain,
    options: RenderOptions,
}

/// Per-node positional arguments grouped to keep recursion signatures small.
struct NodePosition {
    prefix: String,
    last_child: bool,
}

/// Render a processed explain tree into terminal-friendly text.
pub fn render_explain(explain: &Explain, options: RenderOptions) -> String {
    let mut buffer = String::new();
    writeln!(&mut buffer, "○ Total Cost {}", explain.total_cost).expect("write to string");
    writeln!(
        &mut buffer,
        "○ Planning Time: {}",
        duration_to_string(explain.planning_time)
    )
    .expect("write to string");
    writeln!(
        &mut buffer,
        "○ Execution Time: {}",
        duration_to_string(explain.execution_time)
    )
    .expect("write to string");
    writeln!(
        &mut buffer,
        "{}",
        color_format(TREE_ROOT_MARKER, "output")
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
    let NodePosition { prefix, last_child } = position;
    let mut source_prefix = prefix;

    writeln!(
        ctx.buffer,
        "{}{}",
        color_format(&source_prefix, "prefix"),
        color_format(TREE_VERTICAL, "prefix")
    )
    .expect("write to string");

    let joint = node_joint(plan.plans.len(), last_child);

    writeln!(
        ctx.buffer,
        "{}{} {}{} {}",
        color_format(&source_prefix, "prefix"),
        color_format(format!("{joint}{TREE_NODE_CONNECTOR}"), "prefix"),
        color_format(&plan.identity.node_type, "bold"),
        color_format(format_details(plan), "muted"),
        color_format(format_tags(plan), "tag")
    )
    .expect("write to string");

    let continuation = prefix_continuation(plan.plans.len(), last_child);
    if continuation == TREE_VERTICAL {
        source_prefix.push_str(TREE_VERTICAL);
        source_prefix.push(' ');
    } else {
        source_prefix.push_str(continuation);
    }

    let mut current_prefix = String::with_capacity(source_prefix.len() + 4);
    current_prefix.push_str(&source_prefix);
    current_prefix.push_str(TREE_VERTICAL);
    current_prefix.push(' ');
    let cols = width.saturating_sub(current_prefix.len());

    for line in textwrap::fill(DESCRIPTIONS[plan.identity.node_type.as_str()], cols).split('\n') {
        writeln!(
            ctx.buffer,
            "{}{}",
            color_format(&current_prefix, "prefix"),
            color_format(line, "muted")
        )
        .expect("write to string");
    }

    writeln!(
        ctx.buffer,
        "{}○ Duration: {} {}",
        color_format(&current_prefix, "prefix"),
        duration_to_string(plan.actuals.actual_duration),
        format_percent(
            (plan.actuals.actual_duration / explain.execution_time) * 100.0,
            1
        )
    )
    .expect("write to string");
    writeln!(
        ctx.buffer,
        "{}○ Cost: {} {}",
        color_format(&current_prefix, "prefix"),
        duration_to_string(plan.actuals.actual_cost),
        format_percent((plan.actuals.actual_cost / explain.total_cost) * 100.0, 1)
    )
    .expect("write to string");
    writeln!(
        ctx.buffer,
        "{}○ Rows: {}",
        color_format(&current_prefix, "prefix"),
        plan.actuals.actual_rows
    )
    .expect("write to string");

    current_prefix.push_str("  ");

    if !plan.identity.join_type.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            color_format(&current_prefix, "prefix"),
            color_format("join", "muted"),
            color_format(&plan.identity.join_type, "muted")
        )
        .expect("write to string");
    }

    if !plan.identity.relation_name.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {} {}",
            color_format(&current_prefix, "prefix"),
            color_format("on", "muted"),
            color_format(&plan.identity.schema, "muted"),
            color_format(&plan.identity.relation_name, "muted")
        )
        .expect("write to string");
    }

    if !plan.identity.index_name.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            color_format(&current_prefix, "prefix"),
            color_format("using", "muted"),
            plan.identity.index_name
        )
        .expect("write to string");
    }

    if !plan.predicates.index_condition.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            color_format(&current_prefix, "prefix"),
            color_format("condition", "muted"),
            plan.predicates.index_condition
        )
        .expect("write to string");
    }

    if !plan.predicates.filter.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {} [-{} rows]",
            color_format(&current_prefix, "prefix"),
            color_format("filter", "muted"),
            plan.predicates.filter,
            color_format(plan.predicates.rows_removed_by_filter.to_string(), "muted")
        )
        .expect("write to string");
    }

    if !plan.predicates.hash_condition.is_empty() {
        writeln!(
            ctx.buffer,
            "{}{} {}",
            color_format(&current_prefix, "prefix"),
            color_format("on", "muted"),
            plan.predicates.hash_condition
        )
        .expect("write to string");
    }

    if !plan.identity.cte_name.is_empty() {
        writeln!(
            ctx.buffer,
            "{}CTE {}",
            color_format(&current_prefix, "prefix"),
            plan.identity.cte_name
        )
        .expect("write to string");
    }

    if plan.analysis_flags.planner_row_estimate_factor != 0.0 {
        writeln!(
            ctx.buffer,
            "{}{} {}estimated {} {:.2}x",
            color_format(&current_prefix, "prefix"),
            color_format("rows", "muted"),
            plan.analysis_flags.planner_row_estimate_direction,
            color_format("by", "muted"),
            plan.analysis_flags.planner_row_estimate_factor
        )
        .expect("write to string");
    }

    if !plan.predicates.output.is_empty() {
        let joined_output = plan.predicates.output.join(" + ");
        let wrapped_output = textwrap::fill(&joined_output, cols);
        for (index, line) in wrapped_output.split('\n').enumerate() {
            writeln!(
                ctx.buffer,
                "{}{}{}",
                color_format(&source_prefix, "prefix"),
                color_format(output_terminator(index, plan), "output"),
                color_format(line, "output")
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

    #[test]
    fn render_context_threads_explain_and_options_into_recursion() {
        let mut root = child_plan("Hash Join");
        root.plans.push(child_plan("Seq Scan"));
        root.plans.push(child_plan("Index Scan"));
        let explain = Explain {
            execution_time: 5.0,
            total_cost: 4.0,
            plan: root,
            ..Explain::default()
        };

        let rendered = render_explain(&explain, RenderOptions { width: 80 });

        assert!(rendered.contains("Hash Join"));
        assert!(rendered.contains("Seq Scan"));
        assert!(rendered.contains("Index Scan"));
        assert!(rendered.contains("○ Total Cost"));
    }
}
