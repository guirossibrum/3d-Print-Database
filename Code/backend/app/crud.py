# backend/app/crud.py
from typing import Optional
from sqlalchemy.orm import Session
from sqlalchemy import func
from typing import List
from . import models, schemas
from . import tag_utils
from ensure_file_structure import (
    create_product_folder,
    update_metadata,
)  # import both functions


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


def create_product_db(
    db: Session, product: schemas.ProductCreate, sku: Optional[str] = None
) -> str:
    """
    Create a new product in database and return SKU.
    Handles product_id logic for frontend DRY architecture.
    """
    # Check if product_id is provided for create operation
    if product.product_id is not None and product.product_id != 0:
        raise ValueError("product_id must be empty for new product creation")

    if not sku:
        if not product.category_id:
            raise ValueError("Category is required for SKU generation")
        sku = generate_sku(db, product.category_id)

    # Get tag and material names for metadata (from IDs)
    tag_names = get_tag_names_by_ids(db, product.tag_ids)
    material_names = get_material_names_by_ids(db, product.material_ids)

    # 1️⃣ Create folder & metadata first
    folder_path, _ = create_product_folder(
        sku=sku,
        name=product.name,
        description=product.description or "",
        tags=tag_names,
        production=product.production,
        materials=material_names,
        category=None,  # Will be handled after product creation
    )

    # 2️⃣ Save product to DB with folder_path
    db_product = models.Product(
        sku=sku,
        name=product.name,
        description=product.description,
        folder_path=folder_path,
        production=product.production,
        category_id=product.category_id,
        color=product.color,
        print_time=product.print_time,
        weight=product.weight,
        stock_quantity=product.stock_quantity or 0,
        reorder_point=product.reorder_point or 0,
        unit_cost=product.unit_cost,
        selling_price=product.selling_price,
    )
    db.add(db_product)
    db.commit()
    db.refresh(db_product)

    # 3️⃣ Associate tags with product (using IDs)
    associate_tags_with_product_by_ids(db, db_product, product.tag_ids)

    # 4️⃣ Associate materials with product (using IDs)
    associate_materials_with_product_by_ids(db, db_product, product.material_ids)
    db.commit()

    # 5️⃣ Update metadata with category details
    category_details = None
    if product.category_id:
        category_obj = (
            db.query(models.Category)
            .filter(models.Category.id == product.category_id)
            .first()
        )
        if category_obj:
            category_details = {
                "id": category_obj.id,
                "name": category_obj.name,
                "sku_initials": category_obj.sku_initials,
                "description": category_obj.description,
            }

    update_metadata(
        sku=sku,
        category=category_details,
        materials=material_names,
        color=product.color,
        print_time=product.print_time,
        weight=product.weight,
        stock_quantity=product.stock_quantity,
        reorder_point=product.reorder_point,
        unit_cost=product.unit_cost,
        selling_price=product.selling_price,
    )

    return sku


def get_product_db(db: Session, sku: str) -> Optional[models.Product]:
    """
    Get a product by SKU from the database.
    """
    return db.query(models.Product).filter(models.Product.sku == sku).first()


def associate_tags_with_product(
    db: Session, product_db: models.Product, tags: List[str]
):
    """
    Associate normalized and validated tags with a product.
    """
    for tag_name in tags:
        if not tag_name.strip():
            continue  # Skip empty tags

        # Normalize the tag
        normalized_tag = tag_utils.normalize_tag(tag_name)

        if not normalized_tag or not tag_utils.validate_tag(normalized_tag):
            continue  # Skip invalid tags

        # Check if normalized tag already exists
        tag_obj = (
            db.query(models.Tag)
            .filter(func.lower(models.Tag.name) == normalized_tag.lower())
            .first()
        )

        if not tag_obj:
            tag_obj = models.Tag(name=normalized_tag)
            db.add(tag_obj)
            db.commit()
            db.refresh(tag_obj)

        product_db.tags.append(tag_obj)


def associate_materials_with_product(
    db: Session, product_db: models.Product, materials: List[str]
):
    """
    Associate materials with a product.
    Materials are simpler than tags - no special normalization/validation needed.
    """
    for material_name in materials:
        if not material_name.strip():
            continue  # Skip empty materials

        # Check if material already exists (case-insensitive)
        material_obj = (
            db.query(models.Material)
            .filter(func.lower(models.Material.name) == material_name.lower())
            .first()
        )

        if not material_obj:
            material_obj = models.Material(name=material_name.strip())
            db.add(material_obj)
            db.commit()
            db.refresh(material_obj)

        product_db.materials.append(material_obj)


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


def update_product_materials(
    db: Session, product: models.Product, new_materials: List[str]
):
    """
    Update the materials for a product by clearing existing and adding new ones.
    """
    product.materials.clear()
    associate_materials_with_product(db, product, new_materials)


def update_product_tags(db: Session, product: models.Product, new_tags: List[str]):
    """
    Update the tags for a product by clearing existing and adding new ones.
    """
    product.tags.clear()
    associate_tags_with_product(db, product, new_tags)


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


def update_product_db(db: Session, sku: str, update: schemas.ProductUpdate):
    """
    Update product by SKU or product_id
    Handles product_id logic for frontend DRY architecture.
    """
    # Check if product_id is provided for ID-based update
    if update.product_id is not None and update.product_id != 0:
        product = (
            db.query(models.Product)
            .filter(models.Product.id == update.product_id)
            .first()
        )
        if not product:
            return None
    else:
        # Fallback to SKU-based lookup for backward compatibility
        product = db.query(models.Product).filter(models.Product.sku == sku).first()
        if not product:
            return None

    if update.name is not None:
        product.name = update.name
    if update.description is not None:
        product.description = update.description
    if update.production is not None:
        product.production = update.production
    if update.color is not None:
        product.color = update.color
    if update.print_time is not None:
        product.print_time = update.print_time
    if update.weight is not None:
        product.weight = update.weight
    if update.stock_quantity is not None:
        product.stock_quantity = update.stock_quantity
    if update.reorder_point is not None:
        product.reorder_point = update.reorder_point
    if update.unit_cost is not None:
        product.unit_cost = update.unit_cost
    if update.selling_price is not None:
        product.selling_price = update.selling_price

    if update.tag_ids is not None:
        update_product_tags_by_ids(db, product, update.tag_ids)

    if update.material_ids is not None:
        update_product_materials_by_ids(db, product, update.material_ids)

    db.commit()
    db.refresh(product)
    return product


def update_product_inventory(db: Session, sku: str, inventory: schemas.InventoryUpdate):
    """
    Update inventory fields for a product
    """
    product = db.query(models.Product).filter(models.Product.sku == sku).first()
    if not product:
        return None

    if inventory.stock_quantity is not None:
        product.stock_quantity = inventory.stock_quantity
    if inventory.reorder_point is not None:
        product.reorder_point = inventory.reorder_point
    if inventory.unit_cost is not None:
        product.unit_cost = inventory.unit_cost
    if inventory.selling_price is not None:
        product.selling_price = inventory.selling_price

    db.commit()
    db.refresh(product)
    return product
