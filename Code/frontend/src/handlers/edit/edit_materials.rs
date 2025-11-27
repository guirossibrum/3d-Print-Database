// src/handlers/edit/edit_materials.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product materials UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditMaterials => {
            match key.code {
                KeyCode::Esc | KeyCode::Tab => {
                    // Cancel changes (discard) and return to normal mode
                    if let Some(original) = app.edit_backup.take() {
                        // Restore original product data
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
                }
                KeyCode::Enter => {
                    // Save changes and return to normal mode
                    app.save_current_product()?;
                }
                KeyCode::Tab => {
                    // Open material selection
                    app.tag_selection = vec![false; app.materials.len()];
                    // Pre-select materials that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, material) in app.materials.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
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
                    app.create_form.material_selected_index = 0;
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditMaterialSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditTags;
                }
                KeyCode::Down => {
                    // Circular navigation: Materials → Name
                    app.input_mode = crate::models::InputMode::EditName;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}