use serde_json::json;

use crate::cli::{ModelsAction, WhisperModel};
use crate::error::Error;
use crate::model::{download, registry, storage};
use crate::output;

pub fn run(action: &ModelsAction, correlation_id: &str) -> Result<(), Error> {
    match action {
        ModelsAction::Download { model } => run_download(*model, correlation_id),
        ModelsAction::List => run_list(correlation_id),
        ModelsAction::Path { model } => run_path(*model, correlation_id),
        ModelsAction::Remove { model, dry_run } => run_remove(*model, *dry_run, correlation_id),
    }
}

fn run_download(name: Option<WhisperModel>, correlation_id: &str) -> Result<(), Error> {
    let model = match name {
        Some(m) => registry::get_model(m.as_str()).ok_or_else(|| Error::ModelNotFound {
            name: m.as_str().to_string(),
        })?,
        None => registry::default_model(),
    };

    if storage::is_model_downloaded(model)? {
        let value = json!({
            "schema_version": env!("CARGO_PKG_VERSION"),
            "correlation_id": correlation_id,
            "model": model.name,
            "action": "already_downloaded",
            "path": null,
        });
        output::write_json_value(&value).map_err(Error::Io)?;
        return Ok(());
    }

    let dest = storage::model_path(model)?;
    tracing::info!(
        model = model.name,
        bytes = model.size_bytes,
        "downloading model"
    );
    let etag = download::download_model(model.url, &dest, model.size_bytes, model.min_size_bytes)?;
    let value = json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "model": model.name,
        "action": "downloaded",
        "path": dest.display().to_string(),
        "etag": etag,
    });
    output::write_json_value(&value).map_err(Error::Io)?;
    Ok(())
}

fn run_list(correlation_id: &str) -> Result<(), Error> {
    let models: Vec<serde_json::Value> = registry::all_models()
        .iter()
        .map(|model| {
            let downloaded = storage::is_model_downloaded(model).unwrap_or(false);
            json!({
                "name": model.name,
                "size_bytes": model.size_bytes,
                "min_size_bytes": model.min_size_bytes,
                "downloaded": downloaded,
                "description": model.description,
            })
        })
        .collect();

    let envelope = json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "models": models,
        "total": models.len(),
    });
    output::write_json_value(&envelope).map_err(Error::Io)
}

fn run_path(name: Option<WhisperModel>, correlation_id: &str) -> Result<(), Error> {
    let model = match name {
        Some(m) => registry::get_model(m.as_str()).ok_or_else(|| Error::ModelNotFound {
            name: m.as_str().to_string(),
        })?,
        None => registry::default_model(),
    };

    let path = storage::model_path(model)?;
    let downloaded = storage::is_model_downloaded(model)?;

    let value = json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "model": model.name,
        "path": path.display().to_string(),
        "downloaded": downloaded,
    });
    output::write_json_value(&value).map_err(Error::Io)
}

fn run_remove(name: WhisperModel, dry_run: bool, correlation_id: &str) -> Result<(), Error> {
    let model = registry::get_model(name.as_str()).ok_or_else(|| Error::ModelNotFound {
        name: name.as_str().to_string(),
    })?;

    let path = storage::model_path(model)?;

    if dry_run {
        let value = json!({
            "schema_version": env!("CARGO_PKG_VERSION"),
            "correlation_id": correlation_id,
            "model": model.name,
            "action": "would_remove",
            "path": path.display().to_string(),
            "size_bytes": model.size_bytes,
        });
        output::write_json_value(&value).map_err(Error::Io)?;
        return Ok(());
    }

    storage::remove_model(model)?;
    let value = json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "model": model.name,
        "action": "removed",
        "path": path.display().to_string(),
    });
    output::write_json_value(&value).map_err(Error::Io)
}
