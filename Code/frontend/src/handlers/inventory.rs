// src/handlers/inventory.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::state::InputMode::Normal => {
            if matches!(app.current_tab, crate::state::Tab::Inventory) {
                match key.code {
                    KeyCode::Char(c) => {
                        app.inventory_search_query.push(c);
                        app.clear_selection();
                        return Ok(true);
                    }
                    KeyCode::Backspace => {
                        if !app.inventory_search_query.is_empty() {
                            app.inventory_search_query.pop();
                            app.clear_selection();
                            return Ok(true);
                        }
                    }
                    _ => {}
                }
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}
