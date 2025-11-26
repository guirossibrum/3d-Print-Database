use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use std::time::Duration;

use crate::api::{ApiClient, Category, Product};
use crate::models::*;
use crate::ui;
use crate::handlers::AppHandlers;

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
    pub categories: Vec<Category>,

    // UI state
    pub selected_product_id: Option<i32>,
    pub search_query: String,
    pub inventory_search_query: String,
    pub status_message: String,

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
    pub tag_form: TagForm,
    pub popup_field: usize,
    pub tag_selection: Vec<bool>,
    pub edit_tags_string: String,
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
        let categories = api_client.get_categories()?;

        Ok(Self {
            running: true,
            current_tab: Tab::Search,
            input_mode: InputMode::Normal,
            active_pane: ActivePane::Left,
            api_client,
            products,
            tags,
            categories,
            selected_product_id: None,
            search_query: String::new(),
            inventory_search_query: String::new(),
            status_message: String::new(),
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
            tag_form: TagForm::default(),
            popup_field: 0,
            tag_selection: Vec::new(),
            edit_tags_string: String::new(),
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

    pub fn get_max_items(&self) -> usize {
        match self.current_tab {
            Tab::Search | Tab::Inventory => self.products.len(),
            Tab::Create => 0,
        }
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

    pub fn get_selected_product(&self) -> Option<&crate::api::Product> {
        let filtered_products = self.get_filtered_products();
        match self.selected_product_id {
            Some(product_id) => filtered_products.iter().find(|p| p.id == Some(product_id)),
            None => filtered_products.first(),
        }
    }

    pub fn get_selected_index(&self) -> usize {
        let filtered_products = self.get_filtered_products();
        match self.selected_product_id {
            Some(product_id) => {
                filtered_products.iter().position(|p| p.id == Some(product_id)).unwrap_or(0)
            }
            None => 0,
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
                    product
                        .name
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                        || product
                            .sku
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                })
                .collect()
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

    pub fn get_selected_product(&self) -> Option<&crate::api::Product> {
        let filtered_products = self.get_filtered_products();
        match self.selected_product_id {
            Some(product_id) => filtered_products.iter().find(|p| p.id == Some(product_id)),
            None => filtered_products.first(),
        }
    }

    pub fn get_selected_index(&self) -> usize {
        let filtered_products = self.get_filtered_products();
        match self.selected_product_id {
            Some(product_id) => {
                filtered_products.iter().position(|p| p.id == Some(product_id)).unwrap_or(0)
            }
            None => 0,
        }
    }

    pub fn handle_mouse_event(&mut self, _mouse_event: crossterm::event::MouseEvent) {
        // Mouse handling not yet implemented
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        self.handle_key_event(key)
    }

    pub fn refresh_data(&mut self) {
        // Refresh products, tags, and categories from the database
        match self.api_client.get_products() {
            Ok(products) => self.products = products,
            Err(e) => self.status_message = format!("Failed to refresh products: {:?}", e),
        }
        match self.api_client.get_tags() {
            Ok(tags) => {
                self.tags = tags.into_iter().map(|tag| tag.name).collect();
            }
            Err(e) => self.status_message = format!("Failed to refresh tags: {:?}", e),
        }
        match self.api_client.get_categories() {
            Ok(categories) => self.categories = categories,
            Err(e) => self.status_message = format!("Failed to refresh categories: {:?}", e),
        }
    }

    pub fn save_product(&mut self) -> Result<()> {
        // Validate required fields
        if self.create_form.name.trim().is_empty() {
            self.status_message = "Error: Product name is required".to_string();
            return Ok(());
        }

        let category_id = match self.create_form.category_id {
            Some(id) => id,
            None => {
                self.status_message = "Error: Category must be selected".to_string();
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
            material: None,
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
            Ok(response) => {
                self.status_message = format!("Product {} created successfully", response.sku);
                // Clear form
                self.create_form = CreateForm {
                    production: true,
                    ..Default::default()
                };
                // Refresh data
                self.refresh_data();
            }
            Err(e) => self.status_message = format!("Error creating product: {:?}", e),
        }
        Ok(())
    }
}