// src/handlers/normal/search_input.rs
//! Handle direct typing in search boxes

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    // Only handle search input in Normal mode
    if app.input_mode != crate::models::InputMode::Normal {
        return Ok(false);
    }

    match key.code {
        KeyCode::Char(c) => {
            // Direct typing in search box for Search and Inventory tabs
            if matches!(app.current_tab, crate::models::Tab::Search) {
                app.search_query.push(c);
                app.clear_selection(); // Reset selection when typing
            } else if matches!(app.current_tab, crate::models::Tab::Inventory) {
                app.inventory_search_query.push(c);
                app.clear_selection(); // Reset selection when typing
            } else {
                return Ok(false);
            }
        }
        KeyCode::Backspace => {
            // Handle backspace for search boxes
            if matches!(app.current_tab, crate::models::Tab::Search) && !app.search_query.is_empty() {
                app.search_query.pop();
                app.clear_selection(); // Reset selection when typing
            } else if matches!(app.current_tab, crate::models::Tab::Inventory)
                && !app.inventory_search_query.is_empty()
            {
                app.inventory_search_query.pop();
                app.clear_selection(); // Reset selection when typing
            } else {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }

    Ok(true)
}