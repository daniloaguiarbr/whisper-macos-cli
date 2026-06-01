pub mod audio;
pub mod cli;
pub mod commands;
pub mod error;
pub mod language;
pub mod model;
pub mod output;
pub mod signal;
pub mod whisper;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::error::Error;
use crate::language::detect::resolve_language;

pub fn run() -> ExitCode {
    signal::reset_sigpipe();
    signal::install_ctrlc_handler();

    let cli = Cli::parse();

    init_tracing(cli.quiet, cli.verbose);

    whisper_rs::install_logging_hooks();

    if cli.print_schema {
        print_schema();
        return ExitCode::SUCCESS;
    }

    match cli.command {
        Some(Commands::Transcribe(args)) => run_transcribe(args),
        Some(Commands::Models { action }) => run_models(action),
        Some(Commands::Doctor) => run_doctor(),
        Some(Commands::Schema) => {
            print_schema();
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("No command provided. Use --help for usage.");
            ExitCode::from(64)
        }
    }
}

fn run_transcribe(args: cli::TranscribeArgs) -> ExitCode {
    let (language, language_source) = resolve_language(args.language.as_deref());
    tracing::info!(language, language_source, model = %args.model, "starting transcription");

    if args.files.is_empty() && is_terminal::is_terminal(std::io::stdin()) {
        let err = Error::NoInput;
        let _ = output::write_error(&err);
        return err.to_exit_code();
    }

    match commands::transcribe::run(&args, language, language_source) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => exit_with_error(e),
    }
}

fn run_models(action: cli::ModelsAction) -> ExitCode {
    match commands::models::run(&action) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => exit_with_error(e),
    }
}

fn run_doctor() -> ExitCode {
    match commands::doctor::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => exit_with_error(e),
    }
}

fn exit_with_error(e: Error) -> ExitCode {
    if let Error::Io(ref io_err) = e {
        if io_err.kind() == std::io::ErrorKind::BrokenPipe {
            return ExitCode::from(141);
        }
    }
    let _ = output::write_error(&e);
    e.to_exit_code()
}

fn print_schema() {
    let schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "TranscriptionResult",
        "type": "object",
        "properties": {
            "file": { "type": "string" },
            "language": { "type": "string" },
            "language_source": { "type": "string", "enum": ["cli", "whisper_auto", "os_locale"] },
            "model": { "type": "string" },
            "duration_seconds": { "type": "number" },
            "text": { "type": "string" },
            "segments": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "start": { "type": "number" },
                        "end": { "type": "number" },
                        "text": { "type": "string" }
                    },
                    "required": ["start", "end", "text"]
                }
            },
            "vad_chunks": { "type": "integer" },
            "processing_time_ms": { "type": "integer" }
        },
        "required": ["file", "language", "language_source", "model", "duration_seconds", "text", "vad_chunks", "processing_time_ms"]
    });
    let _ = output::write_schema(&schema);
}

fn init_tracing(quiet: bool, verbose: u8) {
    use tracing_subscriber::EnvFilter;

    if quiet {
        return;
    }

    let level = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();
}
