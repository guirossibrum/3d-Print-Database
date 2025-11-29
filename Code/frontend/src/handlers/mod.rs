// handlers/mod.rs - Module declarations and event dispatcher

pub mod normal;
pub mod edit;
pub mod create;
pub mod delete;
pub mod select;

use anyhow::Result;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use crate::state::App;
use crate::models::InputMode;

/// Main event dispatcher - routes events to appropriate handlers based on input mode
pub fn handle_event(app: &mut App, event: Event) -> Result<()> {
    match event {
        Event::Key(key) => handle_key_event(app, key),
        Event::Mouse(mouse) => handle_mouse_event(app, mouse),
        Event::Resize(_, _) => Ok(()), // Handle terminal resize if needed
    }
}

/// Route keyboard events to appropriate mode-specific handler
fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.input_mode() {
        InputMode::Normal => normal::handle(app, key),
        InputMode::Edit => edit::handle(app, key),
        InputMode::Create => create::handle(app, key),
        InputMode::Select => select::handle(app, key),
        InputMode::Delete => delete::handle(app, key),
    }
}

/// Handle mouse events (basic implementation for now)
fn handle_mouse_event(app: &mut App, _mouse: MouseEvent) -> Result<()> {
    // Basic mouse handling - can be expanded later
    Ok(())
}

// Re-export commonly used handler functionality
pub use normal::*;