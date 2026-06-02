use std::fs;

use serial_test::serial;
use tempfile::TempDir;

use whisper_macos_cli::model::registry;
use whisper_macos_cli::model::storage;

#[test]
#[serial]
fn tempdir_creates_isolated_models_dir() {
    let _temp = TempDir::new().expect("create temp dir");
    let path = storage::models_dir().unwrap();
    assert!(path.exists());
    assert!(path.is_dir());
}

#[test]
#[serial]
fn model_path_under_registry() {
    let _temp = TempDir::new().expect("create temp dir");
    let model = &registry::all_models()[0];
    let path = storage::model_path(model).unwrap();
    assert!(path.to_string_lossy().contains("whisper-macos-cli"));
    assert!(path.ends_with(model.filename));
}

#[test]
#[serial]
fn is_model_downloaded_false_on_empty_dir() {
    let _temp = TempDir::new().expect("create temp dir");
    let model = &registry::all_models()[0];
    let result = storage::is_model_downloaded(model).unwrap();
    assert!(!result, "no model file should be downloaded in fresh state");
}

#[test]
#[serial]
fn is_model_downloaded_true_when_file_above_min_size() {
    let temp = TempDir::new().expect("create temp dir");
    let model = &registry::all_models()[0];
    let dest = storage::model_path(model).unwrap();
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    let data = vec![0u8; (model.min_size_bytes + 1024) as usize];
    fs::write(&dest, &data).unwrap();
    assert!(storage::is_model_downloaded(model).unwrap());
    drop(temp);
}

#[test]
#[serial]
fn is_model_downloaded_false_when_file_below_min_size() {
    let temp = TempDir::new().expect("create temp dir");
    let model = &registry::all_models()[0];
    let dest = storage::model_path(model).unwrap();
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    let data = vec![0u8; 1024];
    fs::write(&dest, &data).unwrap();
    assert!(!storage::is_model_downloaded(model).unwrap());
    drop(temp);
}

#[test]
#[serial]
fn remove_model_removes_existing_file() {
    let temp = TempDir::new().expect("create temp dir");
    let model = &registry::all_models()[0];
    let dest = storage::model_path(model).unwrap();
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    fs::write(&dest, b"data").unwrap();
    assert!(dest.exists());
    storage::remove_model(model).unwrap();
    assert!(!dest.exists());
    drop(temp);
}

#[test]
#[serial]
fn model_temp_path_uses_tmp_suffix() {
    let _temp = TempDir::new().expect("create temp dir");
    let model = &registry::all_models()[0];
    let path = storage::model_temp_path(model).unwrap();
    assert!(path.to_string_lossy().ends_with(".bin.tmp"));
}

#[test]
#[serial]
fn model_temp_paths_for_different_models_are_distinct() {
    let _temp = TempDir::new().expect("create temp dir");
    let m1 = &registry::all_models()[0];
    let m2 = &registry::all_models()[1];
    let p1 = storage::model_temp_path(m1).unwrap();
    let p2 = storage::model_temp_path(m2).unwrap();
    assert_ne!(p1, p2, "different models should have different temp paths");
}
