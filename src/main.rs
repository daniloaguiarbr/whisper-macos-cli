use std::process::ExitCode;

fn main() -> ExitCode {
    human_panic::setup_panic!();
    whisper_macos_cli::run()
}
