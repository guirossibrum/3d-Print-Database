# backend/app/schemas.py
from pydantic import BaseModel
from typing import List, Optional


class TagBase(BaseModel):
    name: str


class CategoryBase(BaseModel):
    name: str
    sku_initials: str
    description: Optional[str] = None


class CategoryCreate(CategoryBase):
    pass


class Category(CategoryBase):
    id: int

    class Config:
        from_attributes = True


# Base class for products
class ProductBase(BaseModel):
    name: str
    description: Optional[str] = None
    tags: List[str] = []
    production: bool = False  # default = prototype / not production
    category_id: Optional[int] = None
    material: Optional[str] = None
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
    tags: Optional[List[str]] = None
    production: Optional[bool] = None  # optional for updates
    material: Optional[str] = None
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


# Schema for returning a product from DB
class Product(ProductBase):
    id: int
    sku: str
    folder_path: str

    class Config:
        from_attributes = True  # Pydantic V2 update (was orm_mode)
