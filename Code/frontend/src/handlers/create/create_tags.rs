// src/handlers/create/create_tags.rs
//! Handle tag input and selection during creation

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateTags => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.tags.clear();
                    app.create_form = crate::models::CreateForm {
                        production: true, // Reset to default
                        ..Default::default()
                    };
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Navigate to next field
                    app.input_mode = crate::models::InputMode::CreateDescription;
                }
                KeyCode::Tab => {
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in create_form.tags
                    for (i, tag) in app.tags.iter().enumerate() {
                        if app.create_form.tags.contains(tag) {
                            app.tag_selection[i] = true;
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Create;
                    app.input_mode = crate::models::InputMode::CreateTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::CreateProduction;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::CreateMaterials;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}