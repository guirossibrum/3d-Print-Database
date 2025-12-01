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
    product_id: Optional[int] = None  # New field for frontend DRY architecture
    name: str
    description: Optional[str] = None
    tag_ids: List[int] = []  # ID-based for many-to-many relationship
    production: bool = False  # default = prototype / not production
    active: bool = True  # default = active
    category_id: Optional[int] = None  # ID-based for one-to-many relationship
    material_ids: List[int] = []  # ID-based for many-to-many relationship
    color: Optional[str] = None
    print_time: Optional[str] = None
    weight: Optional[int] = None
    rating: int = 0  # 0-5 star rating
    # Inventory management fields
    stock_quantity: Optional[int] = 0
    reorder_point: Optional[int] = 0
    unit_cost: Optional[int] = None  # Cost in cents
    selling_price: Optional[int] = None  # Price in cents
