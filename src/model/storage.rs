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
