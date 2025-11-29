// handlers/normal.rs - Normal mode handlers for navigation and basic operations

use crate::models::InputMode;
use crate::state::App;
use crossterm::event::KeyEvent;
use anyhow::Result;

/// Handle normal mode events (when not in edit/create/delete modes)
pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    use crossterm::event::KeyCode;
    
    match key.code {
        // Navigation between tabs
        KeyCode::Right => {
            let new_tab = app.current_tab().next();
            app.set_current_tab(new_tab);
            app.set_status(format!("Switched to {:?} tab", app.current_tab()));
        }
        
        KeyCode::Left => {
            let new_tab = app.current_tab().prev();
            app.set_current_tab(new_tab);
            app.set_status(format!("Switched to {:?} tab", app.current_tab()));
        }
        
        // Navigate product list using efficient index management
        KeyCode::Up => {
            if !app.products().is_empty() && app.selected_index() > 0 {
                let new_index = app.selected_index() - 1;
                app.set_selected_index(new_index);
                if let Some(product) = app.selected_product() {
                    app.set_status(format!("Selected: {}", product.name));
                }
            }
        }
        
        KeyCode::Down => {
            if !app.products().is_empty() && app.selected_index() + 1 < app.products().len() {
                let new_index = app.selected_index() + 1;
                app.set_selected_index(new_index);
                if let Some(product) = app.selected_product() {
                    app.set_status(format!("Selected: {}", product.name));
                }
            }
        }
        
        // Enter edit mode with Tab key
        KeyCode::Tab => {
            if app.selected_product_id().is_some() {
                app.set_input_mode(InputMode::EditName);
                app.set_status("Enter edit mode".to_string());
            }
        }
        
        // Create new record
        KeyCode::Char('n') => {
            app.set_input_mode(InputMode::Create);
            app.set_status("Enter create mode".to_string());
        }
        
        // Delete selected record
        KeyCode::Char('d') => {
            if app.selected_product_id().is_some() {
                app.set_input_mode(InputMode::DeleteConfirm);
                app.set_status("Delete confirmation - press 'y' to confirm".to_string());
            }
        }
        
        _ => {}
    }
    
    Ok(())
}