use crate::error::Error;
use crate::model::{registry, storage};

pub fn run() -> Result<(), Error> {
    eprintln!("System Check");
    let mut all_ok = true;

    check_macos(&mut all_ok);
    check_arch(&mut all_ok);
    check_command("cmake", &["--version"], "brew install cmake", &mut all_ok);
    check_command(
        "clang",
        &["--version"],
        "xcode-select --install",
        &mut all_ok,
    );
    check_xcode(&mut all_ok);
    check_model(&mut all_ok);

    if all_ok {
        eprintln!("\nAll checks passed.");
        Ok(())
    } else {
        eprintln!("\nSome checks failed. Fix the issues above before using whisper-macos-cli.");
        Err(Error::Config("system check failed".into()))
    }
}

fn print_row(label: &str, status: &str) {
    let dots = ".".repeat(18_usize.saturating_sub(label.len()));
    eprintln!("  {label} {dots} {status}");
}

fn check_macos(all_ok: &mut bool) {
    if cfg!(target_os = "macos") {
        print_row("macOS", "OK");
    } else {
        print_row("macOS", "FAIL (not macOS — Metal GPU unavailable)");
        *all_ok = false;
    }
}

fn check_arch(all_ok: &mut bool) {
    if cfg!(target_arch = "aarch64") {
        print_row("Apple Silicon", "OK (aarch64)");
    } else {
        let arch = std::env::consts::ARCH;
        print_row(
            "Apple Silicon",
            &format!("FAIL ({arch} — Metal GPU requires Apple Silicon)"),
        );
        *all_ok = false;
    }
}

fn check_command(cmd: &str, args: &[&str], hint: &str, all_ok: &mut bool) {
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
            print_row(cmd, &format!("OK ({version})"));
        }
        _ => {
            print_row(cmd, &format!("FAIL (not found — install via: {hint})"));
            *all_ok = false;
        }
    }
}

fn check_xcode(all_ok: &mut bool) {
    match std::process::Command::new("xcode-select")
        .arg("-p")
        .output()
    {
        Ok(out) if out.status.success() => {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            print_row("Xcode CLI Tools", &format!("OK ({path})"));
        }
        _ => {
            print_row(
                "Xcode CLI Tools",
                "FAIL (not installed — run: xcode-select --install)",
            );
            *all_ok = false;
        }
    }
}

fn check_model(all_ok: &mut bool) {
    let model = registry::default_model();
    let label = format!("Model {}", model.name);
    match storage::is_model_downloaded(model) {
        Ok(true) => print_row(&label, "OK (downloaded)"),
        Ok(false) => {
            print_row(
                &label,
                "FAIL (not found — run: whisper-macos-cli models download)",
            );
            *all_ok = false;
        }
        Err(e) => {
            print_row(&label, &format!("FAIL ({e})"));
            *all_ok = false;
        }
    }
}
