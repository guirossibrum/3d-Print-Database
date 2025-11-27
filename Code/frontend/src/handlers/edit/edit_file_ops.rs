//! File operations for edit handlers
//! 
//! This module contains file system operations used by edit handlers.

use anyhow::Result;
use std::path::Path;

/// Build a file tree representation for a product SKU
pub fn build_file_tree(sku: &str) -> Result<Vec<String>> {
    let mut content = Vec::new();
    let base_path = Path::new("/home/grbrum/Work/3d_print/Products").join(sku);

    if !base_path.exists() {
        content.push("No files found for this product".to_string());
        return Ok(content);
    }

    content.push(format!("📁 {}/", sku));

    // Scan subdirectories
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

    // Check for metadata.json
    let metadata_path = base_path.join("metadata.json");
    if metadata_path.exists() {
        content.push("└── 📄 metadata.json".to_string());
    }

    Ok(content)
}

/// Scan a directory and return formatted file list
fn scan_directory(dir_path: &Path, prefix: &str) -> Result<Vec<String>> {
    let mut content = Vec::new();
    let entries = match std::fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return Ok(content),
    };

    let mut file_entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_file()))
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