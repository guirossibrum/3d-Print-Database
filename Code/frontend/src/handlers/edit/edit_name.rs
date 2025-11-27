// src/handlers/edit/edit_name.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product name UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditName => {
            match key.code {

                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditDescription;
                }
                KeyCode::Up => {
                    // Circular navigation: Name → Materials
                    app.input_mode = crate::models::InputMode::EditMaterials;
                }
                KeyCode::Backspace => {
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                        {
                            product.name.pop();
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                        {
                            product.name.push(c);
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