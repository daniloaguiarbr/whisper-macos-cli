use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum, ValueHint};

pub fn long_version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("GIT_SHA"),
        " ",
        env!("BUILD_DATE"),
        " ",
        env!("TARGET"),
        ")"
    )
}

/// Transcribe audio files locally on Apple Silicon via whisper.cpp with Metal GPU.
///
/// Emits structured JSON to stdout for AI agent integration and Unix pipelines.
/// Stdin/stdout contract — stderr reserved for logs.
#[derive(Debug, Parser)]
#[command(
    name = "whisper-macos-cli",
    version,
    long_version = long_version(),
    propagate_version = true,
    arg_required_else_help = true,
    max_term_width = 100,
    after_help = "\
EXAMPLES:
  whisper-macos-cli transcribe voice.ogg
  whisper-macos-cli transcribe --model base --language pt audio.mp3
  whisper-macos-cli transcribe --timestamps --ndjson *.ogg
  cat audio.wav | whisper-macos-cli transcribe
  whisper-macos-cli models download base
  whisper-macos-cli doctor
  whisper-macos-cli commands --format json

ENVIRONMENT:
  WHISPER_MODEL       Override default model (e.g. base, small, medium)
  WHISPER_LANGUAGE    Override default language (e.g. pt, en, es, auto)
  NO_COLOR            Disable colored output (see https://no-color.org)
  CI                  Disable all interactive prompts when set to true
  RUST_LOG            Override tracing log level filter
  SOURCE_DATE_EPOCH   Unix timestamp for reproducible builds

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
  143   Terminated (SIGTERM)

FILES:
  ~/Library/Application Support/whisper-macos-cli/models/
      Downloaded Whisper model files (ggml-*.bin)

SEE ALSO:
  Project:  https://github.com/daniloaguiarbr/whisper-macos-cli
  whisper.cpp: https://github.com/ggml-org/whisper.cpp

BUGS:
  Report bugs at https://github.com/daniloaguiarbr/whisper-macos-cli/issues"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Suppress stderr output
    #[arg(long, global = true, env = "QUIET")]
    pub quiet: bool,

    /// Increase verbosity (-v info, -vv debug, -vvv trace)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Print JSON schema of the output envelope and exit
    #[arg(long, global = true)]
    pub print_schema: bool,

    /// Print the current effective configuration as JSON and exit
    #[arg(long, global = true)]
    pub print_config: bool,

    /// Disable interactive fallbacks (for agent/script use; honored when CI=true)
    #[arg(long, global = true, env = "NO_INPUT")]
    pub no_input: bool,

    /// Control colored output
    #[arg(long, global = true, value_name = "WHEN", default_value = "auto")]
    pub color: ColorChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Transcribe audio files to text
    Transcribe(TranscribeArgs),
    /// Manage Whisper models
    Models {
        #[command(subcommand)]
        action: ModelsAction,
    },
    /// Check system environment
    Doctor,
    /// Print JSON schema of output envelope
    Schema,
    /// Print current effective configuration as JSON
    Config,
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_name = "SHELL")]
        shell: clap_complete::Shell,
    },
    /// Emit the full command tree as JSON for agent discovery
    Commands {
        /// Output format
        #[arg(long, value_name = "FMT", default_value = "json")]
        format: CommandsFormat,
    },
    /// Generate a starter SKILL.md and AGENTS.md scaffolding for downstream agents
    Init {
        /// Target directory (defaults to current)
        #[arg(long, value_name = "DIR", default_value = ".")]
        target: PathBuf,
    },
    /// Print third-party license attribution
    Licenses,
    /// Resume a previously interrupted batch by workflow_id (no-op for v0.1)
    Resume {
        /// Workflow id from a prior interrupted run
        #[arg(value_name = "WORKFLOW_ID")]
        workflow_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CommandsFormat {
    Json,
    Yaml,
}

#[derive(Debug, Args)]
pub struct TranscribeArgs {
    /// Audio files to transcribe (reads stdin if omitted and not a TTY)
    #[arg(value_hint = ValueHint::FilePath)]
    pub files: Vec<PathBuf>,

    /// Language for transcription (e.g. pt, en, es, auto)
    #[arg(
        short,
        long,
        value_name = "LANG",
        env = "WHISPER_LANGUAGE",
        help_heading = "Transcription"
    )]
    pub language: Option<String>,

    /// Whisper model to use
    #[arg(
        short,
        long,
        value_name = "MODEL",
        env = "WHISPER_MODEL",
        default_value = "large-v3",
        help_heading = "Transcription"
    )]
    pub model: WhisperModel,

    /// Beam size for BeamSearch decoding [1-16]
    #[arg(long, value_name = "N", default_value_t = 8, value_parser = parse_beam_size, help_heading = "Transcription")]
    pub beam_size: i32,

    /// Include timestamped segments in output
    #[arg(long, help_heading = "Output")]
    pub timestamps: bool,

    /// Emit NDJSON (one JSON object per line per file)
    #[arg(long, help_heading = "Output", conflicts_with = "output_format")]
    pub ndjson: bool,

    /// Output format
    #[arg(long, value_name = "FMT", help_heading = "Output")]
    pub output_format: Option<OutputFormat>,

    /// VAD threshold [0.0-1.0]
    #[arg(long, value_name = "FLOAT", default_value_t = 0.5, value_parser = parse_vad_threshold, help_heading = "Transcription")]
    pub vad_threshold: f32,

    /// Maximum parallel transcriptions [1-32]
    #[arg(long, value_name = "N", default_value_t = 2, value_parser = parse_concurrency, help_heading = "Transcription")]
    pub concurrency: usize,

    /// Force input audio format (ogg, mp3, wav, flac)
    #[arg(long, value_name = "FMT", help_heading = "Input")]
    pub input_format: Option<String>,

    /// Resolve inputs and exit without transcribing
    #[arg(long, help_heading = "Execution")]
    pub dry_run: bool,

    /// Per-attempt request timeout in seconds [1-3600]
    #[arg(long, value_name = "SECS", value_parser = parse_timeout_secs, help_heading = "Execution")]
    pub timeout: Option<u64>,

    /// Total retry attempts for transient errors [0-10]
    #[arg(long, value_name = "N", value_parser = parse_retry_count, help_heading = "Execution")]
    pub retry_count: Option<u32>,

    /// Total elapsed time budget for retries in seconds [1-3600]
    #[arg(long, value_name = "SECS", value_parser = parse_retry_elapsed, help_heading = "Execution")]
    pub retry_max_elapsed: Option<u64>,

    /// Fail fast in air-gapped environments without network connectivity
    #[arg(long, help_heading = "Execution")]
    pub offline: bool,

    /// Resume a previously interrupted batch (no-op for v0.1; reserved)
    #[arg(long, value_name = "WORKFLOW_ID", help_heading = "Execution")]
    pub resume: Option<String>,
}

