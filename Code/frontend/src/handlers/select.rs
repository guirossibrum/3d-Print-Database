// handlers/select.rs - Selection mode handlers for choosing tags, materials, categories

use crate::models::InputMode;
use crate::state::App;
use crossterm::event::KeyEvent;
use anyhow::Result;

/// Handle select mode events (when choosing from lists)
pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    use crossterm::event::KeyCode;
    
    match key.code {
        // ESC to cancel selection and return to previous mode
        KeyCode::Esc => {
            app.set_input_mode(InputMode::Normal);
            app.set_status("Selection cancelled".to_string());
        }
        
        // ENTER to confirm selection
        KeyCode::Enter => {
            // TODO: Implement selection confirmation
            app.set_input_mode(InputMode::Normal);
            app.set_status("Selection confirmed".to_string());
        }
        
        // UP/DOWN to navigate selection list
        KeyCode::Up => {
            // TODO: Implement selection navigation
            app.set_status("Navigate up".to_string());
        }
        
        KeyCode::Down => {
            // TODO: Implement selection navigation
            app.set_status("Navigate down".to_string());
        }
        
        // SPACE to toggle selection
        KeyCode::Char(' ') => {
            // TODO: Implement toggle selection
            app.set_status("Toggle selection".to_string());
        }
        
        _ => {}
    }
    
    Ok(())
}