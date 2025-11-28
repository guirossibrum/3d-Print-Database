// src/handlers/edit/edit_description.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product description UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditDescription => {
            match key.code {

                KeyCode::Down => {
                    app.input_mode = crate::models::InputMode::EditProduction;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::EditName;
                }
                KeyCode::Backspace => {
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                            && let Some(ref mut desc) = product.description
                        {
                            desc.pop();
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(selected_id) = app.get_selected_product_id() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == Some(selected_id))
                            && let Some(ref mut desc) = product.description
                        {
                            desc.push(c);
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