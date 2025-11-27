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

                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditDescription;
                }
                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditTags;
                }
                KeyCode::Left => {
                    // Toggle production to true
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                        {
                            product.production = true;
                        }
                    }
                }
                KeyCode::Right => {
                    // Toggle production to false
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                        {
                            product.production = false;
                        }
                    }
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                        {
                            product.production = true;
                        }
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                        {
                            product.production = false;
                        }
                    }
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}