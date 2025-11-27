// src/handlers/util.rs
use anyhow::{anyhow, Context, Result};
use crossterm::event::KeyEvent;
use std::path::PathBuf;
use std::process::Command;

use crate::app::App;

/// Normalize a tag name.
///
/// NOTE: this function must match backend normalization semantics exactly.
/// The original implementation used ASCII-only filtering; if the backend strips diacritics,
/// implement the same (or change both sides). For now we preserve Unicode letters.
pub fn normalize_tag_name(tag: &str) -> String {
    let lowered = tag.to_lowercase();
    // replace spaces and underscores with hyphens and keep alphanumeric/unicode and hyphen
    let mut tmp = String::with_capacity(lowered.len());
    for c in lowered.chars() {
        if c == ' ' || c == '_' {
            tmp.push('-');
        } else if c.is_alphanumeric() || c == '-' {
            tmp.push(c);
        }
        // drop other punctuation
    }

    // collapse multiple hyphens
    let mut out = String::with_capacity(tmp.len());
    let mut prev_hyphen = false;
    for c in tmp.chars() {
        if c == '-' {
            if !prev_hyphen {
                out.push('-');
                prev_hyphen = true;
            }
        } else {
            out.push(c);
            prev_hyphen = false;
        }
    }

    out.trim_matches('-').to_string()
}

/// Try to open a product folder with a set of strategies.
/// Returns Ok(()) if a command was successfully started; Err with context otherwise.
pub fn open_product_folder_cmd(product_root: &PathBuf, sku: &str) -> Result<()> {
    let base_path = product_root.join(sku);
    if !base_path.exists() {
        return Err(anyhow!("Product folder not found: {}", base_path.display()));
    }

    // Try platform-generic opener first.
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = Command::new("xdg-open").arg(&base_path).status() {
            if status.success() {
                return Ok(());
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(status) = Command::new("open").arg(&base_path).status() {
            if status.success() {
                return Ok(());
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        // On Windows we delegate to explorer
        if let Ok(status) = Command::new("explorer").arg(&base_path).status() {
            if status.success() {
                return Ok(());
            }
        }
    }

    // Last resort: attempt common file managers (Linux)
    let file_managers = ["dolphin", "nautilus", "yazi", "thunar", "pcmanfm", "nemo"];
    for fm in &file_managers {
        if let Ok(status) = Command::new(fm).arg(&base_path).status() {
            if status.success() {
                return Ok(());
            }
        }
    }

    Err(anyhow!("Failed to open folder: {}", base_path.display()))
}

/// Small event handler for miscellaneous shortcuts (e.g., Ctrl+O open folder).
/// Returns Ok(true) if the event was handled for current mode.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;
    use crossterm::event::KeyModifiers;

    // Ctrl+o shortcut to open product folder (legacy behavior)
    if let KeyCode::Char('o') = key.code {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if matches!(app.current_tab, crate::state::Tab::Search)
                && !app.products.is_empty()
                && let Some(product) = app.get_selected_product()
            {
                // Use product root from app config (fallback to default if not set)
                let product_root = app
                    .config
                    .as_ref()
                    .map(|c| c.product_root.clone())
                    .unwrap_or_else(|| PathBuf::from("/home/grbrum/Work/3d_print/Products"));

                match open_product_folder_cmd(&product_root, &product.sku) {
                    Ok(_) => {
                        app.set_status_message(format!("Opened folder for product {}", product.sku));
                    }
                    Err(e) => {
                        app.set_status_message(format!("Error opening folder: {}", e));
                    }
                }
                return Ok(true);
            }
        }
    }

    Ok(false)
}
