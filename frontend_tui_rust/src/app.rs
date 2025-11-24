use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::Terminal;
use anyhow::Result;

use crate::ui;
use crate::api::{ApiClient, Product, Category};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Create,
    Search,
    Inventory,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::Create => Tab::Search,
            Tab::Search => Tab::Inventory,
            Tab::Inventory => Tab::Create,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Tab::Create => Tab::Inventory,
            Tab::Search => Tab::Create,
            Tab::Inventory => Tab::Search,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    CreateName,
    CreateDescription,
    EditName,
    EditDescription,
}



pub struct App {
    pub running: bool,
    pub current_tab: Tab,
    pub input_mode: InputMode,
    pub api_client: ApiClient,

    // Data
    pub products: Vec<Product>,
    pub categories: Vec<Category>,
    pub tags: Vec<String>,

    // UI state
    pub selected_index: usize,
    pub search_query: String,
    pub status_message: String,

    // Create form
    pub create_form: CreateForm,
}

#[derive(Debug, Default)]
pub struct CreateForm {
    pub name: String,
    pub description: String,
    pub category_id: Option<i32>,
    pub tags: Vec<String>,
    pub production: bool,
}

impl App {
    pub async fn new() -> Result<Self> {
        let api_client = ApiClient::new("http://localhost:8000".to_string());

        // Load initial data
        let categories = api_client.get_categories().await?;
        let tags = api_client.get_tags().await?;
        let products = api_client.search_products("").await?;

        Ok(Self {
            running: true,
            current_tab: Tab::Search,
            input_mode: InputMode::Normal,
            api_client,
            products,
            categories,
            tags: tags.into_iter().map(|t| t.name).collect(),
            selected_index: 0,
            search_query: String::new(),
            status_message: "Welcome to 3D Print Database TUI".to_string(),
            create_form: CreateForm::default(),
        })
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => {
                        self.handle_key(key).await?;
                    }
                    Event::Mouse(mouse_event) => {
                        self.handle_mouse_event(mouse_event);
                    }
                    _ => {}
                }
            }

            if !self.running {
                break;
            }
        }
        Ok(())
    }

    async fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        // Handle global keys
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if matches!(self.input_mode, InputMode::Normal) {
                    self.running = false;
                } else {
                    self.input_mode = InputMode::Normal;
                }
                return Ok(());
            }
            _ => {}
        }

        // Handle mode-specific keys
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Tab => {
                    // Move to right pane and start editing Name
                    self.input_mode = InputMode::EditName;
                }
                KeyCode::BackTab | KeyCode::Left => {
                    self.current_tab = self.current_tab.prev();
                    self.selected_index = 0;
                }
                KeyCode::Right => {
                    self.current_tab = self.current_tab.next();
                    self.selected_index = 0;
                }
                KeyCode::Char('j') | KeyCode::Down => self.next_item(),
                KeyCode::Char('k') | KeyCode::Up => self.prev_item(),
                KeyCode::Char('/') => {
                    self.input_mode = InputMode::Search;
                    self.current_tab = Tab::Search;
                }
                KeyCode::Char('n') => {
                    self.input_mode = InputMode::CreateName;
                    self.current_tab = Tab::Create;
                }
                KeyCode::Char('e') => {
                    if matches!(self.current_tab, Tab::Search) {
                        self.input_mode = InputMode::EditName;
                    }
                }
                KeyCode::Char('+') => {
                    if matches!(self.current_tab, Tab::Inventory) {
                        self.adjust_stock(1).await?;
                    }
                }
                KeyCode::Char('-') => {
                    if matches!(self.current_tab, Tab::Inventory) {
                        self.adjust_stock(-1).await?;
                    }
                }
                KeyCode::Enter => {
                    match self.current_tab {
                        Tab::Search => {
                            // Enter edit mode for name (simplified)
                            self.input_mode = InputMode::EditName;
                        }
                        _ => {
                            self.select_item().await?;
                        }
                    }
                }
                KeyCode::Char('d') if matches!(key.modifiers, KeyModifiers::CONTROL) => {
                    self.delete_selected().await?;
                }
                _ => {}
            },
            InputMode::Search => match key.code {
                KeyCode::Enter => {
                    self.search_products().await?;
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Backspace => { self.search_query.pop(); }
                KeyCode::Char(c) => self.search_query.push(c),
                _ => {}
            },
            InputMode::CreateName => match key.code {
                KeyCode::Enter => self.input_mode = InputMode::CreateDescription,
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Backspace => { self.create_form.name.pop(); }
                KeyCode::Char(c) => self.create_form.name.push(c),
                _ => {}
            },
            InputMode::CreateDescription => match key.code {
                KeyCode::Enter => {
                    self.create_product().await?;
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Backspace => { self.create_form.description.pop(); }
                KeyCode::Char(c) => self.create_form.description.push(c),
                _ => {}
            },
            InputMode::EditName => match key.code {
                KeyCode::Right | KeyCode::Enter => self.input_mode = InputMode::EditDescription,
                KeyCode::Left | KeyCode::BackTab | KeyCode::Esc => self.input_mode = InputMode::Normal,
                KeyCode::Backspace => {
                    if let Some(product) = self.products.get_mut(self.selected_index) {
                        product.name.pop();
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(product) = self.products.get_mut(self.selected_index) {
                        product.name.push(c);
                    }
                }
                _ => {}
            },
            InputMode::EditDescription => match key.code {
                KeyCode::Left => self.input_mode = InputMode::EditName,
                KeyCode::Up | KeyCode::Esc => self.input_mode = InputMode::Normal,
                KeyCode::Enter => {
                    self.update_selected_product().await?;
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Backspace => {
                    if let Some(product) = self.products.get_mut(self.selected_index) {
                        if let Some(ref mut desc) = product.description {
                            desc.pop();
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(product) = self.products.get_mut(self.selected_index) {
                        if let Some(ref mut desc) = product.description {
                            desc.push(c);
                        } else {
                            product.description = Some(c.to_string());
                        }
                    }
                }
                _ => {}
            },

        }
        Ok(())
    }



    fn next_item(&mut self) {
        let max_items = match self.current_tab {
            Tab::Search => self.products.len(),
            Tab::Inventory => self.products.len(),
            Tab::Create => 0,
        };

        if max_items > 0 {
            self.selected_index = (self.selected_index + 1) % max_items;
        }
    }

    fn prev_item(&mut self) {
        let max_items = match self.current_tab {
            Tab::Search => self.products.len(),
            Tab::Inventory => self.products.len(),
            Tab::Create => 0,
        };

        if max_items > 0 {
            self.selected_index = if self.selected_index == 0 {
                max_items - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    async fn select_item(&mut self) -> Result<()> {
        match self.current_tab {
            Tab::Search => {
                if let Some(product) = self.products.get(self.selected_index) {
                    self.status_message = format!("Selected: {} ({})", product.name, product.sku);
                }
            }
            Tab::Inventory => {
                if let Some(product) = self.products.get(self.selected_index) {
                    self.status_message = format!("Inventory: {} - Stock: Check API", product.name);
                }
            }
            Tab::Create => {
                self.create_product().await?;
            }
        }
        Ok(())
    }

    async fn delete_selected(&mut self) -> Result<()> {
        if let Some(product) = self.products.get(self.selected_index) {
            if let Some(_id) = product.id {
                self.api_client.delete_product(&product.sku).await?;
                self.status_message = format!("Deleted: {}", product.name);
                // Refresh products
                self.products = self.api_client.search_products("").await?;
            }
        }
        Ok(())
    }

    async fn search_products(&mut self) -> Result<()> {
        self.products = self.api_client.search_products(&self.search_query).await?;
        self.selected_index = 0;
        self.status_message = format!("Found {} products", self.products.len());
        Ok(())
    }

    async fn adjust_stock(&mut self, delta: i32) -> Result<()> {
        // Mock stock adjustment - in real app, this would call API
        self.status_message = format!("Stock adjusted by {}", delta);
        Ok(())
    }

    async fn update_selected_product(&mut self) -> Result<()> {
        if let Some(product) = self.products.get(self.selected_index) {
            self.api_client.update_product(&product.sku, product).await?;
            self.status_message = format!("Updated product: {}", product.name);
        }
        Ok(())
    }

    async fn create_product(&mut self) -> Result<()> {
        if self.create_form.name.is_empty() {
            self.status_message = "Error: Product name is required".to_string();
            return Ok(());
        }

        let product = Product {
            id: None,
            sku: format!("{}-001", self.create_form.name.to_uppercase().replace(" ", "-")),
            name: self.create_form.name.clone(),
            description: Some(self.create_form.description.clone()),
            production: self.create_form.production,
            tags: self.create_form.tags.clone(),
        };

        self.api_client.create_product(&product).await?;
        self.status_message = format!("Created product: {}", product.name);

        // Reset form
        self.create_form = CreateForm::default();

        // Refresh products
        self.products = self.api_client.search_products("").await?;

        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: crossterm::event::MouseEvent) {
        use crossterm::event::{MouseButton, MouseEventKind};

        if let MouseEventKind::Down(MouseButton::Left) = mouse_event.kind {
            let x = mouse_event.column as usize;
            let y = mouse_event.row as usize;

            // Header area (y = 0-2)
            if y < 3 {
                // Tab area (y = 1-2, x varies)
                if y >= 1 && y <= 2 {
                    let tab_width = 12; // Approximate width per tab
                    let tab_index = x / tab_width;
                    match tab_index {
                        0 => self.current_tab = Tab::Create,
                        1 => self.current_tab = Tab::Search,
                        2 => self.current_tab = Tab::Inventory,
                        _ => {}
                    }
                    self.selected_index = 0;
                    self.input_mode = InputMode::Normal;
                }
            }
            // Content area (y = 3+)
            else if y >= 3 {
                match self.current_tab {
                    Tab::Search => self.handle_search_mouse_click(x, y),
                    Tab::Create => self.handle_create_mouse_click(x, y),
                    Tab::Inventory => self.handle_inventory_mouse_click(x, y),
                }
            }
        }
    }

    fn handle_search_mouse_click(&mut self, x: usize, y: usize) {
        // Left pane (search results) - x < 50% of width
        // Right pane (product details/edit) - x >= 50% of width

        let content_y = y - 3; // Adjust for header/tabs

        if x < 40 { // Left pane - search results
            // Each result takes about 1 line, starting from y=1 in the pane
            if content_y >= 1 {
                let result_index = content_y - 1;
                if result_index < self.products.len() {
                    self.selected_index = result_index;
                    self.status_message = format!("Selected product {}", result_index + 1);
                }
            }
        } else { // Right pane - product details/edit
            // Check if clicking on editable fields
            let field_y = content_y;
            match field_y {
                1 => { // Name field
                    if matches!(self.input_mode, InputMode::Normal) {
                        self.input_mode = InputMode::EditName;
                    }
                }
                2 => { // Description field
                    if matches!(self.input_mode, InputMode::Normal) {
                        self.input_mode = InputMode::EditDescription;
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_create_mouse_click(&mut self, _x: usize, y: usize) {
        let content_y = y - 3; // Adjust for header/tabs

        // Click on form fields to select them
        match content_y {
            1 => { // Name field
                self.input_mode = InputMode::CreateName;
            }
            3 => { // Description field
                self.input_mode = InputMode::CreateDescription;
            }
            _ => {}
        }
    }

    fn handle_inventory_mouse_click(&mut self, x: usize, y: usize) {
        let content_y = y - 3; // Adjust for header/tabs

        if x < 40 { // Left pane - product list
            // Header is at content_y = 0, products start at content_y = 1
            if content_y >= 1 {
                let product_index = content_y - 1;
                if product_index < self.products.len() {
                    self.selected_index = product_index;
                    self.status_message = format!("Selected inventory item {}", product_index + 1);
                }
            }
        }
        // Right pane - stock adjustment (no specific clickable areas yet)
    }
}