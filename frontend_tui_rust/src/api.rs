use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub production: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub sku_initials: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub usage_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub sku: String,
    pub name: String,
    pub stock_quantity: i32,
    pub reorder_point: i32,
    pub status: String,
}

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

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let url = format!("{}/categories", self.base_url);
        let response = self.client.get(&url).send().await?;
        let categories = response.json().await?;
        Ok(categories)
    }

    pub async fn get_tags(&self) -> Result<Vec<Tag>> {
        let url = format!("{}/tags", self.base_url);
        let response = self.client.get(&url).send().await?;
        let tags = response.json().await?;
        Ok(tags)
    }

    pub async fn search_products(&self, query: &str) -> Result<Vec<Product>> {
        let url = format!("{}/products/search", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await?;
        let products = response.json().await?;
        Ok(products)
    }

    pub async fn create_product(&self, product: &Product) -> Result<Product> {
        let url = format!("{}/products/", self.base_url);
        let response = self.client
            .post(&url)
            .json(product)
            .send()
            .await?;
        let created_product = response.json().await?;
        Ok(created_product)
    }

    pub async fn update_product(&self, sku: &str, product: &Product) -> Result<Product> {
        let url = format!("{}/products/{}", self.base_url, sku);
        let response = self.client
            .put(&url)
            .json(product)
            .send()
            .await?;
        let updated_product = response.json().await?;
        Ok(updated_product)
    }

    pub async fn delete_product(&self, sku: &str) -> Result<()> {
        let url = format!("{}/products/{}", self.base_url, sku);
        self.client.delete(&url).send().await?;
        Ok(())
    }

    pub async fn get_inventory(&self) -> Result<Vec<InventoryItem>> {
        let url = format!("{}/inventory/status", self.base_url);
        let response = self.client.get(&url).send().await?;
        let inventory = response.json().await?;
        Ok(inventory)
    }
}