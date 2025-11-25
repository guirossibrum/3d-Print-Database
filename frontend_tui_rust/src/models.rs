/// Represents the different tabs in the TUI application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Create,
    Search,
    Inventory,
}

/// Mode for tag selection
#[derive(Debug, Clone, PartialEq)]
pub enum TagSelectMode {
    Create,
    #[allow(dead_code)]
    Edit,
}

/// Type of item being created/edited
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Tag,
    #[allow(dead_code)]
    Category,
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
    EditTags,
    EditTagSelect,
    #[allow(dead_code)]
    NewCategory,
    #[allow(dead_code)]
    EditCategory,
    #[allow(dead_code)]
    NewTag,
    #[allow(dead_code)]
    EditTag,
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