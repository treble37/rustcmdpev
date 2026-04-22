use crate::structure::data::plan;
use crate::constants::{
    BAD_ESTIMATE_FACTOR_THRESHOLD, TAG_BAD_ESTIMATE, TAG_COSTLIEST, TAG_LARGEST, TAG_SLOWEST,
    TREE_OUTPUT_BRANCH, TREE_OUTPUT_CHILD, TREE_OUTPUT_CONTINUATION, TREE_OUTPUT_PADDING,
};
use colored::*;

pub fn duration_to_string(value: f64) -> colored::ColoredString {
    if value < 100.0 {
        format!("{0:.2} ms", value).green()
    } else if value < 1000.0 {
        format!("{0:.2} ms", value).yellow()
    } else if value < 60000.0 {
        format!("{0:.2} s", value / 2000.0).red()
    } else {
        format!("{0:.2} m", value / 60000.0).red()
    }
}

pub fn format_details(plan: plan::Plan) -> String {
    let mut details = vec![];

    if plan.scan_direction != "" {
        details.push(plan.scan_direction);
    }

    if plan.strategy != "" {
        details.push(plan.strategy);
    }

    if !details.is_empty() {
        return details.join(", ");
    }

    "".to_string()
}

pub fn format_tags(plan: plan::Plan) -> String {
    let mut tags = vec![];

    if plan.analysis_flags.slowest {
        tags.push(TAG_SLOWEST);
    }
    if plan.analysis_flags.costliest {
        tags.push(TAG_COSTLIEST);
    }
    if plan.analysis_flags.largest {
        tags.push(TAG_LARGEST);
    }
    if plan.analysis_flags.planner_row_estimate_factor >= BAD_ESTIMATE_FACTOR_THRESHOLD {
        tags.push(TAG_BAD_ESTIMATE);
    }
    tags.join(" ")
}

pub fn get_terminator(index: usize, plan: plan::Plan) -> String {
    if index == 0 {
        if plan.plans.is_empty() {
            TREE_OUTPUT_CHILD.to_string()
        } else {
            TREE_OUTPUT_BRANCH.to_string()
        }
    } else if plan.plans.is_empty() {
        TREE_OUTPUT_PADDING.to_string()
    } else {
        TREE_OUTPUT_CONTINUATION.to_string()
    }
}

pub fn format_percent(number: f64, precision: usize) -> String {
    return format!("{:.1$}%", number, precision);
}
