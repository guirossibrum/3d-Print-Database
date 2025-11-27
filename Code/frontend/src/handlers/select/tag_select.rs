// src/handlers/select/tag_select.rs
//! Handle tag selection UI for both create and edit modes

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateTagSelect | crate::models::InputMode::EditTagSelect => {
            match key.code {
                KeyCode::Esc => {
                    let return_mode = match app.input_mode {
                        crate::models::InputMode::CreateTagSelect => crate::models::InputMode::CreateTags,
                        crate::models::InputMode::EditTagSelect => crate::models::InputMode::EditTags,
                        _ => crate::models::InputMode::Normal,
                    };
                    app.input_mode = return_mode;
                    app.tag_selection.clear();
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                KeyCode::Enter => {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                    // Apply selected tags
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                    // Apply selected tags
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                    let target_mode = match app.input_mode {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                    let target_mode = match app.input_mode {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                        crate::models::InputMode::CreateTagSelect => {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                        crate::models::InputMode::CreateTagSelect => {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            app.create_form.tags.clear();
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            app.create_form.tags.clear();
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                if selected {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                if selected {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                    if let Some(tag) = app.tags.get(i) {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                    if let Some(tag) = app.tags.get(i) {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                        app.create_form.tags.push(tag.clone());
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                        app.create_form.tags.push(tag.clone());
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                    }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                    }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            crate::models::InputMode::CreateTags
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            crate::models::InputMode::CreateTags
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                        }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                        }
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                    app.active_pane = crate::models::ActivePane::Right;
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                    app.active_pane = crate::models::ActivePane::Right;
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                        crate::models::InputMode::EditTagSelect => {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                        crate::models::InputMode::EditTagSelect => {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                product.tags.clear();
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                product.tags.clear();
                KeyCode::Tab => {
                    // Open tag selection UI
                    app.tag_selection = vec![false; app.tags.len()];
                    // Pre-select tags that are already in product
                    if let Some(selected_id) = app.get_selected_product_id() {
                        for (i, tag) in app.tags.iter().enumerate() {
                            if let Some(product) = app
                                .products
                                .iter_mut()
                                .find(|p| p.id == Some(selected_id))
                            {
                                if product.tags.contains(tag) {
                                    app.tag_selection[i] = true;
                                }
                            }
                        }
                    }
                    app.tag_select_mode = crate::models::TagSelectMode::Edit;
                    app.input_mode = crate::models::InputMode::EditTagSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }                                for (i, &selected) in app.tag_selection.iter().enumerate() {
                                for (i, &selected) in app.tag_selection.iter().enumerate() {
                                    if selected {
                                    if selected {
                                        if let Some(tag) = app.tags.get(i) {
                                        if let Some(tag) = app.tags.get(i) {
                                            product.tags.push(tag.clone());
                                            product.tags.push(tag.clone());
                                        }
                                        }
                                    }
                                    }
                    }
                    }
                            crate::models::InputMode::EditTags
                            crate::models::InputMode::EditTags
                        }
                        }
                        _ => crate::models::InputMode::Normal,
                        _ => crate::models::InputMode::Normal,
                    };
                    };
                    
                    
                    app.tag_selection.clear();
                    app.tag_selection.clear();
                    app.input_mode = target_mode;
                    app.input_mode = target_mode;
                    app.active_pane = crate::models::ActivePane::Left;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                }
                KeyCode::Down => {
                KeyCode::Down => {
                    if !app.tags.is_empty() {
                    if !app.tags.is_empty() {
                        app.create_form.tag_selected_index = (app.create_form.tag_selected_index + 1) % app.tags.len();
                    }
                }
                KeyCode::Up => {
                    if !app.tags.is_empty() {
                        app.create_form.tag_selected_index = if app.create_form.tag_selected_index == 0 {
                            app.tags.len() - 1
                        } else {
                            app.create_form.tag_selected_index - 1
                        };
                    }
                }
                KeyCode::Char(' ') => {
                    if app.create_form.tag_selected_index < app.tag_selection.len() {
                        let idx = app.create_form.tag_selected_index;
                        app.tag_selection[idx] = !app.tag_selection[idx];
                    }
                }
                KeyCode::Char('d') => {
                    // Delete selected tag if unused
                    let idx = app.create_form.tag_selected_index;
                    if idx < app.tags.len() {
                        let item_to_delete = app.tags[idx].clone();
                        let normalized = crate::handlers::util::normalize_tag_name(&item_to_delete);
                        let in_use = app.products.iter().any(|p| {
                            p.tags.iter().any(|t| crate::handlers::util::normalize_tag_name(t) == normalized)
                        });
                        if in_use {
                            app.set_status_message(format!(
                                "Cannot delete tag '{}' - it is in use by products", 
                                item_to_delete
                            ));
                        } else {
                            match app.api_client.delete_tag(&normalized) {
                                Ok(_) => {
                                    app.tags.retain(|t| t != &item_to_delete);
                                    app.set_status_message(format!("Tag '{}' deleted successfully", item_to_delete));
                                    if app.create_form.tag_selected_index >= app.tags.len() && !app.tags.is_empty() {
                                        app.create_form.tag_selected_index = app.tags.len() - 1;
                                    }
                                }
                                Err(e) => {
                                    app.set_status_message(format!("Error deleting tag '{}': {}", item_to_delete, e));
                                }
                            }
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