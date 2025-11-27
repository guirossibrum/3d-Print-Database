// src/handlers/edit/edit_file_ops.rs
use anyhow::{Context, Result};
use crossterm::event::KeyEvent;
use std::path::Path;

use crate::app::App;

/// Build a textual file tree for a product SKU.
/// Behavior preserved from original.
pub fn build_file_tree(sku: &str) -> Result<Vec<String>> {
    let mut content = Vec::new();
    let base_path = Path::new("/home/grbrum/Work/3d_print/Products").join(sku);

    if !base_path.exists() {
        content.push("No files found for this product".to_string());
        return Ok(content);
    }

    content.push(format!("📁 {}/", sku));

    let subdirs = ["images", "models", "notes", "print_files"];
    for subdir in &subdirs {
        let subdir_path = base_path.join(subdir);
        if subdir_path.exists() {
            content.push(format!("├── 📁 {}/", subdir));
            match scan_directory(&subdir_path, "    │   ") {
                Ok(files) => content.extend(files),
                Err(_) => content.push("    │       └── (Error reading directory)".to_string()),
            }
        } else {
            content.push(format!("├── 📁 {}/ (empty)", subdir));
        }
    }

    let metadata_path = base_path.join("metadata.json");
    if metadata_path.exists() {
        content.push("└── 📄 metadata.json".to_string());
    }

    Ok(content)
}

pub fn scan_directory(dir_path: &std::path::Path, prefix: &str) -> Result<Vec<String>> {
    let mut content = Vec::new();
    let entries = std::fs::read_dir(dir_path).with_context(|| format!("reading {}", dir_path.display()))?;

    let mut file_entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .collect();

    file_entries.sort_by_key(|a| a.file_name());

    for (i, entry) in file_entries.iter().enumerate() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let is_last = i == file_entries.len() - 1;
        let connector = if is_last { "└──" } else { "├──" };
        content.push(format!("{}{} 📄 {}", prefix, connector, file_name));
    }

    Ok(content)
}

/// handle: allow edit_file_ops to respond to DeleteFileConfirm and DeleteConfirm modes etc.
/// For now we return false; the dispatcher will call specific functions when needed.
pub fn handle(_app: &mut App, _key: KeyEvent) -> Result<bool> {
    Ok(false)
}
