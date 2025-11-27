// src/handlers/edit/edit_materials.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product materials UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditMaterials => {
            match key.code {

                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditTags;
                }
                KeyCode::Down => {
                    // Circular navigation: Materials → Name
                    app.input_mode = crate::models::InputMode::EditName;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}