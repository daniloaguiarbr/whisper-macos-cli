use serde::Serialize;
use serde_json::json;
use std::io::{self, BufWriter, Write};

pub const SCHEMA_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
pub struct TranscriptionResult {
    pub schema_version: &'static str,
    pub correlation_id: String,
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
    pub schema_version: &'static str,
    pub error: bool,
    pub code: u8,
    pub message: String,
    pub category: String,
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    pub docs_url: &'static str,
    pub correlation_id: String,
}

pub fn write_json(result: &TranscriptionResult) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, result)?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

pub fn write_ndjson(result: &TranscriptionResult) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, result)?;
    stdout.write_all(b"\n")?;
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

pub fn write_error(err: &crate::error::Error, correlation_id: &str) -> io::Result<()> {
    let output = ErrorOutput {
        schema_version: SCHEMA_VERSION,
        error: true,
        code: err.exit_code(),
        message: err.to_string(),
        category: err.category().to_string(),
        retryable: err.retryable(),
        retry_after_ms: err.retry_after_ms(),
        hint: err.hint().map(String::from),
        docs_url: err.docs_url(),
        correlation_id: correlation_id.to_string(),
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, &output)?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

pub fn write_models_json(models: &[serde_json::Value], correlation_id: &str) -> io::Result<()> {
    let envelope = json!({
        "schema_version": SCHEMA_VERSION,
        "correlation_id": correlation_id,
        "models": models,
    });
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, &envelope)?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

pub fn write_json_value(value: &serde_json::Value) -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, value)?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

pub fn write_model_status(
    name: &str,
    action: &str,
    path: Option<&str>,
    correlation_id: &str,
) -> io::Result<()> {
    let value = json!({
        "schema_version": SCHEMA_VERSION,
        "correlation_id": correlation_id,
        "model": name,
        "action": action,
        "path": path,
    });
    write_json_value(&value)
}
