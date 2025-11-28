// src/handlers/new_item.rs
use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;


pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    // We only act if we're in a create/new-item related input mode.
    match app.input_mode {
        crate::models::InputMode::CreateTagSelect
        | crate::models::InputMode::CreateMaterialSelect
        | crate::models::InputMode::CreateCategorySelect
        | crate::models::InputMode::CreateName
        | crate::models::InputMode::CreateDescription
        | crate::models::InputMode::EditTag
        | crate::models::InputMode::EditCategory
        | crate::models::InputMode::EditMaterial => {
            match key.code {
                KeyCode::Esc => {
                    // Reset forms and return to previous mode
                    app.item_form = crate::models::TagForm::default();
                    app.input_mode = app.previous_input_mode.unwrap_or(crate::models::InputMode::Normal);
                }
                KeyCode::Enter => {
                    match app.item_type {
                        crate::models::ItemType::Tag => {
                            if matches!(app.input_mode, crate::models::InputMode::EditTag) {
                                edit_tag_handler(app)?;
                            } else {
                                create_tags_handler(app)?;
                            }
                        }
                        crate::models::ItemType::Material => {
                            if matches!(app.input_mode, crate::models::InputMode::EditMaterial) {
                                edit_material_handler(app)?;
                            } else {
                                create_materials_handler(app)?;
                            }
                        }
                        crate::models::ItemType::Category => {
                            if matches!(app.input_mode, crate::models::InputMode::EditCategory) {
                                edit_category_handler(app)?;
                            } else {
                                create_category_handler(app)?;
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    match app.item_type {
                        crate::models::ItemType::Tag => { app.item_form.name.pop(); }
                        crate::models::ItemType::Material => { app.item_form.name.pop(); }
                        crate::models::ItemType::Category => {
                            match app.popup_field {
                                0 => { app.category_form.name.pop(); }
                                1 => { app.category_form.sku.pop(); }
                                2 => { app.category_form.description.pop(); }
                                _ => {}
                            }
                        }
                    }
                }
                KeyCode::Char(c) => {
                    match app.item_type {
                        crate::models::ItemType::Tag => app.item_form.name.push(c),
                        crate::models::ItemType::Material => app.item_form.name.push(c),
                        crate::models::ItemType::Category => {
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
        let created_count = created.len();
        for name in created {
            if let Some(index) = app.tags.iter().position(|t| t == &name) {
                app.tag_selection[index] = true;
                app.create_form.tag_selected_index = index;
            }
        }
        app.set_status_message(format!("{} tags created", created_count));
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
        let created_count = created.len();
        for name in created {
            if let Some(index) = app.materials.iter().position(|t| t == &name) {
                app.tag_selection[index] = true;
                app.create_form.material_selected_index = index;
            }
        }
        app.set_status_message(format!("{} materials created", created_count));
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

fn edit_tag_handler(app: &mut App) -> Result<()> {
    // Save edited tag
    if !app.item_form.name.trim().is_empty() {
        if app.create_form.tag_selected_index < app.tags.len() {
            let old_name = app.tags[app.create_form.tag_selected_index].clone();
            let tag = crate::api::Tag {
                name: old_name.clone(),
                usage_count: 0, // Not used for update
            };
            let mut updated_tag = tag.clone();
            updated_tag.name = app.item_form.name.clone();
            match app.api_client.update_tag(&updated_tag) {
                Ok(_) => {
                    app.tags[app.create_form.tag_selected_index] = app.item_form.name.clone();
                    app.tags.sort();
                    app.create_form.tag_selected_index = app
                        .tags
                        .iter()
                        .position(|t| t == &app.item_form.name)
                        .unwrap_or(app.create_form.tag_selected_index);
                    app.status_message = format!("Tag '{}' updated", app.item_form.name);
                    app.refresh_data();
                }
                Err(e) => {
                    app.set_status_message(format!("Error updating tag: {:?}", e));
                }
            }
        }
    } else {
        app.set_status_message("Error: Tag name required".to_string());
    }
    app.item_form = crate::models::TagForm::default();
    app.input_mode = match app.tag_select_mode {
        crate::models::TagSelectMode::Create => crate::models::InputMode::CreateTagSelect,
        crate::models::TagSelectMode::Edit => {
            app.selection_type = Some(crate::models::SelectionType::Tag);
            crate::models::InputMode::EditSelect
        },
    };
    Ok(())
}

fn edit_material_handler(app: &mut App) -> Result<()> {
    // Save edited material
    if !app.item_form.name.trim().is_empty() {
        if app.create_form.material_selected_index < app.materials.len() {
            let old_name = app.materials[app.create_form.material_selected_index].clone();
            let material = crate::api::Material {
                name: old_name.clone(),
                usage_count: 0, // Not used for update
            };
            let mut updated_material = material.clone();
            updated_material.name = app.item_form.name.clone();
            match app.api_client.update_material(&updated_material) {
                Ok(updated_material) => {
                    app.materials[app.create_form.material_selected_index] = updated_material.name.clone();
                    app.materials.sort();
                    app.create_form.material_selected_index = app
                        .materials
                        .iter()
                        .position(|m| m == &app.item_form.name)
                        .unwrap_or(app.create_form.material_selected_index);
                    app.status_message = format!("Material '{}' updated", app.item_form.name);
                    app.refresh_data();
                }
                Err(e) => {
                    app.set_status_message(format!("Error updating material: {:?}", e));
                }
            }
        }
    } else {
        app.set_status_message("Error: Material name required".to_string());
    }
    app.item_form = crate::models::TagForm::default();
    app.input_mode = match app.tag_select_mode {
        crate::models::TagSelectMode::Create => crate::models::InputMode::CreateMaterialSelect,
        crate::models::TagSelectMode::Edit => {
            app.selection_type = Some(crate::models::SelectionType::Material);
            crate::models::InputMode::EditSelect
        },
    };
    Ok(())
}

fn edit_category_handler(app: &mut App) -> Result<()> {
    // Save edited category
    if !app.category_form.name.trim().is_empty() && app.category_form.sku.len() == 3 {
        if app.create_form.category_selected_index < app.categories.len() {
            let mut category = app.categories[app.create_form.category_selected_index].clone();
            category.name = app.category_form.name.clone();
            category.sku_initials = app.category_form.sku.clone();
            category.description = if app.category_form.description.trim().is_empty() {
                None
            } else {
                Some(app.category_form.description.clone())
            };
            match app.api_client.update_category(&category) {
                Ok(updated_category) => {
                    app.categories[app.create_form.category_selected_index] = updated_category;
                    app.categories.sort_by(|a, b| a.name.cmp(&b.name));
                    app.create_form.category_selected_index = app
                        .categories
                        .iter()
                        .position(|c| c.name == app.category_form.name)
                        .unwrap_or(app.create_form.category_selected_index);
                    app.status_message = format!("Category '{}' updated", app.category_form.name);
                    app.refresh_data();
                }
                Err(e) => {
                    app.set_status_message(format!("Error updating category: {:?}", e));
                }
            }
        }
    } else {
        app.set_status_message("Error: Name required, SKU must be 3 letters".to_string());
    }
    app.category_form = crate::models::CategoryForm::default();
    app.popup_field = 0;
    app.input_mode = crate::models::InputMode::CreateCategorySelect;
    Ok(())
}
