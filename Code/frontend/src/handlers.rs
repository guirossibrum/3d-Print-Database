use anyhow::Result;
use crossterm::event::KeyCode;

use crate::models::*;

/// Dispatch key events to appropriate handler functions
pub fn handle_key_dispatch(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::CreateName => handle_create_name_mode(app, key),
        InputMode::CreateDescription => handle_create_description_mode(app, key),
        InputMode::CreateCategory => handle_create_category_mode(app, key),
        InputMode::CreateCategorySelect => handle_create_category_select_mode(app, key),
        InputMode::CreateProduction => handle_create_production_mode(app, key),
        InputMode::CreateTags => handle_create_tags_mode(app, key),
        InputMode::CreateMaterials => handle_create_materials_mode(app, key),
        InputMode::CreateTagSelect => handle_tag_select_mode(app, key),
        InputMode::CreateMaterialSelect => handle_material_select_mode(app, key),
        InputMode::EditName => handle_edit_name_mode(app, key),
        InputMode::EditDescription => handle_edit_description_mode(app, key),
        InputMode::EditProduction => handle_edit_production_mode(app, key),
        InputMode::EditCategories => handle_edit_categories_mode(app, key),
        InputMode::EditTags => handle_edit_tags_mode(app, key),
        InputMode::EditTagSelect => handle_tag_select_mode(app, key),
        InputMode::EditMaterials => handle_edit_materials_mode(app, key),
        InputMode::EditMaterialSelect => handle_material_select_mode(app, key),
        InputMode::NewTag | InputMode::NewCategory | InputMode::NewMaterial => handle_new_item_mode(app, key),
        InputMode::EditTag | InputMode::EditCategory | InputMode::EditMaterial => handle_edit_item_mode(app, key),
        InputMode::DeleteConfirm => handle_delete_confirm_mode(app, key),
        InputMode::DeleteFileConfirm => handle_delete_file_confirm_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.running = false;
        }
        // q or Ctrl+q for quit
        KeyCode::Char('q') => {
            if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                app.running = false;
            }
            // If not Ctrl+q, let it fall through to search input
        }
        KeyCode::Tab => {
            if app.has_multiple_panes()
                && matches!(app.active_pane, ActivePane::Left)
                && !app.products.is_empty()
            {
                // Refresh data before editing
                app.refresh_data();
                // Backup current product for potential cancellation
                if let Some(product) = app.get_selected_product() {
                    app.edit_backup = Some(product.clone());
                }
                // Initialize edit_tags_string with current product tags
                if let Some(product) = app.get_selected_product() {
                    app.edit_tags_string = product.tags.join(", ");
                }
                // Switch to right pane and enter edit mode
                app.active_pane = ActivePane::Right;
                app.input_mode = InputMode::EditName;
            } else if app.has_multiple_panes() {
                // Regular pane switching
                app.next_pane();
            }
        }
        KeyCode::BackTab if app.has_multiple_panes() => {
            app.prev_pane();
        }
        KeyCode::BackTab => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = ActivePane::Left;
            app.clear_selection();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Always use filtered navigation (filter returns all items when empty)
            app.next_filtered_item();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Always use filtered navigation (filter returns all items when empty)
            app.prev_filtered_item();
        }
        KeyCode::Left => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = ActivePane::Left;
            app.clear_selection();
            app.refresh_data();
        }
        KeyCode::Right => {
            app.current_tab = app.current_tab.next();
            app.active_pane = ActivePane::Left;
            app.clear_selection();
            app.refresh_data();
        }
        // Ctrl+d for delete (only with control modifier)
        KeyCode::Char('d') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
            // Delete functionality for Search tab
            if matches!(app.current_tab, Tab::Search) && !app.products.is_empty()
                && let Some(product) = app.get_selected_product() {
                app.selected_product_for_delete = Some(product.clone());
                app.delete_option = 0;
                app.popup_field = 0;
                app.input_mode = InputMode::DeleteConfirm;
            }
        }
        // Ctrl+o for open (only with control modifier)
        KeyCode::Char('o') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
            // Open product folder functionality for Search tab
            if matches!(app.current_tab, Tab::Search) && !app.products.is_empty()
                && let Some(product) = app.get_selected_product() {
                match open_product_folder(&product.sku) {
                    Ok(_) => {
                        app.set_status_message(format!("Opened folder for product {}", product.sku));
                    }
                    Err(e) => {
                        app.set_status_message(format!("Error opening folder: {}", e));
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            // Direct typing in search box for Search and Inventory tabs
            if matches!(app.current_tab, Tab::Search) {
                app.search_query.push(c);
                app.clear_selection(); // Reset selection when typing
            } else if matches!(app.current_tab, Tab::Inventory) {
                app.inventory_search_query.push(c);
                app.clear_selection(); // Reset selection when typing
            }
        }
        KeyCode::Backspace => {
            // Handle backspace for search boxes
            if matches!(app.current_tab, Tab::Search) && !app.search_query.is_empty() {
                app.search_query.pop();
                app.clear_selection(); // Reset selection when typing
            } else if matches!(app.current_tab, Tab::Inventory) && !app.inventory_search_query.is_empty() {
                app.inventory_search_query.pop();
                app.clear_selection(); // Reset selection when typing
            }
        }
        KeyCode::Enter => {
            match app.input_mode {
                InputMode::Normal => {
                    if matches!(app.current_tab, Tab::Create) {
                        app.input_mode = InputMode::CreateName;
                    } else if matches!(app.current_tab, Tab::Search)
                        && !app.products.is_empty()
                    {
                        // Direct edit from normal mode (legacy behavior)
                        app.input_mode = InputMode::EditName;
                    }
                }
                InputMode::EditName
                    | InputMode::EditDescription
                    | InputMode::EditProduction
                    | InputMode::EditCategories
                    | InputMode::EditTags
                    | InputMode::EditMaterials => {
                    // Save changes and return to normal mode
                    app.input_mode = InputMode::Normal;
                    app.active_pane = ActivePane::Left;
                    // TODO: Persist changes to backend
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}



fn handle_create_name_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.name.clear();
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateDescription;
        }
        KeyCode::Up => {
            // Circular navigation: Name → Materials
            app.input_mode = InputMode::CreateMaterials;
        }
        KeyCode::Backspace => {
            app.create_form.name.pop();
        }
        KeyCode::Char(c) => {
            app.create_form.name.push(c);
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_description_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.production = true; // Reset to default
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateCategory;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateName;
        }
        KeyCode::Backspace => {
            app.create_form.description.pop();
        }
        KeyCode::Char(c) => {
            app.create_form.description.push(c);
        }
        KeyCode::Enter => {
            // Navigate to next field
            app.input_mode = InputMode::CreateCategory;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_category_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.description.clear();
        }
        KeyCode::Tab => {
            app.input_mode = InputMode::CreateCategorySelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateProduction;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateDescription;
        }
        KeyCode::Enter => {
            // Navigate to next field
            app.input_mode = InputMode::CreateProduction;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_category_select_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.category_id = None;
            app.create_form.category_selected_index = 0;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Select the current category
            if let Some(category) = app
                .categories
                .get(app.create_form.category_selected_index)
            {
                app.create_form.category_id = category.id;
            }
            app.input_mode = InputMode::CreateCategory;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            if !app.categories.is_empty() {
                app.create_form.category_selected_index =
                    (app.create_form.category_selected_index + 1) % app.categories.len();
            }
        }
        KeyCode::Up => {
            if !app.categories.is_empty() {
                app.create_form.category_selected_index =
                    if app.create_form.category_selected_index == 0 {
                        app.categories.len() - 1
                    } else {
                        app.create_form.category_selected_index - 1
                    };
            }
        }
        KeyCode::Char('n') => {
            // Create new category
            app.item_type = ItemType::Category;
            app.category_form = CategoryForm::default();
            app.popup_field = 0;
            app.previous_input_mode = Some(InputMode::CreateCategorySelect);
            app.input_mode = InputMode::NewCategory;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_production_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.description.clear();
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateTags;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateCategory;
        }
        KeyCode::Left => {
            app.create_form.production = true;
        }
        KeyCode::Right => {
            app.create_form.production = false;
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.create_form.production = true;
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.create_form.production = false;
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_tags_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.tags.clear();
            app.create_form = CreateForm {
                production: true, // Reset to default
                ..Default::default()
            };
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Navigate to next field
            app.input_mode = InputMode::CreateDescription;
        }
        KeyCode::Tab => {
            app.tag_selection = vec![false; app.tags.len()];
            // Pre-select tags that are already in create_form.tags
            for (i, tag) in app.tags.iter().enumerate() {
                if app.create_form.tags.contains(tag) {
                    app.tag_selection[i] = true;
                }
            }
            app.tag_select_mode = TagSelectMode::Create;
            app.input_mode = InputMode::CreateTagSelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateProduction;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateMaterials;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_materials_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.materials.clear();
            app.create_form = CreateForm {
                production: true, // Reset to default
                ..Default::default()
            };
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Tab => {
            app.tag_selection = vec![false; app.materials.len()];
            // Pre-select materials that are already in create_form.materials
            for (i, material) in app.materials.iter().enumerate() {
                if app.create_form.materials.contains(material) {
                    app.tag_selection[i] = true;
                }
            }
            app.create_form.material_selected_index = 0; // Initialize selection index
            app.item_type = ItemType::Material;
            app.input_mode = InputMode::CreateMaterialSelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateTags;
        }
        KeyCode::Down => {
            // Circular navigation: Materials → Name
            app.input_mode = InputMode::CreateName;
        }
        _ => {}
    }
    Ok(())
}

fn handle_tag_select_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = match app.tag_select_mode {
                TagSelectMode::Create => InputMode::CreateTags,
                TagSelectMode::Edit => InputMode::EditTags,
            };
            app.tag_selection.clear();
            app.active_pane = ActivePane::Left; // Return to left pane
        }
        KeyCode::Enter => {
            // Handle based on context (Create vs Edit)
            match app.tag_select_mode {
                TagSelectMode::Create => {
                    // Add selected tags to create_form.tags
                    app.create_form.tags.clear();
                    for (i, &selected) in app.tag_selection.iter().enumerate() {
                        if selected
                            && let Some(tag) = app.tags.get(i) {
                                app.create_form.tags.push(tag.clone());
                            }
                    }
                    app.tag_selection.clear();
                    app.input_mode = InputMode::CreateTags;
                    app.active_pane = ActivePane::Left;
                }
                TagSelectMode::Edit => {
                    // Update product tags with selected tags
                    if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        product.tags.clear();
                        for (i, &selected) in app.tag_selection.iter().enumerate() {
                            if selected
                                && let Some(tag) = app.tags.get(i) {
                                    product.tags.push(tag.clone());
                                }
                        }
                        // Update edit_tags_string to reflect changes
                        app.edit_tags_string = product.tags.join(", ");
                    }
                    app.tag_selection.clear();
                    app.input_mode = InputMode::EditTags;
                    app.active_pane = ActivePane::Right; // Return to Product Details pane (right)
                }
            }
        }
        KeyCode::Down => {
            if !app.tags.is_empty() {
                app.create_form.tag_selected_index =
                    (app.create_form.tag_selected_index + 1) % app.tags.len();
            }
        }
        KeyCode::Up => {
            if !app.tags.is_empty() {
                app.create_form.tag_selected_index =
                    if app.create_form.tag_selected_index == 0 {
                        app.tags.len() - 1
                    } else {
                        app.create_form.tag_selected_index - 1
                    };
            }
        }
        KeyCode::Char(' ') => {
            // Toggle selection
            if app.create_form.tag_selected_index < app.tag_selection.len() {
                app.tag_selection[app.create_form.tag_selected_index] =
                    !app.tag_selection[app.create_form.tag_selected_index];
            }
        }
        KeyCode::Char('d') => {
            // Delete selected tag
            if app.create_form.tag_selected_index < app.tags.len() {
                let tag_to_delete = app.tags[app.create_form.tag_selected_index].clone();
                
                // Normalize tag name to match backend behavior
                let normalized_tag = normalize_tag_name(&tag_to_delete);
                
                // Check if tag is in use by any product (checking normalized names)
                let tag_in_use = app.products.iter().any(|p| {
                    p.tags.iter().any(|t| normalize_tag_name(t) == normalized_tag)
                });
                
                if tag_in_use {
                    app.set_status_message(format!("Cannot delete tag '{}' - it is in use by products", tag_to_delete));
                } else {
                    // Delete tag from backend (use normalized name)
                    match app.api_client.delete_tag(&normalized_tag) {
                        Ok(_) => {
                            // Remove tag from local list
                            app.tags.retain(|t| t != &tag_to_delete);
                            
                            // Adjust selected index if needed
                            if app.create_form.tag_selected_index >= app.tags.len() && !app.tags.is_empty() {
                                app.create_form.tag_selected_index = app.tags.len() - 1;
                            }
                            
                            app.set_status_message(format!("Tag '{}' deleted successfully", tag_to_delete));
                        }
                        Err(e) => {
                            app.set_status_message(format!("Error deleting tag '{}': {}", tag_to_delete, e));
                        }
                    }
                }
            }
        }
        KeyCode::Char('n') => {
            // Create new tag
            app.item_type = ItemType::Tag;
            app.tag_form = TagForm::default();
            app.previous_input_mode = Some(app.input_mode);
            app.input_mode = InputMode::NewTag;
        }
        _ => {}
    }
    Ok(())
}


