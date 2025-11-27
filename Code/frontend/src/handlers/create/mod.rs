// src/handlers/create/mod.rs
//! Create mode handlers for product creation workflow

use anyhow::Result;
use crossterm::event::KeyEvent;

pub mod create_name;
pub mod create_description;
pub mod create_category;
pub mod create_production;
pub mod create_tags;
pub mod create_materials;

/// Combined entry point for create-related handlers.
/// Returns Ok(true) if create subsystem handled the key.
pub fn handle(app: &mut crate::App, key: KeyEvent) -> Result<bool> {
    if create_name::handle(app, key)? { return Ok(true); }
    if create_description::handle(app, key)? { return Ok(true); }
    if create_category::handle(app, key)? { return Ok(true); }
    if create_production::handle(app, key)? { return Ok(true); }
    if create_tags::handle(app, key)? { return Ok(true); }
    if create_materials::handle(app, key)? { return Ok(true); }
    Ok(false)
}