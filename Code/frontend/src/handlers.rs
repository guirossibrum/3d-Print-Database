// src/handlers.rs
//! Top-level handler dispatcher. Keeps behavior identical while delegating to smaller modules.

use anyhow::Result;
use crossterm::event::KeyEvent;

pub mod handlers_internal {
    pub use crate::handlers::delete;
    pub use crate::handlers::edit;
    pub use crate::handlers::inventory;
    pub use crate::handlers::navigation;
    pub use crate::handlers::new_item;
    pub use crate::handlers::search;
    pub use crate::handlers::selection;
    pub use crate::handlers::util;
}

pub fn handle_input(app: &mut crate::app::App, key: KeyEvent) -> Result<()> {
    // Module order: specific handlers first (edit/new/delete), then generic ones.
    // Each module returns Ok(true) if it handled the key for the current app.input_mode.
    if handlers_internal::edit::handle(app, key)? {
        return Ok(());
    }
    if handlers_internal::new_item::handle(app, key)? {
        return Ok(());
    }
    if handlers_internal::delete::handle(app, key)? {
        return Ok(());
    }
    if handlers_internal::search::handle(app, key)? {
        return Ok(());
    }
    if handlers_internal::inventory::handle(app, key)? {
        return Ok(());
    }
    if handlers_internal::navigation::handle(app, key)? {
        return Ok(());
    }

    // If none of the above handled the event, fall back to selection & util modules (they
    // only act on specific input_mode values and return false otherwise).
    if handlers_internal::selection::handle(app, key)? {
        return Ok(());
    }

    // Last attempt: util module for miscellaneous keystrokes (e.g., open folder shortcut).
    if handlers_internal::util::handle(app, key)? {
        return Ok(());
    }

    // Not handled here.
    Ok(())
}
