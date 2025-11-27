// src/handlers/edit/edit_categories.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product categories UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditCategories => {
            match key.code {
                KeyCode::Esc => {
                    // Cancel changes and return to normal mode
                    if let Some(original) = app.edit_backup.take() {
                        // Restore original product data
                        if let Some(current) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == app.selected_product_id)
                        {
                            *current = original;
                        }
                    }
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                    return Ok(true);
                }
                KeyCode::Enter => {
                    // Category is read-only, navigate to next field
                    app.input_mode = crate::models::InputMode::EditTags;
                    return Ok(true);
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditDescription;
                    return Ok(true);
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditMaterials;
                    return Ok(true);
                }
                KeyCode::Left => {
                    // Already at first field, do nothing
                    return Ok(true);
                }
                KeyCode::Right => {
                    // Already at last field, do nothing
                    return Ok(true);
                }
                KeyCode::Backspace => {
                    // Go back to previous field
                    app.input_mode = crate::models::InputMode::EditName;
                    return Ok(true);
                }
                _ => return Ok(true),
            }
        }
        _ => Ok(false),
    }
}