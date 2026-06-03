pub mod audio;
pub mod cli;
pub mod commands;
pub mod error;
pub mod language;
pub mod model;
pub mod output;
pub mod signal;
pub mod video;
pub mod whisper;

use std::io::Write;
use std::process::ExitCode;
use std::time::Duration;

use clap::CommandFactory;
use clap::Parser;
use uuid::Uuid;

use crate::cli::{Cli, Commands, CommandsFormat};
use crate::error::Error;
use crate::language::detect::resolve_language;

const DOCTOR_TIMEOUT_SECS: u64 = 5;
const DEFAULT_INPUT_TIMEOUT_SECS: u64 = 30;

pub fn run() -> ExitCode {
    signal::reset_sigpipe();
    signal::install_handlers();

    let cli = Cli::parse();
    let correlation_id = generate_correlation_id();

    init_tracing(cli.quiet, cli.verbose);

    if cli.print_schema {
        print_schema_envelope(&correlation_id);
        return ExitCode::SUCCESS;
    }

    if cli.print_config {
        print_config_envelope(&cli, &correlation_id);
        return ExitCode::SUCCESS;
    }

    match cli.command {
        Some(Commands::Transcribe(args)) => run_transcribe(args, cli.no_input, &correlation_id),
        Some(Commands::Models { action }) => run_models(action, &correlation_id),
        Some(Commands::Doctor) => run_doctor(&correlation_id),
        Some(Commands::Schema) => {
            print_schema_envelope(&correlation_id);
            ExitCode::SUCCESS
        }
        Some(Commands::Config) => {
            print_config_envelope(&cli, &correlation_id);
            ExitCode::SUCCESS
        }
        Some(Commands::Completions { shell }) => {
            run_completions(shell);
            ExitCode::SUCCESS
        }
        Some(Commands::Commands { format }) => {
            run_commands_tree(format, &correlation_id);
            ExitCode::SUCCESS
        }
        Some(Commands::Init { target }) => run_init(&target, &correlation_id),
        Some(Commands::Licenses) => run_licenses(&correlation_id),
        Some(Commands::Resume { workflow_id }) => {
            tracing::info!(workflow_id, "resume not yet supported in v0.1");
            let value = serde_json::json!({
                "schema_version": env!("CARGO_PKG_VERSION"),
                "correlation_id": correlation_id,
                "resume_supported": false,
                "workflow_id": workflow_id,
                "hint": "v0.1 does not persist checkpoints; see --dry-run for input validation",
            });
            let _ = output::write_json_value(&value);
            ExitCode::SUCCESS
        }
        None => {
            let err = Error::NoInput;
            let _ = output::write_error(&err, &correlation_id);
            err.to_exit_code()
        }
    }
}

fn generate_correlation_id() -> String {
    Uuid::now_v7().to_string()
}

fn run_transcribe(args: cli::TranscribeArgs, no_input: bool, correlation_id: &str) -> ExitCode {
    let (language, language_source) = resolve_language(args.language.as_deref());
    tracing::info!(language, language_source, model = %args.model, "starting transcription");

    let effective_no_input = no_input || is_ci();

    if args.files.is_empty() && (effective_no_input || is_terminal::is_terminal(std::io::stdin())) {
        let err = Error::NoInput;
        let _ = output::write_error(&err, correlation_id);
        return err.to_exit_code();
    }

    match commands::transcribe::run(&args, language, language_source, correlation_id) {
        Ok(()) => {
            if signal::is_shutdown_requested() {
                ExitCode::from(signal::shutdown_signal_exit_code())
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => exit_with_error(e, correlation_id),
    }
}

fn run_models(action: cli::ModelsAction, correlation_id: &str) -> ExitCode {
    match commands::models::run(&action, correlation_id) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => exit_with_error(e, correlation_id),
    }
}

fn run_completions(shell: clap_complete::Shell) {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "whisper-macos-cli", &mut std::io::stdout());
}

