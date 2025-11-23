from sqlalchemy.orm import Session
from . import models


def get_or_create_tag(db: Session, name: str):
    tag = db.query(models.Tag).filter(models.Tag.name == name).first()
    if tag:
        return tag

    tag = models.Tag(name=name)
    db.add(tag)
    db.commit()
    db.refresh(tag)
    return tag


def create_product(db: Session, product_data):
    tags = product_data.tags
    product = models.Product(
        sku=product_data.sku,
        name=product_data.name,
        description=product_data.description,
        folder_path=product_data.folder_path
    )

    db.add(product)
    db.commit()
    db.refresh(product)

    # Assign tags
    for tag_name in tags:
        tag = get_or_create_tag(db, tag_name)
        link = models.ProductTag(product_id=product.id, tag_id=tag.id)
        db.add(link)

    db.commit()
    return product
