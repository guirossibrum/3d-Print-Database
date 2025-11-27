// src/handlers/edit/edit_tags.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;
use crate::handlers::selection;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::state::InputMode::EditTags => {
            match key.code {
                KeyCode::Esc => {
                    if let Some(original) = app.edit_backup.take() {
                        if let Some(current) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                            *current = original;
                        }
                    }
                    app.input_mode = crate::state::InputMode::Normal;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Parse and save tags string
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        product.tags = app
                            .edit_tags_string
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    app.edit_backup = None;

                    if let Some((sku, product)) = app.get_selected_product_data() {
                        let mut update = crate::api::ProductUpdate::default();
                        // Use full update helper if desired (not changing backend API)
                        update.name = Some(product.name.clone());
                        update.description = product.description.clone();
                        update.tags = Some(product.tags.clone());
                        update.production = Some(product.production);
                        // Full update to ensure backend state matches UI
                        update.material = Some(product.material.clone().unwrap_or_default());
                        update.color = product.color;
                        update.print_time = product.print_time;
                        update.weight = product.weight;
                        update.stock_quantity = product.stock_quantity;
                        update.reorder_point = product.reorder_point;
                        update.unit_cost = product.unit_cost;
                        update.selling_price = product.selling_price;

                        app.perform_update(&sku, update)?;
                    }
                    app.input_mode = crate::state::InputMode::Normal;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                _ => {}
            }
            Ok(true)
        }
        crate::state::InputMode::EditTagSelect => {
            // Tag selection UI (toggle, navigate)
            match key.code {
                KeyCode::Char(' ') => {
                    if app.create_form.tag_selected_index < app.tag_selection.len() {
                        let idx = app.create_form.tag_selected_index;
                        app.tag_selection[idx] = !app.tag_selection[idx];
                    }
                }
                KeyCode::Down => {
                    if !app.tags.is_empty() {
                        app.create_form.tag_selected_index = (app.create_form.tag_selected_index + 1) % app.tags.len();
                    }
                }
                KeyCode::Up => {
                    if !app.tags.is_empty() {
                        app.create_form.tag_selected_index = if app.create_form.tag_selected_index == 0 { app.tags.len() - 1 } else { app.create_form.tag_selected_index - 1 };
                    }
                }
                KeyCode::Char('d') => {
                    // delete selected tag if unused
                    let idx = app.create_form.tag_selected_index;
                    if idx < app.tags.len() {
                        let item_to_delete = app.tags[idx].clone();
                        let normalized = crate::handlers::util::normalize_tag_name(&item_to_delete);
                        let in_use = app.products.iter().any(|p| p.tags.iter().any(|t| crate::handlers::util::normalize_tag_name(t) == normalized));
                        if in_use {
                            app.set_status_message(format!("Cannot delete tag '{}' - it is in use by products", item_to_delete));
                        } else {
                            match app.api_client.delete_tag(&normalized) {
                                Ok(_) => {
                                    app.tags.retain(|t| t != &item_to_delete);
                                    app.set_status_message(format!("Tag '{}' deleted successfully", item_to_delete));
                                    if app.create_form.tag_selected_index >= app.tags.len() && !app.tags.is_empty() {
                                        app.create_form.tag_selected_index = app.tags.len() - 1;
                                    }
                                }
                                Err(e) => {
                                    app.set_status_message(format!("Error deleting tag '{}': {}", item_to_delete, e));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}
