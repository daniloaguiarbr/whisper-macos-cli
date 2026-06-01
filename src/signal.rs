pub fn reset_sigpipe() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

pub fn install_ctrlc_handler() {
    ctrlc::set_handler(|| {
        std::process::exit(130);
    })
    .expect("failed to install Ctrl+C handler");
}
