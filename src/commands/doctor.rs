use std::time::Duration;

use serde_json::json;

use crate::error::Error;
use crate::model::{registry, storage};
use crate::output;

const NETWORK_PROBE_HOST: &str = "https://huggingface.co";
const NETWORK_PROBE_TIMEOUT: Duration = Duration::from_secs(3);

pub fn run(correlation_id: &str) -> Result<(), Error> {
    let mut checks = Vec::new();

    check_macos(&mut checks);
    check_arch(&mut checks);
    check_command("cmake", &["--version"], "brew install cmake", &mut checks);
    check_command(
        "clang",
        &["--version"],
        "xcode-select --install",
        &mut checks,
    );
    check_xcode(&mut checks);
    check_model(&mut checks);
    check_network(&mut checks);
    check_disk_space(&mut checks);

    let all_ok = checks.iter().all(|c| c["status"] == "ok");

    let result = json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "checks": checks,
        "all_ok": all_ok,
    });
    output::write_json_value(&result).map_err(Error::Io)?;

    if all_ok {
        Ok(())
    } else {
        Err(Error::Config("system check failed".into()))
    }
}

fn check_macos(checks: &mut Vec<serde_json::Value>) {
    if cfg!(target_os = "macos") {
        checks.push(json!({"name": "macOS", "status": "ok", "detail": "macOS detected"}));
    } else {
        checks.push(json!({"name": "macOS", "status": "fail", "detail": "not macOS — Metal GPU unavailable"}));
    }
}

fn check_arch(checks: &mut Vec<serde_json::Value>) {
    if cfg!(target_arch = "aarch64") {
        checks.push(json!({"name": "Apple Silicon", "status": "ok", "detail": "aarch64"}));
    } else {
        let arch = std::env::consts::ARCH;
        checks.push(json!({
            "name": "Apple Silicon",
            "status": "fail",
            "detail": format!("{arch} — Metal GPU requires Apple Silicon"),
        }));
    }
}

fn check_command(cmd: &str, args: &[&str], hint: &str, checks: &mut Vec<serde_json::Value>) {
    match std::process::Command::new(cmd).args(args).output() {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let version = stdout.lines().next().unwrap_or("").trim().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr);
            let version = if version.is_empty() {
                stderr.lines().next().unwrap_or("").trim().to_string()
            } else {
                version
            };
            checks.push(json!({"name": cmd, "status": "ok", "detail": version}));
        }
        _ => {
            checks.push(json!({
                "name": cmd,
                "status": "fail",
                "detail": format!("not found — install via: {hint}"),
            }));
        }
    }
}

fn check_xcode(checks: &mut Vec<serde_json::Value>) {
    match std::process::Command::new("xcode-select")
        .arg("-p")
        .output()
    {
        Ok(out) if out.status.success() => {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            checks.push(json!({"name": "Xcode CLI Tools", "status": "ok", "detail": path}));
        }
        _ => {
            checks.push(json!({
                "name": "Xcode CLI Tools",
                "status": "fail",
                "detail": "not installed — run: xcode-select --install",
            }));
        }
    }
}

fn check_model(checks: &mut Vec<serde_json::Value>) {
    let model = registry::default_model();
    let name = format!("Model {}", model.name);
    match storage::is_model_downloaded(model) {
        Ok(true) => {
            checks.push(json!({"name": name, "status": "ok", "detail": "downloaded"}));
        }
        Ok(false) => {
            checks.push(json!({
                "name": name,
                "status": "fail",
                "detail": "not found — run: whisper-macos-cli models download",
            }));
        }
        Err(e) => {
            checks.push(json!({"name": name, "status": "fail", "detail": e.to_string()}));
        }
    }
}

fn check_network(checks: &mut Vec<serde_json::Value>) {
    let client = match reqwest::blocking::Client::builder()
        .timeout(NETWORK_PROBE_TIMEOUT)
        .user_agent(concat!(
            "whisper-macos-cli/",
            env!("CARGO_PKG_VERSION"),
            " (doctor)"
        ))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            checks.push(json!({
                "name": "Network",
                "status": "fail",
                "detail": format!("cannot build HTTP client: {e}"),
            }));
            return;
        }
    };

    match client.head(NETWORK_PROBE_HOST).send() {
        Ok(resp) if resp.status().is_success() || resp.status().is_redirection() => {
            checks.push(json!({"name": "Network", "status": "ok", "detail": NETWORK_PROBE_HOST}));
        }
        Ok(resp) => {
            checks.push(json!({
                "name": "Network",
                "status": "fail",
                "detail": format!("HTTP {} from {}", resp.status(), NETWORK_PROBE_HOST),
            }));
        }
        Err(e) => {
            checks.push(json!({
                "name": "Network",
                "status": "fail",
                "detail": format!("air-gapped or unreachable: {e}"),
            }));
        }
    }
}

fn check_disk_space(checks: &mut Vec<serde_json::Value>) {
    if let Ok(dirs) = crate::model::storage::models_dir() {
        if let Some(parent) = dirs.parent() {
            let available = available_disk_bytes(parent);
            let required = 4 * 1024 * 1024 * 1024u64;
            match available {
                Some(avail) if avail >= required => {
                    checks.push(json!({
                        "name": "Disk Space",
                        "status": "ok",
                        "detail": format!("{} MB available", avail / (1024 * 1024)),
                    }));
                }
                Some(avail) => {
                    checks.push(json!({
                        "name": "Disk Space",
                        "status": "fail",
                        "detail": format!("only {} MB available, need at least 4GB", avail / (1024 * 1024)),
                    }));
                }
                None => {
                    checks.push(json!({
                        "name": "Disk Space",
                        "status": "ok",
                        "detail": "unable to determine (non-POSIX platform)",
                    }));
                }
            }
        }
    }
}

#[cfg(unix)]
fn available_disk_bytes(path: &std::path::Path) -> Option<u64> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let cpath = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::statvfs(cpath.as_ptr(), &mut stat) };
    if result != 0 {
        return None;
    }
    let available = stat.f_bavail as u64 * stat.f_frsize;
    Some(available)
}

#[cfg(not(unix))]
fn available_disk_bytes(_path: &std::path::Path) -> Option<u64> {
    None
}
