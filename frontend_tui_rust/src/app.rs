use std::time::Duration;
use crossterm::event::{Event, KeyCode};
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
            KeyCode::Char('c') => {
                if matches!(self.current_tab, Tab::Create) {
                    self.input_mode = InputMode::CreateName;
                }
            }
            KeyCode::Enter => {
                match self.input_mode {
                    InputMode::Normal => {
                        if matches!(self.current_tab, Tab::Search) && !self.products.is_empty() {
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
            KeyCode::Esc | KeyCode::Tab => {
                // Cancel changes (discard) and return to normal mode
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
                // Note: No save logic here - TAB cancels, ENTER saves
            }
            KeyCode::Enter => {
                // Save changes and return to normal mode
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
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
                // Note: No save logic here - TAB cancels, ENTER saves
            }
            KeyCode::Enter => {
                // Save changes and return to normal mode
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

    fn handle_edit_production_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Tab => {
                // Cancel changes (discard) and return to normal mode
                self.input_mode = InputMode::Normal;
                self.active_pane = ActivePane::Left;
                // Note: No save logic here - TAB cancels, ENTER saves
            }
            KeyCode::Enter => {
                // Save changes and return to normal mode
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
}