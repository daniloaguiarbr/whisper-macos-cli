use std::path::PathBuf;

use directories::ProjectDirs;

use super::registry::ModelInfo;
use crate::error::Error;

fn project_dirs() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("", "", "whisper-macos-cli")
        .ok_or_else(|| Error::Config("cannot determine application data directory".into()))
}

pub fn models_dir() -> Result<PathBuf, Error> {
    let dirs = project_dirs()?;
    let path = dirs.data_dir().join("models");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn model_path(model: &ModelInfo) -> Result<PathBuf, Error> {
    Ok(models_dir()?.join(model.filename))
}

pub fn is_model_downloaded(model: &ModelInfo) -> Result<bool, Error> {
    let path = model_path(model)?;
    match std::fs::metadata(&path) {
        Ok(meta) => Ok(meta.len() >= model.min_size_bytes),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(Error::Io(e)),
    }
}

pub fn remove_model(model: &ModelInfo) -> Result<(), Error> {
    let path = model_path(model)?;
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Error::Io(e)),
    }
}

pub fn model_temp_path(model: &ModelInfo) -> Result<PathBuf, Error> {
    Ok(model_path(model)?.with_extension("bin.tmp"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::registry;

    #[test]
    fn model_path_ends_with_filename() {
        let model = &registry::all_models()[0];
        let path = model_path(model).unwrap();
        assert!(path.ends_with(model.filename));
    }

    #[test]
    fn model_temp_path_uses_bin_tmp_suffix() {
        let model = &registry::all_models()[0];
        let path = model_temp_path(model).unwrap();
        let path_str = path.to_string_lossy();
        assert!(
            path_str.ends_with(".bin.tmp"),
            "expected .bin.tmp suffix, got {path_str}"
        );
    }

    #[test]
    fn is_model_downloaded_returns_false_when_missing() {
        let model = &registry::all_models()[0];
        let path = model_path(model).unwrap();
        let _ = std::fs::remove_file(&path);
        let result = is_model_downloaded(model).unwrap();
        assert!(!result);
    }

    #[test]
    fn remove_model_is_idempotent_when_missing() {
        let model = &registry::all_models()[0];
        let path = model_path(model).unwrap();
        let _ = std::fs::remove_file(&path);
        assert!(remove_model(model).is_ok());
    }

    #[test]
    fn models_dir_creates_directory() {
        let path = models_dir().unwrap();
        assert!(path.exists());
        assert!(path.is_dir());
        assert!(path.ends_with("models"));
    }
}
