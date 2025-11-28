// src/handlers/select/material_select.rs
//! Handle material selection UI for both create and edit modes

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditSelect => {
            if matches!(app.selection_type, Some(crate::models::SelectionType::Material)) {
                match key.code {
                    KeyCode::Esc => {
                        app.input_mode = crate::models::InputMode::EditMaterials;
                        app.tag_selection.clear();
                        app.active_pane = crate::models::ActivePane::Left;
                    }
                    KeyCode::Enter => {
                        // Apply material selection to current product
                        app.current_product.material = Some(vec![]);
                        if let Some(ref mut materials) = app.current_product.material {
                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                                if selected {
                                    if let Some(material) = app.materials.get(i) {
                                        materials.push(material.clone());
                                    }
                                }
                            }
                        }
                        app.tag_selection.clear();
                        app.input_mode = crate::models::InputMode::EditMaterials;
                        app.active_pane = crate::models::ActivePane::Left;
                    }
                    KeyCode::Down => {
                        if !app.materials.is_empty() {
                            app.material_selected_index = (app.material_selected_index + 1) % app.materials.len();
                        }
                    }
                    KeyCode::Up => {
                        if !app.materials.is_empty() {
                            app.material_selected_index = if app.material_selected_index == 0 {
                                app.materials.len() - 1
                            } else {
                                app.material_selected_index - 1
                            };
                        }
                    }
                    KeyCode::Char(' ') => {
                        if app.material_selected_index < app.tag_selection.len() {
                            let idx = app.material_selected_index;
                            app.tag_selection[idx] = !app.tag_selection[idx];
                        }
                    }
                    KeyCode::Char('d') => {
                        // Delete selected material if unused
                        let idx = app.material_selected_index;
                        if idx < app.materials.len() {
                            let item_to_delete = app.materials[idx].clone();
                            let in_use = app.products.iter().any(|p| {
                                p.material.as_ref()
                                    .map(|m| m.contains(&item_to_delete))
                                    .unwrap_or(false)
                            });
                            if in_use {
                                app.set_status_message(format!(
                                    "Cannot delete material '{}' - it is in use by products",
                                    item_to_delete
                                ));
                            } else {
                                match app.api_client.delete_material(&item_to_delete) {
                                    Ok(_) => {
                                        app.materials.retain(|m| m != &item_to_delete);
                                        app.set_status_message(format!("Material '{}' deleted successfully", item_to_delete));
                                        if app.material_selected_index >= app.materials.len() && !app.materials.is_empty() {
                                            app.material_selected_index = app.materials.len() - 1;
                                        }
                                    }
                                    Err(e) => {
                                        app.set_status_message(format!("Error deleting material '{}': {}", item_to_delete, e));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}