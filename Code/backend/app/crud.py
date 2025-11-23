# backend/app/crud.py
from typing import Optional
from sqlalchemy.orm import Session
from sqlalchemy import func
from typing import List
from . import models, schemas
from . import tag_utils
from ensure_file_structure import create_product_folder  # import to create folders


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
    Create a new product in the database and return the SKU.
    Folder is created first, then stored in DB.
    """
    if not sku:
        if not product.category_id:
            raise ValueError("Category is required for SKU generation")
        sku = generate_sku(db, product.category_id)

    # 1️⃣ Create folder & metadata first
    folder_path, _ = create_product_folder(
        sku=sku,
        name=product.name,
        description=product.description or "",
        tags=product.tags,
        production=product.production,
    )

    # 2️⃣ Save product to DB with folder_path
    db_product = models.Product(
        sku=sku,
        name=product.name,
        description=product.description,
        folder_path=folder_path,
        production=product.production,
        category_id=product.category_id,
        material=product.material,
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

    # 3️⃣ Associate tags with product
    associate_tags_with_product(db, db_product, product.tags)
    db.commit()

    return sku


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


def update_product_tags(db: Session, product: models.Product, new_tags: List[str]):
    """
    Update the tags for a product by clearing existing and adding new ones.
    """
    product.tags.clear()
    associate_tags_with_product(db, product, new_tags)


def update_product_db(db: Session, sku: str, update: schemas.ProductUpdate):
    """
    Update product by SKU
    """
    product = db.query(models.Product).filter(models.Product.sku == sku).first()
    if not product:
        return None

    if update.name is not None:
        product.name = update.name
    if update.description is not None:
        product.description = update.description
    if update.production is not None:
        product.production = update.production
    if update.material is not None:
        product.material = update.material
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

    if update.tags is not None:
        update_product_tags(db, product, update.tags)

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
