// src/handlers/create/create_description.rs
//! Handle product description input during creation

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateDescription => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.production = true; // Reset to default
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::CreateCategory;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::CreateName;
                }
                KeyCode::Backspace => {
                    app.create_form.description.pop();
                }
                KeyCode::Char(c) => {
                    app.create_form.description.push(c);
                }
                KeyCode::Enter => {
                    // Navigate to next field
                    app.input_mode = crate::models::InputMode::CreateCategory;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}