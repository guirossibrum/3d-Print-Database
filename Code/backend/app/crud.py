# backend/app/crud.py
import os
import json
from typing import Optional, List
from sqlalchemy.orm import Session
from . import models, schemas
from ensure_file_structure import (
    create_product_folder,
)  # import function


def generate_sku(db: Session, category_id: int) -> str:
    """
    Generate a unique SKU using category initials
    Format: XXX-0001 where XXX is the category's sku_initials
    """
    # Get the category
    category = (
        db.query(models.Category).filter(models.Category.id == category_id).first()
    )
    if not category:
        raise ValueError(f"Category with id {category_id} not found")

    prefix = category.sku_initials.upper()

    # Find all SKUs with this prefix, extract numbers, find max
    existing_skus = (
        db.query(models.Product.sku)
        .filter(models.Product.sku.like(f"{prefix}-%"))
        .all()
    )

    max_num = 0
    for sku_row in existing_skus:
        sku = sku_row.sku
        try:
            num = int(sku.split("-")[-1])
            max_num = max(max_num, num)
        except (ValueError, IndexError):
            continue  # Skip malformed SKUs

    return f"{prefix}-{max_num + 1:04d}"


def save_product(db: Session, product: schemas.ProductBase) -> dict:
    """
    Unified product save function - replaces create_product_db() and update_product_by_id().
    Handles both create and update based on product_id.

    FRONTEND: Use this unified function for both create and update operations.
    """
    if product.product_id is None or product.product_id == 0:
        # CREATE NEW PRODUCT
        # Generate SKU if not provided
        if not product.category_id:
            raise ValueError("Category is required for SKU generation")
        sku = generate_sku(db, product.category_id)

        # Get tag and material names for metadata
        tag_names = get_tag_names_by_ids(db, product.tag_ids)
        material_names = get_material_names_by_ids(db, product.material_ids)

        # Get category details for folder creation
        category_details = None
        if product.category_id:
            category_obj = (
                db.query(models.Category)
                .filter(models.Category.id == product.category_id)
                .first()
            )
            if category_obj:
                category_details = {
                    "name": category_obj.name,
                    "sku_initials": category_obj.sku_initials,
                    "description": category_obj.description,
                }

        # Create folder & metadata first
        folder_path, _ = create_product_folder(
            sku=sku,
            name=product.name,
            description=product.description or "",
            tags=tag_names,
            production=product.production,
            materials=material_names,
            category=category_details,
            rating=product.rating,
        )

        # Save product to DB with folder_path
        db_product = models.Product(
            sku=sku,
            name=product.name,
            description=product.description,
            folder_path=folder_path,
            production=product.production,
            active=product.active,
            category_id=product.category_id,
            color=product.color,
            print_time=product.print_time,
            weight=product.weight,
            rating=product.rating,
            stock_quantity=product.stock_quantity or 0,
            reorder_point=product.reorder_point or 0,
            unit_cost=product.unit_cost,
            selling_price=product.selling_price,
        )
        db.add(db_product)
        db.commit()

        # Handle relationships after product has ID
        if product.tag_ids:
            associate_tags_with_product_by_ids(db, db_product, product.tag_ids)
        if product.material_ids:
            associate_materials_with_product_by_ids(
                db, db_product, product.material_ids
            )

        # Handle category after product creation
        if product.category_id:
            category_obj = (
                db.query(models.Category)
                .filter(models.Category.id == product.category_id)
                .first()
            )
            if category_obj:
                db_product.category = category_obj

        return {
            "sku": sku,
            "product_id": db_product.id,
            "message": "Product created successfully",
        }
    else:
        # UPDATE EXISTING PRODUCT
        product_db = (
            db.query(models.Product)
            .filter(models.Product.id == product.product_id)
            .first()
        )
        if not product_db:
            raise ValueError(f"Product with ID {product.product_id} not found")

        # Update fields if provided
        old_name = product_db.name
        if product.name is not None:
            product_db.name = product.name
        if product.description is not None:
            product_db.description = product.description
        if product.production is not None:
            product_db.production = product.production
        if product.active is not None:
            product_db.active = product.active
        if product.color is not None:
            product_db.color = product.color
        if product.print_time is not None:
            product_db.print_time = product.print_time
        if product.weight is not None:
            product_db.weight = product.weight
        if product.stock_quantity is not None:
            product_db.stock_quantity = product.stock_quantity
        if product.reorder_point is not None:
            product_db.reorder_point = product.reorder_point
        if product.unit_cost is not None:
            product_db.unit_cost = product.unit_cost
        if product.selling_price is not None:
            product_db.selling_price = product.selling_price
        if product.rating is not None:
            product_db.rating = product.rating

        # Rename folder if name changed
        if product.name is not None and old_name != product.name:
            # Convert host paths to container paths for Docker volume access
            host_products_dir = "/home/grbrum/Work/3d_print/Products"
            container_products_dir = "/Products"

            old_folder = product_db.folder_path.replace(
                host_products_dir, container_products_dir, 1
            )
            new_folder_name = f"{product_db.sku} - {product_db.name}"
            new_folder = os.path.join(container_products_dir, new_folder_name)

            os.rename(old_folder, new_folder)
            # Store the new folder path as host path in database
            product_db.folder_path = new_folder.replace(
                container_products_dir, host_products_dir, 1
            )

        # Update relationships
        if product.tag_ids is not None:
            update_product_tags_by_ids(db, product_db, product.tag_ids)
        if product.material_ids is not None:
            update_product_materials_by_ids(db, product_db, product.material_ids)
        if product.category_id is not None:
            category_obj = (
                db.query(models.Category)
                .filter(models.Category.id == product.category_id)
                .first()
            )
            if category_obj:
                product_db.category = category_obj

        db.commit()
        db.refresh(product_db)

        # Update metadata
        tag_names = [t.name for t in product_db.tags]
        material_names = [m.name for m in product_db.materials]
        category_details = None
        if product_db.category:
            category_details = {
                "name": product_db.category.name,
                "sku_initials": product_db.category.sku_initials,
                "description": product_db.category.description,
            }
        metadata = {
            "sku": product_db.sku,
            "name": product_db.name,
            "description": product_db.description,
            "category": category_details,
            "tags": tag_names,
            "materials": material_names,
            "production": product_db.production,
            "active": product_db.active,
            "color": product_db.color,
            "print_time": product_db.print_time,
            "weight": product_db.weight,
            "stock_quantity": product_db.stock_quantity,
            "reorder_point": product_db.reorder_point,
            "unit_cost": product_db.unit_cost,
            "selling_price": product_db.selling_price,
        }
        # Convert host path to container path for Docker volume access
        # Host path: /home/grbrum/Work/3d_print/Products/... -> Container path: /Products/...
        container_folder_path = product_db.folder_path.replace(
            "/home/grbrum/Work/3d_print/Products", "/Products", 1
        )
        metadata_file = os.path.join(container_folder_path, "metadata.json")
        with open(metadata_file, "w") as f:
            json.dump(metadata, f, indent=4)

        return {
            "sku": product_db.sku,
            "product_id": product_db.id,
            "message": "Product updated successfully",
        }


