// src/handlers/select/mod.rs
//! Selection mode handlers for tag and material selection

use anyhow::Result;
use crossterm::event::KeyEvent;

pub mod tag_select;
pub mod material_select;

/// Combined entry point for selection-related handlers.
/// Returns Ok(true) if selection subsystem handled the key.
pub fn handle(app: &mut crate::App, key: KeyEvent) -> Result<bool> {
    if tag_select::handle(app, key)? { return Ok(true); }
    if material_select::handle(app, key)? { return Ok(true); }
    Ok(false)
}