fn handle_edit_name_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                data
            } else {
                return Ok(());
            };
            let mut update = crate::api::ProductUpdate::default();
            update.name = Some(product.name);
            app.perform_update(&sku, update)?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditDescription;
        }
        KeyCode::Up => {
            // Already at first field, do nothing
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
    Ok(())
}

fn handle_edit_description_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                data
            } else {
                return Ok(());
            };
            let mut update = crate::api::ProductUpdate::default();
            update.description = product.description;
            app.perform_update(&sku, update)?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditProduction;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditName;
        }
        KeyCode::Backspace => {
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id)
                && let Some(ref mut desc) = product.description
            {
                desc.pop();
            }
        }
        KeyCode::Char(c) => {
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.description.get_or_insert_with(String::new).push(c);
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_production_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                data
            } else {
                return Ok(());
            };
            let mut update = crate::api::ProductUpdate::default();
            update.production = Some(product.production);
            app.perform_update(&sku, update)?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditDescription;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditCategories;
        }
        KeyCode::Left => {
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.production = true;
            }
        }
        KeyCode::Right => {
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.production = false;
            }
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.production = true;
            }
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.production = false;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_categories_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            // Cancel changes and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Category is read-only, navigate to next field
            app.input_mode = InputMode::EditTags;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditProduction;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditTags;
        }
        _ => {}
    }
    Ok(())
}

