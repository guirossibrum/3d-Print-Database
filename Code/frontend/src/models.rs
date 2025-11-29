/// Core application types and enums for the 3D Print Database TUI

/// Represents the three main tabs in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    /// Create new products, tags, materials, categories
    Create,
    /// Search and browse existing products
    Search,
    /// View inventory and product details
    Inventory,
}

impl Tab {
    /// Navigate to the next tab in the cycle
    pub fn next(&self) -> Self {
        match self {
            Tab::Create => Tab::Search,
            Tab::Search => Tab::Inventory,
            Tab::Inventory => Tab::Create,
        }
    }

    /// Navigate to the previous tab in the cycle
    pub fn prev(&self) -> Self {
        match self {
            Tab::Create => Tab::Inventory,
            Tab::Search => Tab::Create,
            Tab::Inventory => Tab::Search,
        }
    }
}

/// Application input modes - simplified from deprecated version
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    /// Normal navigation mode - browse products, switch tabs
    Normal,
    /// Edit mode - modify existing product
    Edit,
    /// Create mode - add new product
    Create,
    /// Selection mode - choose tags, materials, categories
    Select,
    /// Delete confirmation mode
    Delete,
    /// Edit specific field modes
    EditName,
    EditDescription,
    EditCategory,
    EditProduction,
    EditTags,
    EditMaterials,
    /// Delete confirmation with file options
    DeleteConfirm,
    DeleteFileConfirm,
}

/// Tag structure matching backend TagResponse schema
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

/// Material structure matching backend MaterialResponse schema
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub id: i32,
    pub name: String,
}

/// Category structure matching backend CategoryResponse schema
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub sku_initials: String,
    pub description: Option<String>,
}

/// Complete product structure matching backend Product schema
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    // Basic product fields
    pub id: i32,
    #[serde(default)]
    pub product_id: i32,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub folder_path: String,
    
    // Production settings
    pub production: bool,
    pub color: Option<String>,
    pub print_time: Option<String>,
    pub weight: Option<i32>,
    
    // Relationships (nested objects from backend)
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub materials: Vec<Material>,
    #[serde(default)]
    pub category: Option<Category>,
    pub category_id: Option<i32>,
    
    // Inventory management
    #[serde(default)]
    pub stock_quantity: Option<i32>,
    #[serde(default)]
    pub reorder_point: Option<i32>,
    #[serde(default)]
    pub unit_cost: Option<i32>,  // Cost in cents
    #[serde(default)]
    pub selling_price: Option<i32>,  // Price in cents
    
    // ✅ NEW: Active status field
    #[serde(default)]
    pub active: bool,
}

