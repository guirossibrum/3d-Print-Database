// src/handlers/key_handlers.rs
//! Key-centric handler architecture for consistent key behavior
//!
//! Each key has its own handler function with mode dispatch inside.
//! This ensures consistent behavior across all application modes.

use anyhow::Result;

use crate::{App, models::{SelectionType, Tab, ActivePane, CategoryForm}};

/// Handle Enter key - confirms and saves record, or applies selection
pub fn handle_enter(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Edit modes - save current product
        InputMode::EditName | InputMode::EditDescription |
        InputMode::EditProduction | InputMode::EditTags |
        InputMode::EditMaterials => {
            app.save_current_product()?;
        }

        // Selection mode - apply selection and return to edit mode
        InputMode::EditSelect => {
            match app.selection_type {
                Some(SelectionType::Category) => {
                    // Apply category selection and return to edit mode
                    if app.selected_category_index < app.categories.len() {
                        if let Some(category) = app.categories.get(app.selected_category_index) {
                            app.current_product.category_id = category.id;
                        }
                    }
                    app.input_mode = InputMode::EditCategory;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                Some(SelectionType::Tag) => {
                    // Apply tag selection and return to edit mode
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
                    app.input_mode = InputMode::EditTags;
                }
                Some(SelectionType::Material) => {
                    // Apply material selection and return to edit mode
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        // Ensure material vec exists
                        if product.material.is_none() {
                            product.material = Some(Vec::new());
                        }
                        if let Some(ref mut materials) = product.material {
                            materials.clear();
                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                                if selected {
                                    if let Some(material) = app.materials.get(i) {
                                        materials.push(material.clone());
                                    }
                                }
                            }
                        }
                        app.edit_materials_string = product.material.as_ref().unwrap_or(&vec![]).join(", ");
                    }
                    app.input_mode = InputMode::EditMaterials;
                }
                _ => {
                    // Unknown selection type, return to normal
                    app.input_mode = InputMode::Normal;
                }
            }
            app.tag_selection.clear();
            app.active_pane = crate::models::ActivePane::Right;
            app.selection_type = None;
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

        // Selection mode - cancel and return to edit mode
        InputMode::EditSelect => {
            app.tag_selection.clear();
            app.input_mode = match app.selection_type {
                Some(SelectionType::Tag) => InputMode::EditTags,
                Some(SelectionType::Material) => InputMode::EditMaterials,
                _ => InputMode::Normal, // Fallback
            };
            app.active_pane = crate::models::ActivePane::Right;
            app.selection_type = None;
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
            if matches!(app.current_tab, Tab::Search) {
                // Enter edit mode - fetch product by SKU from database
                if let Some(product_data) = app.get_selected_product_data() {
                    let (sku, _) = product_data;
                    // Fetch fresh product data by SKU from database
                    match app.api_client.get_product_by_sku(&sku) {
                        Ok(product) => {
                            app.current_product = product.clone();
                            app.edit_backup = Some(product.clone());
                            app.edit_tags_string = product.tags.join(", ");
                            app.edit_materials_string = product.material.as_ref().map(|m| m.join(", ")).unwrap_or_default();
                            app.active_pane = crate::models::ActivePane::Right;
                            app.input_mode = InputMode::EditName;
                        }
                        Err(e) => {
                            app.set_status_message(format!("Error loading product: {:?}", e));
                        }
                    }
                }
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
        InputMode::EditCategory => {
            // Enter category selection
            app.selection_type = Some(SelectionType::Category);
            app.category_selection = vec![false; app.categories.len()];
            // Pre-select current category if any
            if let Some(category_id) = app.current_product.category_id {
                for (i, category) in app.categories.iter().enumerate() {
                    if category.id == Some(category_id) {
                        app.category_selection[i] = true;
                        app.selected_category_index = i;
                        break;
                    }
                }
            }
            app.input_mode = InputMode::EditSelect;
            app.active_pane = crate::models::ActivePane::Right;
        }
        InputMode::EditTags => {
            // Enter tag selection
            app.selection_type = Some(SelectionType::Tag);
            app.tag_selection = vec![false; app.tags.len()];
            // Use current_product for both create and edit
            for (i, tag) in app.tags.iter().enumerate() {
                if app.current_product.tags.contains(tag) {
                    app.tag_selection[i] = true;
                }
            }
            app.input_mode = InputMode::EditSelect;
            app.active_pane = crate::models::ActivePane::Right;
        }
        InputMode::EditMaterials => {
            // Enter material selection
            app.selection_type = Some(SelectionType::Material);
            app.tag_selection = vec![false; app.materials.len()];
            // Use current_product for both create and edit
            for (i, material) in app.materials.iter().enumerate() {
                if let Some(ref materials) = app.current_product.material.as_ref() {
                    if materials.contains(material) {
                        app.tag_selection[i] = true;
                    }
                }
            }
            app.input_mode = InputMode::EditSelect;
            app.active_pane = crate::models::ActivePane::Right;
        }

        // Selection mode - no action (or cycle?)
        InputMode::EditSelect => {
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

        // Selection mode - navigate selection list
        InputMode::EditSelect => {
            match app.selection_type {
                Some(SelectionType::Category) => {
                    if !app.categories.is_empty() {
                        app.selected_category_index = if app.selected_category_index == 0 {
                            app.categories.len() - 1
                        } else {
                            app.selected_category_index - 1
                        };
                    }
                }
                Some(SelectionType::Tag) => {
                    if !app.tags.is_empty() {
                        app.tag_selected_index = if app.tag_selected_index == 0 {
                            app.tags.len() - 1
                        } else {
                            app.tag_selected_index - 1
                        };
                    }
                }
                Some(SelectionType::Material) => {
                    if !app.materials.is_empty() {
                        app.material_selected_index = if app.material_selected_index == 0 {
                            app.materials.len() - 1
                        } else {
                            app.material_selected_index - 1
                        };
                    }
                }
                _ => {}
            }
        }



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

        // Selection mode - navigate selection list
        InputMode::EditSelect => {
            match app.selection_type {
                Some(SelectionType::Category) => {
                    if !app.categories.is_empty() {
                        app.selected_category_index = (app.selected_category_index + 1) % app.categories.len();
                    }
                }
                Some(SelectionType::Tag) => {
                    if !app.tags.is_empty() {
                        app.tag_selected_index = (app.tag_selected_index + 1) % app.tags.len();
                    }
                }
                Some(SelectionType::Material) => {
                    if !app.materials.is_empty() {
                        app.material_selected_index = (app.material_selected_index + 1) % app.materials.len();
                    }
                }
                _ => {}
            }
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Left arrow - change tabs in normal mode, toggle boolean left
pub fn handle_left(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Normal mode - change tabs
        InputMode::Normal => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
            // Clear search queries when leaving search tabs
            if matches!(app.current_tab, crate::models::Tab::Create) {
                app.clear_search_queries();
            }
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

        // Edit production mode - toggle to true (Yes)
        InputMode::EditProduction => {
            if let Some(selected_id) = app.get_selected_product_id() {
                if let Some(product) = app.products.iter_mut().find(|p| p.id == Some(selected_id)) {
                    product.production = true;
                }
            }
        }

        // Selection mode - could navigate, but for now no action
        InputMode::EditSelect => {
            // Could implement left/right navigation in selection
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle Right arrow - change tabs in normal mode, toggle boolean right
pub fn handle_right(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Normal mode - change tabs
        InputMode::Normal => {
            app.current_tab = app.current_tab.next();
            app.active_pane = crate::models::ActivePane::Left;
            app.clear_selection();
            // Clear search queries when leaving search tabs
            if matches!(app.current_tab, crate::models::Tab::Create) {
                app.clear_search_queries();
            }
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

        // Edit production mode - toggle to false (No)
        InputMode::EditProduction => {
            if let Some(selected_id) = app.get_selected_product_id() {
                if let Some(product) = app.products.iter_mut().find(|p| p.id == Some(selected_id)) {
                    product.production = false;
                }
            }
        }

        // Selection mode - could navigate, but for now no action
        InputMode::EditSelect => {
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
        // Normal mode - create new product (only in Create tab)
        InputMode::Normal => {
            if app.current_tab == Tab::Create {
                // Initialize new product and enter EditCategory mode first
                app.current_product = crate::api::Product::default();
                app.active_pane = ActivePane::Right;
                app.input_mode = InputMode::EditCategory;
            }
        }

        // Selection modes - create new tag/material/category (works for both create and edit)
        InputMode::EditSelect => {
            match app.selection_type {
                Some(SelectionType::Tag) => {
                    // TODO: Implement new tag creation
                }
                Some(SelectionType::Material) => {
                    // TODO: Implement new material creation
                }
                Some(SelectionType::Category) => {
                    // Initialize category form and enter new category mode
                    app.category_form = CategoryForm::default();
                    app.input_mode = InputMode::NewCategory;
                    app.active_pane = ActivePane::Right;
                }
                _ => {}
            }
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
        // Selection mode - delete selected tag/material
        InputMode::EditSelect => {
            match app.selection_type {
                Some(SelectionType::Tag) => {
                    // TODO: Implement tag deletion with safety checks
                }
                Some(SelectionType::Material) => {
                    // TODO: Implement material deletion with safety checks
                }
                _ => {}
            }
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
        // Selection mode - toggle selection
        InputMode::EditSelect => {
            match app.selection_type {
                Some(SelectionType::Category) => {
                    // For category, Space just moves selection (same as arrow keys)
                    if !app.categories.is_empty() {
                        app.selected_category_index = (app.selected_category_index + 1) % app.categories.len();
                    }
                }
                Some(SelectionType::Tag) => {
                    if app.tag_selected_index < app.tag_selection.len() {
                        let idx = app.tag_selected_index;
                        app.tag_selection[idx] = !app.tag_selection[idx];
                    }
                }
                Some(SelectionType::Material) => {
                    if app.material_selected_index < app.tag_selection.len() {
                        let idx = app.material_selected_index;
                        app.tag_selection[idx] = !app.tag_selection[idx];
                    }
                }
                _ => {}
            }
        }

        // Other modes - no action
        _ => {}
    }

    Ok(())
}

/// Handle character input (typing)
pub fn handle_char(app: &mut App, c: char) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Edit name - add character to product name
        InputMode::EditName => {
            app.current_product.name.push(c);
        }

        // Edit description - add character to product description
        InputMode::EditDescription => {
            if let Some(ref mut desc) = app.current_product.description {
                desc.push(c);
            } else {
                app.current_product.description = Some(c.to_string());
            }
        }

        // Edit tags - add character to tag string
        InputMode::EditTags => {
            app.edit_tags_string.push(c);
        }

        // Other modes - ignore character input
        _ => {}
    }

    Ok(())
}

/// Handle Backspace key - delete character
pub fn handle_backspace(app: &mut App) -> Result<()> {
    use crate::models::InputMode;

    match app.input_mode {
        // Edit name - remove last character
        InputMode::EditName => {
            app.current_product.name.pop();
        }

        // Edit description - remove last character
        InputMode::EditDescription => {
            if let Some(ref mut desc) = app.current_product.description {
                desc.pop();
            }
        }

        // Edit tags - remove last character from tag string
        InputMode::EditTags => {
            app.edit_tags_string.pop();
        }



        // Other modes - ignore backspace
        _ => {}
    }

    Ok(())
}