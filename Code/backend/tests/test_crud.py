# tests/test_crud.py
import pytest
from ..app import crud, schemas
from ..app.models import Product


def test_generate_sku_new_category(db_session, sample_category):
    """Test SKU generation for a new category"""
    sku = crud.generate_sku(db_session, sample_category.id)
    assert sku == "TT-0001"


def test_generate_sku_existing_products(db_session, sample_category, sample_product):
    """Test SKU generation when products already exist"""
    sku = crud.generate_sku(db_session, sample_category.id)
    assert sku == "TT-0002"


def test_create_product_db(db_session, sample_category):
    """Test creating a product in the database"""
    product_data = schemas.ProductCreate(
        name="Test Product",
        description="A test product",
        tags=["test"],
        production=False,
        category_id=sample_category.id,
    )

    sku = crud.create_product_db(db_session, product_data)
    assert sku.startswith("TT-")
    assert len(sku) == 7  # TT-0001 format

    # Verify product was created
    product = db_session.query(Product).filter(Product.sku == sku).first()
    assert product is not None
    assert product.name == "Test Product"
    assert product.description == "A test product"
    assert product.tags == ["test"]


def test_update_product_db(db_session, sample_product):
    """Test updating a product"""
    update_data = schemas.ProductUpdate(
        name="Updated Product", description="Updated description"
    )

    updated_product = crud.update_product_db(
        db_session, sample_product.sku, update_data
    )
    assert updated_product is not None
    assert updated_product.name == "Updated Product"
    assert updated_product.description == "Updated description"


def test_update_product_inventory(db_session, sample_product):
    """Test updating product inventory"""
    inventory_data = schemas.InventoryUpdate(
        stock_quantity=10,
        reorder_point=5,
        unit_cost=1000,  # $10.00
        selling_price=1500,  # $15.00
    )

    updated_product = crud.update_product_inventory(
        db_session, sample_product.sku, inventory_data
    )
    assert updated_product is not None
    assert updated_product.stock_quantity == 10
    assert updated_product.reorder_point == 5
    assert updated_product.unit_cost == 1000
    assert updated_product.selling_price == 1500
