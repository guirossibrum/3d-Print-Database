#!/usr/bin/env python3
"""
Test script to create a new product with existing database references.
This script reads existing data from the database and creates a new product.
"""

import sys
import os

backend_path = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "../../Code/backend")
)
sys.path.insert(0, backend_path)

from sqlalchemy.orm import Session
from app.database import SessionLocal, engine
from app.models import Product, Tag, Material, Category
from app import crud
import requests
import json


def read_database_data():
    """Read existing tags, materials, and categories from database."""
    db = SessionLocal()
    try:
        # Get all tags
        tags = db.query(Tag).all()
        tag_list = [{"id": tag.id, "name": tag.name} for tag in tags]

        # Get all materials
        materials = db.query(Material).all()
        material_list = [
            {"id": material.id, "name": material.name} for material in materials
        ]

        # Get all categories
        categories = db.query(Category).all()
        category_list = [
            {
                "id": category.id,
                "name": category.name,
                "sku_initials": category.sku_initials,
            }
            for category in categories
        ]

        print(
            f"Found {len(tag_list)} tags, {len(material_list)} materials, {len(category_list)} categories"
        )

        return {
            "tags": tag_list,
            "materials": material_list,
            "categories": category_list,
        }
    finally:
        db.close()


def create_test_product(db_data):
    """Create a new product using existing database references."""

    # Select existing IDs (use first available)
    if not db_data["tags"] or not db_data["materials"] or not db_data["categories"]:
        print("ERROR: Database must have at least one tag, material, and category")
        return False

    tag_id = db_data["tags"][0]["id"]
    material_id = db_data["materials"][0]["id"]
    category_id = db_data["categories"][0]["id"]

    # Create product payload based on schema
    product_payload = {
        "product_id": None,  # Blank for new product creation
        "name": "Test Product from Python Script",
        "description": "This is a test product created by automated test script",
        "tag_ids": [tag_id],  # Use existing tag
        "production": True,  # Boolean field
        "category_id": category_id,  # Use existing category
        "material_ids": [material_id],  # Use existing material
        "color": "Red",  # Text field
        "print_time": "02:30:00",  # Text field (HH:MM:SS format)
        "weight": 250,  # Number field (grams)
        "stock_quantity": 10,  # Number field
        "reorder_point": 5,  # Number field
        "unit_cost": 500,  # Number field (cents = $5.00)
        "selling_price": 1500,  # Number field (cents = $15.00)
    }

    print(f"Creating product with payload:")
    print(json.dumps(product_payload, indent=2))

    # Send request to backend API
    try:
        response = requests.post(
            "http://localhost:8000/products/",
            json=product_payload,
            headers={"Content-Type": "application/json"},
        )

        if response.status_code == 200:
            result = response.json()
            print(f"SUCCESS: Created product with ID {result.get('id')}")
            print(f"Product details: {json.dumps(result, indent=2)}")
            return True
        else:
            print(f"ERROR: Failed to create product")
            print(f"Status code: {response.status_code}")
            print(f"Response: {response.text}")
            return False

    except requests.exceptions.ConnectionError:
        print(
            "ERROR: Cannot connect to backend. Make sure backend is running on localhost:8000"
        )
        return False
    except Exception as e:
        print(f"ERROR: Unexpected error: {e}")
        return False


def main():
    """Main test function."""
    print("=== Product Creation Test ===")

    # Step 1: Read database contents
    print("\nStep 1: Reading database contents...")
    db_data = read_database_data()

    if not db_data["tags"] or not db_data["materials"] or not db_data["categories"]:
        print("ERROR: Database is missing required data. Please ensure you have:")
        print("- At least one tag")
        print("- At least one material")
        print("- At least one category")
        return False

    print(f"Database ready:")
    print(f"  Tags: {[tag['name'] for tag in db_data['tags']]}")
    print(f"  Materials: {[mat['name'] for mat in db_data['materials']]}")
    print(f"  Categories: {[cat['name'] for cat in db_data['categories']]}")

    # Step 2: Create test product
    print("\nStep 2: Creating test product...")
    success = create_test_product(db_data)

    if success:
        print("\n✅ TEST PASSED: Product created successfully")
        return True
    else:
        print("\n❌ TEST FAILED: Product creation failed")
        print("\n💡 To run this test successfully:")
        print(
            "   1. Start the backend: cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000"
        )
        print("   2. Run this script again")
        return False

    print(f"Database ready:")
    print(f"  Tags: {[tag['name'] for tag in db_data['tags']]}")
    print(f"  Materials: {[mat['name'] for mat in db_data['materials']]}")
    print(f"  Categories: {[cat['name'] for cat in db_data['categories']]}")

    # Step 2: Create test product
    print("\nStep 2: Creating test product...")
    success = create_test_product(db_data)

    if success:
        print("\n✅ TEST PASSED: Product created successfully")
        return True
    else:
        print("\n❌ TEST FAILED: Product creation failed")
        return False


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
