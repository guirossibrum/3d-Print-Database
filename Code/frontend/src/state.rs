use anyhow::Result;
use crossterm::event::Event;
use ratatui::Terminal;
use std::time::Duration;

use crate::api::{ApiClient, Category, Product};
use crate::models::*;
use crate::ui;

// Constants
const EVENT_POLL_TIMEOUT_MS: u64 = 100;
const DEFAULT_API_BASE_URL: &str = "http://localhost:8000";

/// Main application state for the 3D Print Database TUI
pub struct App {
    pub running: bool,
    pub current_tab: Tab,
    pub input_mode: InputMode,
    pub active_pane: ActivePane,

    // API client
    pub api_client: ApiClient,

    // Data
    pub products: Vec<Product>,
    pub tags: Vec<String>,
    pub materials: Vec<String>,
    pub categories: Vec<Category>,

    // UI state
    pub selected_product_id: Option<i32>,
    pub search_query: String,
    pub inventory_search_query: String,
    pub status_message: String,
    pub status_message_timestamp: Option<std::time::Instant>,

    // Edit backup (for cancelling changes)
    pub edit_backup: Option<Product>,
    pub previous_input_mode: Option<InputMode>,

    // Consolidated modes
    pub tag_select_mode: TagSelectMode,
    pub item_type: ItemType,
    pub edit_item_type: ItemType,

    // Create form
    pub create_form: CreateForm,
    pub category_form: CategoryForm,
    pub item_form: TagForm,
    pub popup_field: usize,
    pub tag_selection: Vec<bool>, // Selection state for tags and materials
    #[allow(dead_code)]
    pub category_selection: Vec<bool>,
    #[allow(dead_code)]
    pub selected_category_index: usize,
    pub edit_tags_string: String,

    // Delete state
    pub delete_option: usize,           // 0=database only, 1=database+files
    pub file_tree_content: Vec<String>, // File tree for display
    pub selected_product_for_delete: Option<crate::api::Product>, // Store product being deleted
}

impl App {
    /// Creates a new App instance, initializing data from the backend API
    pub fn new() -> Result<Self> {
        let api_client = ApiClient::new(DEFAULT_API_BASE_URL.to_string());
        let products = api_client.get_products()?;
        let tags = api_client
            .get_tags()?
            .into_iter()
            .map(|tag| tag.name)
            .collect::<Vec<String>>();
        let materials = api_client
            .get_materials()?
            .into_iter()
            .map(|material| material.name)
            .collect::<Vec<String>>();
        let categories = api_client.get_categories()?;

        Ok(Self {
            running: true,
            current_tab: Tab::Search,
            input_mode: InputMode::Normal,
            active_pane: ActivePane::Left,
            api_client,
            products,
            tags,
            materials,
            categories,
            selected_product_id: None,
            search_query: String::new(),
            inventory_search_query: String::new(),
            status_message: String::new(),
            status_message_timestamp: None,
            edit_backup: None,
            previous_input_mode: None,
            tag_select_mode: TagSelectMode::Create,
            item_type: ItemType::Tag,
            edit_item_type: ItemType::Tag,
            create_form: CreateForm {
                production: true, // Default to production ready
                ..Default::default()
            },
            category_form: CategoryForm::default(),
            item_form: TagForm::default(),
            popup_field: 0,
            tag_selection: Vec::new(),
            category_selection: Vec::new(),
            selected_category_index: 0,
            edit_tags_string: String::new(),

            // Delete state
            delete_option: 0,
            file_tree_content: Vec::new(),
            selected_product_for_delete: None,
        })
    }

