# backend/app/schemas.py
from pydantic import BaseModel
from typing import List, Optional


class TagBase(BaseModel):
    name: str


class TagCreate(TagBase):
    pass


class TagUpdate(BaseModel):
    name: Optional[str] = None


class Tag(TagBase):
    usage_count: int

    class Config:
        from_attributes = True


class MaterialBase(BaseModel):
    name: str


class MaterialCreate(MaterialBase):
    pass


class MaterialUpdate(BaseModel):
    name: Optional[str] = None


class Material(MaterialBase):
    usage_count: int

    class Config:
        from_attributes = True


class CategoryBase(BaseModel):
    name: str
    sku_initials: str
    description: Optional[str] = None


class CategoryCreate(CategoryBase):
    pass


class CategoryUpdate(BaseModel):
    name: Optional[str] = None
    sku_initials: Optional[str] = None
    description: Optional[str] = None


class Category(CategoryBase):
    id: int

    class Config:
        from_attributes = True


# Base class for products
class ProductBase(BaseModel):
    name: str
    description: Optional[str] = None
    tag_ids: List[int] = []  # ID-based for many-to-many relationship
    production: bool = False  # default = prototype / not production
    category_id: Optional[int] = None  # ID-based for one-to-many relationship
    material_ids: List[int] = []  # ID-based for many-to-many relationship
    color: Optional[str] = None
    print_time: Optional[str] = None
    weight: Optional[int] = None
    # Inventory management fields
    stock_quantity: Optional[int] = 0
    reorder_point: Optional[int] = 0
    unit_cost: Optional[int] = None  # Cost in cents
    selling_price: Optional[int] = None  # Price in cents


# Schema for creating a product (DB generates SKU and folder_path)
class ProductCreate(ProductBase):
    pass


# Schema for updating a product (all fields optional)
class ProductUpdate(BaseModel):
    name: Optional[str] = None
    description: Optional[str] = None
    tag_ids: Optional[List[int]] = None  # ID-based for many-to-many relationship
    production: Optional[bool] = None  # optional for updates
    material_ids: Optional[List[int]] = None  # ID-based for many-to-many relationship
    color: Optional[str] = None
    print_time: Optional[str] = None
    weight: Optional[int] = None
    # Inventory management fields
    stock_quantity: Optional[int] = None
    reorder_point: Optional[int] = None
    unit_cost: Optional[int] = None
    selling_price: Optional[int] = None


# Schema for inventory-specific updates
class InventoryUpdate(BaseModel):
    stock_quantity: Optional[int] = None
    reorder_point: Optional[int] = None
    unit_cost: Optional[int] = None
    selling_price: Optional[int] = None


# Helper schemas for nested responses
class TagResponse(BaseModel):
    id: int
    name: str

    class Config:
        from_attributes = True


class MaterialResponse(BaseModel):
    id: int
    name: str

    class Config:
        from_attributes = True


class CategoryResponse(BaseModel):
    id: int
    name: str
    sku_initials: str
    description: Optional[str] = None

    class Config:
        from_attributes = True


# Schema for returning a product from DB (with full nested objects)
class Product(BaseModel):
    # Product fields
    name: str
    description: Optional[str] = None
    production: bool = False
    category_id: Optional[int] = None
    color: Optional[str] = None
    print_time: Optional[str] = None
    weight: Optional[int] = None
    stock_quantity: Optional[int] = 0
    reorder_point: Optional[int] = 0
    unit_cost: Optional[int] = None
    selling_price: Optional[int] = None

    # Product-specific fields
    id: int
    sku: str
    folder_path: str

    # Nested relationships (both ID and name for frontend convenience)
    tags: List[TagResponse] = []
    materials: List[MaterialResponse] = []
    category: Optional[CategoryResponse] = None

    class Config:
        from_attributes = True  # Pydantic V2 update (was orm_mode)
