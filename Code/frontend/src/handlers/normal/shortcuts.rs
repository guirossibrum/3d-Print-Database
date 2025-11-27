// src/handlers/normal/shortcuts.rs
//! Handle shortcut keys in normal mode

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::{KeyCode, KeyModifiers};

    // Only handle shortcuts in Normal mode
    if app.input_mode != crate::models::InputMode::Normal {
        return Ok(false);
    }

    match key.code {
        // Ctrl+d for delete (only with control modifier)
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Delete functionality for Search tab
            if matches!(app.current_tab, crate::models::Tab::Search)
                && !app.products.is_empty()
                && let Some(product) = app.get_selected_product()
            {
                app.selected_product_for_delete = Some(product.clone());
                app.delete_option = 0;
                app.popup_field = 0;
                app.input_mode = crate::models::InputMode::DeleteConfirm;
            }
        }
        // q or Ctrl+q for quit
        KeyCode::Char('q') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.running = false;
            }
            // If not Ctrl+q, let it fall through to search input
        }
        _ => return Ok(false),
    }

    Ok(true)
}