// src/handlers/edit/edit_categories.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    match app.input_mode {
        crate::state::InputMode::EditCategories | crate::state::InputMode::EditCategorySelect => {
            // Keep the old behavior: Esc cancels, Enter applies selection, Arrow keys navigate.
            use crossterm::event::KeyCode;
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::state::InputMode::EditCategories;
                    app.category_selection.clear();
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Enter => {
                    for (i, &selected) in app.category_selection.iter().enumerate() {
                        if selected {
                            if let Some(category) = app.categories.get(i)
                                && let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                                product.category_id = category.id;
                            }
                            break;
                        }
                    }
                    app.category_selection.clear();
                    app.input_mode = crate::state::InputMode::EditCategories;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Down => {
                    if !app.categories.is_empty() {
                        app.selected_category_index = (app.selected_category_index + 1) % app.categories.len();
                    }
                }
                KeyCode::Up => {
                    if !app.categories.is_empty() {
                        app.selected_category_index = if app.selected_category_index == 0 { app.categories.len() - 1 } else { app.selected_category_index - 1 };
                    }
                }
                KeyCode::Char(' ') => {
                    app.category_selection = vec![false; app.categories.len()];
                    if app.selected_category_index < app.category_selection.len() {
                        app.category_selection[app.selected_category_index] = true;
                    }
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}
