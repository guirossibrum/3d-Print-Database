// src/handlers/edit/edit_materials.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

/// Handle editing product materials UI.
/// Returns Ok(true) if handled.
pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::EditMaterials => {
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
                    let (sku, _product) = if let Some(data) = app.get_selected_product_data() {
                        data
                    } else {
                        return Ok(false);
                    };
                    if let Some(product) = app
                        .products
                        .iter_mut()
                        .find(|p| p.id == app.selected_product_id)
                    {
                        let mut update = crate::api::ProductUpdate::default();
                        update.name = Some(product.name.clone());
                        update.description = product.description.clone();
                        update.tags = Some(product.tags.clone());
                        update.production = Some(product.production);
                        update.material = product.material.clone();
                        update.color = product.color.clone();
                        update.print_time = product.print_time;
                        update.weight = product.weight;
                        update.stock_quantity = product.stock_quantity;
                        update.reorder_point = product.reorder_point;
                        update.unit_cost = product.unit_cost;
                        update.selling_price = product.selling_price;
                        app.perform_update(&sku, update)?;
                    }
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Tab => {
                    // Open material selection
                    app.tag_selection = vec![false; app.materials.len()];
                    // Pre-select materials that are already in product
                    for (i, material) in app.materials.iter().enumerate() {
                        if let Some(product) = app
                            .products
                            .iter_mut()
                            .find(|p| p.id == app.selected_product_id)
                        {
                            if let Some(ref materials) = product.material.as_ref() {
                                if materials.contains(material) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.create_form.material_selected_index = 0;
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditMaterialSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }
                KeyCode::Up => {
                    // Circular navigation: Materials → Name
                    app.input_mode = crate::models::InputMode::CreateName;
                }
                KeyCode::Down => {
                    // Circular navigation: Materials → Tags
                    app.input_mode = crate::models::InputMode::CreateTags;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}