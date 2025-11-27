// src/handlers/key_handlers.rs
//! Key-centric handler architecture for consistent key behavior
//!
//! Each key has its own handler function with mode dispatch inside.
//! This ensures consistent behavior across all application modes.

use anyhow::Result;

use crate::App;

/// Handle Enter key - always confirms and saves record
pub fn handle_enter(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Edit modes - save current product
        InputMode::EditName | InputMode::EditDescription |
        InputMode::EditProduction | InputMode::EditTags |
        InputMode::EditMaterials => {
            app.save_current_product()?;
        }

        // Selection modes - apply selection and save
        InputMode::EditTagSelect => {
            // Apply tag selection and save
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.tags.clear();
                for (i, &selected) in app.tag_selection.iter().enumerate() {
                    if selected {
                        if let Some(tag) = app.tags.get(i) {
                            product.tags.push(tag.clone());
                        }
                    }
                }
                app.edit_tags_string = product.tags.join(", ");
            }
            app.tag_selection.clear();
            app.input_mode = InputMode::EditTags; // Set correct mode for save
            app.save_current_product()?;
        }

        InputMode::EditMaterialSelect => {
            // Apply material selection and save
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                let selected_materials: Vec<String> = app.tag_selection.iter()
                    .enumerate()
                    .filter(|&(_, &selected)| selected)
                    .filter_map(|(i, _)| app.materials.get(i).cloned())
                    .collect();
                product.material = if selected_materials.is_empty() {
                    None
                } else {
                    Some(selected_materials)
                };
            }
            app.tag_selection.clear();
            app.save_current_product()?;
        }

        // Create modes - save new product
        InputMode::CreateName | InputMode::CreateDescription |
        InputMode::CreateCategory | InputMode::CreateCategorySelect |
        InputMode::CreateProduction | InputMode::CreateTags |
        InputMode::CreateTagSelect | InputMode::CreateMaterials |
        InputMode::CreateMaterialSelect => {
            // TODO: Implement create product saving
            // For now, do nothing
        }

        // Delete confirmation - confirm deletion
        InputMode::DeleteConfirm | InputMode::DeleteFileConfirm => {
            // TODO: Implement delete confirmation
            // For now, do nothing
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Escape key - always cancels and returns to previous state/mode
pub fn handle_escape(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Edit modes - cancel and return to normal
        InputMode::EditName | InputMode::EditDescription |
        InputMode::EditProduction | InputMode::EditTags |
        InputMode::EditMaterials => {
            // Restore from backup and return to normal
            if let Some(original) = app.edit_backup.take() {
                if let Some(selected_id) = app.get_selected_product_id() {
                    if let Some(current) = app.products.iter_mut().find(|p| p.id == Some(selected_id)) {
                        *current = original;
                    }
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = crate::models::ActivePane::Left;
        }

        // Selection modes - cancel and return to edit mode
        InputMode::EditTagSelect | InputMode::EditMaterialSelect => {
            app.tag_selection.clear();
            app.input_mode = InputMode::EditTags; // Return to appropriate edit mode
            app.active_pane = crate::models::ActivePane::Right;
        }

        // Create modes - cancel creation
        InputMode::CreateName | InputMode::CreateDescription |
        InputMode::CreateCategory | InputMode::CreateCategorySelect |
        InputMode::CreateProduction | InputMode::CreateTags |
        InputMode::CreateTagSelect | InputMode::CreateMaterials |
        InputMode::CreateMaterialSelect => {
            // TODO: Reset create form and return to normal
            app.input_mode = InputMode::Normal;
        }

        // Delete modes - cancel deletion
        InputMode::DeleteConfirm | InputMode::DeleteFileConfirm => {
            app.input_mode = InputMode::Normal;
        }

        // Other modes - no action (never close app)
        _ => {}
    }

    Ok(())
}

/// Handle Tab key - always advances to next level
pub fn handle_tab(app: &mut App) -> Result<()> {
    use crate::models::{InputMode, Tab};

    match app.input_mode {
        // Normal mode - enter edit mode (only in Search tab)
        InputMode::Normal => {
            if matches!(app.current_tab, Tab::Search) && !app.products.is_empty() {
                // Enter edit mode
                app.refresh_data();
                let selected_product = app.get_selected_product().cloned();
                if let Some(product) = selected_product {
                    app.edit_backup = Some(product.clone());
                    app.edit_tags_string = product.tags.join(", ");
                }
                app.active_pane = crate::models::ActivePane::Right;
                app.input_mode = InputMode::EditName;
            }
        }

        // Edit field modes - advance to next field
        InputMode::EditName => {
            app.input_mode = InputMode::EditDescription;
        }
        InputMode::EditDescription => {
            app.input_mode = InputMode::EditProduction;
        }
        InputMode::EditProduction => {
            app.input_mode = InputMode::EditTags;
        }
        InputMode::EditTags => {
            // Enter tag selection
            app.tag_selection = vec![false; app.tags.len()];
            if let Some(selected_id) = app.get_selected_product_id() {
                for (i, tag) in app.tags.iter().enumerate() {
                    if let Some(product) = app.products.iter().find(|p| p.id == Some(selected_id)) {
                        if product.tags.contains(tag) {
                            app.tag_selection[i] = true;
                        }
                    }
                }
            }
            app.input_mode = InputMode::EditTagSelect;
            app.active_pane = crate::models::ActivePane::Right;
        }
        InputMode::EditMaterials => {
            // Enter material selection
            app.tag_selection = vec![false; app.materials.len()];
            if let Some(selected_id) = app.get_selected_product_id() {
                for (i, material) in app.materials.iter().enumerate() {
                    if let Some(product) = app.products.iter().find(|p| p.id == Some(selected_id)) {
                        if let Some(ref materials) = product.material.as_ref() {
                            if materials.contains(material) {
                                app.tag_selection[i] = true;
                            }
                        }
                    }
                }
            }
            app.input_mode = InputMode::EditMaterialSelect;
            app.active_pane = crate::models::ActivePane::Right;
        }

        // Selection modes - no action (or cycle?)
        InputMode::EditTagSelect | InputMode::EditMaterialSelect => {
            // Could cycle through selection items, but for now do nothing
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle BackTab key - not implemented
pub fn handle_backtab(_app: &mut App) -> Result<()> {
    // Backtab not implemented as per AGENTS.md
    Ok(())
}

/// Handle Up arrow - always navigate item lists
pub fn handle_up(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Normal mode - navigate products
        InputMode::Normal => {
            app.prev_filtered_item();
        }

        // Edit field modes - navigate between fields
        InputMode::EditName => {
            // Circular: Name → Materials
            app.input_mode = InputMode::EditMaterials;
        }
        InputMode::EditDescription => {
            app.input_mode = InputMode::EditName;
        }
        InputMode::EditProduction => {
            app.input_mode = InputMode::EditDescription;
        }
        InputMode::EditTags => {
            app.input_mode = InputMode::EditProduction;
        }
        InputMode::EditMaterials => {
            app.input_mode = InputMode::EditTags;
        }

        // Selection modes - navigate selection list
        InputMode::EditTagSelect => {
            if !app.tags.is_empty() {
                app.create_form.tag_selected_index = if app.create_form.tag_selected_index == 0 {
                    app.tags.len() - 1
                } else {
                    app.create_form.tag_selected_index - 1
                };
            }
        }
        InputMode::EditMaterialSelect => {
            if !app.materials.is_empty() {
                app.create_form.material_selected_index = if app.create_form.material_selected_index == 0 {
                    app.materials.len() - 1
                } else {
                    app.create_form.material_selected_index - 1
                };
            }
        }

        // Create modes - navigate form fields
        InputMode::CreateName => {
            // TODO: Implement create form navigation
        }
        // ... other create modes

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Down arrow - always navigate item lists
pub fn handle_down(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Normal mode - navigate products
        InputMode::Normal => {
            app.next_filtered_item();
        }

        // Edit field modes - navigate between fields
        InputMode::EditName => {
            app.input_mode = InputMode::EditDescription;
        }
        InputMode::EditDescription => {
            app.input_mode = InputMode::EditProduction;
        }
        InputMode::EditProduction => {
            app.input_mode = InputMode::EditTags;
        }
        InputMode::EditTags => {
            app.input_mode = InputMode::EditMaterials;
        }
        InputMode::EditMaterials => {
            // Circular: Materials → Name
            app.input_mode = InputMode::EditName;
        }

        // Selection modes - navigate selection list
        InputMode::EditTagSelect => {
            if !app.tags.is_empty() {
                app.create_form.tag_selected_index = (app.create_form.tag_selected_index + 1) % app.tags.len();
            }
        }
        InputMode::EditMaterialSelect => {
            if !app.materials.is_empty() {
                app.create_form.material_selected_index = (app.create_form.material_selected_index + 1) % app.materials.len();
            }
        }

        // Create modes - navigate form fields
        InputMode::CreateName => {
            // TODO: Implement create form navigation
        }
        // ... other create modes

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Left arrow - change tabs in normal mode, toggle in selection
pub fn handle_left(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Normal mode - change tabs
        InputMode::Normal => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
            // Auto-select first item for product tabs
            if matches!(app.current_tab, crate::models::Tab::Search | crate::models::Tab::Inventory) {
                if !app.products.is_empty() {
                    if let Some(first_product) = app.get_filtered_products().first() {
                        if let Some(product_id) = first_product.id {
                            app.selected_product_id = Some(product_id);
                        }
                    }
                }
            }
        }

        // Edit production mode - toggle to false
        InputMode::EditProduction => {
            if let Some(selected_id) = app.get_selected_product_id() {
                if let Some(product) = app.products.iter_mut().find(|p| p.id == Some(selected_id)) {
                    product.production = false;
                }
            }
        }

        // Selection modes - could navigate, but for now no action
        InputMode::EditTagSelect | InputMode::EditMaterialSelect => {
            // Could implement left/right navigation in selection
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Right arrow - change tabs in normal mode, toggle in selection
pub fn handle_right(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Normal mode - change tabs
        InputMode::Normal => {
            app.current_tab = app.current_tab.next();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
            // Auto-select first item for product tabs
            if matches!(app.current_tab, crate::models::Tab::Search | crate::models::Tab::Inventory) {
                if !app.products.is_empty() {
                    if let Some(first_product) = app.get_filtered_products().first() {
                        if let Some(product_id) = first_product.id {
                            app.selected_product_id = Some(product_id);
                        }
                    }
                }
            }
        }

        // Edit production mode - toggle to true
        InputMode::EditProduction => {
            if let Some(selected_id) = app.get_selected_product_id() {
                if let Some(product) = app.products.iter_mut().find(|p| p.id == Some(selected_id)) {
                    product.production = true;
                }
            }
        }

        // Selection modes - could navigate, but for now no action
        InputMode::EditTagSelect | InputMode::EditMaterialSelect => {
            // Could implement left/right navigation in selection
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle 'n' key - always create new record
pub fn handle_new(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Selection modes - create new tag/material
        InputMode::EditTagSelect => {
            // TODO: Implement new tag creation
        }
        InputMode::EditMaterialSelect => {
            // TODO: Implement new material creation
        }
        InputMode::CreateTagSelect => {
            // TODO: Implement new tag creation in create mode
        }
        InputMode::CreateMaterialSelect => {
            // TODO: Implement new material creation in create mode
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle 'd' key - always delete selected record
pub fn handle_delete(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Selection modes - delete selected tag/material
        InputMode::EditTagSelect => {
            // TODO: Implement tag deletion with safety checks
        }
        InputMode::EditMaterialSelect => {
            // TODO: Implement material deletion with safety checks
        }

        // Normal mode - delete product
        InputMode::Normal => {
            if matches!(app.current_tab, crate::models::Tab::Search) && !app.products.is_empty() {
                if let Some(product) = app.get_selected_product() {
                    app.selected_product_for_delete = Some(product.clone());
                    app.input_mode = InputMode::DeleteConfirm;
                }
            }
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Space key - always toggle selection
pub fn handle_space(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Selection modes - toggle selection
        InputMode::EditTagSelect => {
            if app.create_form.tag_selected_index < app.tag_selection.len() {
                let idx = app.create_form.tag_selected_index;
                app.tag_selection[idx] = !app.tag_selection[idx];
            }
        }
        InputMode::EditMaterialSelect => {
            if app.create_form.material_selected_index < app.tag_selection.len() {
                let idx = app.create_form.material_selected_index;
                app.tag_selection[idx] = !app.tag_selection[idx];
            }
        }
        InputMode::CreateTagSelect => {
            if app.create_form.tag_selected_index < app.tag_selection.len() {
                let idx = app.create_form.tag_selected_index;
                app.tag_selection[idx] = !app.tag_selection[idx];
            }
        }
        InputMode::CreateMaterialSelect => {
            if app.create_form.material_selected_index < app.tag_selection.len() {
                let idx = app.create_form.material_selected_index;
                app.tag_selection[idx] = !app.tag_selection[idx];
            }
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}