fn run_doctor(correlation_id: &str) -> ExitCode {
    match commands::doctor::run(correlation_id) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => exit_with_error(e, correlation_id),
    }
}

fn run_commands_tree(format: CommandsFormat, correlation_id: &str) -> ExitCode {
    let cmd = Cli::command();
    let name = cmd.get_name().to_string();
    let about = cmd.get_about().map(|s| s.to_string()).unwrap_or_default();
    let version = cmd.get_version().unwrap_or("unknown").to_string();

    let subcommands: Vec<serde_json::Value> = cmd
        .get_subcommands()
        .map(|sc| {
            serde_json::json!({
                "name": sc.get_name(),
                "about": sc.get_about().map(|a| a.to_string()).unwrap_or_default(),
                "subcommands": collect_subs(sc),
            })
        })
        .collect();

    let tree = serde_json::json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "name": name,
        "about": about,
        "version": version,
        "subcommands": subcommands,
    });

    let result = match format {
        CommandsFormat::Json => {
            let _ = output::write_json_value(&tree);
            Ok(())
        }
        CommandsFormat::Yaml => {
            tracing::warn!("YAML output not yet supported; emitting JSON");
            let _ = output::write_json_value(&tree);
            Ok(())
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::from(74),
    }
}

fn collect_subs(cmd: &clap::Command) -> Vec<serde_json::Value> {
    cmd.get_subcommands()
        .map(|sc| {
            serde_json::json!({
                "name": sc.get_name(),
                "about": sc.get_about().map(|a| a.to_string()).unwrap_or_default(),
                "subcommands": collect_subs(sc),
            })
        })
        .collect()
}

fn run_init(target: &std::path::Path, correlation_id: &str) -> ExitCode {
    let skill_path = target.join("SKILL.md");
    let agents_path = target.join("AGENTS.md");

    let skill_content = format!(
        "---\nname: whisper-macos-cli\nversion: {ver}\ndescription: Transcribe audio via whisper.cpp on macOS Apple Silicon\n---\n\n# whisper-macos-cli\n\nSee https://github.com/daniloaguiarbr/whisper-macos-cli for full documentation.\n",
        ver = env!("CARGO_PKG_VERSION")
    );

    let agents_content = format!(
        "# Agent Integration Guide\n\nGenerated by whisper-macos-cli v{ver}.\n\n## Quickstart\n\n```bash\nwhisper-macos-cli transcribe audio.ogg\n```\n\nSee `whisper-macos-cli schema` for the JSON Schema contract.\n",
        ver = env!("CARGO_PKG_VERSION")
    );

    let result: Result<(), std::io::Error> = (|| {
        std::fs::write(&skill_path, skill_content)?;
        std::fs::write(&agents_path, agents_content)?;
        Ok(())
    })();

    let value = match &result {
        Ok(()) => serde_json::json!({
            "schema_version": env!("CARGO_PKG_VERSION"),
            "correlation_id": correlation_id,
            "action": "initialized",
            "skill": skill_path.display().to_string(),
            "agents": agents_path.display().to_string(),
        }),
        Err(e) => serde_json::json!({
            "schema_version": env!("CARGO_PKG_VERSION"),
            "correlation_id": correlation_id,
            "error": true,
            "message": e.to_string(),
        }),
    };
    let _ = output::write_json_value(&value);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::from(74),
    }
}

fn run_licenses(correlation_id: &str) -> ExitCode {
    let value = serde_json::json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "license": "MIT",
        "third_party_notice": "Run `cargo about generate about.hbs > THIRD-PARTY-LICENSES.md` to regenerate the full attribution report.",
        "key_dependencies": [
            {"name": "whisper.cpp", "license": "MIT", "url": "https://github.com/ggml-org/whisper.cpp"},
            {"name": "symphonia", "license": "MPL-2.0", "url": "https://github.com/pdeljanov/symphonia"},
            {"name": "clap", "license": "MIT OR Apache-2.0", "url": "https://github.com/clap-rs/clap"},
            {"name": "serde", "license": "MIT OR Apache-2.0", "url": "https://github.com/serde-rs/serde"},
        ]
    });
    let _ = output::write_json_value(&value);
    ExitCode::SUCCESS
}

