use std::path::Path;
use whisper_rs::{WhisperContext, WhisperContextParameters};

#[cfg(unix)]
pub struct StderrSilencer {
    fd_backup: libc::c_int,
}

#[cfg(unix)]
impl StderrSilencer {
    /// # Safety
    ///
    /// This function duplicates the process stderr file descriptor and replaces
    /// fd 2 with /dev/null. The caller MUST hold the returned guard for the
    /// duration of the silenced operation. Failure to drop the guard restores
    /// the original stderr.
    ///
    /// The function is safe to call in single-threaded contexts at startup
    /// (before any other thread is created that depends on stderr). whisper.cpp
    /// prints verbose initialization messages to stderr that are not captured
    /// by the tracing subscriber.
    pub fn start() -> Self {
        unsafe {
            let fd_backup = libc::dup(libc::STDERR_FILENO);
            let devnull = libc::open(c"/dev/null".as_ptr(), libc::O_WRONLY);
            libc::dup2(devnull, libc::STDERR_FILENO);
            libc::close(devnull);
            Self { fd_backup }
        }
    }
}

#[cfg(unix)]
impl Drop for StderrSilencer {
    fn drop(&mut self) {
        // # Safety
        //
        // fd_backup is a valid duplicate of STDERR_FILENO captured at construction.
        // The original STDERR_FILENO may have been replaced with /dev/null; restoring
        // it via dup2 is atomic and safe. Closing the duplicate after restore is
        // also safe because we own the descriptor.
        unsafe {
            libc::dup2(self.fd_backup, libc::STDERR_FILENO);
            libc::close(self.fd_backup);
        }
    }
}

pub fn load_model(model_path: &Path) -> Result<WhisperContext, crate::error::Error> {
    let path_str = model_path
        .to_str()
        .ok_or_else(|| crate::error::Error::WhisperInference("invalid model path".to_string()))?;

    let mut params = WhisperContextParameters::default();
    params.use_gpu(true);
    params.flash_attn(true);
    params.gpu_device(0);

    #[cfg(unix)]
    let _silence = StderrSilencer::start();

    WhisperContext::new_with_params(path_str, params)
        .map_err(|e| crate::error::Error::WhisperInference(e.to_string()))
}
