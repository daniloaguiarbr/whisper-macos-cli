use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

use sha2::{Digest, Sha256};

const USER_AGENT: &str = concat!(
    "whisper-macos-cli/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("TARGET"),
    ")"
);
const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF_MS: u64 = 500;
const MAX_BACKOFF_MS: u64 = 30_000;

pub fn download_model(
    url: &str,
    dest: &Path,
    expected_size: u64,
    min_size: u64,
) -> Result<Option<String>, crate::error::Error> {
    tracing::info!(url, dest = %dest.display(), expected_size, "downloading model");

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(3600))
        .build()
        .map_err(|e| crate::error::Error::ModelDownload(anyhow::anyhow!("http client: {e}")))?;

    let mut backoff = INITIAL_BACKOFF_MS;
    let mut last_err: Option<crate::error::Error> = None;

    for attempt in 1..=MAX_RETRIES {
        match try_download(&client, url, dest, expected_size, min_size) {
            Ok(etag) => return Ok(etag),
            Err(e) => {
                let transient = is_transient(&e);
                tracing::warn!(
                    attempt,
                    max = MAX_RETRIES,
                    transient,
                    error = %e,
                    "download attempt failed"
                );
                if !transient || attempt == MAX_RETRIES {
                    return Err(e);
                }
                last_err = Some(e);
                let jitter = fastrand_u64() % (backoff / 2 + 1);
                let sleep_ms = backoff + jitter;
                tracing::info!(sleep_ms, "backing off before retry");
                std::thread::sleep(Duration::from_millis(sleep_ms));
                backoff = (backoff * 2).min(MAX_BACKOFF_MS);
            }
        }
    }

    Err(last_err.unwrap_or_else(|| {
        crate::error::Error::ModelDownload(anyhow::anyhow!("download failed after retries"))
    }))
}

fn try_download(
    client: &reqwest::blocking::Client,
    url: &str,
    dest: &Path,
    expected_size: u64,
    min_size: u64,
) -> Result<Option<String>, crate::error::Error> {
    let mut response = client
        .get(url)
        .send()
        .map_err(|e| crate::error::Error::ModelDownload(anyhow::anyhow!("download start: {e}")))?;

    if !response.status().is_success() {
        return Err(crate::error::Error::ModelDownload(anyhow::anyhow!(
            "HTTP {}",
            response.status()
        )));
    }

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_matches('"').to_string());

    let content_length = response.content_length().unwrap_or(expected_size);

    let bar = indicatif::ProgressBar::with_draw_target(
        Some(content_length),
        indicatif::ProgressDrawTarget::stderr(),
    );
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )
            .expect("static template")
            .progress_chars("#>-"),
    );

    let temp = dest.with_extension("bin.tmp");

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(crate::error::Error::Io)?;
    }

    let mut file = match fs::File::create(&temp) {
        Ok(f) => f,
        Err(e) => {
            bar.finish_and_clear();
            return Err(crate::error::Error::Io(e));
        }
    };
    let mut hasher = Sha256::new();
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];

    loop {
        if crate::signal::is_shutdown_requested() {
            drop(file);
            bar.finish_and_clear();
            let _ = fs::remove_file(&temp);
            return Err(crate::error::Error::ModelDownload(anyhow::anyhow!(
                "download interrupted by signal"
            )));
        }

        let n = match response.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => {
                drop(file);
                bar.finish_and_clear();
                let _ = fs::remove_file(&temp);
                return Err(crate::error::Error::ModelDownload(anyhow::anyhow!(
                    "read: {e}"
                )));
            }
        };

        if let Err(e) = file.write_all(&buffer[..n]) {
            drop(file);
            bar.finish_and_clear();
            let _ = fs::remove_file(&temp);
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return Err(crate::error::Error::ModelDownload(anyhow::anyhow!(
                    "BrokenPipe during write"
                )));
            }
            return Err(crate::error::Error::Io(e));
        }
        hasher.update(&buffer[..n]);
        downloaded += n as u64;
        bar.set_position(downloaded);
    }

    bar.finish_with_message("Download complete");

    if let Err(e) = file.sync_all() {
        drop(file);
        let _ = fs::remove_file(&temp);
        return Err(crate::error::Error::Io(e));
    }
    drop(file);

    if downloaded < min_size {
        let _ = fs::remove_file(&temp);
        return Err(crate::error::Error::ModelDownload(anyhow::anyhow!(
            "downloaded {downloaded} bytes is below minimum {min_size} — likely partial"
        )));
    }

    if let Err(e) = fs::rename(&temp, dest) {
        let _ = fs::remove_file(&temp);
        return Err(crate::error::Error::Io(e));
    }

    let digest = hex::encode(hasher.finalize());
    tracing::info!(
        bytes = downloaded,
        sha256 = %digest,
        etag = etag.as_deref().unwrap_or("none"),
        "model downloaded"
    );

    Ok(etag)
}

fn is_transient(err: &crate::error::Error) -> bool {
    let s = err.to_string();
    s.contains("download start")
        || s.contains("HTTP 5")
        || s.contains("HTTP 429")
        || s.contains("HTTP 408")
        || s.contains("connection")
        || s.contains("timeout")
        || s.contains("timed out")
}

fn fastrand_u64() -> u64 {
    use std::cell::Cell;
    use std::num::Wrapping;
    thread_local!(static SEED: Cell<Wrapping<u64>> = const { Cell::new(Wrapping(0x9E3779B97F4A7C15)) });
    SEED.with(|s| {
        let mut x = s.get();
        x = (x ^ (x << 13)) ^ ((x ^ (x << 13)) >> 7) ^ ((x ^ (x << 13)) << 17);
        s.set(x);
        x.0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_agent_includes_version_and_target() {
        assert!(USER_AGENT.starts_with("whisper-macos-cli/"));
        assert!(USER_AGENT.contains(env!("TARGET")));
    }
}
