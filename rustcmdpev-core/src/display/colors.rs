use colored::*;

pub fn color_format<S: AsRef<str>>(s: S, format: &str) -> colored::ColoredString {
    let text = s.as_ref();
    match format {
        "prefix" => text.bright_black(),
        "muted" => text.bright_black(),
        "bold" => text.bright_white(),
        "good" => text.green(),
        "warning" => text.yellow(),
        "critical" => text.red(),
        "output" => text.cyan(),
        "tag" => text.on_bright_red().bright_white(),
        _ => text.green(),
    }
}
