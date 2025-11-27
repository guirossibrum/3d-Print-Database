// src/handlers/edit/edit_materials.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::state::InputMode::EditMaterials => {
            match key.code {
                KeyCode::Esc => {
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
                        // Full update (similar to other handlers)
                        update.name = Some(product.name.clone());
                        update.description = product.description.clone();
                        update.tags = Some(product.tags.clone());
                        update.production = Some(product.production);
                        update.material = Some(product.material.clone().unwrap_or_default());
                        update.color = product.color;
                        update.print_time = product.print_time;
                        update.weight = product.weight;
                        update.stock_quantity = product.stock_quantity;
                        update.reorder_point = product.reorder_point;
                        update.unit_cost = product.unit_cost;
                        update.selling_price = product.selling_price;
                        app.perform_update(&sku, update)?;
                    }
                    app.input_mode = crate::state::InputMode::Normal;
                    app.active_pane = crate::state::ActivePane::Left;
                }
                KeyCode::Tab => {
                    // Open material selection
                    app.tag_selection = vec![false; app.materials.len()];
                    if let Some(product) = app.products.iter().find(|p| p.id == app.selected_product_id) {
                        for (i, material) in app.materials.iter().enumerate() {
                            if product.material.as_ref().map(|m| m.contains(material)).unwrap_or(false) {
                                app.tag_selection[i] = true;
                            }
                        }
                    }
                    app.create_form.material_selected_index = 0;
                    app.tag_select_mode = crate::state::TagSelectMode::Edit;
                    app.input_mode = crate::state::InputMode::EditMaterialSelect;
                    app.active_pane = crate::state::ActivePane::Right;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}
