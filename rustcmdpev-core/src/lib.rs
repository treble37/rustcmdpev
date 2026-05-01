//! Core parsing, analysis, and rendering primitives for `rustcmdpev`.

use std::error::Error;
use std::fmt;

pub mod analysis;
pub mod constants;
pub mod display;
pub mod parser;
pub mod render;
pub mod structure;
pub mod summary;

use structure::data::explain::Explain;

#[derive(Debug)]
pub enum VisualizeError {
    InvalidJson(serde_json::Error),
    EmptyExplainArray,
    MissingPlan,
    InvalidPlan(String),
}

impl fmt::Display for VisualizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VisualizeError::InvalidJson(err) => write!(f, "invalid JSON input: {err}"),
            VisualizeError::EmptyExplainArray => {
                write!(
                    f,
                    "top-level JSON array must contain at least one explain object"
                )
            }
            VisualizeError::MissingPlan => {
                write!(f, "first explain object must contain 'Plan' object")
            }
            VisualizeError::InvalidPlan(message) => write!(f, "{message}"),
        }
    }
}

impl Error for VisualizeError {}

/// Apply analysis passes to a validated explain document.
pub fn analyze_explain(explain: Explain) -> Explain {
    analysis::process_all(explain)
}

/// Parse raw input and run the full validation and analysis pipeline.
pub fn parse_and_process(input: &str) -> Result<Explain, VisualizeError> {
    let explain = parser::parse_explain_document(input)?;
    Ok(analyze_explain(explain))
}

/// Produce pretty rendered output without performing stdout I/O.
pub fn render_visualization(input: &str, width: usize) -> Result<String, VisualizeError> {
    render_visualization_with(input, render::RenderOptions::new(width))
}

/// Produce pretty rendered output using fully customised render options.
pub fn render_visualization_with(
    input: &str,
    options: render::RenderOptions,
) -> Result<String, VisualizeError> {
    let explain = parse_and_process(input)?;
    Ok(render::render_explain(&explain, options))
}

/// Legacy convenience entry point that returns the analyzed explain.
pub fn visualize(input: String, width: usize) -> Result<Explain, VisualizeError> {
    let explain = parse_and_process(input.as_str())?;
    let _rendered = render::render_explain(&explain, render::RenderOptions::new(width));
    Ok(explain)
}
