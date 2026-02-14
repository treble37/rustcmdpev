use colored::*;

pub fn color_format(s: String, format: &str) -> colored::ColoredString {
    match format {
        "prefix" => s.bright_black(),
        "muted" => s.bright_black(),
        "bold" => s.bright_white(),
        "good" => s.green(),
        "warning" => s.yellow(),
        "critical" => s.red(),
        "output" => s.cyan(),
        "tag" => s.on_bright_red().bright_white(),
        _ => s.green(),
    }
}
