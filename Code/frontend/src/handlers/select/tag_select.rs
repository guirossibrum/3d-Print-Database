// src/handlers/select/tag_select.rs
//! Handle tag selection UI for both create and edit modes

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditSelect => {
            if matches!(app.selection_type, Some(crate::models::SelectionType::Tag)) {
                match key.code {
                    KeyCode::Esc => {
                        app.input_mode = crate::models::InputMode::EditTags;
                        app.tag_selection.clear();
                        app.active_pane = crate::models::ActivePane::Left;
                    }
                    KeyCode::Enter => {
                        // Apply tag selection to current product
                        app.current_product.tags.clear();
                        for (i, &selected) in app.tag_selection.iter().enumerate() {
                            if selected {
                                if let Some(tag) = app.tags.get(i) {
                                    app.current_product.tags.push(tag.clone());
                                }
                            }
                        }
                        app.tag_selection.clear();
                        app.input_mode = crate::models::InputMode::EditTags;
                        app.active_pane = crate::models::ActivePane::Left;
                    }
                    KeyCode::Down => {
                        if !app.tags.is_empty() {
                            app.tag_selected_index = (app.tag_selected_index + 1) % app.tags.len();
                        }
                    }
                    KeyCode::Up => {
                        if !app.tags.is_empty() {
                            app.tag_selected_index = if app.tag_selected_index == 0 {
                                app.tags.len() - 1
                            } else {
                                app.tag_selected_index - 1
                            };
                        }
                    }
                    KeyCode::Char(' ') => {
                        if app.tag_selected_index < app.tag_selection.len() {
                            let idx = app.tag_selected_index;
                            app.tag_selection[idx] = !app.tag_selection[idx];
                        }
                    }
                    KeyCode::Char('d') => {
                        // Delete selected tag if unused
                        let idx = app.tag_selected_index;
                        if idx < app.tags.len() {
                            let item_to_delete = app.tags[idx].clone();
                            let normalized = crate::handlers::util::normalize_tag_name(&item_to_delete);
                            let in_use = app.products.iter().any(|p| {
                                p.tags.iter().any(|t| crate::handlers::util::normalize_tag_name(t) == normalized)
                            });
                            if in_use {
                                app.set_status_message(format!(
                                    "Cannot delete tag '{}' - it is in use by products",
                                    item_to_delete
                                ));
                            } else {
                                match app.api_client.delete_tag(&normalized) {
                                    Ok(_) => {
                                        app.tags.retain(|t| t != &item_to_delete);
                                        app.set_status_message(format!("Tag '{}' deleted successfully", item_to_delete));
                                        if app.tag_selected_index >= app.tags.len() && !app.tags.is_empty() {
                                            app.tag_selected_index = app.tag_selected_index - 1;
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
            } else {
                Ok(false)
            }
        }
                    KeyCode::Enter => {
                        // Apply tag selection to current product
                        app.current_product.tags.clear();
                        for (i, &selected) in app.tag_selection.iter().enumerate() {
                            if selected {
                                if let Some(tag) = app.tags.get(i) {
                                    app.current_product.tags.push(tag.clone());
                                }
                            }
                        }
                        app.tag_selection.clear();
                        app.input_mode = crate::models::InputMode::EditTags;
                        app.active_pane = crate::models::ActivePane::Left;
                    }
                    KeyCode::Down => {
                        if !app.tags.is_empty() {
                            app.tag_selected_index = (app.tag_selected_index + 1) % app.tags.len();
                        }
                    }
                    KeyCode::Up => {
                        if !app.tags.is_empty() {
                            app.tag_selected_index = if app.tag_selected_index == 0 {
                                app.tags.len() - 1
                            } else {
                                app.tag_selected_index - 1
                            };
                        }
                    }
                    KeyCode::Char(' ') => {
                        if app.tag_selected_index < app.tag_selection.len() {
                            let idx = app.tag_selected_index;
                            app.tag_selection[idx] = !app.tag_selection[idx];
                        }
                    }
                    KeyCode::Char('d') => {
                        // Delete selected tag if unused
                        let idx = app.tag_selected_index;
                        if idx < app.tags.len() {
                            let item_to_delete = app.tags[idx].clone();
                            let normalized = crate::handlers::util::normalize_tag_name(&item_to_delete);
                            let in_use = app.products.iter().any(|p| {
                                p.tags.iter().any(|t| crate::handlers::util::normalize_tag_name(t) == normalized)
                            });
                            if in_use {
                                app.set_status_message(format!(
                                    "Cannot delete tag '{}' - it is in use by products",
                                    item_to_delete
                                ));
                            } else {
                                match app.api_client.delete_tag(&normalized) {
                                    Ok(_) => {
                                        app.tags.retain(|t| t != &item_to_delete);
                                        app.set_status_message(format!("Tag '{}' deleted successfully", item_to_delete));
                                        if app.tag_selected_index >= app.tags.len() && !app.tags.is_empty() {
                                            app.tag_selected_index = app.tag_selected_index - 1;
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
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}
