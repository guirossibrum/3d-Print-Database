// src/handlers/normal/mod.rs
//! Normal mode handlers for navigation and shortcuts

use anyhow::Result;
use crossterm::event::KeyEvent;

pub mod navigation;
pub mod shortcuts;
pub mod search_input;

/// Combined entry point for normal mode handlers.
/// Returns Ok(true) if normal mode subsystem handled the key.
pub fn handle(app: &mut crate::App, key: KeyEvent) -> Result<bool> {
    if navigation::handle(app, key)? { return Ok(true); }
    if shortcuts::handle(app, key)? { return Ok(true); }
    if search_input::handle(app, key)? { return Ok(true); }
    Ok(false)
}