#[allow(dead_code)]
fn handle_category_select_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::EditCategories;
            app.category_selection.clear();
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Apply selected category to product
            for (i, &selected) in app.category_selection.iter().enumerate() {
                if selected {
                    if let Some(category) = app.categories.get(i)
                        && let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                        product.category_id = category.id;
                    }
                    break; // Only one category can be selected
                }
            }
            app.category_selection.clear();
            app.input_mode = InputMode::EditCategories;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            if !app.categories.is_empty() {
                app.selected_category_index = (app.selected_category_index + 1) % app.categories.len();
            }
        }
        KeyCode::Up => {
            if !app.categories.is_empty() {
                app.selected_category_index = if app.selected_category_index == 0 {
                    app.categories.len() - 1
                } else {
                    app.selected_category_index - 1
                };
            }
        }
        KeyCode::Char(' ') => {
            // Toggle selection (only one category can be selected)
            app.category_selection = vec![false; app.categories.len()];
            if app.selected_category_index < app.category_selection.len() {
                app.category_selection[app.selected_category_index] = true;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_materials_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            // Cancel changes and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                data
            } else {
                return Ok(());
            };
            let mut update = crate::api::ProductUpdate::default();
            update.name = Some(product.name);
            update.description = product.description;
            update.tags = Some(product.tags);
            update.production = Some(product.production);
            update.material = Some(product.material.unwrap_or_default());
            update.color = product.color;
            update.print_time = product.print_time;
            update.weight = product.weight;
            update.stock_quantity = product.stock_quantity;
            update.reorder_point = product.reorder_point;
            update.unit_cost = product.unit_cost;
            update.selling_price = product.selling_price;
            app.perform_update(&product_data.sku, update)?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Tab => {
            // Open material selection
            app.tag_selection = vec![false; app.materials.len()];
            // Pre-select materials that are already in the current product
            if let Some(product) = app.products.iter().find(|p| p.id == app.selected_product_id) {
                for (i, material) in app.materials.iter().enumerate() {
                    if product.material.as_ref()
                        .map(|m| m.contains(material))
                        .unwrap_or(false) {
                        app.tag_selection[i] = true;
                    }
                }
            }
            app.create_form.material_selected_index = 0; // Initialize selection index
            app.input_mode = InputMode::EditMaterialSelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditTags;
        }
        _ => {}
    }
    Ok(())
}

