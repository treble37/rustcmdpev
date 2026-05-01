use crate::constants::{
    BAD_ESTIMATE_FACTOR_THRESHOLD, TAG_BAD_ESTIMATE, TAG_COSTLIEST, TAG_LARGEST, TAG_SLOWEST,
};
use crate::display::tree;
use crate::structure::data::plan;
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

    if !plan.scan_direction.is_empty() {
        details.push(plan.scan_direction);
    }

    if !plan.strategy.is_empty() {
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

/// Backwards-compatible shim that defers to [`tree::output_terminator`].
pub fn get_terminator(index: usize, plan: plan::Plan) -> String {
    tree::output_terminator(index, &plan).to_string()
}

pub fn format_percent(number: f64, precision: usize) -> String {
    format!("{:.1$}%", number, precision)
}
