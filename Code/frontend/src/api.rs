use anyhow::Result;
use anyhow::anyhow;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    pub id: Option<i32>,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub production: bool,
    pub tags: Vec<String>,
    pub category_id: Option<i32>,
    pub material: Option<Vec<String>>, // Changed to support multiple materials
    pub color: Option<String>,
    pub print_time: Option<i32>,
    pub weight: Option<f64>,
    pub stock_quantity: Option<i32>,
    pub reorder_point: Option<i32>,
    pub unit_cost: Option<f64>,
    pub selling_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub production: Option<bool>,
    pub materials: Option<Vec<String>>, // Match backend schema (plural)
    pub color: Option<String>,
    pub print_time: Option<String>, // Match backend (string, not i32)
    pub weight: Option<i32>, // Match backend (i32, not f64)
    pub stock_quantity: Option<i32>,
    pub reorder_point: Option<i32>,
    pub unit_cost: Option<i32>, // Match backend (i32, not f64)
    pub selling_price: Option<i32>, // Match backend (i32, not f64)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub usage_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub usage_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
    pub sku_initials: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveProductResponse {
    pub sku: String,
    pub message: String,
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

    pub fn get_tags(&self) -> Result<Vec<Tag>> {
        let url = format!("{}/tags", self.base_url);
        let response = self.client.get(&url).send()?;
        let tags = response.json()?;
        Ok(tags)
    }

    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let url = format!("{}/categories", self.base_url);
        let response = self.client.get(&url).send()?;
        let categories = response.json()?;
        Ok(categories)
    }



    pub fn update_product(&self, sku: &str, update: &ProductUpdate) -> Result<String> {
        let url = format!("{}/products/{}", self.base_url, sku);
        let response = self.client.put(&url).json(update).send()?;
        if response.status().is_success() {
            Ok("Product updated successfully".to_string())
        } else {
            Err(anyhow!("Failed to update product: {}", response.status()))
        }
    }

    pub fn save_product(&self, product: &Product) -> Result<SaveProductResponse> {
        match product.id {
            Some(_) => {
                // Update existing product - use PUT /products/{sku}
                let url = format!("{}/products/{}", self.base_url, product.sku);
                let update_data = ProductUpdate {
                    name: Some(product.name.clone()),
                    description: product.description.clone(),
                    tags: Some(product.tags.clone()),
                    production: Some(product.production),
                    materials: product.material.clone(), // Match backend field name
                    color: product.color.clone(),
                    print_time: product.print_time.clone().map(|t| t.to_string()), // Convert to string
                    weight: product.weight.map(|w| w as i32), // Convert to i32
                    stock_quantity: product.stock_quantity,
                    reorder_point: product.reorder_point,
                    unit_cost: product.unit_cost.map(|c| c as i32), // Convert to i32
                    selling_price: product.selling_price.map(|p| p as i32), // Convert to i32
                };
                let response = self.client.put(&url).json(&update_data).send()?;
                if response.status().is_success() {
                    let save_response = response.json()?;
                    Ok(save_response)
                } else {
                    // Try to get error message from response
                    let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                    Err(anyhow!("Save failed: {}", error_text))
                }
            }
            None => {
                // Create new product - use POST /products/
                let url = format!("{}/products/", self.base_url);
                let create_data = serde_json::json!({
                    "name": product.name,
                    "description": product.description,
                    "tags": product.tags,
                    "production": product.production,
                    "materials": product.material,
                    "color": product.color,
                    "print_time": product.print_time.map(|t| t.to_string()),
                    "weight": product.weight.map(|w| w as i32),
                    "stock_quantity": product.stock_quantity,
                    "reorder_point": product.reorder_point,
                    "unit_cost": product.unit_cost.map(|c| c as i32),
                    "selling_price": product.selling_price.map(|p| p as i32)
                });
                let response = self.client.post(&url).json(&create_data).send()?;
                if response.status().is_success() {
                    let save_response = response.json()?;
                    Ok(save_response)
                } else {
                    // Try to get error message from response
                    let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                    Err(anyhow!("Create failed: {}", error_text))
                }
            }
        }
    }

    pub fn get_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/products/", self.base_url);
        let response = self.client.get(&url).send()?;
        let products = response.json()?;
        Ok(products)
    }

    pub fn get_product_by_sku(&self, sku: &str) -> Result<Product> {
        let url = format!("{}/products/{}", self.base_url, sku);
        let response = self.client.get(&url).send()?;
        let product = response.json()?;
        Ok(product)
    }

    pub fn create_category(&self, category: &Category) -> Result<Category> {
        let url = format!("{}/categories/", self.base_url);
        let response = self.client.post(&url).json(category).send()?;
        let created_category = response.json()?;
        Ok(created_category)
    }

    pub fn update_category(&self, category: &Category) -> Result<Category> {
        let id = match category.id {
            Some(id) => id,
            None => return Err(anyhow::anyhow!("Category ID is required for updates")),
        };
        let url = format!("{}/categories/{}", self.base_url, id);
        let response = self.client.put(&url).json(category).send()?;
        let updated_category = response.json()?;
        Ok(updated_category)
    }

    pub fn create_tag(&self, tag: &Tag) -> Result<Tag> {
        let url = format!("{}/tags/", self.base_url);
        let response = self.client.post(&url).json(tag).send()?;
        let created_tag = response.json()?;
        Ok(created_tag)
    }

    pub fn update_tag(&self, tag: &Tag) -> Result<Tag> {
        let url = format!("{}/tags/{}", self.base_url, tag.name);
        let response = self.client.put(&url).json(tag).send()?;
        let updated_tag = response.json()?;
        Ok(updated_tag)
    }

    pub fn delete_tag(&self, tag_name: &str) -> Result<()> {
        let url = format!("{}/tags/{}", self.base_url, tag_name);
        self.client.delete(&url).send()?;
        Ok(())
    }

    pub fn get_materials(&self) -> Result<Vec<Material>> {
        let url = format!("{}/materials", self.base_url);
        let response = self.client.get(&url).send()?;
        let materials = response.json()?;
        Ok(materials)
    }

    pub fn create_material(&self, material: &Material) -> Result<Material> {
        let url = format!("{}/materials/", self.base_url);
        let response = self.client.post(&url).json(material).send()?;
        let created_material = response.json()?;
        Ok(created_material)
    }

    pub fn update_material(&self, material: &Material) -> Result<Material> {
        let url = format!("{}/materials/{}", self.base_url, material.name);
        let response = self.client.put(&url).json(material).send()?;
        let updated_material = response.json()?;
        Ok(updated_material)
    }

    pub fn delete_material(&self, material_name: &str) -> Result<()> {
        let url = format!("{}/materials/{}", self.base_url, material_name);
        self.client.delete(&url).send()?;
        Ok(())
    }

    pub fn delete_product(&self, sku: &str, delete_files: bool) -> Result<String> {
        let url = format!(
            "{}/products/{}?delete_files={}",
            self.base_url, sku, delete_files
        );
        let response = self.client.delete(&url).send()?;
        if response.status().is_success() {
            Ok("Product deleted successfully".to_string())
        } else {
            Err(anyhow!("Failed to delete product: {}", response.status()))
        }
    }
}
