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

/// Main handler dispatcher - delegates to appropriate subsystems
pub fn handle_input(app: &mut crate::App, key: KeyEvent) -> Result<()> {
    // Priority order: specific modes first, then general navigation
    
    // Edit modes (highest priority)
    if edit::handle(app, key)? {
        return Ok(());
    }
    
    // Create modes
    if create::handle(app, key)? {
        return Ok(());
    }
    
    // Selection modes
    if select::handle(app, key)? {
        return Ok(());
    }
    
    // Delete modes
    if delete::handle(app, key)? {
        return Ok(());
    }
    
    // New item creation (tag/material/category)
    if new_item::handle(app, key)? {
        return Ok(());
    }
    
    // Normal mode navigation
    if normal::handle(app, key)? {
        return Ok(());
    }
    
    // Search and inventory (fallback)
    if search::handle(app, key)? {
        return Ok(());
    }
    if inventory::handle(app, key)? {
        return Ok(());
    }
    
    // Global navigation (Esc, etc.)
    if navigation::handle(app, key)? {
        return Ok(());
    }
    
    // Utilities (Ctrl+o folder open)
    if util::handle(app, key)? {
        return Ok(());
    }
    
    // Not handled here.
    Ok(())
}