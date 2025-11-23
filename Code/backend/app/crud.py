# backend/app/crud.py
from sqlalchemy.orm import Session
from sqlalchemy import func
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
    db: Session, product: schemas.ProductCreate, sku: str = None
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
        description=product.description,
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
    )
    db.add(db_product)
    db.commit()
    db.refresh(db_product)

    # 3️⃣ Create tags with normalization
    for tag_name in product.tags:
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

        db_product.tags.append(tag_obj)
    db.commit()

    return sku


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

    if update.tags is not None:
        product.tags.clear()
        for tag_name in update.tags:
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

            product.tags.append(tag_obj)

    db.commit()
    db.refresh(product)
    return product
