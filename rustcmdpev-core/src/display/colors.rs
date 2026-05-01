use colored::*;

/// Visual theme controlling how semantic roles are mapped to ANSI styles.
///
/// `Dark` is the historical default tuned for dark terminals; `Light` swaps the
/// brightest foregrounds so labels remain legible on light backgrounds; and
/// `NoColor` returns raw text and is appropriate when ANSI styling has already
/// been disabled (or for plain-text snapshot tests).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    NoColor,
}

impl Theme {
    pub fn parse(name: &str) -> Option<Theme> {
        match name {
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            "no-color" | "none" | "plain" => Some(Theme::NoColor),
            _ => None,
        }
    }
}

/// Default-themed color formatting kept for backwards compatibility.
pub fn color_format<S: AsRef<str>>(s: S, format: &str) -> colored::ColoredString {
    themed_format(s, format, Theme::Dark)
}

/// Apply the named semantic role for the given theme.
pub fn themed_format<S: AsRef<str>>(s: S, format: &str, theme: Theme) -> colored::ColoredString {
    let text = s.as_ref();
    match theme {
        Theme::Dark => match format {
            "prefix" => text.bright_black(),
            "muted" => text.bright_black(),
            "bold" => text.bright_white(),
            "good" => text.green(),
            "warning" => text.yellow(),
            "critical" => text.red(),
            "output" => text.cyan(),
            "tag" => text.on_bright_red().bright_white(),
            _ => text.green(),
        },
        Theme::Light => match format {
            "prefix" => text.black(),
            "muted" => text.black(),
            "bold" => text.bold().blue(),
            "good" => text.green(),
            "warning" => text.magenta(),
            "critical" => text.red(),
            "output" => text.blue(),
            "tag" => text.on_red().white(),
            _ => text.green(),
        },
        Theme::NoColor => text.normal(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_theme_names() {
        assert_eq!(Theme::parse("dark"), Some(Theme::Dark));
        assert_eq!(Theme::parse("light"), Some(Theme::Light));
        assert_eq!(Theme::parse("no-color"), Some(Theme::NoColor));
        assert_eq!(Theme::parse("plain"), Some(Theme::NoColor));
        assert_eq!(Theme::parse("solarized"), None);
    }

    #[test]
    fn no_color_theme_returns_unstyled_text() {
        let styled = themed_format("hello", "bold", Theme::NoColor);
        assert!(styled.fgcolor.is_none());
        assert!(styled.bgcolor.is_none());
        assert_eq!(styled.to_string(), "hello");
    }

    #[test]
    fn light_theme_remaps_bold_role_away_from_bright_white() {
        let dark = themed_format("x", "bold", Theme::Dark);
        let light = themed_format("x", "bold", Theme::Light);
        assert_ne!(dark.fgcolor, light.fgcolor);
    }

    #[test]
    fn color_format_matches_dark_theme_for_compatibility() {
        let legacy = color_format("y", "muted");
        let dark = themed_format("y", "muted", Theme::Dark);
        assert_eq!(legacy.fgcolor, dark.fgcolor);
    }
}
