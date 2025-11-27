// src/handlers/mod.rs
//! Main handlers module exports

use anyhow::Result;
use crossterm::event::KeyEvent;

pub mod delete;
pub mod edit;
pub mod inventory;
pub mod navigation;
pub mod new_item;
pub mod search;
pub mod selection;
pub mod util;

// New modules
pub mod create;
pub mod select;
pub mod normal;

/// Main handler dispatcher - routes based on current input mode
pub fn handle_input(app: &mut crate::App, key: KeyEvent) -> Result<()> {
    use crate::models::InputMode;

    // Global keys that work in any mode (highest priority)
    use crossterm::event::KeyCode;
    match key.code {
        KeyCode::Char('q') => {
            app.running = false;
            return Ok(());
        }
        _ => {}
    }

    // Route to appropriate handler based on current input mode
    match app.input_mode {
        InputMode::Normal => {
            // Normal mode: navigation, search, inventory
            if normal::handle(app, key)? {
                return Ok(());
            }
            if inventory::handle(app, key)? {
                return Ok(());
            }
            if search::handle(app, key)? {
                return Ok(());
            }
            if navigation::handle(app, key)? {
                return Ok(());
            }
        }
        mode if mode.is_create_mode() => {
            // Create modes: form input, selection
            if create::handle(app, key)? {
                return Ok(());
            }
            if select::handle(app, key)? {
                return Ok(());
            }
            if new_item::handle(app, key)? {
                return Ok(());
            }
        }
        mode if mode.is_edit_mode() => {
            // Edit modes: field editing, selection
            if edit::handle(app, key)? {
                return Ok(());
            }
            if select::handle(app, key)? {
                return Ok(());
            }
        }
        mode if mode.is_select_mode() => {
            // Selection modes: tag/material/category selection
            if select::handle(app, key)? {
                return Ok(());
            }
        }
        mode if mode.is_delete_mode() => {
            // Delete confirmation modes
            if delete::handle(app, key)? {
                return Ok(());
            }
        }
        _ => {
            // Fallback for any unhandled modes
            // This should not happen in normal operation
        }
    }

    // Utilities (Ctrl+o folder open) - work in any mode
    if util::handle(app, key)? {
        return Ok(());
    }

    // Key not handled
    Ok(())
}