//! Terminal-oriented rendering helpers and formatting utilities.
//!
//! - [`colors`] applies ANSI styles by semantic role.
//! - [`format`] formats domain values (durations, tags, percentages) into strings.
//! - [`tree`] owns the tree-drawing glyphs and joint-selection logic.

pub mod colors;
pub mod format;
pub mod tree;
