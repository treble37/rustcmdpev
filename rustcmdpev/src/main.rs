use clap::{Parser, ValueEnum};
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

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
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short = 'q', long)]
    quiet: bool,
}

fn read_input(input: Option<&PathBuf>) -> Result<String, String> {
    if let Some(path) = input {
        return fs::read_to_string(path)
            .map_err(|err| format!("failed to read input file '{}': {err}", path.display()));
    }

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|err| format!("failed to read stdin: {err}"))?;
    if buffer.trim().is_empty() {
        return Err(
            "no input provided; pass JSON via stdin or --input <PATH> (try --help)".to_string(),
        );
    }
    Ok(buffer)
}

fn validate_stdin_json_contract(input: &str) -> Result<(), String> {
    let parsed: Value =
        serde_json::from_str(input).map_err(|err| format!("invalid JSON input: {err}"))?;

    let arr = parsed
        .as_array()
        .ok_or_else(|| "top-level JSON must be an array".to_string())?;
    let first = arr
        .first()
        .ok_or_else(|| "top-level JSON array must contain at least one explain object".to_string())?;
    let first_obj = first
        .as_object()
        .ok_or_else(|| "first explain entry must be a JSON object".to_string())?;

    match first_obj.get("Plan") {
        Some(Value::Object(_)) => Ok(()),
        _ => Err("first explain object must contain 'Plan' object".to_string()),
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let input = read_input(cli.input.as_ref())?;

    // Color/compat/verbosity parsing is now in place for parity; rendering behavior
    // will be fully wired in a follow-up change.
    let _ = (cli.color, cli.verbose, cli.quiet);

    if cli.compat && cli.format != OutputFormat::Pretty {
        return Err("--compat currently supports only --format pretty".to_string());
    }

    let width = match (cli.compat, cli.width) {
        (true, Some(60)) | (true, None) => 60,
        (true, Some(_)) => {
            return Err("--compat requires --width 60 for parity-target output".to_string());
        }
        (false, Some(width)) => width,
        (false, None) => 60,
    };

    match cli.format {
        OutputFormat::Pretty => {
            validate_stdin_json_contract(&input)?;
            rustcmdpev_core::visualize(input, width);
            Ok(())
        }
        OutputFormat::Json => Err(
            "--format json is parsed but not implemented yet; use --format pretty".to_string(),
        ),
        OutputFormat::Table => Err(
            "--format table is parsed but not implemented yet; use --format pretty".to_string(),
        ),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}
