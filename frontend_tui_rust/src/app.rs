use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use anyhow::Result;

use crate::ui;
use crate::api::{ApiClient, Product, Category};

// Constants
const EVENT_POLL_TIMEOUT_MS: u64 = 100;
const DEFAULT_API_BASE_URL: &str = "http://localhost:8000";

/// Represents the different tabs in the TUI application
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

/// Represents the different input modes for handling user interactions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivePane {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    InventorySearch,
    CreateName,
    CreateDescription,
    CreateCategory,
    CreateCategorySelect,
    CreateProduction,
    CreateTags,
    CreateTagSelect,
    EditName,
    EditDescription,
    EditProduction,
    NewCategory,
    EditCategory,
    NewTag,
    EditTag,
}



/// Main application state for the 3D Print Database TUI
pub struct App {
    pub running: bool,
    pub current_tab: Tab,
    pub input_mode: InputMode,
    pub active_pane: ActivePane,

    // Data
    pub products: Vec<Product>,
    pub tags: Vec<String>,
    pub categories: Vec<Category>,

    // UI state
    pub selected_index: usize,
    pub search_query: String,
    pub inventory_search_query: String,
    pub status_message: String,

    // Edit backup (for cancelling changes)
    pub edit_backup: Option<Product>,

    // Create form
    pub create_form: CreateForm,
    pub category_form: CategoryForm,
    pub tag_form: TagForm,
    pub popup_field: usize,
}

#[derive(Debug, Default)]
pub struct CreateForm {
    pub name: String,
    pub description: String,
    pub category_id: Option<i32>,
    pub category_selected_index: usize,
    pub production: bool,
    pub tags: Vec<String>,
    pub tag_selected_index: usize,
}

