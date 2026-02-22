use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

use crate::notebook::Notebook;

pub fn read_notebook_file<P: AsRef<Path>>(path: P) -> Result<Notebook> {
    let path = path.as_ref();
    debug!("Reading notebook from: {}", path.display());

    let content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read file: {}", path.display()))?;

    let notebook: Notebook = serde_json::from_str(&content)
        .with_context(|| format!("Invalid JSON format in notebook file: {}", path.display()))?;

    info!("Successfully read notebook from: {}", path.display());
    Ok(notebook)
}

pub fn write_notebook_file<P: AsRef<Path>>(path: P, notebook: &Notebook) -> Result<()> {
    let path = path.as_ref();
    debug!("Writing notebook to: {}", path.display());

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            anyhow::bail!("Directory does not exist: {}", parent.display());
        }
    }

    if path.exists() {
        backup_notebook(path)?;
    }

    let content = serde_json::to_string_pretty(notebook)
        .context("Failed to serialize notebook to JSON")?;

    let temp_path = path.with_extension("ipynb.tmp");
    fs::write(&temp_path, content)
        .with_context(|| format!("Cannot write temporary file: {}", temp_path.display()))?;

    fs::rename(&temp_path, path)
        .with_context(|| format!("Cannot rename temp file to: {}", path.display()))?;

    info!("Successfully wrote notebook to: {}", path.display());
    Ok(())
}

pub fn backup_notebook<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();

    if !path.exists() {
        anyhow::bail!("Cannot backup, file does not exist: {}", path.display());
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = path.with_extension(format!("ipynb.bak.{}", timestamp));

    fs::copy(path, &backup_path)
        .with_context(|| format!("Cannot create backup: {}", backup_path.display()))?;

    warn!("Created backup at: {}", backup_path.display());
    Ok(backup_path)
}