fn exit_with_error(e: Error, correlation_id: &str) -> ExitCode {
    if let Error::Io(ref io_err) = e {
        if io_err.kind() == std::io::ErrorKind::BrokenPipe {
            let _ = std::io::stdout().flush();
            return ExitCode::from(141);
        }
    }
    let _ = output::write_error(&e, correlation_id);
    e.to_exit_code()
}

fn is_ci() -> bool {
    matches!(
        std::env::var("CI").ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes")
    )
}

fn print_schema_envelope(correlation_id: &str) {
    let result_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "TranscriptionResult",
        "type": "object",
        "properties": {
            "schema_version": { "type": "string" },
            "correlation_id": { "type": "string" },
            "file": { "type": "string" },
            "language": { "type": "string" },
            "language_source": { "type": "string", "enum": ["cli", "whisper_auto", "os_locale"] },
            "model": { "type": "string" },
            "duration_seconds": { "type": "number" },
            "text": { "type": "string" },
            "segments": { "type": "array" },
            "vad_chunks": { "type": "integer" },
            "processing_time_ms": { "type": "integer" }
        },
        "required": ["schema_version", "correlation_id", "file", "language", "model", "duration_seconds", "text", "vad_chunks", "processing_time_ms"]
    });

    let envelope = serde_json::json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "agentNotes": "whisper-macos-cli emits a single JSON object per invocation. Use schema_version to gate downstream consumers. correlation_id is a UUID v7 generated per process invocation. text is NFC-normalized.",
        "invariants": [
            "stdout is always valid JSON or NDJSON",
            "stderr is always human-readable logs (suppressed with --quiet)",
            "exit codes follow sysexits.h convention",
            "large-v3 is the default model",
            "OGG/Opus (WhatsApp voice messages) is supported natively",
            "output is reproducible given same input and same model"
        ],
        "sideEffects": [
            "may download a model file on first use (~75MB to ~3GB)",
            "may write to ~/Library/Application Support/whisper-macos-cli/models/",
            "may load ~3GB into unified memory on Apple Silicon"
        ],
        "idempotent": true,
        "checkpointable": false,
        "tokenBudget": {
            "invocation_overhead": 200,
            "per_file_transcription": "50 + transcribed text length"
        },
        "result_schema": result_schema,
        "error_schema": {
            "type": "object",
            "required": ["schema_version", "error", "code", "message", "category", "retryable", "docs_url", "correlation_id"],
            "properties": {
                "schema_version": { "type": "string" },
                "error": { "type": "boolean" },
                "code": { "type": "integer" },
                "message": { "type": "string" },
                "category": { "type": "string", "enum": ["usage", "input", "data", "config", "service", "internal", "io"] },
                "retryable": { "type": "boolean" },
                "retry_after_ms": { "type": ["integer", "null"] },
                "hint": { "type": ["string", "null"] },
                "docs_url": { "type": "string" },
                "correlation_id": { "type": "string" }
            }
        }
    });
    let _ = output::write_schema(&envelope);
}

fn print_config_envelope(cli: &Cli, correlation_id: &str) {
    let value = serde_json::json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "config": {
            "quiet": cli.quiet,
            "verbose": cli.verbose,
            "no_input": cli.no_input,
            "ci_mode": is_ci(),
            "color": format!("{:?}", cli.color),
        }
    });
    let _ = output::write_json_value(&value);
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

    if is_terminal::is_terminal(std::io::stderr()) {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .with_target(false)
            .try_init();
    } else {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .with_target(false)
            .json()
            .try_init();
    }
}

#[doc(hidden)]
pub const _DOCTOR_TIMEOUT: Duration = Duration::from_secs(DOCTOR_TIMEOUT_SECS);
#[doc(hidden)]
pub const _DEFAULT_INPUT_TIMEOUT: Duration = Duration::from_secs(DEFAULT_INPUT_TIMEOUT_SECS);