impl TranscribeArgs {
    pub fn is_ndjson(&self) -> bool {
        self.ndjson || matches!(self.output_format, Some(OutputFormat::Ndjson))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Ndjson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum WhisperModel {
    Tiny,
    Base,
    Small,
    Medium,
    #[value(name = "large-v3")]
    LargeV3,
}

impl WhisperModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Base => "base",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::LargeV3 => "large-v3",
        }
    }
}

impl std::fmt::Display for WhisperModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Subcommand)]
pub enum ModelsAction {
    /// Download a model
    Download {
        /// Model name (default: large-v3)
        #[arg(value_name = "MODEL")]
        model: Option<WhisperModel>,
    },
    /// List available and downloaded models
    List,
    /// Show model file path
    Path {
        /// Model name (default: large-v3)
        #[arg(value_name = "MODEL")]
        model: Option<WhisperModel>,
    },
    /// Remove a downloaded model
    Remove {
        /// Model name to remove
        #[arg(value_name = "MODEL")]
        model: WhisperModel,
        /// Show what would be removed without deleting
        #[arg(long)]
        dry_run: bool,
    },
}

fn parse_beam_size(s: &str) -> Result<i32, String> {
    let val: i32 = s.parse().map_err(|e| format!("invalid integer: {e}"))?;
    if !(1..=16).contains(&val) {
        return Err(format!("beam size must be between 1 and 16, got {val}"));
    }
    Ok(val)
}

fn parse_vad_threshold(s: &str) -> Result<f32, String> {
    let val: f32 = s.parse().map_err(|e| format!("invalid float: {e}"))?;
    if !(0.0..=1.0).contains(&val) {
        return Err(format!(
            "VAD threshold must be between 0.0 and 1.0, got {val}"
        ));
    }
    Ok(val)
}

