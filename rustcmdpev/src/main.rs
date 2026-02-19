use clap::{Parser, ValueEnum};
use colored::control;
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
        return fs::read_to_string(path)
            .map_err(|err| CliError::InputRead(format!("failed to read input file '{}': {err}", path.display())));
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

    let arr = parsed
        .as_array()
        .ok_or_else(|| CliError::ContractViolation("top-level JSON must be an array".to_string()))?;
    let first = arr
        .first()
        .ok_or_else(|| CliError::ContractViolation("top-level JSON array must contain at least one explain object".to_string()))?;
    let first_obj = first
        .as_object()
        .ok_or_else(|| CliError::ContractViolation("first explain entry must be a JSON object".to_string()))?;

    match first_obj.get("Plan") {
        Some(Value::Object(_)) => Ok(()),
        _ => Err(CliError::ContractViolation(
            "first explain object must contain 'Plan' object".to_string(),
        )),
    }
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
    if plan.analysis_flags.planner_row_estimate_factor >= 100.0 {
        tags.push("bad_estimate");
    }

    println!(
        "{}{} | {:.3} | {:.3} | {} | {}",
        indent,
        plan.node_type,
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

    match cli.format {
        OutputFormat::Pretty => {
            info!("rendering pretty output");
            validate_stdin_json_contract(&input)?;
            rustcmdpev_core::visualize(input, width).map_err(CliError::Core)?;
            Ok(())
        }
        OutputFormat::Json => {
            info!("rendering json output");
            validate_stdin_json_contract(&input)?;
            let explain = parse_and_process_explain(&input)?;
            let output = serde_json::to_string_pretty(&explain)
                .map_err(|err| CliError::OutputSerialization(format!("failed to serialize JSON output: {err}")))?;
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
