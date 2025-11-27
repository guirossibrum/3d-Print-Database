// src/handlers/search.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::state::InputMode::Normal => {
            // only act here if in search tab and direct typing should affect the search box
            if matches!(app.current_tab, crate::state::Tab::Search) {
                match key.code {
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                        app.clear_selection();
                        return Ok(true);
                    }
                    KeyCode::Backspace => {
                        if !app.search_query.is_empty() {
                            app.search_query.pop();
                            app.clear_selection();
                            return Ok(true);
                        }
                    }
                    KeyCode::Enter => {
                        // Direct edit from normal mode (legacy behavior)
                        if !app.products.is_empty() {
                            app.input_mode = crate::state::InputMode::EditName;
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
