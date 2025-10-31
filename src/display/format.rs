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

    if plan.slowest {
        tags.push(" slowest ");
    }
    if plan.costliest {
        tags.push(" costliest ");
    }
    if plan.largest {
        tags.push(" largest ");
    }
    if plan.planner_row_estimate_factor >= 100.0 {
        tags.push(" bad estimate ");
    }
    tags.join(" ")
}

pub fn get_terminator(index: usize, plan: plan::Plan) -> String {
    if index == 0 {
        if plan.plans.is_empty() {
            "⌡► ".to_string()
        } else {
            "├►  ".to_string()
        }
    } else if plan.plans.is_empty() {
        "   ".to_string()
    } else {
        "│  ".to_string()
    }
}

pub fn format_percent(number: f64, precision: usize) -> String {
    return format!("{:.1$}%", number, precision);
}
