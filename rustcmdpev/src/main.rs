use clap::{Parser, ValueEnum};
use colored::control;
use rustcmdpev_core::constants::{BAD_ESTIMATE_FACTOR_THRESHOLD, MAX_PLAN_DEPTH, MAX_PLAN_NODES};
use rustcmdpev_core::display::colors::Theme;
use rustcmdpev_core::render::{RenderMode, RenderOptions, SummaryStyle};
use rustcmdpev_core::structure::data::explain::Explain;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;
use std::process::ExitCode;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
    Table,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum CliTheme {
    Dark,
    Light,
    NoColor,
}

impl From<CliTheme> for Theme {
    fn from(theme: CliTheme) -> Theme {
        match theme {
            CliTheme::Dark => Theme::Dark,
            CliTheme::Light => Theme::Light,
            CliTheme::NoColor => Theme::NoColor,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum CliRenderMode {
    Default,
    Condensed,
    Verbose,
}

impl From<CliRenderMode> for RenderMode {
    fn from(mode: CliRenderMode) -> RenderMode {
        match mode {
            CliRenderMode::Default => RenderMode::Default,
            CliRenderMode::Condensed => RenderMode::Condensed,
            CliRenderMode::Verbose => RenderMode::Verbose,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum CliSummary {
    Compact,
    Detailed,
}

impl From<CliSummary> for SummaryStyle {
    fn from(s: CliSummary) -> SummaryStyle {
        match s {
            CliSummary::Compact => SummaryStyle::Compact,
            CliSummary::Detailed => SummaryStyle::Detailed,
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "rustcmdpev",
    about = "Visualize PostgreSQL EXPLAIN JSON output",
    version
)]
struct Cli {
    #[arg(long, short, value_name = "PATH")]
    input: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
    format: OutputFormat,
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    color: ColorMode,
    #[arg(long, value_enum, default_value_t = CliTheme::Dark)]
    theme: CliTheme,
    #[arg(long = "render-mode", value_enum, default_value_t = CliRenderMode::Default)]
    render_mode: CliRenderMode,
    #[arg(long, value_enum, default_value_t = CliSummary::Compact)]
    summary: CliSummary,
    #[arg(long)]
    width: Option<usize>,
    #[arg(long)]
    compat: bool,
    #[arg(short = 'v', long, action = clap::ArgAction::Count, conflicts_with = "quiet")]
    verbose: u8,
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    quiet: bool,
}

#[derive(Debug)]
enum CliError {
    InputRead(String),
    ContractViolation(String),
    InvalidInput(String),
    InvalidCompatibility(String),
    OutputSerialization(String),
    Core(rustcmdpev_core::VisualizeError),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InputRead(msg)
            | CliError::ContractViolation(msg)
            | CliError::InvalidInput(msg)
            | CliError::InvalidCompatibility(msg)
            | CliError::OutputSerialization(msg) => write!(f, "{msg}"),
            CliError::Core(err) => write!(f, "{err}"),
        }
    }
}

impl CliError {
    fn exit_code(&self) -> u8 {
        match self {
            CliError::InputRead(_) => 2,
            CliError::ContractViolation(_) | CliError::InvalidInput(_) => 3,
            CliError::InvalidCompatibility(_) => 4,
            CliError::OutputSerialization(_) => 5,
            CliError::Core(_) => 6,
        }
    }
}

fn init_logging(verbose: u8, quiet: bool) {
    let default_level = if quiet {
        "error"
    } else {
        match verbose {
            0 => "warn",
            1 => "info",
            _ => "debug",
        }
    };

    let filter = match EnvFilter::try_from_default_env() {
        Ok(f) => f,
        Err(_) => EnvFilter::new(default_level),
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .try_init();
}

fn read_input(input: Option<&PathBuf>) -> Result<String, CliError> {
    if let Some(path) = input {
        info!(path = %path.display(), "reading input from file");
        return fs::read_to_string(path).map_err(|err| {
            CliError::InputRead(format!(
                "failed to read input file '{}': {err}",
                path.display()
            ))
        });
    }

    info!("reading input from stdin");
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|err| CliError::InputRead(format!("failed to read stdin: {err}")))?;
    if buffer.trim().is_empty() {
        return Err(CliError::InputRead(
            "no input provided; pass JSON via stdin or --input <PATH> (try --help)".to_string(),
        ));
    }
    Ok(buffer)
}

fn validate_stdin_json_contract(input: &str) -> Result<(), CliError> {
    debug!("validating stdin JSON contract");
    let parsed: Value = serde_json::from_str(input).map_err(|err| {
        CliError::InvalidInput(format!(
            "invalid JSON input: {err}. Ensure input is a PostgreSQL EXPLAIN FORMAT JSON array."
        ))
    })?;

    let arr = parsed.as_array().ok_or_else(|| {
        CliError::ContractViolation("top-level JSON must be an array".to_string())
    })?;
    let first = arr.first().ok_or_else(|| {
        CliError::ContractViolation(
            "top-level JSON array must contain at least one explain object".to_string(),
        )
    })?;
    let first_obj = first.as_object().ok_or_else(|| {
        CliError::ContractViolation("first explain entry must be a JSON object".to_string())
    })?;

    validate_optional_non_negative_number(first_obj, "Planning Time", "$[0]")?;
    validate_optional_non_negative_number(first_obj, "Execution Time", "$[0]")?;

    match first_obj.get("Plan") {
        Some(Value::Object(plan)) => {
            let mut node_count = 0;
            validate_plan_node(plan, "$[0].Plan", 0, &mut node_count)
        }
        _ => Err(CliError::ContractViolation(
            "first explain object must contain 'Plan' object".to_string(),
        )),
    }
}

fn validate_optional_non_negative_number(
    obj: &serde_json::Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<(), CliError> {
    if let Some(value) = obj.get(key) {
        let n = value.as_f64().ok_or_else(|| {
            CliError::ContractViolation(format!("{path}.{key} must be a number if present"))
        })?;
        if n < 0.0 {
            return Err(CliError::ContractViolation(format!(
                "{path}.{key} must be non-negative"
            )));
        }
    }
    Ok(())
}

fn validate_optional_non_negative_u64(
    obj: &serde_json::Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<(), CliError> {
    if let Some(value) = obj.get(key) {
        if value.as_u64().is_none() {
            return Err(CliError::ContractViolation(format!(
                "{path}.{key} must be a non-negative integer if present"
            )));
        }
    }
    Ok(())
}

fn validate_plan_node(
    plan: &serde_json::Map<String, Value>,
    path: &str,
    depth: usize,
    node_count: &mut usize,
) -> Result<(), CliError> {
    const NON_NEGATIVE_FLOAT_FIELDS: &[&str] = &[
        "Startup Cost",
        "Total Cost",
        "Actual Cost",
        "Actual Duration",
        "Actual Startup Time",
        "Actual Total Time",
        "I/O Read Time",
        "I/O Write Time",
    ];
    const NON_NEGATIVE_INT_FIELDS: &[&str] = &[
        "Actual Loops",
        "Actual Rows",
        "Plan Rows",
        "Plan Width",
        "Rows Removed By Filter",
        "Rows Removed By Index Recheck",
        "Shared Dirtied Blocks",
        "Shared Hit Blocks",
        "Shared Read Blocks",
        "Shared Written Blocks",
        "Local Dirtied Blocks",
        "Local Hit Blocks",
        "Local Read Blocks",
        "Local Written Blocks",
        "Temp Read Blocks",
        "Temp Written Blocks",
        "Heap Fetches",
    ];

    if depth > MAX_PLAN_DEPTH {
        return Err(CliError::ContractViolation(format!(
            "{path} exceeds maximum supported plan depth of {MAX_PLAN_DEPTH}"
        )));
    }
    *node_count += 1;
    if *node_count > MAX_PLAN_NODES {
        return Err(CliError::ContractViolation(format!(
            "plan exceeds maximum supported node count of {MAX_PLAN_NODES}"
        )));
    }

    for field in NON_NEGATIVE_FLOAT_FIELDS {
        validate_optional_non_negative_number(plan, field, path)?;
    }
    for field in NON_NEGATIVE_INT_FIELDS {
        validate_optional_non_negative_u64(plan, field, path)?;
    }

    if let Some(children) = plan.get("Plans") {
        let child_arr = children.as_array().ok_or_else(|| {
            CliError::ContractViolation(format!("{path}.Plans must be an array if present"))
        })?;
        for (idx, child) in child_arr.iter().enumerate() {
            let child_obj = child.as_object().ok_or_else(|| {
                CliError::ContractViolation(format!("{path}.Plans[{idx}] must be an object"))
            })?;
            validate_plan_node(
                child_obj,
                &format!("{path}.Plans[{idx}]"),
                depth + 1,
                node_count,
            )?;
        }
    }

    Ok(())
}

fn configure_color(mode: ColorMode) {
    let use_color = match mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => {
            let no_color = env::var_os("NO_COLOR").is_some();
            !no_color && io::stdout().is_terminal()
        }
    };
    control::set_override(use_color);
}

fn parse_and_process_explain(input: &str) -> Result<Explain, CliError> {
    debug!("parsing and processing explain payload");
    rustcmdpev_core::parse_and_process(input).map_err(CliError::Core)
}

fn write_table(explain: &Explain) {
    println!("NODE | DURATION_MS | COST | ROWS | TAGS");
    println!("-----|-------------|------|------|-----");
    write_table_plan(&explain.plan, 0);
}

fn write_table_plan(plan: &rustcmdpev_core::structure::data::plan::Plan, depth: usize) {
    let indent = "  ".repeat(depth);
    let mut tags: Vec<&str> = Vec::new();
    if plan.analysis_flags.slowest {
        tags.push("slowest");
    }
    if plan.analysis_flags.costliest {
        tags.push("costliest");
    }
    if plan.analysis_flags.largest {
        tags.push("largest");
    }
    if plan.analysis_flags.planner_row_estimate_factor >= BAD_ESTIMATE_FACTOR_THRESHOLD {
        tags.push("bad_estimate");
    }

    println!(
        "{}{} | {:.3} | {:.3} | {} | {}",
        indent,
        plan.identity.node_type,
        plan.actuals.actual_duration,
        plan.actuals.actual_cost,
        plan.actuals.actual_rows,
        tags.join(",")
    );

    for child in &plan.plans {
        write_table_plan(child, depth + 1);
    }
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    init_logging(cli.verbose, cli.quiet);
    info!(
        format = ?cli.format,
        color = ?cli.color,
        compat = cli.compat,
        width = ?cli.width,
        verbose = cli.verbose,
        quiet = cli.quiet,
        "starting rustcmdpev"
    );
    let input = read_input(cli.input.as_ref())?;

    configure_color(cli.color);

    if cli.compat && cli.format != OutputFormat::Pretty {
        return Err(CliError::InvalidCompatibility(
            "--compat currently supports only --format pretty".to_string(),
        ));
    }
    if cli.compat && cli.render_mode != CliRenderMode::Default {
        return Err(CliError::InvalidCompatibility(
            "--compat requires the default render mode for parity-target output".to_string(),
        ));
    }
    if cli.compat && cli.summary != CliSummary::Compact {
        return Err(CliError::InvalidCompatibility(
            "--compat requires the compact summary style for parity-target output".to_string(),
        ));
    }

    let width = match (cli.compat, cli.width) {
        (true, Some(60)) | (true, None) => 60,
        (true, Some(_)) => {
            return Err(CliError::InvalidCompatibility(
                "--compat requires --width 60 for parity-target output".to_string(),
            ));
        }
        (false, Some(width)) => width,
        (false, None) => 60,
    };
    debug!(width, "resolved render width");

    let render_options = RenderOptions::new(width)
        .with_theme(Theme::from(cli.theme))
        .with_mode(RenderMode::from(cli.render_mode))
        .with_summary(SummaryStyle::from(cli.summary));

    match cli.format {
        OutputFormat::Pretty => {
            info!("rendering pretty output");
            validate_stdin_json_contract(&input)?;
            print!(
                "{}",
                rustcmdpev_core::render_visualization_with(&input, render_options)
                    .map_err(CliError::Core)?
            );
            Ok(())
        }
        OutputFormat::Json => {
            info!("rendering json output");
            validate_stdin_json_contract(&input)?;
            let explain = parse_and_process_explain(&input)?;
            let output = serde_json::to_string_pretty(&explain).map_err(|err| {
                CliError::OutputSerialization(format!("failed to serialize JSON output: {err}"))
            })?;
            println!("{output}");
            Ok(())
        }
        OutputFormat::Table => {
            info!("rendering table output");
            validate_stdin_json_contract(&input)?;
            let explain = parse_and_process_explain(&input)?;
            write_table(&explain);
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(err.exit_code())
        }
    }
}
