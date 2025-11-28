// src/handlers/edit/edit_tags.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product tags UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditTags => {
            match key.code {

                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditProduction;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditMaterials;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}