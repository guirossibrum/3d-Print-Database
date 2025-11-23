# backend/app/models.py
from sqlalchemy import Column, Integer, Text, TIMESTAMP, Boolean, ForeignKey, Table
from sqlalchemy.orm import relationship
from .database import Base
from sqlalchemy.sql import func

# Association table for many-to-many
product_tags = Table(
    "product_tags",
    Base.metadata,
    Column("product_id", Integer, ForeignKey("products.id"), primary_key=True),
    Column("tag_id", Integer, ForeignKey("tags.id"), primary_key=True)
)

class Product(Base):
    __tablename__ = "products"

    id = Column(Integer, primary_key=True, index=True)
    sku = Column(Text, unique=True, nullable=False)
    name = Column(Text, nullable=False)
    description = Column(Text)
    folder_path = Column(Text, nullable=False)
    production = Column(Boolean, nullable=False, default=True)
    created_at = Column(TIMESTAMP, server_default=func.now())

    # many-to-many relationship via association table
    tags = relationship("Tag", secondary=product_tags, back_populates="products")

class Tag(Base):
    __tablename__ = "tags"

    id = Column(Integer, primary_key=True)
    name = Column(Text, unique=True, nullable=False)

    products = relationship("Product", secondary=product_tags, back_populates="tags")
