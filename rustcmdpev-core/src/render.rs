use std::fmt::Write;

use crate::constants::{
    DESCRIPTIONS, TREE_BRANCH, TREE_ELBOW, TREE_NODE_CONNECTOR, TREE_ROOT_MARKER, TREE_VERTICAL,
};
use crate::display::colors::color_format;
use crate::display::format::{duration_to_string, format_details, format_percent, format_tags, get_terminator};
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub width: usize,
}

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
        color_format(TREE_ROOT_MARKER.to_string(), "output")
    )
    .expect("write to string");
    write_plan(
        &mut buffer,
        explain,
        &explain.plan,
        String::new(),
        options.width,
        explain.plan.plans.len() == 1,
    );
    buffer
}

fn write_plan(
    buffer: &mut String,
    explain: &Explain,
    plan: &Plan,
    prefix: String,
    width: usize,
    last_child: bool,
) {
    let mut source_prefix = prefix.clone();
    let mut current_prefix = prefix;

    writeln!(
        buffer,
        "{}{}",
        color_format(source_prefix.clone(), "prefix"),
        color_format(TREE_VERTICAL.to_string(), "prefix")
    )
    .expect("write to string");

    let joint = if plan.plans.len() > 1 || last_child {
        TREE_ELBOW
    } else {
        TREE_BRANCH
    };

    writeln!(
        buffer,
        "{}{} {}{} {}",
        color_format(current_prefix.clone(), "prefix"),
        color_format(format!("{joint}{TREE_NODE_CONNECTOR}"), "prefix"),
        color_format(plan.node_type.clone(), "bold"),
        color_format(format_details(plan.clone()), "muted"),
        color_format(format_tags(plan.clone()), "tag")
    )
    .expect("write to string");

    if plan.plans.len() > 1 || last_child {
        source_prefix.push_str("  ");
    } else {
        source_prefix.push_str(&format!("{TREE_VERTICAL} "));
    }

    current_prefix = format!("{source_prefix}{TREE_VERTICAL} ");
    let cols = width.saturating_sub(current_prefix.len());

    for line in textwrap::fill(DESCRIPTIONS[plan.node_type.as_str()], cols)
        .split('\n')
        .collect::<Vec<_>>()
    {
        writeln!(
            buffer,
            "{}{}",
            color_format(current_prefix.clone(), "prefix"),
            color_format(line.to_string(), "muted")
        )
        .expect("write to string");
    }

    writeln!(
        buffer,
        "{}○ Duration: {} {}",
        color_format(current_prefix.clone(), "prefix"),
        duration_to_string(plan.actuals.actual_duration),
        format_percent((plan.actuals.actual_duration / explain.execution_time) * 100.0, 1)
    )
    .expect("write to string");
    writeln!(
        buffer,
        "{}○ Cost: {} {}",
        color_format(current_prefix.clone(), "prefix"),
        duration_to_string(plan.actuals.actual_cost),
        format_percent((plan.actuals.actual_cost / explain.total_cost) * 100.0, 1)
    )
    .expect("write to string");
    writeln!(
        buffer,
        "{}○ Rows: {}",
        color_format(current_prefix.clone(), "prefix"),
        plan.actuals.actual_rows
    )
    .expect("write to string");

    current_prefix.push_str("  ");

    if !plan.join_type.is_empty() {
        writeln!(
            buffer,
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("join".to_string(), "muted"),
            color_format(plan.join_type.clone(), "muted")
        )
        .expect("write to string");
    }

    if !plan.relation_name.is_empty() {
        writeln!(
            buffer,
            "{}{} {} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("on".to_string(), "muted"),
            color_format(plan.schema.clone(), "muted"),
            color_format(plan.relation_name.clone(), "muted")
        )
        .expect("write to string");
    }

    if !plan.index_name.is_empty() {
        writeln!(
            buffer,
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("using".to_string(), "muted"),
            plan.index_name
        )
        .expect("write to string");
    }

    if !plan.index_condition.is_empty() {
        writeln!(
            buffer,
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("condition".to_string(), "muted"),
            plan.index_condition
        )
        .expect("write to string");
    }

    if !plan.filter.is_empty() {
        writeln!(
            buffer,
            "{}{} {} [-{} rows]",
            color_format(current_prefix.clone(), "prefix"),
            color_format("filter".to_string(), "muted"),
            plan.filter,
            color_format(plan.rows_removed_by_filter.to_string(), "muted")
        )
        .expect("write to string");
    }

    if !plan.hash_condition.is_empty() {
        writeln!(
            buffer,
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("on".to_string(), "muted"),
            plan.hash_condition
        )
        .expect("write to string");
    }

    if !plan.cte_name.is_empty() {
        writeln!(
            buffer,
            "{}CTE {}",
            color_format(current_prefix.clone(), "prefix"),
            plan.cte_name
        )
        .expect("write to string");
    }

    if plan.analysis_flags.planner_row_estimate_factor != 0.0 {
        writeln!(
            buffer,
            "{}{} {}estimated {} {:.2}x",
            color_format(current_prefix.clone(), "prefix"),
            color_format("rows".to_string(), "muted"),
            plan.analysis_flags.planner_row_estimate_direction,
            color_format("by".to_string(), "muted"),
            plan.analysis_flags.planner_row_estimate_factor
        )
        .expect("write to string");
    }

    current_prefix = source_prefix.clone();

    if !plan.output.is_empty() {
        let joined_output = plan.output.join(" + ");
        let wrapped_output = textwrap::fill(&joined_output, cols);
        for (index, line) in wrapped_output.split('\n').enumerate() {
            writeln!(
                buffer,
                "{}{}",
                color_format(current_prefix.clone(), "prefix"),
                color_format(get_terminator(index, plan.clone()) + line, "output")
            )
            .expect("write to string");
        }
    }

    for (index, child_plan) in plan.plans.iter().enumerate() {
        write_plan(
            buffer,
            explain,
            child_plan,
            source_prefix.clone(),
            width,
            index == plan.plans.len() - 1,
        );
    }
}
