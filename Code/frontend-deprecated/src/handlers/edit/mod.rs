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
pub mod edit_production;
pub mod edit_file_ops;

/// Main edit mode handler
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    // Common actions for all edit modes
    match key.code {
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.save_current_product()?;
            return Ok(true);
        }
        KeyCode::Esc => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original) = app.edit_backup.take() {
                if let Some(selected_id) = app.get_selected_product_id() {
                    if let Some(current) = app
                        .products
                        .iter_mut()
                        .find(|p| p.id == Some(selected_id))
                    {
                        *current = original;
                    }
                }
            }
            app.input_mode = crate::models::InputMode::Normal;
            app.active_pane = crate::models::ActivePane::Left;
            return Ok(true);
        }
        KeyCode::Tab => {
            // Tab behavior: open selection UI for Tags/Materials, do nothing for others
            match app.input_mode {
                InputMode::EditTags => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.selection_type = Some(crate::models::SelectionType::Tag);
                    app.input_mode = InputMode::EditSelect;
                    return Ok(true);
                }
                InputMode::EditMaterials => {
                    // Open material selection UI
                    app.tag_selection = vec![false; app.materials.len()];
                    // Pre-select materials that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, material) in app.materials.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if let Some(ref materials) = product.material.as_ref() {
                                    if materials.contains(material) {
                                        app.tag_selection[i] = true;
                                    }
                                }
                            }
                        }
                    }
                    app.selection_type = Some(crate::models::SelectionType::Material);
                    app.input_mode = InputMode::EditSelect;
                    return Ok(true);
                }
                _ => {
                    // For other fields (Name, Description, Production), Tab does nothing
                    return Ok(true);
                }
            }
        }
        _ => {}
    }

    // Field-specific handling
    match app.input_mode {
        InputMode::EditName => return edit_name::handle(app, key),
        InputMode::EditDescription => return edit_description::handle(app, key),
        InputMode::EditTags => return edit_tags::handle(app, key),
        InputMode::EditMaterials => return edit_materials::handle(app, key),
        InputMode::EditProduction => return edit_production::handle(app, key),
        _ => {}
    }
    Ok(false)
}