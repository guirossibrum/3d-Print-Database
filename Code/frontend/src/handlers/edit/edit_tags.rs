// src/handlers/edit/edit_tags.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product tags UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditTags => {
            match key.code {
                KeyCode::Esc => {
                    // Cancel changes and return to normal mode
                    if let Some(original) = app.edit_backup.take() {
                        // Restore original product data
                        if let Some(current) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == app.selected_product_id)
                        {
                            *current = original;
                        }
                    }
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Parse and save changes
                    if let Some(product) = app
                        .products
                        .iter_mut()
                        .find(|p| p.id == app.selected_product_id)
                    {
                        product.tags = app
                            .edit_tags_string
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    app.edit_backup = None;
                    let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                        data
                    } else {
                        return Ok(false);
                    };
                    let mut update = crate::api::ProductUpdate::default();
                    update.tags = Some(product.tags.clone());
                    app.perform_update(&sku, update)?;
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    for (i, tag) in app.tags.iter().enumerate() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == app.selected_product_id)
                        {
                            if product.tags.contains(tag) {
                                app.tag_selection[i] = true;
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditCategories;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditMaterials;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}