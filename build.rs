use std::process::Command;

const AFTER_HELP: &str = "\
EXAMPLES:
  whisper-macos-cli transcribe voice.ogg
  whisper-macos-cli transcribe --model base --language pt audio.mp3
  whisper-macos-cli transcribe --timestamps --ndjson *.ogg
  cat audio.wav | whisper-macos-cli transcribe
  whisper-macos-cli models download base
  whisper-macos-cli doctor

ENVIRONMENT:
  WHISPER_MODEL       Override default model (e.g. base, small, medium)
  NO_COLOR            Disable colored output (see https://no-color.org)
  RUST_LOG            Override tracing log level filter

EXIT STATUS:
  0     Success
  2     Usage error (invalid arguments)
  64    No input provided
  65    Invalid input data (corrupt audio, unsupported format)
  66    Input file not found
  69    Service unavailable (download failed, unsupported platform)
  70    Internal error (whisper inference failed)
  74    I/O error
  78    Configuration error (model not found)
  130   Interrupted (SIGINT / Ctrl+C)
  141   Broken pipe (SIGPIPE)

FILES:
  ~/Library/Application Support/whisper-macos-cli/models/
      Downloaded Whisper model files (ggml-*.bin)

SEE ALSO:
  Project:  https://github.com/daniloaguiarbr/whisper-macos-cli
  whisper.cpp: https://github.com/ggerganov/whisper.cpp

BUGS:
  Report bugs at https://github.com/daniloaguiarbr/whisper-macos-cli/issues";

fn main() {
    set_git_env();
    generate_manpages();
}

fn set_git_env() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");

    let sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_SHA={}", sha.trim());
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap_or_default()
    );

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| {
            let secs = d.as_secs();
            let days = secs / 86400;
            let years = 1970 + days / 365;
            let remaining = (days % 365) as u32;
            let months = remaining / 30 + 1;
            let day = remaining % 30 + 1;
            format!("{years}-{months:02}-{day:02}")
        })
        .unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_DATE={now}");
}

fn generate_manpages() {
    let out_dir = match std::env::var_os("OUT_DIR") {
        Some(dir) => std::path::PathBuf::from(dir),
        None => return,
    };

    let man_dir = out_dir.join("man");
    std::fs::create_dir_all(&man_dir).ok();

    let cmd = build_command();
    clap_mangen::generate_to(cmd, &man_dir).ok();
}

fn build_command() -> clap::Command {
    use clap::Arg;

    clap::Command::new("whisper-macos-cli")
        .about("macOS-exclusive audio transcription CLI via whisper.cpp with Metal GPU")
        .long_about(
            "Transcribes audio files to text using OpenAI Whisper models with Apple \
             Silicon Metal GPU acceleration. Outputs structured JSON to stdout for \
             seamless integration with AI agents and Unix pipelines.",
        )
        .version(env!("CARGO_PKG_VERSION"))
        .author("Danilo Teixeira")
        .after_help(AFTER_HELP)
        .subcommand(build_transcribe_command())
        .subcommand(build_models_command())
        .subcommand(clap::Command::new("doctor").about("Check system environment"))
        .subcommand(clap::Command::new("schema").about("Print JSON schema of output"))
        .subcommand(
            clap::Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    Arg::new("shell")
                        .help("Shell to generate completions for")
                        .required(true),
                ),
        )
}

fn build_transcribe_command() -> clap::Command {
    use clap::{Arg, ArgAction};

    clap::Command::new("transcribe")
        .about("Transcribe audio files to text")
        .arg(
            Arg::new("files")
                .help("Audio files to transcribe (reads stdin if omitted and not a TTY)")
                .num_args(0..),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANG")
                .help("Language for transcription (e.g. pt, en, es, auto)"),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Whisper model to use [tiny, base, small, medium, large-v3]")
                .default_value("large-v3"),
        )
        .arg(
            Arg::new("beam-size")
                .long("beam-size")
                .value_name("N")
                .help("Beam size for BeamSearch decoding [1-16]")
                .default_value("8"),
        )
        .arg(
            Arg::new("timestamps")
                .long("timestamps")
                .help("Include timestamped segments in output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ndjson")
                .long("ndjson")
                .help("Emit NDJSON (one JSON object per line per file)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("vad-threshold")
                .long("vad-threshold")
                .value_name("FLOAT")
                .help("VAD threshold [0.0-1.0]")
                .default_value("0.5"),
        )
        .arg(
            Arg::new("concurrency")
                .long("concurrency")
                .value_name("N")
                .help("Maximum parallel transcriptions [1-32]")
                .default_value("2"),
        )
        .arg(
            Arg::new("input-format")
                .long("input-format")
                .value_name("FMT")
                .help("Force input audio format (ogg, mp3, wav, flac)"),
        )
}

fn build_models_command() -> clap::Command {
    clap::Command::new("models")
        .about("Manage Whisper models")
        .subcommand(
            clap::Command::new("download")
                .about("Download a model")
                .arg(
                    clap::Arg::new("model")
                        .value_name("MODEL")
                        .help("Model name (default: large-v3)"),
                ),
        )
        .subcommand(clap::Command::new("list").about("List available and downloaded models"))
        .subcommand(
            clap::Command::new("path")
                .about("Show model file path")
                .arg(
                    clap::Arg::new("model")
                        .value_name("MODEL")
                        .help("Model name (default: large-v3)"),
                ),
        )
        .subcommand(
            clap::Command::new("remove")
                .about("Remove a downloaded model")
                .arg(
                    clap::Arg::new("model")
                        .value_name("MODEL")
                        .help("Model name to remove")
                        .required(true),
                ),
        )
}
