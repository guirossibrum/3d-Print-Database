// src/handlers/select/material_select.rs
//! Handle material selection UI for both create and edit modes

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateMaterialSelect => {
            match key.code {
                KeyCode::Esc => {
                    let return_mode = match app.input_mode {
                        crate::models::InputMode::CreateMaterialSelect => crate::models::InputMode::CreateMaterials,
                        _ => crate::models::InputMode::Normal,
                    };
                    app.input_mode = return_mode;
                    app.tag_selection.clear();
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                    match app.input_mode {
                        crate::models::InputMode::CreateMaterialSelect => {
                            app.create_form.materials.clear();
                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                                if selected {
                                    if let Some(material) = app.materials.get(i) {
                                        app.create_form.materials.push(material.clone());
                                    }
                                }
                            }
                            app.tag_selection.clear();
                            app.input_mode = crate::models::InputMode::CreateMaterials;
                            app.active_pane = crate::models::ActivePane::Left;
                        }
                        _ => {}
                    }
                }
                KeyCode::Down => {
                    if !app.materials.is_empty() {
                        app.create_form.material_selected_index = (app.create_form.material_selected_index + 1) % app.materials.len();
                    }
                }
                KeyCode::Up => {
                    if !app.materials.is_empty() {
                        app.create_form.material_selected_index = if app.create_form.material_selected_index == 0 {
                            app.materials.len() - 1
                        } else {
                            app.create_form.material_selected_index - 1
                        };
                    }
                }
                KeyCode::Char(' ') => {
                    if app.create_form.material_selected_index < app.tag_selection.len() {
                        let idx = app.create_form.material_selected_index;
                        app.tag_selection[idx] = !app.tag_selection[idx];
                    }
                }
                KeyCode::Char('d') => {
                    // Delete selected material if unused
                    let idx = app.create_form.material_selected_index;
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
                                    if app.create_form.material_selected_index >= app.materials.len() && !app.materials.is_empty() {
                                        app.create_form.material_selected_index = app.materials.len() - 1;
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
        }
        _ => Ok(false),
    }
}