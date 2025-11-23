from pydantic import BaseModel
from typing import List, Optional


class TagBase(BaseModel):
    name: str


class ProductBase(BaseModel):
    sku: str
    name: str
    description: Optional[str] = None
    folder_path: str
    tags: List[str] = []


class ProductCreate(ProductBase):
    pass


class Product(ProductBase):
    id: int

    class Config:
        orm_mode = True
