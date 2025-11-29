use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::{Product, Tag, Material, Category};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    // ✅ GET ALL PRODUCTS (for search tab)
    pub async fn get_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/products/", self.base_url);
        let response = self.client.get(&url).send().await?;
        let products = response.json().await?;
        Ok(products)
    }
    
    // ✅ GET ONE PRODUCT (natural naming - ID is implied)
    pub async fn get_product(&self, product_id: i32) -> Result<Product> {
        let url = format!("{}/products/{}", self.base_url, product_id);
        let response = self.client.get(&url).send().await?;
        let product = response.json().await?;
        Ok(product)
    }
    
    // ✅ CREATE PRODUCT
    pub async fn create_product(&self, product: &Product) -> Result<CreateResponse> {
        let url = format!("{}/products/", self.base_url);
        let response = self.client.post(&url).json(product).send().await?;
        let result = response.json().await?;
        Ok(result)
    }
    
    // ✅ UPDATE PRODUCT BY ID
    pub async fn update_product(&self, product_id: i32, product: &Product) -> Result<UpdateResponse> {
        let url = format!("{}/products/{}", self.base_url, product_id);
        let response = self.client.put(&url).json(product).send().await?;
        let result = response.json().await?;
        Ok(result)
    }
    
    // ✅ DELETE PRODUCT BY ID
    pub async fn delete_product(&self, product_id: i32, delete_files: bool) -> Result<String> {
        let url = format!("{}/products/{}?delete_files={}", self.base_url, product_id, delete_files);
        let response = self.client.delete(&url).send().await?;
        let result = response.json().await?;
        Ok(result["message"].as_str().unwrap_or("Deleted").to_string())
    }
    
    // ✅ SUPPORTING DATA
    pub async fn get_tags(&self) -> Result<Vec<Tag>> {
        let url = format!("{}/tags", self.base_url);
        let response = self.client.get(&url).send().await?;
        let tags = response.json().await?;
        Ok(tags)
    }
    
    pub async fn get_materials(&self) -> Result<Vec<Material>> {
        let url = format!("{}/materials", self.base_url);
        let response = self.client.get(&url).send().await?;
        let materials = response.json().await?;
        Ok(materials)
    }
    
    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let url = format!("{}/categories", self.base_url);
        let response = self.client.get(&url).send().await?;
        let categories = response.json().await?;
        Ok(categories)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResponse {
    pub product_id: i32,
    pub sku: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub product_id: i32,
    pub sku: String,
    pub message: String,
}