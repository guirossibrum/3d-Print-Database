// src/handlers/edit/edit_name.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;

/// Handle editing product name UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::state::InputMode::EditName => {
            match key.code {
                KeyCode::Esc | KeyCode::Tab => {
                    if let Some(original) = app.edit_backup.take() {
                        if let Some(current) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                            *current = original;
                        }
                    }
                    app.input_mode = crate::state::InputMode::Normal;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Save changes
                    app.edit_backup = None;
                    if let Some((sku, product)) = app.get_selected_product_data() {
                        let mut update = crate::api::ProductUpdate::default();
                        update.name = Some(product.name.clone());
                        // Use helper perform_update; ensure it ends with ; in call sites.
                        app.perform_update(&sku, update)?;
                    }
                    app.input_mode = crate::state::InputMode::Normal;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Down => {
                    app.input_mode = crate::state::InputMode::EditDescription;
                }
                KeyCode::Backspace => {
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        product.name.pop();
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        product.name.push(c);
                    }
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}
