use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

pub fn long_version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("GIT_SHA"),
        " ",
        env!("TARGET"),
        ")"
    )
}

#[derive(Parser)]
#[command(
    name = "whisper-macos-cli",
    about = "macOS-exclusive audio transcription CLI via whisper.cpp with Metal GPU",
    version,
    long_version = long_version(),
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, global = true, help = "Suppress stderr output")]
    pub quiet: bool,

    #[arg(short, long, global = true, action = clap::ArgAction::Count, help = "Increase verbosity")]
    pub verbose: u8,

    #[arg(long, global = true, help = "Print JSON schema and exit")]
    pub print_schema: bool,
}

#[derive(Subcommand)]
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
    /// Print JSON schema of output
    Schema,
}

#[derive(Args)]
pub struct TranscribeArgs {
    /// Audio files to transcribe (reads stdin if omitted and not a TTY)
    pub files: Vec<PathBuf>,

    /// Language for transcription (e.g. pt, en, es, auto)
    #[arg(short, long)]
    pub language: Option<String>,

    /// Whisper model to use
    #[arg(short, long, default_value = "large-v3")]
    pub model: String,

    /// Beam size for BeamSearch decoding
    #[arg(long, default_value_t = 8)]
    pub beam_size: i32,

    /// Include timestamped segments in output
    #[arg(long)]
    pub timestamps: bool,

    /// Emit NDJSON (one JSON object per line per file)
    #[arg(long)]
    pub ndjson: bool,

    /// VAD threshold [0.0-1.0]
    #[arg(long, default_value_t = 0.5)]
    pub vad_threshold: f32,

    /// Maximum parallel transcriptions
    #[arg(long, default_value_t = 2)]
    pub concurrency: usize,

    /// Force input audio format (ogg, mp3, wav, flac)
    #[arg(long)]
    pub input_format: Option<String>,
}

#[derive(Subcommand)]
pub enum ModelsAction {
    /// Download a model
    Download {
        /// Model name (default: large-v3)
        model: Option<String>,
    },
    /// List available and downloaded models
    List,
    /// Show model file path
    Path {
        /// Model name (default: large-v3)
        model: Option<String>,
    },
    /// Remove a downloaded model
    Remove {
        /// Model name to remove
        model: String,
    },
}
