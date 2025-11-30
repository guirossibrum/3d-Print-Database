# index_products.py
import os, json
from sqlalchemy.orm import Session
from app import crud, schemas
from app.database import SessionLocal
from ensure_file_structure import (
    create_product_folder,
)  # import function

PRODUCTS_DIR = "/Products"


def index_products():
    db: Session = SessionLocal()
    try:
        for folder_name in os.listdir(PRODUCTS_DIR):
            folder_path = os.path.join(PRODUCTS_DIR, folder_name)
            metadata_file = os.path.join(folder_path, "metadata.json")
            if os.path.exists(metadata_file):
                with open(metadata_file) as f:
                    data = json.load(f)

                # Check if SKU already exists in DB
                product_in_db = (
                    db.query(crud.models.Product)
                    .filter(crud.models.Product.sku == data["sku"])
                    .first()
                )

                product_schema = schemas.ProductCreate(
                    name=data.get("name", ""),
                    description=data.get("description", ""),
                    tags=data.get("tags", []),
                    production=data.get("production", True),
                )

                if product_in_db:
                    # Update metadata in DB
                    crud.update_product_db(
                        db,
                        data["sku"],
                        schemas.ProductUpdate(
                            name=data.get("name"),
                            description=data.get("description"),
                            tags=data.get("tags"),
                            production=data.get("production", True),
                        ),
                    )
                else:
                    # Use folder_name as SKU to match the folder
                    sku = crud.create_product_db(db, product_schema, sku=folder_name)
                    # Ensure folder and metadata.json are correct
                    create_product_folder(
                        sku=sku,
                        name=product_schema.name,
                        description=product_schema.description or "",
                        tags=product_schema.tags,
                        production=product_schema.production,
                    )
    finally:
        db.close()
    print("Indexing complete")


if __name__ == "__main__":
    index_products()
