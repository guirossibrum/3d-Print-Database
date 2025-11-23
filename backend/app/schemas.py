# backend/app/schemas.py
from pydantic import BaseModel
from typing import List, Optional

class TagBase(BaseModel):
    name: str

# Base class for products
class ProductBase(BaseModel):
    name: str
    description: Optional[str] = None
    tags: List[str] = []
    production: bool = False  # default = prototype / not production

# Schema for creating a product (DB generates SKU and folder_path)
class ProductCreate(ProductBase):
    pass

# Schema for updating a product (all fields optional)
class ProductUpdate(BaseModel):
    name: Optional[str] = None
    description: Optional[str] = None
    tags: Optional[List[str]] = None
    production: Optional[bool] = None  # optional for updates

# Schema for returning a product from DB
class Product(ProductBase):
    id: int
    sku: str
    folder_path: str

    class Config:
        from_attributes = True  # Pydantic V2 update (was orm_mode)
