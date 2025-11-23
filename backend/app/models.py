from sqlalchemy import Column, Integer, Text, TIMESTAMP, ForeignKey
from sqlalchemy.orm import relationship
from .database import Base
from sqlalchemy.sql import func


class Product(Base):
    __tablename__ = "products"

    id = Column(Integer, primary_key=True, index=True)
    sku = Column(Text, unique=True, nullable=False)
    name = Column(Text, nullable=False)
    description = Column(Text)
    folder_path = Column(Text, nullable=False)
    created_at = Column(TIMESTAMP, server_default=func.now())

    tags = relationship("ProductTag", back_populates="product", cascade="all, delete")


class Tag(Base):
    __tablename__ = "tags"

    id = Column(Integer, primary_key=True)
    name = Column(Text, unique=True, nullable=False)

    products = relationship("ProductTag", back_populates="tag", cascade="all, delete")


class ProductTag(Base):
    __tablename__ = "product_tags"

    product_id = Column(Integer, ForeignKey("products.id"), primary_key=True)
    tag_id = Column(Integer, ForeignKey("tags.id"), primary_key=True)

    product = relationship("Product", back_populates="tags")
    tag = relationship("Tag", back_populates="products")
