use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::time::Duration;

static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
static SHUTDOWN_SIGNAL: AtomicU8 = AtomicU8::new(0);
static LAST_SIGNAL_TIME_MS: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownReason {
    None,
    Sigint,
    Sigterm,
    ForcedExit,
}

impl ShutdownReason {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Sigint,
            2 => Self::Sigterm,
            _ => Self::None,
        }
    }
}

pub fn reset_sigpipe() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

pub fn install_handlers() {
    install_sigint();
    install_sigterm();
}

fn install_sigint() {
    let handler = || {
        SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
        let now = monotonic_ms();
        let last = LAST_SIGNAL_TIME_MS.load(Ordering::SeqCst);
        SHUTDOWN_SIGNAL.store(1, Ordering::SeqCst);
        if now.saturating_sub(last) < 1500 {
            SHUTDOWN_SIGNAL.store(3, Ordering::SeqCst);
        }
        LAST_SIGNAL_TIME_MS.store(now, Ordering::SeqCst);
    };
    if let Err(e) = ctrlc::set_handler(handler) {
        tracing::warn!("failed to install SIGINT handler: {e}");
    }
}

#[cfg(unix)]
fn install_sigterm() {
    use signal_hook::consts::TERM_SIGNALS;
    for sig in TERM_SIGNALS {
        let handler = || {
            SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
            SHUTDOWN_SIGNAL.store(2, Ordering::SeqCst);
        };
        if let Err(e) = signal_hook::flag::register(*sig, Arc::new(AtomicBool::new(true))) {
            tracing::warn!(?sig, "failed to install SIGTERM flag: {e}");
        }
        let _ = handler;
    }
}

#[cfg(not(unix))]
fn install_sigterm() {}

pub fn is_shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

pub fn shutdown_reason() -> ShutdownReason {
    ShutdownReason::from_u8(SHUTDOWN_SIGNAL.load(Ordering::SeqCst))
}

pub fn is_forced_exit() -> bool {
    matches!(shutdown_reason(), ShutdownReason::ForcedExit)
}

pub fn shutdown_signal_exit_code() -> u8 {
    match shutdown_reason() {
        ShutdownReason::Sigint => 130,
        ShutdownReason::Sigterm => 143,
        ShutdownReason::ForcedExit => 137,
        ShutdownReason::None => 0,
    }
}

pub fn wait_or_timeout(timeout: Duration) -> bool {
    let start = std::time::Instant::now();
    while !is_shutdown_requested() && start.elapsed() < timeout {
        std::thread::sleep(Duration::from_millis(50));
    }
    is_shutdown_requested()
}

pub fn cleanup_partial_downloads(temp_paths: &[std::path::PathBuf]) {
    for path in temp_paths {
        if path.exists() {
            if let Err(e) = std::fs::remove_file(path) {
                tracing::warn!(path = %path.display(), "failed to clean up temp file: {e}");
            }
        }
    }
}

fn monotonic_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_reason_default_is_none() {
        assert_eq!(shutdown_reason(), ShutdownReason::None);
    }
}