    /// Runs the main application loop, handling events and rendering the UI
    pub fn run(
        &mut self,
        terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
        version: &str,
    ) -> Result<()> {
        while self.running {
            // Draw the UI
            terminal.draw(|f| {
                ui::draw(f, self, version);
            })?;

            // Poll for events with timeout
            if crossterm::event::poll(Duration::from_millis(EVENT_POLL_TIMEOUT_MS))? {
                match crossterm::event::read()? {
                    Event::Key(key) => self.handle_key(key)?,
                    Event::Mouse(mouse) => self.handle_mouse_event(mouse),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn has_multiple_panes(&self) -> bool {
        matches!(self.current_tab, Tab::Search | Tab::Inventory)
    }

    pub fn next_pane(&mut self) {
        if self.has_multiple_panes() {
            self.active_pane = match self.active_pane {
                ActivePane::Left => ActivePane::Right,
                ActivePane::Right => ActivePane::Left,
            };
        }
    }

    pub fn prev_pane(&mut self) {
        if self.has_multiple_panes() {
            self.active_pane = match self.active_pane {
                ActivePane::Left => ActivePane::Right,
                ActivePane::Right => ActivePane::Left,
            };
        }
    }

    pub fn get_filtered_products(&self) -> Vec<&crate::api::Product> {
        let query = if matches!(self.current_tab, Tab::Search) {
            &self.search_query
        } else if matches!(self.current_tab, Tab::Inventory) {
            &self.inventory_search_query
        } else {
            return self.products.iter().collect();
        };

        if query.is_empty() {
            self.products.iter().collect()
        } else {
            self.products
                .iter()
                .filter(|product| {
                    product.name.to_lowercase().contains(&query.to_lowercase())
                        || product.sku.to_lowercase().contains(&query.to_lowercase())
                })
                .collect()
        }
    }

    pub fn get_selected_product(&self) -> Option<&crate::api::Product> {
        let filtered_products = self.get_filtered_products();
        match self.selected_product_id {
            Some(product_id) => filtered_products
                .iter()
                .find(|p| p.id == Some(product_id))
                .copied(),
            None => filtered_products.first().copied(),
        }
    }

    pub fn get_selected_index(&self) -> usize {
        let filtered_products = self.get_filtered_products();
        match self.selected_product_id {
            Some(product_id) => filtered_products
                .iter()
                .position(|p| p.id == Some(product_id))
                .unwrap_or(0),
            None => 0,
        }
    }

    pub fn next_filtered_item(&mut self) {
        let filtered_products: Vec<&crate::api::Product> = self.get_filtered_products();
        if !filtered_products.is_empty() {
            let current_index = self.get_selected_index();
            let new_index = (current_index + 1) % filtered_products.len();
            self.selected_product_id = filtered_products[new_index].id;
        }
    }

    pub fn prev_filtered_item(&mut self) {
        let filtered_products: Vec<&crate::api::Product> = self.get_filtered_products();
        if !filtered_products.is_empty() {
            let current_index = self.get_selected_index();
            let new_index = if current_index == 0 {
                filtered_products.len() - 1
            } else {
                current_index - 1
            };
            self.selected_product_id = filtered_products[new_index].id;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_product_id = None;
    }

    pub fn handle_mouse_event(&mut self, _mouse_event: crossterm::event::MouseEvent) {
        // Mouse handling not yet implemented
    }

    pub fn refresh_data(&mut self) {
        // Refresh products, tags, and categories from the database
        match self.api_client.get_products() {
            Ok(products) => self.products = products,
            Err(e) => self.set_status_message(format!("Failed to refresh products: {:?}", e)),
        }
        match self.api_client.get_tags() {
            Ok(tags) => {
                self.tags = tags.into_iter().map(|tag| tag.name).collect();
            }
            Err(e) => self.set_status_message(format!("Failed to refresh tags: {:?}", e)),
        }
        match self.api_client.get_materials() {
            Ok(materials) => {
                self.materials = materials
                    .into_iter()
                    .map(|material| material.name)
                    .collect();
            }
            Err(e) => self.set_status_message(format!("Failed to refresh materials: {:?}", e)),
        }
        match self.api_client.get_categories() {
            Ok(categories) => self.categories = categories,
            Err(e) => self.set_status_message(format!("Failed to refresh categories: {:?}", e)),
        }
    }

    pub fn save_product(&mut self) -> Result<()> {
        // Validate required fields
        if self.create_form.name.trim().is_empty() {
            self.set_status_message("Error: Product name is required".to_string());
            return Ok(());
        }

        let category_id = match self.create_form.category_id {
            Some(id) => id,
            None => {
                self.set_status_message("Error: Category must be selected".to_string());
                return Ok(());
            }
        };

        // Create product struct for API call
        let product = Product {
            id: None,
            sku: "".to_string(), // Backend will generate SKU
            name: self.create_form.name.clone(),
            description: if self.create_form.description.trim().is_empty() {
                None
            } else {
                Some(self.create_form.description.clone())
            },
            production: self.create_form.production,
            tags: self.create_form.tags.clone(),
            category_id: Some(category_id),
            material: if self.create_form.materials.is_empty() {
                None
            } else {
                Some(self.create_form.materials.clone())
            },
            color: None,
            print_time: None,
            weight: None,
            stock_quantity: None,
            reorder_point: None,
            unit_cost: None,
            selling_price: None,
        };

        // Call API to create product
        match self.api_client.create_product(&product) {
            Ok(_) => {
                self.set_status_message("Product created successfully".to_string());
                // Clear form
                self.create_form = CreateForm {
                    production: true,
                    ..Default::default()
                };
                // Refresh data
                self.refresh_data();
            }
            Err(e) => self.set_status_message(format!("Error creating product: {:?}", e)),
        }
        Ok(())
    }

    pub fn perform_update(&mut self, sku: &str, update: crate::api::ProductUpdate) -> Result<()> {
        match self.api_client.update_product(sku, &update) {
            Ok(_) => {
                self.set_status_message("Product updated successfully".to_string());
                self.refresh_data();
            }
            Err(e) => self.set_status_message(format!("Error updating product: {:?}", e)),
        }
        Ok(())
    }

    pub fn get_selected_product_data(&self) -> Option<(String, crate::api::Product)> {
        self.products
            .iter()
            .find(|p| p.id == self.selected_product_id)
            .map(|p| (p.sku.clone(), p.clone()))
    }

    // Status message management with 20-second persistence
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
        self.status_message_timestamp = Some(std::time::Instant::now());
    }

    // Check if status message should still be shown (20-second timeout)
    pub fn should_show_status(&self) -> bool {
        if let Some(timestamp) = self.status_message_timestamp {
            timestamp.elapsed() < std::time::Duration::from_secs(20)
        } else {
            false
        }
    }

// Basic key handling - delegates to handlers module for complex logic
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        // Import and use handlers module
        use crate::handlers::*;

        handle_input(self, key)
    }
}
