//! Core parsing, analysis, and rendering primitives for `rustcmdpev`.

use std::error::Error;
use std::fmt;

pub mod analysis;
pub mod constants;
pub mod display;
pub mod parser;
pub mod render;
pub mod structure;

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
                write!(f, "top-level JSON array must contain at least one explain object")
            }
            VisualizeError::MissingPlan => write!(f, "first explain object must contain 'Plan' object"),
            VisualizeError::InvalidPlan(message) => write!(f, "{message}"),
        }
    }
}

impl Error for VisualizeError {}

pub fn parse_and_process(input: &str) -> Result<Explain, VisualizeError> {
    let explain = parser::parse_explain_document(input)?;
    Ok(analysis::process_all(explain))
}

pub fn visualize(input: String, width: usize) -> Result<Explain, VisualizeError> {
    let explain = parse_and_process(input.as_str())?;
    let rendered = render::render_explain(&explain, render::RenderOptions { width });
    print!("{rendered}");
    Ok(explain)
}