fn handle_material_select_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            // Auto-apply current selections before exiting
            let mut selected_materials = Vec::new();
            for (i, &selected) in app.tag_selection.iter().enumerate() {
                if selected
                    && let Some(material) = app.materials.get(i) {
                    selected_materials.push(material.to_string());
                }
            }
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.material = if selected_materials.is_empty() {
                    None
                } else {
                    Some(selected_materials)
                };
            }
            app.tag_selection.clear();
            app.input_mode = InputMode::EditMaterials;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Apply selected materials to product
            let mut selected_materials = Vec::new();
            for (i, &selected) in app.tag_selection.iter().enumerate() {
                if selected
                    && let Some(material) = app.materials.get(i) {
                    selected_materials.push(material.to_string());
                }
            }
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.material = if selected_materials.is_empty() {
                    None
                } else {
                    Some(selected_materials)
                };
            }
            app.tag_selection.clear();
            app.input_mode = InputMode::EditMaterials;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Down => {
            if !app.materials.is_empty() {
                app.create_form.material_selected_index = (app.create_form.material_selected_index + 1) % app.materials.len();
            }
        }
        KeyCode::Up => {
            if !app.materials.is_empty() {
                app.create_form.material_selected_index = if app.create_form.material_selected_index == 0 {
                    app.materials.len() - 1
                } else {
                    app.create_form.material_selected_index - 1
                };
            }
        }
        KeyCode::Char(' ') => {
            // Toggle selection
            if app.create_form.material_selected_index < app.tag_selection.len() {
                app.tag_selection[app.create_form.material_selected_index] =
                    !app.tag_selection[app.create_form.material_selected_index];
            }
        }
        KeyCode::Char('n') => {
            // Create new material
            app.item_type = ItemType::Material;
            app.tag_form = TagForm::default(); // Reuse TagForm for Material
            app.previous_input_mode = Some(app.input_mode);
            app.input_mode = InputMode::NewMaterial;
        }
        KeyCode::Char('d') => {
            // Delete selected material
            if app.create_form.material_selected_index < app.materials.len() {
                let material_to_delete = app.materials[app.create_form.material_selected_index].clone();

                // Check if material is in use by any product
                let material_in_use = app.products.iter().any(|p| {
                    p.material.as_ref()
                        .is_some_and(|materials| materials.contains(&material_to_delete))
                });

                if material_in_use {
                    app.status_message = format!("Cannot delete material '{}' - it is in use by products", material_to_delete);
                } else {
                    // Delete material from backend
                    match app.api_client.delete_material(&material_to_delete) {
                        Ok(_) => {
                            // Remove material from local list
                            app.materials.retain(|m| m != &material_to_delete);

                            // Adjust selected index if needed
                   if app.create_form.material_selected_index >= app.materials.len() && !app.materials.is_empty() {
                        app.create_form.material_selected_index = app.materials.len() - 1;
                            }

                            app.set_status_message(format!("Material '{}' deleted successfully", material_to_delete));
                        }
                        Err(e) => {
                            app.set_status_message(format!("Error deleting material '{}': {}", material_to_delete, e));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_tags_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            // Cancel changes and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Parse and save changes
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.tags = app
                    .edit_tags_string
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            app.edit_backup = None;
            let (sku, product) = if let Some(data) = app.get_selected_product_data() {
                data
            } else {
                return Ok(());
            };
            let mut update = crate::api::ProductUpdate::default();
            update.name = Some(product.name);
            update.description = product.description;
            update.tags = Some(product.tags);
            update.production = Some(product.production);
            app.perform_update(&sku, update)?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Tab => {
            // Parse current edit_tags_string to product.tags
            if let Some(product) = app.products.iter_mut().find(|p| p.id == app.selected_product_id) {
                product.tags = app
                    .edit_tags_string
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            app.tag_selection = vec![false; app.tags.len()];
            // Pre-select tags that are already in the current product
            if let Some(product) = app.products.iter().find(|p| p.id == app.selected_product_id) {
                for (i, tag) in app.tags.iter().enumerate() {
                    if product.tags.contains(tag) {
                        app.tag_selection[i] = true;
                    }
                }
            }
            app.tag_select_mode = TagSelectMode::Edit;
            app.input_mode = InputMode::EditTagSelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditCategories;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditMaterials;
        }
        _ => {}
    }
    Ok(())
}

fn handle_new_item_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            match app.item_type {
                ItemType::Tag => {
                    app.tag_form = TagForm::default();
                    app.input_mode = app.previous_input_mode.unwrap_or(InputMode::CreateTagSelect);
                }
                ItemType::Category => {
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
                ItemType::Material => {
                    app.tag_form = TagForm::default(); // Reuse TagForm for Material
                    app.input_mode = app.previous_input_mode.unwrap_or(InputMode::CreateMaterialSelect);
                }
            }
        }
        KeyCode::Enter => {
            match app.item_type {
                ItemType::Tag => {
                    // Save new tag(s) - support comma-separated
                    if !app.tag_form.name.trim().is_empty() {
                        let tag_names: Vec<String> = app.tag_form.name
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        
                        let mut created_count = 0;
                        let mut created_tag_names = Vec::new();
                        
                        for tag_name in tag_names {
                            // Skip if tag already exists
                            if app.tags.contains(&tag_name) {
                                continue;
                            }
                            
                            let tag = crate::api::Tag {
                                name: tag_name.clone(),
                                usage_count: 0,
                            };
                            match app.api_client.create_tag(&tag) {
                                Ok(created_tag) => {
                                    app.tags.push(created_tag.name.clone());
                                    created_tag_names.push(created_tag.name.clone());
                                    created_count += 1;
                                }
                                Err(e) => {
                                    app.set_status_message(format!("Error creating tag '{}': {:?}", tag_name, e));
                                }
                            }
                        }
                        
                        if created_count > 0 {
                            // Remember currently selected tags before sorting
                            let mut previously_selected = Vec::new();
                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                                if selected && i < app.tags.len()
                                    && let Some(tag) = app.tags.get(i) {
                                    previously_selected.push(tag.clone());
                                }
                            }

                            app.tags.sort();
                            app.refresh_data();

                            // Update tag selection array to match new tags length
                            app.tag_selection.resize(app.tags.len(), false);

                            // Re-select previously selected tags
                            for tag in &previously_selected {
                                if let Some(index) = app.tags.iter().position(|t| t == tag) {
                                    app.tag_selection[index] = true;
                                }
                            }

                            // Pre-select all newly created tags
                            let mut first_new_index = None;
                            for created_tag_name in &created_tag_names {
                                if let Some(index) = app.tags.iter().position(|t| t == created_tag_name) {
                                    app.tag_selection[index] = true;
                                    if first_new_index.is_none() {
                                        first_new_index = Some(index);
                                    }
                                }
                            }
                            
                            // Set selection index to first new tag if available
                            if let Some(first_index) = first_new_index {
                                app.create_form.tag_selected_index = first_index;
                            }
                            
                            let message = if created_count == 1 {
                                format!("Tag '{}' created", created_tag_names.first().unwrap_or(&String::new()))
                            } else {
                                format!("{} tags created", created_count)
                            };
                            app.set_status_message(message);
                        }
                    } else {
                        app.set_status_message("Error: Tag name required".to_string());
                    }
                    app.tag_form = TagForm::default();
                    app.input_mode = match app.tag_select_mode {
                        TagSelectMode::Create => InputMode::CreateTagSelect,
                        TagSelectMode::Edit => InputMode::EditTagSelect,
                    };
                }
                ItemType::Category => {
                    // Save new category
                    if !app.category_form.name.trim().is_empty() && app.category_form.sku.len() == 3 {
                        let category = crate::api::Category {
                            id: None,
                            name: app.category_form.name.clone(),
                            sku_initials: app.category_form.sku.clone(),
                            description: if app.category_form.description.trim().is_empty() {
                                None
                            } else {
                                Some(app.category_form.description.clone())
                            },
                        };
                        match app.api_client.create_category(&category) {
                            Ok(_created_category) => {
                                app.status_message =
                                    format!("Category '{}' created", app.category_form.name);
                                app.refresh_data(); // Refresh to get latest data including new category
                                // Set selection index after refresh
                                app.create_form.category_selected_index = app
                                    .categories
                                    .iter()
                                    .position(|c| c.name == app.category_form.name)
                                    .unwrap_or(0);
                            }
                            Err(e) => {
                                app.set_status_message(format!("Error creating category: {:?}", e));
                            }
                        }
                    } else {
                        app.set_status_message("Error: Name required, SKU must be 3 letters".to_string());
                    }
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
                ItemType::Material => {
                    // Save new material
                    if !app.tag_form.name.trim().is_empty() {
                        let material_names: Vec<String> = app.tag_form.name
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();

                        let mut created_count = 0;
                        let mut created_material_names = Vec::new();

                        for material_name in material_names {
                            // Skip if material already exists
                            if app.materials.contains(&material_name) {
                                continue;
                            }

                            let material = crate::api::Material {
                                name: material_name.clone(),
                                usage_count: 0,
                            };
                            match app.api_client.create_material(&material) {
                                Ok(created_material) => {
                                    app.materials.push(created_material.name.clone());
                                    created_material_names.push(created_material.name.clone());
                                    created_count += 1;
                                }
                                Err(e) => {
                                    app.set_status_message(format!("Error creating material '{}': {:?}", material_name, e));
                                }
                            }
                        }

                        if created_count > 0 {
                            // Remember currently selected materials before sorting
                            let mut previously_selected = Vec::new();
                            for (i, &selected) in app.tag_selection.iter().enumerate() {
                                if selected && i < app.materials.len()
                                    && let Some(material) = app.materials.get(i) {
                                    previously_selected.push(material.clone());
                                }
                            }

                            app.materials.sort();
                            app.refresh_data();

                            // Update tag selection array to match new materials length
                            app.tag_selection.resize(app.materials.len(), false);

                            // Re-select previously selected materials
                            for material in &previously_selected {
                                if let Some(index) = app.materials.iter().position(|m| m == material) {
                                    app.tag_selection[index] = true;
                                }
                            }

                            // Pre-select all newly created materials
                            let mut first_new_index = None;
                            for created_material_name in &created_material_names {
                                if let Some(index) = app.materials.iter().position(|m| m == created_material_name) {
                                    app.tag_selection[index] = true;
                                    if first_new_index.is_none() {
                                        first_new_index = Some(index);
                                    }
                                }
                            }

                            // Set selection index to first new material if available
                            if let Some(first_index) = first_new_index {
                                app.create_form.material_selected_index = first_index;
                            }

                            let message = if created_count == 1 {
                                format!("Material '{}' created", created_material_names.first().unwrap_or(&String::new()))
                            } else {
                                format!("{} materials created", created_count)
                            };
                            app.set_status_message(message);
                        }
                    } else {
                        app.set_status_message("Error: Material name required".to_string());
                    }
                    app.tag_form = TagForm::default();
                    app.input_mode = app.previous_input_mode.unwrap_or(InputMode::CreateMaterialSelect);
                }
            }
        }
        KeyCode::Backspace => {
            match app.item_type {
                ItemType::Tag => {
                    app.tag_form.name.pop();
                }
                ItemType::Category => match app.popup_field {
                    0 => {
                        app.category_form.name.pop();
                    }
                    1 => {
                        app.category_form.sku.pop();
                    }
                    2 => {
                        app.category_form.description.pop();
                    }
                    _ => {}
                },
                ItemType::Material => {
                    app.tag_form.name.pop();
                },
            }
        }
        KeyCode::Tab | KeyCode::Down => {
            if app.item_type == ItemType::Category {
                app.popup_field = (app.popup_field + 1) % 3;
            }
        }
        KeyCode::BackTab => {
            if app.item_type == ItemType::Category {
                app.popup_field = if app.popup_field == 0 { 2 } else { app.popup_field - 1 };
            }
        }
        KeyCode::Char(c) => {
            match app.item_type {
                ItemType::Tag => {
                    app.tag_form.name.push(c);
                }
                ItemType::Category => match app.popup_field {
                    0 => {
                        app.category_form.name.push(c);
                    }
                    1 => {
                        if app.category_form.sku.len() < 3 {
                            app.category_form.sku.push(c);
                        }
                    }
                    2 => {
                        app.category_form.description.push(c);
                    }
                    _ => {}
                },
                ItemType::Material => {
                    app.tag_form.name.push(c);
                },
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_delete_confirm_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.selected_product_for_delete = None;
        }
        KeyCode::Up | KeyCode::Down => {
            app.delete_option = if app.delete_option == 0 { 1 } else { 0 };
        }
        KeyCode::Char('1') => {
            app.delete_option = 0;
        }
        KeyCode::Char('2') => {
            app.delete_option = 1;
        }
        KeyCode::Enter => {
            if app.delete_option == 0 {
                // Database only deletion
                if let Some(product) = &app.selected_product_for_delete {
                    match app.api_client.delete_product(&product.sku, false) {
                        Ok(_) => {
                            app.set_status_message(format!("Product {} deleted from database", product.sku));
                            app.refresh_data();
                            app.clear_selection();
                        }
                        Err(e) => {
                            app.set_status_message(format!("Error deleting product: {}", e));
                        }
                    }
                }
                app.input_mode = InputMode::Normal;
                app.selected_product_for_delete = None;
            } else if app.delete_option == 1 {
                // File deletion - show file tree first
                if let Some(product) = &app.selected_product_for_delete {
                    match build_file_tree(&product.sku) {
                        Ok(tree) => {
                            app.file_tree_content = tree;
                            app.input_mode = InputMode::DeleteFileConfirm;
                        }
                        Err(e) => {
                            app.set_status_message(format!("Error scanning files: {}", e));
                            app.input_mode = InputMode::Normal;
                            app.selected_product_for_delete = None;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_delete_file_confirm_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.selected_product_for_delete = None;
            app.file_tree_content.clear();
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            // Confirm file deletion
            if let Some(product) = &app.selected_product_for_delete {
                match app.api_client.delete_product(&product.sku, true) {
                    Ok(_) => {
                        app.set_status_message(format!("Product {} and all files deleted", product.sku));
                        app.refresh_data();
                        app.clear_selection();
                    }
                    Err(e) => {
                        app.set_status_message(format!("Error deleting product: {}", e));
                    }
                }
            }
            app.input_mode = InputMode::Normal;
            app.selected_product_for_delete = None;
            app.file_tree_content.clear();
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            // Cancel file deletion
            app.input_mode = InputMode::Normal;
            app.selected_product_for_delete = None;
            app.file_tree_content.clear();
        }
        _ => {}
    }
    Ok(())
}

fn build_file_tree(sku: &str) -> Result<Vec<String>> {
    use std::path::Path;
    
    let mut content = Vec::new();
    let base_path = Path::new("/home/grbrum/Work/3d_print/Products").join(sku);
    
    if !base_path.exists() {
        content.push("No files found for this product".to_string());
        return Ok(content);
    }
    
    content.push(format!("📁 {}/", sku));
    
    // Scan subdirectories
    let subdirs = ["images", "models", "notes", "print_files"];
    for subdir in &subdirs {
        let subdir_path = base_path.join(subdir);
        if subdir_path.exists() {
            content.push(format!("├── 📁 {}/", subdir));
            match scan_directory(&subdir_path, "    │   ") {
                Ok(files) => content.extend(files),
                Err(_) => content.push("    │       └── (Error reading directory)".to_string()),
            }
        } else {
            content.push(format!("├── 📁 {}/ (empty)", subdir));
        }
    }
    
    // Check for metadata.json
    let metadata_path = base_path.join("metadata.json");
    if metadata_path.exists() {
        content.push("└── 📄 metadata.json".to_string());
    }
    
    Ok(content)
}

fn scan_directory(dir_path: &std::path::Path, prefix: &str) -> Result<Vec<String>> {
    let mut content = Vec::new();
    let entries = match std::fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return Ok(content),
    };
    
    let mut file_entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_file()))
        .collect();
    
    file_entries.sort_by_key(|a| a.file_name());
    
    for (i, entry) in file_entries.iter().enumerate() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let is_last = i == file_entries.len() - 1;
        let connector = if is_last { "└──" } else { "├──" };
        content.push(format!("{}{} 📄 {}", prefix, connector, file_name));
    }
    
    Ok(content)
}

fn normalize_tag_name(tag: &str) -> String {
    // Normalize tag: lowercase, strip whitespace, replace spaces with hyphens
    // This matches the backend normalize_tag function in tag_utils.py
    use std::collections::HashMap;
    
    let mut normalized = tag.to_lowercase();
    normalized = normalized.trim().to_string();
    
    // Replace spaces and underscores with hyphens
    let replacements: HashMap<char, char> = [
        (' ', '-'), ('_', '-')
    ].iter().cloned().collect();
    
    let mut result = String::new();
    for c in normalized.chars() {
        if let Some(&replacement) = replacements.get(&c) {
            result.push(replacement);
        } else if c.is_ascii_alphanumeric() || c == '-' {
            result.push(c);
        }
        // Remove other special characters
    }
    
    // Remove multiple consecutive hyphens
    let mut final_result = String::new();
    let mut prev_was_hyphen = false;
    for c in result.chars() {
        if c == '-' {
            if !prev_was_hyphen {
                final_result.push(c);
                prev_was_hyphen = true;
            }
        } else {
            final_result.push(c);
            prev_was_hyphen = false;
        }
    }
    
    // Remove leading/trailing hyphens
    final_result.trim_matches('-').to_string()
}

fn open_product_folder(sku: &str) -> Result<()> {
    use std::process::Command;
    use std::path::Path;
    use anyhow::anyhow;
    
    let base_path = Path::new("/home/grbrum/Work/3d_print/Products").join(sku);
    
    if !base_path.exists() {
        return Err(anyhow!("Product folder not found: {}", base_path.display()));
    }
    
    // Try different file managers for Linux (following Python frontend pattern)
    let file_managers = ["dolphin", "nautilus", "yazi", "thunar", "pcmanfm", "nemo"];
    let mut opened = false;
    
    for fm in &file_managers {
        match Command::new(fm).arg(&base_path).spawn() {
            Ok(_) => {
                opened = true;
                break;
            }
            Err(_) => continue,
        }
    }
    
    if opened {
        Ok(())
    } else {
        // Fallback to xdg-open if no specific file manager works
        match Command::new("xdg-open").arg(&base_path).spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("Failed to open folder: {}", e))
        }
    }
}

fn handle_edit_item_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            match app.edit_item_type {
                ItemType::Tag => {
                    app.tag_form = TagForm::default();
                    app.input_mode = match app.tag_select_mode {
                        TagSelectMode::Create => InputMode::CreateTagSelect,
                        TagSelectMode::Edit => InputMode::EditTagSelect,
                    };
                }
                ItemType::Category => {
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
                ItemType::Material => {
                    app.tag_form = TagForm::default();
                    app.input_mode = InputMode::CreateMaterialSelect;
                }
            }
        }
        KeyCode::Enter => {
            match app.edit_item_type {
                ItemType::Tag => {
                    // Save edited tag
                    if !app.tag_form.name.trim().is_empty() {
                        if app.create_form.tag_selected_index < app.tags.len() {
                            let old_name = app.tags[app.create_form.tag_selected_index].clone();
                            let tag = crate::api::Tag {
                                name: old_name.clone(),
                                usage_count: 0, // Not used for update
                            };
                            let mut updated_tag = tag.clone();
                            updated_tag.name = app.tag_form.name.clone();
                            match app.api_client.update_tag(&updated_tag) {
                                Ok(_) => {
                                    app.tags[app.create_form.tag_selected_index] =
                                        app.tag_form.name.clone();
                                    app.tags.sort();
                                    app.create_form.tag_selected_index = app
                                        .tags
                                        .iter()
                                        .position(|t| t == &app.tag_form.name)
                                        .unwrap_or(app.create_form.tag_selected_index);
                                    app.status_message =
                                        format!("Tag '{}' updated", app.tag_form.name);
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
                    app.tag_form = TagForm::default();
                    app.input_mode = match app.tag_select_mode {
                        TagSelectMode::Create => InputMode::CreateTagSelect,
                        TagSelectMode::Edit => InputMode::EditTagSelect,
                    };
                }
                ItemType::Category => {
                    // Save edited category
                    if !app.category_form.name.trim().is_empty() && app.category_form.sku.len() == 3 {
                        if app.create_form.category_selected_index < app.categories.len() {
                            let mut category =
                                app.categories[app.create_form.category_selected_index].clone();
                            category.name = app.category_form.name.clone();
                            category.sku_initials = app.category_form.sku.clone();
                            category.description = if app.category_form.description.trim().is_empty() {
                                None
                            } else {
                                Some(app.category_form.description.clone())
                            };
                            match app.api_client.update_category(&category) {
                                Ok(updated_category) => {
                                    app.categories[app.create_form.category_selected_index] =
                                        updated_category;
                                    app.categories.sort_by(|a, b| a.name.cmp(&b.name));
                                    app.create_form.category_selected_index = app
                                        .categories
                                        .iter()
                                        .position(|c| c.name == app.category_form.name)
                                        .unwrap_or(app.create_form.category_selected_index);
                                    app.status_message =
                                        format!("Category '{}' updated", app.category_form.name);
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
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
                ItemType::Material => {
                    // Save edited material
                    if !app.tag_form.name.trim().is_empty() {
                        if app.create_form.material_selected_index < app.materials.len() {
                            let old_name = app.materials[app.create_form.material_selected_index].clone();
                            let material = crate::api::Material {
                                name: old_name.clone(),
                                usage_count: 0, // Not used for update
                            };
                            let mut updated_material = material.clone();
                            updated_material.name = app.tag_form.name.clone();
                            match app.api_client.update_material(&updated_material) {
                                Ok(updated_material) => {
                                    app.materials[app.create_form.material_selected_index] =
                                        updated_material.name.clone();
                                    app.materials.sort();
                                    app.create_form.material_selected_index = app
                                        .materials
                                        .iter()
                                        .position(|m| m == &app.tag_form.name)
                                        .unwrap_or(app.create_form.material_selected_index);
                                    app.status_message =
                                        format!("Material '{}' updated", app.tag_form.name);
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
                    app.tag_form = TagForm::default();
                    app.input_mode = app.previous_input_mode.unwrap_or(InputMode::CreateMaterialSelect);
                }
            }
        }
        KeyCode::Backspace => {
            app.tag_form.name.pop();
        }
        KeyCode::Char(c) => {
            app.tag_form.name.push(c);
        }
        _ => {}
    }
    Ok(())
}