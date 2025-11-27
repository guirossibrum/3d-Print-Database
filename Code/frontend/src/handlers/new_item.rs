// src/handlers/new_item.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::app::App;
use crate::handlers::selection;
use crate::handlers::util;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    // We only act if we're in a create/new-item related input mode.
    match app.input_mode {
        crate::state::InputMode::NewItem
        | crate::state::InputMode::CreateTagSelect
        | crate::state::InputMode::CreateMaterialSelect
        | crate::state::InputMode::CreateCategorySelect
        | crate::state::InputMode::CreateName
        | crate::state::InputMode::CreateDescription => {
            match key.code {
                KeyCode::Esc => {
                    // Reset forms and return to previous mode
                    app.item_form = crate::models::TagForm::default();
                    app.input_mode = app.previous_input_mode.unwrap_or(crate::state::InputMode::Normal);
                }
                KeyCode::Enter => {
                    match app.item_type {
                        crate::state::ItemType::Tag => {
                            create_tags_handler(app)?;
                        }
                        crate::state::ItemType::Material => {
                            create_materials_handler(app)?;
                        }
                        crate::state::ItemType::Category => {
                            create_category_handler(app)?;
                        }
                    }
                }
                KeyCode::Backspace => {
                    match app.item_type {
                        crate::state::ItemType::Tag => { app.item_form.name.pop(); }
                        crate::state::ItemType::Material => { app.item_form.name.pop(); }
                        crate::state::ItemType::Category => {
                            match app.popup_field {
                                0 => app.category_form.name.pop(),
                                1 => app.category_form.sku.pop(),
                                2 => app.category_form.description.pop(),
                                _ => {}
                            }
                        }
                    }
                }
                KeyCode::Char(c) => {
                    match app.item_type {
                        crate::state::ItemType::Tag => app.item_form.name.push(c),
                        crate::state::ItemType::Material => app.item_form.name.push(c),
                        crate::state::ItemType::Category => {
                            match app.popup_field {
                                0 => app.category_form.name.push(c),
                                1 => if app.category_form.sku.len() < 3 { app.category_form.sku.push(c) },
                                2 => app.category_form.description.push(c),
                                _ => {}
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

fn create_tags_handler(app: &mut App) -> Result<()> {
    // Split comma-separated, trim, dedupe, call api_client.create_tag for each new tag.
    if app.item_form.name.trim().is_empty() {
        app.set_status_message("Error: Tag name required".to_string());
        return Ok(());
    }

    let tag_names: Vec<String> = app
        .item_form
        .name
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let mut created = Vec::new();
    for t in tag_names {
        if app.tags.contains(&t) {
            continue;
        }
        let tag = crate::api::Tag { name: t.clone(), usage_count: 0 };
        match app.api_client.create_tag(&tag) {
            Ok(created_tag) => {
                app.tags.push(created_tag.name.clone());
                created.push(created_tag.name);
            }
            Err(e) => {
                app.set_status_message(format!("Error creating tag '{}': {:?}", t, e));
            }
        }
    }

    if !created.is_empty() {
        // remap selection to keep previously selected
        app.tags.sort();
        app.refresh_data();
        app.tag_selection.resize(app.tags.len(), false);
        // Preselect newly created tags
        for name in created {
            if let Some(index) = app.tags.iter().position(|t| t == &name) {
                app.tag_selection[index] = true;
                app.create_form.tag_selected_index = index;
            }
        }
        app.set_status_message(format!("{} tags created", created.len()));
    }

    app.item_form = crate::models::TagForm::default();
    Ok(())
}

fn create_materials_handler(app: &mut App) -> Result<()> {
    // Mirrored logic from tags but for materials
    if app.item_form.name.trim().is_empty() {
        app.set_status_message("Error: Material name required".to_string());
        return Ok(());
    }

    let material_names: Vec<String> = app
        .item_form
        .name
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let mut created = Vec::new();
    for m in material_names {
        if app.materials.contains(&m) { continue; }
        let material = crate::api::Material { name: m.clone(), usage_count: 0 };
        match app.api_client.create_material(&material) {
            Ok(created_mat) => {
                app.materials.push(created_mat.name.clone());
                created.push(created_mat.name);
            }
            Err(e) => {
                app.set_status_message(format!("Error creating material '{}': {:?}", m, e));
            }
        }
    }

    if !created.is_empty() {
        app.materials.sort();
        app.refresh_data();
        app.tag_selection.resize(app.materials.len(), false);
        for name in created {
            if let Some(index) = app.materials.iter().position(|t| t == &name) {
                app.tag_selection[index] = true;
                app.create_form.material_selected_index = index;
            }
        }
        app.set_status_message(format!("{} materials created", created.len()));
    }

    app.item_form = crate::models::TagForm::default();
    Ok(())
}

fn create_category_handler(app: &mut App) -> Result<()> {
    if app.category_form.name.trim().is_empty() || app.category_form.sku.len() != 3 {
        app.set_status_message("Error: Name required, SKU must be 3 letters".to_string());
        return Ok(());
    }

    let category = crate::api::Category {
        id: None,
        name: app.category_form.name.clone(),
        sku_initials: app.category_form.sku.clone(),
        description: if app.category_form.description.trim().is_empty() { None } else { Some(app.category_form.description.clone()) },
    };

    match app.api_client.create_category(&category) {
        Ok(_created) => {
            app.set_status_message(format!("Category '{}' created", category.name));
            app.refresh_data();
            app.create_form.category_selected_index = app.categories.iter().position(|c| c.name == category.name).unwrap_or(0);
        }
        Err(e) => {
            app.set_status_message(format!("Error creating category: {:?}", e));
        }
    }

    app.category_form = crate::models::CategoryForm::default();
    app.popup_field = 0;
    Ok(())
}
