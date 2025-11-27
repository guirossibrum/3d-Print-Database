// src/handlers/create/create_production.rs
//! Handle production toggle during creation

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateProduction => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.description.clear();
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::CreateTags;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::CreateCategory;
                }
                KeyCode::Left => {
                    app.create_form.production = true;
                }
                KeyCode::Right => {
                    app.create_form.production = false;
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    app.create_form.production = true;
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.create_form.production = false;
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