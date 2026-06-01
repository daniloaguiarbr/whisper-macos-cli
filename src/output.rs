use serde::Serialize;
use std::io::{self, BufWriter, Write};

#[derive(Serialize)]
pub struct TranscriptionResult {
    pub file: String,
    pub language: String,
    pub language_source: String,
    pub model: String,
    pub duration_seconds: f64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segment>>,
    pub vad_chunks: usize,
    pub processing_time_ms: u128,
}

#[derive(Serialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Serialize)]
pub struct ErrorOutput {
    pub error: bool,
    pub code: u8,
    pub message: String,
    pub category: String,
    pub retryable: bool,
}

pub fn write_json(result: &TranscriptionResult) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, result)?;
    stdout.write_all(
        b"
",
    )?;
    stdout.flush()
}

pub fn write_ndjson(result: &TranscriptionResult) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, result)?;
    stdout.write_all(
        b"
",
    )?;
    stdout.flush()
}

pub fn write_stdout_line(text: &str) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    stdout.write_all(text.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

pub fn write_schema(schema: &serde_json::Value) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer_pretty(&mut stdout, schema)?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

pub fn write_error(err: &crate::error::Error) -> io::Result<()> {
    let output = ErrorOutput {
        error: true,
        code: err.exit_code(),
        message: err.to_string(),
        category: match err {
            crate::error::Error::NoInput => "usage",
            crate::error::Error::InputNotFound { .. } => "input",
            crate::error::Error::AudioDecode(_) | crate::error::Error::UnsupportedFormat { .. } => {
                "data"
            }
            crate::error::Error::ModelNotFound { .. } | crate::error::Error::Config(_) => "config",
            crate::error::Error::ModelDownload(_) | crate::error::Error::UnsupportedPlatform => {
                "service"
            }
            crate::error::Error::WhisperInference(_) => "internal",
            crate::error::Error::Io(_) => "io",
        }
        .to_string(),
        retryable: matches!(err, crate::error::Error::ModelDownload(_)),
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, &output)?;
    stdout.write_all(
        b"
",
    )?;
    stdout.flush()
}
