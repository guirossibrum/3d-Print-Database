// src/handlers/edit/edit_description.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::state::InputMode::EditDescription => {
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
                    app.edit_backup = None;
                    if let Some((sku, product)) = app.get_selected_product_data() {
                        let mut update = crate::api::ProductUpdate::default();
                        update.description = product.description.clone();
                        app.perform_update(&sku, update)?;
                    }
                    app.input_mode = crate::state::InputMode::Normal;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Down => {
                    app.input_mode = crate::state::InputMode::CreateProduction; // or EditProduction
                }
                KeyCode::Up => {
                    app.input_mode = crate::state::InputMode::EditName;
                }
                KeyCode::Backspace => {
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        if let Some(ref mut desc) = product.description {
                            desc.pop();
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        if let Some(ref mut desc) = product.description {
                            desc.push(c);
                        } else {
                            product.description = Some(c.to_string());
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
