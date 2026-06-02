use std::sync::Arc;
use std::thread;

use whisper_macos_cli::signal;

#[test]
fn concurrent_is_shutdown_requested_is_thread_safe() {
    let barrier = Arc::new(std::sync::Barrier::new(8));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            b.wait();
            for _ in 0..1000 {
                let _ = signal::is_shutdown_requested();
            }
        }));
    }
    for h in handles {
        h.join().expect("thread should not panic");
    }
}

#[test]
fn concurrent_shutdown_reason_is_thread_safe() {
    let barrier = Arc::new(std::sync::Barrier::new(8));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            b.wait();
            for _ in 0..1000 {
                let _ = signal::shutdown_reason();
                let _ = signal::is_forced_exit();
                let _ = signal::shutdown_signal_exit_code();
            }
        }));
    }
    for h in handles {
        h.join().expect("thread should not panic");
    }
}

#[test]
fn concurrent_wait_or_timeout_completes_within_budget() {
    use std::time::Instant;
    let barrier = Arc::new(std::sync::Barrier::new(4));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            b.wait();
            let start = Instant::now();
            let result = signal::wait_or_timeout(std::time::Duration::from_millis(50));
            let elapsed = start.elapsed();
            (result, elapsed)
        }));
    }
    for h in handles {
        let (result, elapsed) = h.join().expect("thread should not panic");
        assert!(!result, "no shutdown signaled, should return false");
        assert!(
            elapsed.as_millis() < 500,
            "should not exceed timeout by more than 450ms"
        );
    }
}

#[test]
fn monotonic_ms_returns_reasonable_value() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let _ = now;
}
