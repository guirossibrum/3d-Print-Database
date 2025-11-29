// handlers/delete.rs - Delete mode handlers for deletion confirmation

use crate::models::InputMode;
use crate::state::App;
use crossterm::event::KeyEvent;
use anyhow::Result;

/// Handle delete mode events (when confirming deletion)
pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    use crossterm::event::KeyCode;
    
    match key.code {
        // ESC to cancel deletion and return to normal mode
        KeyCode::Esc => {
            app.set_input_mode(InputMode::Normal);
            app.set_status("Delete cancelled".to_string());
        }
        
        // 'y' to confirm deletion
        KeyCode::Char('y') => {
            // TODO: Implement actual deletion
            app.set_input_mode(InputMode::Normal);
            app.set_status("Item deleted".to_string());
        }
        
        // 'n' to cancel deletion
        KeyCode::Char('n') => {
            app.set_input_mode(InputMode::Normal);
            app.set_status("Delete cancelled".to_string());
        }
        
        _ => {}
    }
    
    Ok(())
}