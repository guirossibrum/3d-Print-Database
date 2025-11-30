#!/usr/bin/env python3
"""
Database restoration script from metadata.json files
Recreates database entries based on product metadata files
"""

import os
import json
import sys
from pathlib import Path
from typing import Dict, List, Set

# Add backend to path
sys.path.insert(0, "/home/grbrum/Documents/Code/3d_print_database/Code/backend")

from app.database import SessionLocal
from app import crud, models, schemas
from sqlalchemy.orm import Session


def get_all_metadata_files(products_dir: str) -> List[str]:
    """Find all metadata.json files in the products directory"""
    metadata_files = []
    for root, dirs, files in os.walk(products_dir):
        if "metadata.json" in files:
            metadata_files.append(os.path.join(root, "metadata.json"))
    return metadata_files


def load_metadata_file(filepath: str) -> Dict | None:
    """Load and parse a metadata.json file"""
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            return json.load(f)
    except Exception as e:
        print(f"Error loading {filepath}: {e}")
        return None


def create_or_get_category(db: Session, category_data: Dict) -> models.Category | None:
    """Create category if it doesn't exist, otherwise return existing"""
    try:
        # Check if category exists by sku_initials
        existing = (
            db.query(models.Category)
            .filter(models.Category.sku_initials == category_data["sku_initials"])
            .first()
        )

        if existing:
            return existing

        # Create new category
        category = models.Category(
            name=category_data["name"],
            sku_initials=category_data["sku_initials"],
            description=category_data.get("description"),
        )
        db.add(category)
        db.commit()
        db.refresh(category)
        return category
    except Exception as e:
        print(f"Error creating/getting category {category_data}: {e}")
        return None


def create_or_get_tag(db: Session, tag_name: str) -> models.Tag:
    """Create tag if it doesn't exist, otherwise return existing"""
    existing = db.query(models.Tag).filter(models.Tag.name == tag_name).first()
    if existing:
        return existing

    tag = models.Tag(name=tag_name)
    db.add(tag)
    db.commit()
    db.refresh(tag)
    return tag


def create_or_get_material(db: Session, material_name: str) -> models.Material:
    """Create material if it doesn't exist, otherwise return existing"""
    existing = (
        db.query(models.Material).filter(models.Material.name == material_name).first()
    )
    if existing:
        return existing

    material = models.Material(name=material_name)
    db.add(material)
    db.commit()
    db.refresh(material)
    return material


def restore_product_from_metadata(db: Session, metadata: Dict, folder_path: str):
    """Restore a single product from metadata"""
    try:
        # Handle category - some files might not have it
        category = None
        if "category" in metadata:
            category = create_or_get_category(db, metadata["category"])
        else:
            # Create a default category if none exists
            default_category = create_or_get_category(
                db,
                {
                    "name": "Uncategorized",
                    "sku_initials": "UNC",
                    "description": "Products without category",
                },
            )
            category = default_category

        if category is None:
            print(
                f"Warning: Could not create/get category for {metadata.get('sku', 'unknown')}"
            )
            return

        # Get or create tags
        tags = []
        tag_list = metadata.get("tags", [])
        if tag_list:
            for tag_name in tag_list:
                tag = create_or_get_tag(db, tag_name)
                if tag:
                    tags.append(tag)

        # Get or create materials - handle both 'material' (singular) and 'materials' (plural)
        materials = []
        material_list = metadata.get("materials", [])
        if not material_list:
            # Try singular 'material' field
            material_value = metadata.get("material")
            if (
                material_value
                and material_value != "null"
                and material_value is not None
            ):
                if isinstance(material_value, str):
                    material_list = [material_value]
                elif isinstance(material_value, list):
                    material_list = material_value

        if material_list:
            for material_name in material_list:
                if material_name and str(material_name).lower() != "null":
                    material = create_or_get_material(db, str(material_name))
                    if material:
                        materials.append(material)

        # Check if product already exists
        existing_product = (
            db.query(models.Product)
            .filter(models.Product.sku == metadata["sku"])
            .first()
        )

        if existing_product:
            print(f"Product {metadata['sku']} already exists, skipping")
            return

        # Create product
        product = models.Product(
            sku=metadata["sku"],
            name=metadata["name"],
            description=metadata.get("description"),
            folder_path=folder_path,
            production=metadata.get("production", True),
            category_id=category.id,
            color=metadata.get("color"),
            print_time=metadata.get("print_time")
            if metadata.get("print_time") != "__:__"
            else None,
            weight=metadata.get("weight"),
            rating=0,  # Default rating
            stock_quantity=metadata.get("stock_quantity", 0),
            reorder_point=metadata.get("reorder_point", 0),
            unit_cost=metadata.get("unit_cost"),
            selling_price=metadata.get("selling_price"),
        )

        db.add(product)
        db.commit()
        db.refresh(product)

        # Add tag relationships
        for tag in tags:
            product.tags.append(tag)

        # Add material relationships
        for material in materials:
            product.materials.append(material)

        db.commit()
        print(f"Restored product: {metadata['sku']} - {metadata['name']}")

    except Exception as e:
        print(f"Error restoring product {metadata.get('sku', 'unknown')}: {e}")
        db.rollback()


def main():
    products_dir = "/home/grbrum/Work/3d_print/Products"

    if not os.path.exists(products_dir):
        print(f"Products directory not found: {products_dir}")
        return

    print("🔍 Finding metadata.json files...")
    metadata_files = get_all_metadata_files(products_dir)
    print(f"Found {len(metadata_files)} metadata files")

    if not metadata_files:
        print("No metadata files found!")
        return

    db = SessionLocal()
    try:
        restored_count = 0
        for metadata_file in metadata_files:
            metadata = load_metadata_file(metadata_file)
            if metadata:
                # Get folder path (parent directory of metadata.json)
                folder_path = os.path.dirname(metadata_file)
                restore_product_from_metadata(db, metadata, folder_path)
                restored_count += 1

        print(
            f"\n✅ Successfully restored {restored_count} products from metadata files"
        )

        # Print summary
        product_count = db.query(models.Product).count()
        category_count = db.query(models.Category).count()
        tag_count = db.query(models.Tag).count()
        material_count = db.query(models.Material).count()

        print("\n📊 Database Summary:")
        print(f"   Products: {product_count}")
        print(f"   Categories: {category_count}")
        print(f"   Tags: {tag_count}")
        print(f"   Materials: {material_count}")

    except Exception as e:
        print(f"❌ Error during restoration: {e}")
    finally:
        db.close()


if __name__ == "__main__":
    main()
