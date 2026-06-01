use crate::cli::ModelsAction;
use crate::error::Error;
use crate::model::{download, registry, storage};
use crate::output;

pub fn run(action: &ModelsAction) -> Result<(), Error> {
    match action {
        ModelsAction::Download { model } => run_download(model.as_deref()),
        ModelsAction::List => run_list(),
        ModelsAction::Path { model } => run_path(model.as_deref()),
        ModelsAction::Remove { model } => run_remove(model),
    }
}

fn run_download(name: Option<&str>) -> Result<(), Error> {
    let model = match name {
        Some(n) => registry::get_model(n).ok_or_else(|| Error::ModelNotFound {
            name: n.to_string(),
        })?,
        None => registry::default_model(),
    };

    if storage::is_model_downloaded(model)? {
        eprintln!("model '{}' already downloaded", model.name);
        return Ok(());
    }

    let dest = storage::model_path(model)?;
    eprintln!(
        "downloading model '{}' ({} bytes)...",
        model.name, model.size_bytes
    );
    download::download_model(model.url, &dest, model.size_bytes)?;
    eprintln!("model '{}' downloaded successfully", model.name);
    Ok(())
}

fn run_list() -> Result<(), Error> {
    eprintln!(
        "{:<12} {:>12}  {:<14}  DESCRIPTION",
        "NAME", "SIZE", "STATUS"
    );
    eprintln!("{}", "-".repeat(72));

    for model in registry::all_models() {
        let downloaded = storage::is_model_downloaded(model)?;
        let status = if downloaded {
            "downloaded"
        } else {
            "not downloaded"
        };
        let size_mb = model.size_bytes / 1_000_000;
        eprintln!(
            "{:<12} {:>10}MB  {:<14}  {}",
            model.name, size_mb, status, model.description
        );
    }

    Ok(())
}

fn run_path(name: Option<&str>) -> Result<(), Error> {
    let model = match name {
        Some(n) => registry::get_model(n).ok_or_else(|| Error::ModelNotFound {
            name: n.to_string(),
        })?,
        None => registry::default_model(),
    };

    let path = storage::model_path(model)?;

    if !storage::is_model_downloaded(model)? {
        eprintln!("note: model '{}' is not downloaded", model.name);
    }

    output::write_stdout_line(&path.display().to_string()).map_err(Error::Io)?;
    Ok(())
}

fn run_remove(name: &str) -> Result<(), Error> {
    let model = registry::get_model(name).ok_or_else(|| Error::ModelNotFound {
        name: name.to_string(),
    })?;
    storage::remove_model(model)?;
    eprintln!("model '{}' removed", model.name);
    Ok(())
}
