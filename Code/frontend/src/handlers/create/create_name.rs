// src/handlers/create/create_name.rs
//! Handle product name input during creation

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateName => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.name.clear();
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::CreateDescription;
                }
                KeyCode::Up => {
                    // Circular navigation: Name → Materials
                    app.input_mode = crate::models::InputMode::CreateMaterials;
                }
                KeyCode::Backspace => {
                    app.create_form.name.pop();
                }
                KeyCode::Char(c) => {
                    app.create_form.name.push(c);
                }
                KeyCode::Enter => {
                    // Save product
                    app.save_product()?;
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}