def get_product_by_id(db: Session, product_id: int) -> Optional[models.Product]:
    """
    Get a product by product_id from database.
    """
    return db.query(models.Product).filter(models.Product.id == product_id).first()


def get_tag_names_by_ids(db: Session, tag_ids: List[int]) -> List[str]:
    """
    Get tag names by their IDs.
    """
    if not tag_ids:
        return []

    tags = db.query(models.Tag).filter(models.Tag.id.in_(tag_ids)).all()
    return [str(tag.name) for tag in tags]


def get_material_names_by_ids(db: Session, material_ids: List[int]) -> List[str]:
    """
    Get material names by their IDs.
    """
    if not material_ids:
        return []

    materials = (
        db.query(models.Material).filter(models.Material.id.in_(material_ids)).all()
    )
    return [str(material.name) for material in materials]


def associate_tags_with_product_by_ids(
    db: Session, product_db: models.Product, tag_ids: List[int]
):
    """
    Associate tags with a product using tag IDs.
    """
    for tag_id in tag_ids:
        tag_obj = db.query(models.Tag).filter(models.Tag.id == tag_id).first()
        if tag_obj:
            product_db.tags.append(tag_obj)


def associate_materials_with_product_by_ids(
    db: Session, product_db: models.Product, material_ids: List[int]
):
    """
    Associate materials with a product using material IDs.
    """
    for material_id in material_ids:
        material_obj = (
            db.query(models.Material).filter(models.Material.id == material_id).first()
        )
        if material_obj:
            product_db.materials.append(material_obj)


def update_product_tags_by_ids(
    db: Session, product: models.Product, tag_ids: List[int]
):
    """
    Update the tags for a product by clearing existing and adding new ones using IDs.
    """
    product.tags.clear()
    associate_tags_with_product_by_ids(db, product, tag_ids)


def update_product_materials_by_ids(
    db: Session, product: models.Product, material_ids: List[int]
):
    """
    Update the materials for a product by clearing existing and adding new ones using IDs.
    """
    product.materials.clear()
    associate_materials_with_product_by_ids(db, product, material_ids)


def delete_product_by_id(db: Session, product_id: int) -> Optional[models.Product]:
    """
    Delete a product by product_id and return the product object (for folder path access).
    Returns None if product not found.
    """
    product = db.query(models.Product).filter(models.Product.id == product_id).first()
    if product:
        db.delete(product)
        db.commit()
        return product
    return None
