// state.rs - Clean, optimized state management
use anyhow::Result;

use crate::api::ApiClient;
use crate::models::{Product, Tag, Material, Category, Tab, InputMode};

pub struct App {
    // ✅ API CLIENT INTEGRATION
    api_client: ApiClient,
    
    // ✅ APPLICATION STATE
    pub running: bool,
    current_tab: Tab,
    input_mode: InputMode,
    
    // ✅ DATA STATE (SIMPLE VECTORS)
    products: Vec<Product>,
    tags: Vec<Tag>,
    materials: Vec<Material>,
    categories: Vec<Category>,
    
    // ✅ UI STATE
    selected_product_id: Option<i32>,
    selected_index: usize,
    search_query: String,
    status_message: String,
}

impl App {
    // ✅ INITIALIZE APPLICATION
    pub fn new() -> Result<Self> {
        let api_client = ApiClient::new("http://localhost:8000".to_string());
        
        Ok(Self {
            api_client,
            running: true,
            current_tab: Tab::Search,
            input_mode: InputMode::Normal,
            products: Vec::new(),
            tags: Vec::new(),
            materials: Vec::new(),
            categories: Vec::new(),
            selected_product_id: None,
            selected_index: 0,
            search_query: String::new(),
            status_message: String::new(),
        })
    }
    
    // ✅ LOAD ALL DATA (SINGLE CALL)
    pub async fn load_all_data(&mut self) -> Result<()> {
        self.products = self.api_client.get_products().await?;
        self.tags = self.api_client.get_tags().await?;
        self.materials = self.api_client.get_materials().await?;
        self.categories = self.api_client.get_categories().await?;
        
        // ✅ SORT ALL DATA AT ONCE
        self.products.sort_by(|a, b| a.name.cmp(&b.name));
        self.tags.sort_by(|a, b| a.name.cmp(&b.name));
        self.materials.sort_by(|a, b| a.name.cmp(&b.name));
        self.categories.sort_by(|a, b| a.name.cmp(&b.name));
        
        // ✅ NEW: FILTER ONLY ACTIVE PRODUCTS
        self.products.retain(|p| p.active);
        
        // ✅ UNIVERSAL SELECTION LOGIC
        self.ensure_valid_selection();
        
        Ok(())
    }
    
    // ✅ STATE MANAGEMENT
    pub fn select_product(&mut self, product_id: i32) {
        self.selected_product_id = Some(product_id);
        self.selected_index = self.products.iter()
            .position(|p| p.id == product_id)
            .unwrap_or(0);
    }
    
    pub fn update_selected_product_id(&mut self) {
        self.selected_product_id = self.products
            .get(self.selected_index)
            .map(|p| p.id);
    }
    
    pub fn ensure_valid_selection(&mut self) {
        if self.selected_product_id.is_none() && !self.products.is_empty() {
            self.selected_index = 0;
        } else {
            if self.selected_index >= self.products.len() {
                self.selected_index = self.products.len().saturating_sub(1);
            }
        }
        self.update_selected_product_id();
    }
    
    pub fn set_status(&mut self, message: String) {
        self.status_message = message;
    }
    
    pub fn quit(&mut self) {
        self.running = false;
    }
}