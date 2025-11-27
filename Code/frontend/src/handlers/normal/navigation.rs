// src/handlers/normal/navigation.rs
//! Handle navigation keys in normal mode

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    // Only handle navigation in Normal mode
    if app.input_mode != crate::models::InputMode::Normal {
        return Ok(false);
    }

    match key.code {
        KeyCode::Tab => {
            if app.has_multiple_panes()
                && matches!(app.active_pane, crate::models::ActivePane::Left)
                && !app.products.is_empty()
            {
                // Refresh data before editing
                app.refresh_data();
                // Backup current product for potential cancellation
                if let Some(product) = app.get_selected_product() {
                    app.edit_backup = Some(product.clone());
                }
                // Initialize edit_tags_string with current product tags
                if let Some(product) = app.get_selected_product() {
                    app.edit_tags_string = product.tags.join(", ");
                }
                // Switch to right pane and enter edit mode
                app.active_pane = crate::models::ActivePane::Right;
                app.input_mode = crate::models::InputMode::EditName;
            } else if app.has_multiple_panes() {
                // Regular pane switching
                app.next_pane();
            }
        }
        KeyCode::BackTab if app.has_multiple_panes() => {
            app.prev_pane();
        }
        KeyCode::BackTab => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Always use filtered navigation (filter returns all items when empty)
            app.next_filtered_item();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Always use filtered navigation (filter returns all items when empty)
            app.prev_filtered_item();
        }
        KeyCode::Left => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
            app.refresh_data();
            
            // Auto-select first item for tabs with product lists
            if matches!(app.current_tab, crate::models::Tab::Search | crate::models::Tab::Inventory) {
                if !app.products.is_empty() {
                    if let Some(first_product) = app.get_filtered_products().first() {
                        if let Some(product_id) = first_product.id {
                            app.selected_product_id = Some(product_id);
                        }
                    }
                }
            }
        }
        KeyCode::Right => {
            app.current_tab = app.current_tab.next();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
            app.refresh_data();
            
            // Auto-select first item for tabs with product lists
            if matches!(app.current_tab, crate::models::Tab::Search | crate::models::Tab::Inventory) {
                if !app.products.is_empty() {
                    if let Some(first_product) = app.get_filtered_products().first() {
                        if let Some(product_id) = first_product.id {
                            app.selected_product_id = Some(product_id);
                        }
                    }
                }
            }
        }
        KeyCode::Enter => {
            if matches!(app.current_tab, crate::models::Tab::Create) {
                app.input_mode = crate::models::InputMode::CreateName;
            } else if matches!(app.current_tab, crate::models::Tab::Search) && !app.products.is_empty() {
                // Direct edit from normal mode (legacy behavior)
                app.input_mode = crate::models::InputMode::EditName;
            }
        }
        _ => return Ok(false),
    }

    Ok(true)
}