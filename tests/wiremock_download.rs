use std::time::Duration;

use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use whisper_macos_cli::model::download;

async fn run_sync<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .expect("blocking task panicked")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn wiremock_server_responds_to_head() {
    let server = MockServer::start().await;

    Mock::given(method("HEAD"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let uri = server.uri();
    let status = run_sync(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        client.head(&uri).send().unwrap().status()
    })
    .await;

    assert!(status.is_success());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn download_model_to_tempdir_min_size_validation() {
    let server = MockServer::start().await;
    let body = vec![0u8; 1024];

    Mock::given(method("GET"))
        .and(path("/model.bin"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-length", "1024")
                .set_body_bytes(body),
        )
        .mount(&server)
        .await;

    let url = format!("{}/model.bin", server.uri());
    let result = run_sync(move || {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("ggml-tiny.bin");
        let r = download::download_model(&url, &dest, 75_687_065, 70_000_000);
        (r, dest)
    })
    .await;

    let (result, _dest) = result;
    assert!(result.is_err(), "expected error due to size below min_size");
    let err_msg = format!("{:?}", result.err().unwrap());
    assert!(
        err_msg.contains("below minimum") || err_msg.contains("HTTP"),
        "unexpected error: {err_msg}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn download_model_to_tempdir_succeeds_with_correct_size() {
    let server = MockServer::start().await;
    let body = vec![0u8; 1024];

    Mock::given(method("GET"))
        .and(path("/model.bin"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-length", "1024")
                .set_body_bytes(body),
        )
        .mount(&server)
        .await;

    let url = format!("{}/model.bin", server.uri());
    let (result, dest, _temp) = run_sync(move || {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("ggml-tiny.bin");
        let r = download::download_model(&url, &dest, 1024, 512);
        (r, dest, temp)
    })
    .await;

    assert!(result.is_ok(), "expected success, got {result:?}");
    assert!(dest.exists(), "dest file should exist after download");
    let metadata = std::fs::metadata(&dest).unwrap();
    assert_eq!(metadata.len(), 1024);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn download_model_handles_404_no_retry() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let url = format!("{}/missing.bin", server.uri());
    let result = run_sync(move || {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("ggml-tiny.bin");
        download::download_model(&url, &dest, 1024, 512)
    })
    .await;

    assert!(result.is_err(), "404 should fail");
    let err = result.err().unwrap();
    assert!(err.to_string().contains("HTTP 404") || err.to_string().contains("404"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn download_model_to_tempdir_cleans_tempfile_on_failure() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let url = format!("{}/fails.bin", server.uri());
    let (temp_file_exists, dest_exists) = run_sync(move || {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("ggml-tiny.bin");
        let temp_file = dest.with_extension("bin.tmp");
        let _ = download::download_model(&url, &dest, 1024, 512);
        (temp_file.exists(), dest.exists())
    })
    .await;

    assert!(!temp_file_exists, "temp file should be cleaned up");
    assert!(!dest_exists, "dest should not exist on failure");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn download_model_handles_redirect() {
    let server = MockServer::start().await;
    let body = vec![0u8; 2048];

    Mock::given(method("GET"))
        .and(path("/redirect"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/final"))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/final"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-length", "2048")
                .set_body_bytes(body),
        )
        .mount(&server)
        .await;

    let url = format!("{}/redirect", server.uri());
    let (result, dest, _temp) = run_sync(move || {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("ggml-tiny.bin");
        let r = download::download_model(&url, &dest, 2048, 1024);
        (r, dest, temp)
    })
    .await;

    assert!(result.is_ok(), "redirect should be followed");
    assert!(dest.exists());
}
