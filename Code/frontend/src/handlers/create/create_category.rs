// src/handlers/create/create_category.rs
//! Handle category selection during creation

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateCategory => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.description.clear();
                }
                KeyCode::Tab => {
                    app.input_mode = crate::models::InputMode::CreateCategorySelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::CreateProduction;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::CreateDescription;
                }
                KeyCode::Enter => {
                    // Navigate to next field
                    app.input_mode = crate::models::InputMode::CreateProduction;
                }
                _ => {}
            }
            Ok(true)
        }
        crate::models::InputMode::CreateCategorySelect => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.category_id = None;
                    app.create_form.category_selected_index = 0;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Select the current category
                    if let Some(category) = app.categories.get(app.create_form.category_selected_index) {
                        app.create_form.category_id = category.id;
                    }
                    app.input_mode = crate::models::InputMode::CreateCategory;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Down => {
                    if !app.categories.is_empty() {
                        app.create_form.category_selected_index =
                            (app.create_form.category_selected_index + 1) % app.categories.len();
                    }
                }
                KeyCode::Up => {
                    if !app.categories.is_empty() {
                        app.create_form.category_selected_index =
                            if app.create_form.category_selected_index == 0 {
                                app.categories.len() - 1
                            } else {
                                app.create_form.category_selected_index - 1
                            };
                    }
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}