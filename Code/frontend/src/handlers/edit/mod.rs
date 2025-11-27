//! Edit mode handlers
//! 
//! This module contains all handlers for editing existing products.

use crossterm::event::KeyEvent;
use anyhow::Result;

use crate::App;
use crate::models::InputMode;

pub mod edit_name;
pub mod edit_description;
pub mod edit_tags;
pub mod edit_materials;
pub mod edit_categories;
pub mod edit_production;
pub mod edit_file_ops;

/// Main edit mode handler
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    match app.input_mode {
        InputMode::EditName => return edit_name::handle(app, key),
        InputMode::EditDescription => return edit_description::handle(app, key),
        InputMode::EditTags => return edit_tags::handle(app, key),
        InputMode::EditMaterials => return edit_materials::handle(app, key),
        InputMode::EditCategories => return edit_categories::handle(app, key),
        InputMode::EditProduction => return edit_production::handle(app, key),
        _ => {}
    }
    Ok(false)
}