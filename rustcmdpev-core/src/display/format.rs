use crate::constants::{
    BAD_ESTIMATE_FACTOR_THRESHOLD, TAG_BAD_ESTIMATE, TAG_COSTLIEST, TAG_LARGEST, TAG_SLOWEST,
};
use crate::display::colors::{themed_format, Theme};
use crate::display::tree;
use crate::structure::data::plan;

pub fn duration_to_string(value: f64) -> colored::ColoredString {
    duration_to_string_themed(value, Theme::Dark)
}

pub fn duration_to_string_themed(value: f64, theme: Theme) -> colored::ColoredString {
    let (text, role) = if value < 100.0 {
        (format!("{0:.2} ms", value), "good")
    } else if value < 1000.0 {
        (format!("{0:.2} ms", value), "warning")
    } else if value < 60000.0 {
        (format!("{0:.2} s", value / 2000.0), "critical")
    } else {
        (format!("{0:.2} m", value / 60000.0), "critical")
    };
    themed_format(text, role, theme)
}

pub fn format_details(plan: &plan::Plan) -> String {
    let mut details: Vec<&str> = Vec::new();

    if !plan.identity.scan_direction.is_empty() {
        details.push(plan.identity.scan_direction.as_str());
    }

    if !plan.identity.strategy.is_empty() {
        details.push(plan.identity.strategy.as_str());
    }

    if details.is_empty() {
        return String::new();
    }

    details.join(", ")
}

pub fn format_tags(plan: &plan::Plan) -> String {
    let mut tags: Vec<&str> = Vec::new();

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

/// Reference-based variant that avoids cloning the plan in hot render paths.
pub fn output_terminator(index: usize, plan: &plan::Plan) -> &'static str {
    tree::output_terminator(index, plan)
}

pub fn format_percent(number: f64, precision: usize) -> String {
    format!("{:.1$}%", number, precision)
}
