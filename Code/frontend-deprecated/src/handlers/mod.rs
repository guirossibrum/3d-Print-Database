// src/handlers/mod.rs
//! Main handlers module exports

use anyhow::Result;
use crossterm::event::KeyEvent;

pub mod delete;
pub mod edit;
pub mod selection;
pub mod util;
pub mod key_handlers;

/// Main handler dispatcher - key-centric architecture
pub fn handle_input(app: &mut crate::App, key: KeyEvent) -> Result<()> {
    use crossterm::event::KeyCode;

    // Global keys that work in any mode (highest priority)
    match key.code {
        KeyCode::Char('q') => {
            // Only quit if in Normal mode (not during editing)
            if matches!(app.input_mode, crate::models::InputMode::Normal) {
                app.running = false;
                return Ok(());
            }
        }
        _ => {}
    }

    // Key-centric routing - each key has its own handler with mode dispatch inside
    match key.code {
        KeyCode::Esc => key_handlers::handle_escape(app)?,
        KeyCode::Enter => key_handlers::handle_enter(app)?,
        KeyCode::Tab => key_handlers::handle_tab(app)?,
        KeyCode::BackTab => key_handlers::handle_backtab(app)?,
        KeyCode::Up => key_handlers::handle_up(app)?,
        KeyCode::Down => key_handlers::handle_down(app)?,
        KeyCode::Left => key_handlers::handle_left(app)?,
        KeyCode::Right => key_handlers::handle_right(app)?,
        KeyCode::Backspace => key_handlers::handle_backspace(app)?,
        KeyCode::Char('n') => key_handlers::handle_new(app)?,
        KeyCode::Char('d') => key_handlers::handle_delete(app)?,
        KeyCode::Char(' ') => key_handlers::handle_space(app)?,
        KeyCode::Char(c) => key_handlers::handle_char(app, c)?,
        _ => {} // Unhandled keys
    }

    // Utilities (Ctrl+o folder open) - work in any mode
    if util::handle(app, key)? {
        return Ok(());
    }

    Ok(())
}