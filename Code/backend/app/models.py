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

# Association table for many-to-many product-materials relationship
product_materials = Table(
    "product_materials",
    Base.metadata,
    Column(
        "product_id", Integer, ForeignKey("products.id"), primary_key=True, index=True
    ),
    Column(
        "material_id", Integer, ForeignKey("materials.id"), primary_key=True, index=True
    ),
)


class Material(Base):
    __tablename__ = "materials"

    id = Column(Integer, primary_key=True)
    name = Column(Text, unique=True, nullable=False, index=True)  # Added index

    products = relationship(
        "Product", secondary=product_materials, back_populates="materials"
    )


class Tag(Base):
    __tablename__ = "tags"

    id = Column(Integer, primary_key=True)
    name = Column(Text, unique=True, nullable=False, index=True)  # Added index

    products = relationship("Product", secondary=product_tags, back_populates="tags")


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
    color = Column(Text)  # Color of print
    print_time = Column(Text)  # Print time (HH:MM or HH:MM:SS format)
    weight = Column(Integer)  # Weight in grams
    rating = Column(Integer, default=0, nullable=False)  # 0-5 star rating

    # Inventory management fields (sales-focused)
    stock_quantity = Column(Integer, default=0)  # Current stock count
    reorder_point = Column(Integer, default=0)  # Minimum stock level before reorder
    unit_cost = Column(Integer)  # Cost to produce/print (in cents)
    selling_price = Column(Integer)  # Retail selling price (in cents)

    # Relationships
    tags = relationship("Tag", secondary=product_tags, back_populates="products")
    materials = relationship(
        "Material", secondary=product_materials, back_populates="products"
    )
    category = relationship("Category", back_populates="products")


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
