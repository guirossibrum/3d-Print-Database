# backend/app/crud.py
from sqlalchemy.orm import Session
from sqlalchemy import func
from . import models, schemas
from ensure_file_structure import create_product_folder  # import to create folders

def generate_sku(db: Session, prefix: str = "PROD") -> str:
    """
    Generate a SKU like PROD-0001
    """
    last_sku = db.query(models.Product).order_by(models.Product.id.desc()).first()
    if last_sku and last_sku.sku.startswith(prefix):
        num = int(last_sku.sku.split("-")[-1]) + 1
    else:
        num = 1
    return f"{prefix}-{num:04d}"


def create_product_db(db: Session, product: schemas.ProductCreate, sku: str = None) -> str:
    """
    Create a new product in the database and return the SKU.
    Folder is created first, then stored in DB.
    """
    if not sku:
        sku_prefix = product.name[:3].upper() if product.name else "PROD"
        sku = generate_sku(db, prefix=sku_prefix)

    # 1️⃣ Create folder & metadata first
    folder_path, _ = create_product_folder(
        sku=sku,
        name=product.name,
        description=product.description,
        tags=product.tags,
        production=product.production
    )

    # 2️⃣ Save product to DB with folder_path
    db_product = models.Product(
        sku=sku,
        name=product.name,
        description=product.description,
        folder_path=folder_path,
        production=product.production
    )
    db.add(db_product)
    db.commit()
    db.refresh(db_product)

    # 3️⃣ Create tags
    for tag_name in product.tags:
        tag_obj = db.query(models.Tag).filter(models.Tag.name == tag_name).first()
        if not tag_obj:
            tag_obj = models.Tag(name=tag_name)
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

    if update.tags is not None:
        product.tags.clear()
        for tag_name in update.tags:
            tag_obj = db.query(models.Tag).filter(models.Tag.name == tag_name).first()
            if not tag_obj:
                tag_obj = models.Tag(name=tag_name)
                db.add(tag_obj)
                db.commit()
                db.refresh(tag_obj)
            product.tags.append(tag_obj)

    db.commit()
    db.refresh(product)
    return product
