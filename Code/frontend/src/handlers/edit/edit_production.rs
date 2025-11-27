// src/handlers/edit/edit_production.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing production toggle UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditProduction => {
            match key.code {
                KeyCode::Esc | KeyCode::Tab => {
                    // Cancel changes (discard) and return to normal mode
                    if let Some(original) = app.edit_backup.take() {
                        // Restore original product data
                        if let Some(current) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == app.selected_product_id)
                        {
                            *current = original;
                        }
                    }
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Save changes and return to normal mode
                    app.edit_backup = None; // Clear backup since we're saving
                    let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                        data
                    } else {
                        return Ok(false);
                    };
                    let mut update = crate::api::ProductUpdate::default();
                    update.production = Some(product.production);
                    app.perform_update(&sku, update)?;
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditDescription;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditCategories;
                }
                KeyCode::Left => {
                    // Already at first field, do nothing
                }
                KeyCode::Right => {
                    app.input_mode = crate::models::InputMode::EditName;
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if let Some(product) = app
                        .products
                        .iter_mut()
                        .find(|p| p.id == app.selected_product_id)
                        {
                            product.production = true;
                        }
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    if let Some(product) = app
                        .products
                        .iter_mut()
                        .find(|p| p.id == app.selected_product_id)
                        {
                            product.production = false;
                        }
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}