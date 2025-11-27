// src/handlers/edit/mod.rs
pub mod edit_name;
pub mod edit_description;
pub mod edit_tags;
pub mod edit_materials;
pub mod edit_categories;
pub mod edit_file_ops;

use anyhow::Result;
use crossterm::event::KeyEvent;

/// Combined entry point for edit-related handlers.
/// Returns Ok(true) if edit subsystem handled the key.
pub fn handle(app: &mut crate::app::App, key: KeyEvent) -> Result<bool> {
    if edit_name::handle(app, key)? { return Ok(true); }
    if edit_description::handle(app, key)? { return Ok(true); }
    if edit_tags::handle(app, key)? { return Ok(true); }
    if edit_materials::handle(app, key)? { return Ok(true); }
    if edit_categories::handle(app, key)? { return Ok(true); }
    if edit_file_ops::handle(app, key)? { return Ok(true); }
    Ok(false)
}
