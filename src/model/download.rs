use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub fn download_model(
    url: &str,
    dest: &Path,
    expected_size: u64,
) -> Result<(), crate::error::Error> {
    tracing::info!(url, dest = %dest.display(), "downloading model");

    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .map_err(|e| crate::error::Error::ModelDownload(anyhow::anyhow!("http client: {e}")))?;

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

    let total = response.content_length().unwrap_or(expected_size);

    // Progress bar writes to stderr only — stdout is reserved for JSON output
    let bar = indicatif::ProgressBar::with_draw_target(
        Some(total),
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

    // Download to temp file first, then rename atomically
    let temp = dest.with_extension("bin.tmp");

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(crate::error::Error::Io)?;
    }

    let mut file = fs::File::create(&temp).map_err(crate::error::Error::Io)?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let n = response
            .read(&mut buffer)
            .map_err(|e| crate::error::Error::ModelDownload(anyhow::anyhow!("read: {e}")))?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])
            .map_err(crate::error::Error::Io)?;
        downloaded += n as u64;
        bar.set_position(downloaded);
    }

    bar.finish_with_message("Download complete");

    // Atomic rename: temp -> dest
    fs::rename(&temp, dest).map_err(crate::error::Error::Io)?;

    tracing::info!(bytes = downloaded, "model downloaded");
    Ok(())
}
