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
                        if let Some(selected_id) = app.get_selected_product_id() {
                            if let Some(current) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                *current = original;
                            }
                        }
                    }
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                    return Ok(true);
                }
                KeyCode::Enter => {
                    // Save changes and return to normal mode
                    app.save_current_product()?;
                    return Ok(true);
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditProduction;
                    return Ok(true);
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditTags;
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