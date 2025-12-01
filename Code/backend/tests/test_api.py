# tests/test_api.py
import pytest
from fastapi.testclient import TestClient
from app.main import app
from app.database import SessionLocal
from app import models


@pytest.fixture
def client():
    """Create test client"""
    return TestClient(app)


@pytest.fixture
def db_session():
    """Create test database session"""
    db = SessionLocal()
    try:
        yield db
    finally:
        db.rollback()
        db.close()


@pytest.fixture
def sample_category(db_session):
    """Create a sample category"""
    category = models.Category(name="Test Toys Unique", sku_initials="TTU")
    db_session.add(category)
    db_session.commit()
    db_session.refresh(category)
    return category


def test_create_product_api(client, sample_category):
    """Test creating a product via API with ID-based payload"""
    # First create some tags and materials to get IDs
    tag1_response = client.post("/tags", json={"name": "api"})
    tag2_response = client.post("/tags", json={"name": "test"})
    material1_response = client.post("/materials", json={"name": "PLA"})
    material2_response = client.post("/materials", json={"name": "PETG"})

    tag1_id = client.get("/tags").json()[0]["id"]  # Get first tag ID
    tag2_id = client.get("/tags").json()[1]["id"]  # Get second tag ID
    material1_id = client.get("/materials").json()[0]["id"]  # Get first material ID
    material2_id = client.get("/materials").json()[1]["id"]  # Get second material ID

    product_data = {
        "name": "API Test Product",
        "description": "Created via API test",
        "tag_ids": [tag1_id, tag2_id],  # ID-based payload
        "material_ids": [material1_id, material2_id],  # ID-based payload
        "production": False,
        "active": True,
        "category_id": sample_category.id,
    }

    response = client.post("/products/", json=product_data)
    assert response.status_code == 200

    data = response.json()
    assert "sku" in data
    assert data["sku"].startswith("TTU-")
    assert data["message"] == "Product created successfully"


def test_get_products_api_empty(client):
    """Test getting products when none exist"""
    response = client.get("/products/")
    assert response.status_code == 200

    data = response.json()
    assert isinstance(data, list)


def test_search_products_api(client):
    """Test searching products"""
    response = client.get("/products/search?q=test")
    assert response.status_code == 200

    data = response.json()
    assert isinstance(data, list)


def test_get_categories_api(client):
    """Test getting categories"""
    response = client.get("/categories/")
    assert response.status_code == 200

    data = response.json()
    assert isinstance(data, list)


def test_inventory_status_api(client):
    """Test getting inventory status"""
    response = client.get("/inventory/status")
    assert response.status_code == 200

    data = response.json()
    assert "products" in data
    assert "summary" in data
    assert isinstance(data["products"], list)
