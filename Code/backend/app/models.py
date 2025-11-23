# backend/app/models.py
from sqlalchemy import Column, Integer, Text, TIMESTAMP, Boolean, ForeignKey, Table
from sqlalchemy.orm import relationship
from .database import Base
from sqlalchemy.sql import func

# Association table for many-to-many
product_tags = Table(
    "product_tags",
    Base.metadata,
    Column(
        "product_id", Integer, ForeignKey("products.id"), primary_key=True, index=True
    ),
    Column("tag_id", Integer, ForeignKey("tags.id"), primary_key=True, index=True),
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
    category_id = Column(Integer, ForeignKey("categories.id"))

    # New optional fields
    material = Column(Text)  # Material used (PLA, ABS, etc.)
    color = Column(Text)  # Color of the print
    print_time = Column(Text)  # Print time (HH:MM or HH:MM:SS format)
    weight = Column(Integer)  # Weight in grams

    # Relationships
    tags = relationship("Tag", secondary=product_tags, back_populates="products")
    category = relationship("Category", back_populates="products")


class Tag(Base):
    __tablename__ = "tags"

    id = Column(Integer, primary_key=True)
    name = Column(Text, unique=True, nullable=False, index=True)  # Added index

    products = relationship("Product", secondary=product_tags, back_populates="tags")


class Category(Base):
    __tablename__ = "categories"

    id = Column(Integer, primary_key=True)
    name = Column(Text, unique=True, nullable=False)
    sku_initials = Column(Text, unique=True, nullable=False)  # 3-letter code
    description = Column(Text)

    # Relationship to products (optional, for future use)
    products = relationship("Product", back_populates="category")

    def __str__(self):
        return f"{self.name} ({self.sku_initials})"
