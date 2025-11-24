use reqwest::blocking::Client;
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
pub struct Tag {
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

    pub fn create_product(&self, product: &Product) -> Result<Product> {
        let url = format!("{}/products/", self.base_url);
        let response = self.client.post(&url)
            .json(product)
            .send()?;
        let created_product = response.json()?;
        Ok(created_product)
    }

    pub fn get_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/products/", self.base_url);
        let response = self.client.get(&url).send()?;
        let products = response.json()?;
        Ok(products)
    }

    pub fn create_category(&self, category: &Category) -> Result<Category> {
        let url = format!("{}/categories/", self.base_url);
        let response = self.client.post(&url)
            .json(category)
            .send()?;
        let created_category = response.json()?;
        Ok(created_category)
    }

    pub fn update_category(&self, category: &Category) -> Result<Category> {
        let id = category.id.unwrap();
        let url = format!("{}/categories/{}", self.base_url, id);
        let response = self.client.put(&url)
            .json(category)
            .send()?;
        let updated_category = response.json()?;
        Ok(updated_category)
    }

    pub fn create_tag(&self, tag: &Tag) -> Result<Tag> {
        let url = format!("{}/tags/", self.base_url);
        let response = self.client.post(&url)
            .json(tag)
            .send()?;
        let created_tag = response.json()?;
        Ok(created_tag)
    }

    pub fn update_tag(&self, tag: &Tag) -> Result<Tag> {
        let url = format!("{}/tags/{}", self.base_url, tag.name);
        let response = self.client.put(&url)
            .json(tag)
            .send()?;
        let updated_tag = response.json()?;
        Ok(updated_tag)
    }

    pub fn delete_tag(&self, tag_name: &str) -> Result<()> {
        let url = format!("{}/tags/{}", self.base_url, tag_name);
        self.client.delete(&url).send()?;
        Ok(())
    }




}