#[derive(Debug, Default)]
pub struct CategoryForm {
    pub name: String,
    pub sku: String,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct TagForm {
    pub name: String,
}

impl App {
    /// Creates a new App instance, initializing data from the backend API
    pub async fn new() -> Result<Self> {
        let api_client = ApiClient::new(DEFAULT_API_BASE_URL.to_string());
        let products = api_client.get_products().await?;
        let tags = api_client.get_tags().await?
            .into_iter()
            .map(|tag| tag.name)
            .collect::<Vec<String>>();
        let categories = api_client.get_categories().await?;

        Ok(Self {
            running: true,
            current_tab: Tab::Search,
            input_mode: InputMode::Normal,
            active_pane: ActivePane::Left,
            products,
            tags,
            categories,
            selected_index: 0,
            search_query: String::new(),
            inventory_search_query: String::new(),
            status_message: String::new(),
             edit_backup: None,
             create_form: CreateForm {
                 production: true, // Default to production ready
                 ..Default::default()
             },
             category_form: CategoryForm::default(),
             tag_form: TagForm::default(),
             popup_field: 0,
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

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {


        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::Search => self.handle_search_mode(key),
            InputMode::InventorySearch => self.handle_inventory_search_mode(key),
            InputMode::CreateName => self.handle_create_name_mode(key),
            InputMode::CreateDescription => self.handle_create_description_mode(key),
            InputMode::CreateCategory => self.handle_create_category_mode(key),
            InputMode::CreateCategorySelect => self.handle_create_category_select_mode(key),
            InputMode::CreateProduction => self.handle_create_production_mode(key),
            InputMode::CreateTags => self.handle_create_tags_mode(key),
            InputMode::CreateTagSelect => self.handle_create_tag_select_mode(key),
            InputMode::EditName => self.handle_edit_name_mode(key),
            InputMode::EditDescription => self.handle_edit_description_mode(key),
            InputMode::EditProduction => self.handle_edit_production_mode(key),
            InputMode::NewCategory => self.handle_new_category_mode(key),
            InputMode::EditCategory => self.handle_edit_category_mode(key),
            InputMode::NewTag => self.handle_new_tag_mode(key),
            InputMode::EditTag => self.handle_edit_tag_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }

            // Enhanced TAB workflow: View → Edit → View
            KeyCode::Tab => {
                match self.input_mode {
                    InputMode::Normal => {
                        if self.has_multiple_panes() && matches!(self.active_pane, ActivePane::Left) && !self.products.is_empty() {
                            // Switch to right pane and enter edit mode
                            self.active_pane = ActivePane::Right;
                            self.input_mode = InputMode::EditName;
                        } else if self.has_multiple_panes() {
                            // Regular pane switching
                            self.next_pane();
                        }
                    }
                    InputMode::CreateName => {
                        self.input_mode = InputMode::CreateDescription;
                    }
                    InputMode::CreateDescription => {
                        self.input_mode = InputMode::CreateCategory;
                    }
                    InputMode::CreateCategory => {
                        self.input_mode = InputMode::CreateCategorySelect;
                        self.active_pane = ActivePane::Right;
                    }
                    InputMode::CreateProduction => {
                        self.input_mode = InputMode::CreateTags;
                    }
                    InputMode::CreateTags => {
                        self.input_mode = InputMode::CreateTagSelect;
                        self.active_pane = ActivePane::Right;
                    }
                    InputMode::CreateCategorySelect => {
                        self.input_mode = InputMode::CreateProduction;
                        self.active_pane = ActivePane::Left;
                    }
                    InputMode::CreateTagSelect => {
                        // Tab in CreateTagSelect does nothing or save?
                    }
                    _ => {
                        // TAB in other modes (like search) exits to normal mode
                        if matches!(self.input_mode, InputMode::Search | InputMode::InventorySearch) {
                            self.input_mode = InputMode::Normal;
                        }
                    }
                }
            }
            KeyCode::BackTab if self.has_multiple_panes() => {
                self.prev_pane();
            }

            KeyCode::BackTab => {
                self.current_tab = self.current_tab.prev();
                self.active_pane = ActivePane::Left;
                self.selected_index = 0;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next_item();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.prev_item();
            }
            KeyCode::Left => {
                self.current_tab = self.current_tab.prev();
                self.active_pane = ActivePane::Left;
                self.selected_index = 0;
            }
            KeyCode::Right => {
                self.current_tab = self.current_tab.next();
                self.active_pane = ActivePane::Left;
                self.selected_index = 0;
            }
            KeyCode::Char('/') => {
                if matches!(self.current_tab, Tab::Search) {
                    self.input_mode = InputMode::Search;
                } else if matches!(self.current_tab, Tab::Inventory) {
                    self.input_mode = InputMode::InventorySearch;
                }
            }
            KeyCode::Enter => {
                match self.input_mode {
                    InputMode::Normal => {
                        if matches!(self.current_tab, Tab::Create) {
                            self.input_mode = InputMode::CreateName;
                        } else if matches!(self.current_tab, Tab::Search) && !self.products.is_empty() {
                            // Direct edit from normal mode (legacy behavior)
                            self.input_mode = InputMode::EditName;
                        }
                    }
                    input_mode if matches!(input_mode, InputMode::EditName | InputMode::EditDescription | InputMode::EditProduction) => {
                        // Save changes and return to normal mode
                        self.input_mode = InputMode::Normal;
                        self.active_pane = ActivePane::Left;
                        // TODO: Persist changes to backend
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_search_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.search_query.clear();
            }
            KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Tab => {
                self.input_mode = InputMode::Normal;
                // Don't switch panes when exiting search mode
            }
            KeyCode::Backspace => {
                self.search_query.pop();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_inventory_search_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.inventory_search_query.clear();
            }
            KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Tab => {
                self.input_mode = InputMode::Normal;
                // Don't switch panes when exiting search mode
            }
            KeyCode::Backspace => {
                self.inventory_search_query.pop();
            }
            KeyCode::Char(c) => {
                self.inventory_search_query.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_name_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.create_form.name.clear();
            }
            KeyCode::Down => {
                self.input_mode = InputMode::CreateDescription;
            }
            KeyCode::Up => {
                // Already at first field
            }
            KeyCode::Backspace => {
                self.create_form.name.pop();
            }
            KeyCode::Char(c) => {
                self.create_form.name.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_description_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::CreateName;
            }
            KeyCode::Down => {
                self.input_mode = InputMode::CreateCategory;
            }
            KeyCode::Up => {
                self.input_mode = InputMode::CreateName;
            }
            KeyCode::Backspace => {
                self.create_form.description.pop();
            }
            KeyCode::Char(c) => {
                self.create_form.description.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_category_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::CreateDescription;
            }
            KeyCode::Tab => {
                self.input_mode = InputMode::CreateCategorySelect;
                self.active_pane = ActivePane::Right;
            }
            KeyCode::Down => {
                self.input_mode = InputMode::CreateProduction;
            }
            KeyCode::Up => {
                self.input_mode = InputMode::CreateDescription;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_category_select_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::CreateCategory;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Enter => {
                // Select the current category
                if let Some(category) = self.categories.get(self.create_form.category_selected_index) {
                    self.create_form.category_id = Some(category.id);
                }
                self.input_mode = InputMode::CreateCategory;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Down => {
                if !self.categories.is_empty() {
                    self.create_form.category_selected_index =
                        (self.create_form.category_selected_index + 1) % self.categories.len();
                }
            }
            KeyCode::Up => {
                if !self.categories.is_empty() {
                    self.create_form.category_selected_index =
                        if self.create_form.category_selected_index == 0 {
                            self.categories.len() - 1
                        } else {
                            self.create_form.category_selected_index - 1
                        };
                }
            }
            KeyCode::Char('n') => {
                self.input_mode = InputMode::NewCategory;
            }
            KeyCode::Char('e') => {
                if let Some(category) = self.categories.get(self.create_form.category_selected_index) {
                    self.category_form.name = category.name.clone();
                    self.category_form.sku = category.sku_initials.clone();
                    self.category_form.description = category.description.clone().unwrap_or_default();
                    self.input_mode = InputMode::EditCategory;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_production_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::CreateCategory;
            }
            KeyCode::Down => {
                self.input_mode = InputMode::CreateTags;
            }
            KeyCode::Up => {
                self.input_mode = InputMode::CreateCategory;
            }
            KeyCode::Left => {
                self.create_form.production = true;
            }
            KeyCode::Right => {
                self.create_form.production = false;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.create_form.production = true;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.create_form.production = false;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_tags_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::CreateProduction;
            }
            KeyCode::Enter => {
                // Save the product
                self.save_product()?;
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Tab => {
                self.input_mode = InputMode::CreateTagSelect;
                self.active_pane = ActivePane::Right;
            }
            KeyCode::Up => {
                self.input_mode = InputMode::CreateProduction;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_create_tag_select_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::CreateTags;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Enter => {
                // Select tag from available tags
                if let Some(tag) = self.tags.get(self.create_form.tag_selected_index) {
                    if !self.create_form.tags.contains(tag) {
                        self.create_form.tags.push(tag.clone());
                    }
                }
                self.input_mode = InputMode::CreateTags;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Down => {
                if !self.tags.is_empty() {
                    self.create_form.tag_selected_index =
                        (self.create_form.tag_selected_index + 1) % self.tags.len();
                }
            }
            KeyCode::Up => {
                if !self.tags.is_empty() {
                    self.create_form.tag_selected_index =
                        if self.create_form.tag_selected_index == 0 {
                            self.tags.len() - 1
                        } else {
                            self.create_form.tag_selected_index - 1
                        };
                }
            }
            KeyCode::Char('n') => {
                self.input_mode = InputMode::NewTag;
            }
            KeyCode::Char('e') => {
                if let Some(tag) = self.tags.get(self.create_form.tag_selected_index) {
                    self.tag_form.name = tag.clone();
                    self.input_mode = InputMode::EditTag;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_edit_name_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Tab => {
                // Cancel changes (discard) and return to normal mode
                if let Some(original_product) = self.edit_backup.take() {
                    // Restore original product data
                    if let Some(current_product) = self.products.get_mut(self.selected_index) {
                        *current_product = original_product;
                    }
                }
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Enter => {
                // Save changes and return to normal mode
                self.edit_backup = None; // Clear backup since we're saving
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
                // TODO: Persist changes to backend
            }
            KeyCode::Down => {
                self.input_mode = InputMode::EditDescription;
            }
            KeyCode::Up => {
                // Already at first field, do nothing
            }
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
        }
        Ok(())
    }

    fn handle_edit_description_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Tab => {
                // Cancel changes (discard) and return to normal mode
                if let Some(original_product) = self.edit_backup.take() {
                    // Restore original product data
                    if let Some(current_product) = self.products.get_mut(self.selected_index) {
                        *current_product = original_product;
                    }
                }
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Enter => {
                // Save changes and return to normal mode
                self.edit_backup = None; // Clear backup since we're saving
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
                // TODO: Persist changes to backend
            }
            KeyCode::Down => {
                self.input_mode = InputMode::EditProduction;
            }
            KeyCode::Up => {
                self.input_mode = InputMode::EditName;
            }
            KeyCode::Backspace => {
                if let Some(product) = self.products.get_mut(self.selected_index)
                    && let Some(ref mut desc) = product.description {
                    desc.pop();
                }
            }
            KeyCode::Char(c) => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.description.get_or_insert_with(String::new).push(c);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_new_category_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.category_form = CategoryForm::default();
                self.popup_field = 0;
                self.input_mode = InputMode::CreateCategorySelect;
            }
            KeyCode::Enter => {
                // Save new category
                if !self.category_form.name.trim().is_empty() && self.category_form.sku.len() == 3 {
                    let new_category = crate::api::Category {
                        id: (self.categories.len() + 1) as i32, // fake id for local
                        name: self.category_form.name.clone(),
                        sku_initials: self.category_form.sku.clone(),
                        description: if self.category_form.description.trim().is_empty() { None } else { Some(self.category_form.description.clone()) },
                    };
                    self.categories.push(new_category);
                    self.create_form.category_selected_index = self.categories.len() - 1;
                    self.status_message = format!("Category '{}' created", self.category_form.name);
                } else {
                    self.status_message = "Error: Name required, SKU must be 3 letters".to_string();
                }
                self.category_form = CategoryForm::default();
                self.popup_field = 0;
                self.input_mode = InputMode::CreateCategorySelect;
            }
            KeyCode::Tab => {
                self.popup_field = (self.popup_field + 1) % 3;
            }
            KeyCode::BackTab => {
                self.popup_field = if self.popup_field == 0 { 2 } else { self.popup_field - 1 };
            }
            KeyCode::Backspace => {
                match self.popup_field {
                    0 => { self.category_form.name.pop(); }
                    1 => { self.category_form.sku.pop(); }
                    2 => { self.category_form.description.pop(); }
                    _ => {}
                }
            }
            KeyCode::Char(c) => {
                match self.popup_field {
                    0 => { self.category_form.name.push(c); }
                    1 => { if self.category_form.sku.len() < 3 { self.category_form.sku.push(c); } }
                    2 => { self.category_form.description.push(c); }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_edit_category_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.category_form = CategoryForm::default();
                self.popup_field = 0;
                self.input_mode = InputMode::CreateCategorySelect;
            }
            KeyCode::Enter => {
                // Save edited category
                if !self.category_form.name.trim().is_empty() && self.category_form.sku.len() == 3 {
                    if self.create_form.category_selected_index < self.categories.len() {
                        let category = &mut self.categories[self.create_form.category_selected_index];
                        category.name = self.category_form.name.clone();
                        category.sku_initials = self.category_form.sku.clone();
                        category.description = if self.category_form.description.trim().is_empty() { None } else { Some(self.category_form.description.clone()) };
                        self.status_message = format!("Category '{}' updated", self.category_form.name);
                    }
                } else {
                    self.status_message = "Error: Name required, SKU must be 3 letters".to_string();
                }
                self.category_form = CategoryForm::default();
                self.popup_field = 0;
                self.input_mode = InputMode::CreateCategorySelect;
            }
            KeyCode::Tab => {
                self.popup_field = (self.popup_field + 1) % 3;
            }
            KeyCode::BackTab => {
                self.popup_field = if self.popup_field == 0 { 2 } else { self.popup_field - 1 };
            }
            KeyCode::Backspace => {
                match self.popup_field {
                    0 => { self.category_form.name.pop(); }
                    1 => { self.category_form.sku.pop(); }
                    2 => { self.category_form.description.pop(); }
                    _ => {}
                }
            }
            KeyCode::Char(c) => {
                match self.popup_field {
                    0 => { self.category_form.name.push(c); }
                    1 => { if self.category_form.sku.len() < 3 { self.category_form.sku.push(c); } }
                    2 => { self.category_form.description.push(c); }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_new_tag_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.tag_form = TagForm::default();
                self.input_mode = InputMode::CreateTagSelect;
            }
            KeyCode::Enter => {
                // Save new tag
                if !self.tag_form.name.trim().is_empty() {
                    let new_tag = crate::api::Tag {
                        name: self.tag_form.name.clone(),
                        usage_count: 0,
                    };
                    self.tags.push(new_tag.name.clone());
                    self.create_form.tag_selected_index = self.tags.len() - 1;
                    self.status_message = format!("Tag '{}' created", self.tag_form.name);
                } else {
                    self.status_message = "Error: Tag name required".to_string();
                }
                self.tag_form = TagForm::default();
                self.input_mode = InputMode::CreateTagSelect;
            }
            KeyCode::Backspace => {
                self.tag_form.name.pop();
            }
            KeyCode::Char(c) => {
                self.tag_form.name.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_edit_tag_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.tag_form = TagForm::default();
                self.input_mode = InputMode::CreateTagSelect;
            }
            KeyCode::Enter => {
                // Save edited tag
                if !self.tag_form.name.trim().is_empty() {
                    if self.create_form.tag_selected_index < self.tags.len() {
                        self.tags[self.create_form.tag_selected_index] = self.tag_form.name.clone();
                        self.status_message = format!("Tag '{}' updated", self.tag_form.name);
                    }
                } else {
                    self.status_message = "Error: Tag name required".to_string();
                }
                self.tag_form = TagForm::default();
                self.input_mode = InputMode::CreateTagSelect;
            }
            KeyCode::Backspace => {
                self.tag_form.name.pop();
            }
            KeyCode::Char(c) => {
                self.tag_form.name.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_edit_production_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Tab => {
                // Cancel changes (discard) and return to normal mode
                if let Some(original_product) = self.edit_backup.take() {
                    // Restore original product data
                    if let Some(current_product) = self.products.get_mut(self.selected_index) {
                        *current_product = original_product;
                    }
                }
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
            }
            KeyCode::Enter => {
                // Save changes and return to normal mode
                self.edit_backup = None; // Clear backup since we're saving
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
                // TODO: Persist changes to backend
            }
            KeyCode::Up => {
                self.input_mode = InputMode::EditDescription;
            }
            KeyCode::Down => {
                // Already at last field, do nothing
            }
            KeyCode::Left => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.production = true;
                }
            }
            KeyCode::Right => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.production = false;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.production = true;
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.production = false;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_max_items(&self) -> usize {
        match self.current_tab {
            Tab::Search | Tab::Inventory => self.products.len(),
            Tab::Create => 0,
        }
    }

    pub fn has_multiple_panes(&self) -> bool {
        matches!(self.current_tab, Tab::Search | Tab::Inventory)
    }

    fn next_pane(&mut self) {
        if self.has_multiple_panes() {
            self.active_pane = match self.active_pane {
                ActivePane::Left => ActivePane::Right,
                ActivePane::Right => ActivePane::Left,
            };
        }
    }

    fn prev_pane(&mut self) {
        if self.has_multiple_panes() {
            self.active_pane = match self.active_pane {
                ActivePane::Left => ActivePane::Right,
                ActivePane::Right => ActivePane::Left,
            };
        }
    }

    fn next_item(&mut self) {
        let max_items = self.get_max_items();
        if max_items > 0 {
            self.selected_index = (self.selected_index + 1) % max_items;
        }
    }

    fn prev_item(&mut self) {
        let max_items = self.get_max_items();
        if max_items > 0 {
            self.selected_index = if self.selected_index == 0 {
                max_items - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    fn handle_mouse_event(&mut self, _mouse_event: crossterm::event::MouseEvent) {
        // Mouse handling not yet implemented
    }

    fn save_product(&mut self) -> Result<()> {
        // Validate required fields
        if self.create_form.name.trim().is_empty() {
            self.status_message = "Error: Product name is required".to_string();
            return Ok(());
        }

        if self.create_form.category_id.is_none() {
            self.status_message = "Error: Category must be selected".to_string();
            return Ok(());
        }

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
        };

        // For now, simulate API call success
        // TODO: Implement actual async API call
        self.status_message = format!("Product '{}' saved successfully!", product.name);

        // Reset form
        self.create_form = CreateForm {
            production: true,
            ..Default::default()
        };

        Ok(())
    }
}