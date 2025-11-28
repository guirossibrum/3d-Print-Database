/// Represents the different tabs in the TUI application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Create,
    Search,
    Inventory,
}



/// Type of item being created/edited
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Tag,
    Material,
}

/// Type of selection being performed
pub type SelectionType = ItemType;

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
    CreateMaterials,
    CreateMaterialSelect,
    EditName,
    EditDescription,
    EditProduction,
    EditTags,
    EditMaterials,
    EditSelect,
    #[allow(dead_code)]
    NewCategory,
    #[allow(dead_code)]
    EditCategory,
    #[allow(dead_code)]
    NewTag,
    #[allow(dead_code)]
    EditTag,
    #[allow(dead_code)]
    NewMaterial,
    #[allow(dead_code)]
    EditMaterial,
    DeleteConfirm,
    DeleteFileConfirm,
}

impl InputMode {
    /// Check if this mode is a create-related mode
    pub fn is_create_mode(&self) -> bool {
        matches!(
            self,
            InputMode::CreateName
                | InputMode::CreateDescription
                | InputMode::CreateCategory
                | InputMode::CreateCategorySelect
                | InputMode::CreateProduction
                | InputMode::CreateTags
                | InputMode::CreateTagSelect
                | InputMode::CreateMaterials
                | InputMode::CreateMaterialSelect
        )
    }

    /// Check if this mode is an edit-related mode
    pub fn is_edit_mode(&self) -> bool {
        matches!(
            self,
            InputMode::EditName
                | InputMode::EditDescription
                | InputMode::EditProduction
                | InputMode::EditTags
                | InputMode::EditMaterials
                | InputMode::EditSelect
        )
    }

    /// Check if this mode is a selection mode
    pub fn is_select_mode(&self) -> bool {
        matches!(
            self,
            InputMode::CreateCategorySelect
                | InputMode::CreateTagSelect
                | InputMode::CreateMaterialSelect
                | InputMode::EditSelect
        )
    }

    /// Check if this mode is a delete confirmation mode
    pub fn is_delete_mode(&self) -> bool {
        matches!(self, InputMode::DeleteConfirm | InputMode::DeleteFileConfirm)
    }
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
    pub materials: Vec<String>,
    pub material_selected_index: usize,
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
