use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::Terminal;
use anyhow::Result;

use crate::ui;
use crate::api::{ApiClient, Product};

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
    EditName,
    EditDescription,
    EditProduction,
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

    // UI state
    pub selected_index: usize,
    pub search_query: String,
    pub inventory_search_query: String,
    pub status_message: String,

    // Create form
    pub create_form: CreateForm,
}

#[derive(Debug, Default)]
pub struct CreateForm {
    pub name: String,
    pub description: String,
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

        Ok(Self {
            running: true,
            current_tab: Tab::Search,
            input_mode: InputMode::Normal,
            active_pane: ActivePane::Left,
            products,
            tags,
            selected_index: 0,
            search_query: String::new(),
            inventory_search_query: String::new(),
            status_message: String::new(),
            create_form: CreateForm::default(),
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
            InputMode::EditName => self.handle_edit_name_mode(key),
            InputMode::EditDescription => self.handle_edit_description_mode(key),
            InputMode::EditProduction => self.handle_edit_production_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            // Global navigation - Ctrl+Tab switches tabs
            KeyCode::Tab if key.modifiers.intersects(KeyModifiers::CONTROL) => {
                self.current_tab = self.current_tab.next();
                self.active_pane = ActivePane::Left;  // Reset to left pane
                self.selected_index = 0;
            }
            KeyCode::BackTab if key.modifiers.intersects(KeyModifiers::SHIFT) => {
                self.current_tab = self.current_tab.prev();
                self.active_pane = ActivePane::Left;  // Reset to left pane
                self.selected_index = 0;
            }
            // Pane navigation - Tab switches panes (if multiple panes exist)
            KeyCode::Tab if self.has_multiple_panes() => {
                self.next_pane();
            }
            KeyCode::BackTab if self.has_multiple_panes() => {
                self.prev_pane();
            }
            // Regular Tab (no modifier) - fallback for single-pane tabs
            KeyCode::Tab => {
                self.current_tab = self.current_tab.next();
                self.active_pane = ActivePane::Left;
                self.selected_index = 0;
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
            KeyCode::Char('/') => {
                if matches!(self.current_tab, Tab::Search) {
                    self.input_mode = InputMode::Search;
                } else if matches!(self.current_tab, Tab::Inventory) {
                    self.input_mode = InputMode::InventorySearch;
                }
            }
            KeyCode::Char('c') => {
                if matches!(self.current_tab, Tab::Create) {
                    self.input_mode = InputMode::CreateName;
                }
            }
            KeyCode::Enter => {
                if matches!(self.current_tab, Tab::Search) && !self.products.is_empty() {
                    self.input_mode = InputMode::EditName;
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
                if self.has_multiple_panes() {
                    self.next_pane();
                }
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
                if self.has_multiple_panes() {
                    self.next_pane();
                }
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
            KeyCode::Enter => {
                self.input_mode = InputMode::CreateDescription;
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
            KeyCode::Enter => {
                // TODO: Save the product
                self.input_mode = InputMode::Normal;
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

    fn handle_edit_name_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Enter | KeyCode::Down => {
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
            KeyCode::Esc => {
                self.input_mode = InputMode::EditName;
            }
            KeyCode::Enter | KeyCode::Down => {
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

    fn handle_edit_production_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::EditDescription;
            }
            KeyCode::Enter => {
                // TODO: Save changes
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Up => {
                self.input_mode = InputMode::EditDescription;
            }
            KeyCode::Down => {
                // Already at last field, do nothing
            }
            KeyCode::Left => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.production = false;
                }
            }
            KeyCode::Right => {
                if let Some(product) = self.products.get_mut(self.selected_index) {
                    product.production = true;
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
}