fn parse_concurrency(s: &str) -> Result<usize, String> {
    let val: usize = s.parse().map_err(|e| format!("invalid integer: {e}"))?;
    if !(1..=32).contains(&val) {
        return Err(format!("concurrency must be between 1 and 32, got {val}"));
    }
    Ok(val)
}

fn parse_timeout_secs(s: &str) -> Result<u64, String> {
    let val: u64 = s.parse().map_err(|e| format!("invalid integer: {e}"))?;
    if !(1..=3600).contains(&val) {
        return Err(format!(
            "timeout must be between 1 and 3600 seconds, got {val}"
        ));
    }
    Ok(val)
}

fn parse_retry_count(s: &str) -> Result<u32, String> {
    let val: u32 = s.parse().map_err(|e| format!("invalid integer: {e}"))?;
    if val > 10 {
        return Err(format!("retry count must be between 0 and 10, got {val}"));
    }
    Ok(val)
}

fn parse_retry_elapsed(s: &str) -> Result<u64, String> {
    let val: u64 = s.parse().map_err(|e| format!("invalid integer: {e}"))?;
    if !(1..=3600).contains(&val) {
        return Err(format!(
            "retry max elapsed must be between 1 and 3600 seconds, got {val}"
        ));
    }
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_debug_assert() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_beam_size_accepts_boundaries() {
        assert_eq!(parse_beam_size("1").unwrap(), 1);
        assert_eq!(parse_beam_size("8").unwrap(), 8);
        assert_eq!(parse_beam_size("16").unwrap(), 16);
    }

    #[test]
    fn parse_beam_size_rejects_below_range() {
        assert!(parse_beam_size("0").is_err());
        assert!(parse_beam_size("-1").is_err());
    }

    #[test]
    fn parse_beam_size_rejects_above_range() {
        assert!(parse_beam_size("17").is_err());
        assert!(parse_beam_size("100").is_err());
    }

    #[test]
    fn parse_beam_size_rejects_non_integer() {
        assert!(parse_beam_size("abc").is_err());
        assert!(parse_beam_size("1.5").is_err());
        assert!(parse_beam_size("").is_err());
    }

    #[test]
    fn parse_vad_threshold_accepts_boundaries() {
        assert!((parse_vad_threshold("0.0").unwrap() - 0.0).abs() < f32::EPSILON);
        assert!((parse_vad_threshold("0.5").unwrap() - 0.5).abs() < f32::EPSILON);
        assert!((parse_vad_threshold("1.0").unwrap() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn parse_vad_threshold_rejects_out_of_range() {
        assert!(parse_vad_threshold("-0.1").is_err());
        assert!(parse_vad_threshold("1.5").is_err());
        assert!(parse_vad_threshold("2.0").is_err());
    }

    #[test]
    fn parse_vad_threshold_rejects_non_float() {
        assert!(parse_vad_threshold("abc").is_err());
        assert!(parse_vad_threshold("").is_err());
    }

    #[test]
    fn parse_concurrency_accepts_boundaries() {
        assert_eq!(parse_concurrency("1").unwrap(), 1);
        assert_eq!(parse_concurrency("16").unwrap(), 16);
        assert_eq!(parse_concurrency("32").unwrap(), 32);
    }

    #[test]
    fn parse_concurrency_rejects_below_range() {
        assert!(parse_concurrency("0").is_err());
    }

    #[test]
    fn parse_concurrency_rejects_above_range() {
        assert!(parse_concurrency("33").is_err());
        assert!(parse_concurrency("1000").is_err());
    }

    #[test]
    fn parse_timeout_secs_accepts_valid_range() {
        assert_eq!(parse_timeout_secs("1").unwrap(), 1);
        assert_eq!(parse_timeout_secs("3600").unwrap(), 3600);
    }

    #[test]
    fn parse_timeout_secs_rejects_out_of_range() {
        assert!(parse_timeout_secs("0").is_err());
        assert!(parse_timeout_secs("3601").is_err());
    }

    #[test]
    fn parse_retry_count_accepts_max() {
        assert_eq!(parse_retry_count("0").unwrap(), 0);
        assert_eq!(parse_retry_count("10").unwrap(), 10);
    }

    #[test]
    fn parse_retry_count_rejects_above_max() {
        assert!(parse_retry_count("11").is_err());
        assert!(parse_retry_count("100").is_err());
    }
}
