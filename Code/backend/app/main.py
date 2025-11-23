# backend/app/main.py
from fastapi import FastAPI, HTTPException, Path
from sqlalchemy.orm import Session
from . import crud, schemas
from .database import SessionLocal
from ensure_file_structure import create_product_folder, update_metadata  # <-- fixed import

app = FastAPI()

@app.post("/products/")
def create_product(product: schemas.ProductCreate):
    db: Session = SessionLocal()
    try:
        # Create product in DB and get SKU
        sku = crud.create_product_db(db, product)

        # Create folder and metadata.json
        create_product_folder(
            sku=sku,
            name=product.name,
            description=product.description,
            tags=product.tags,
            production=product.production  # <-- updated
        )
    finally:
        db.close()

    return {"sku": sku, "message": "Product created successfully"}

@app.put("/products/{sku}")
def update_product(
    sku: str = Path(...),
    update: schemas.ProductUpdate = None
):
    db: Session = SessionLocal()
    try:
        product_db = crud.update_product_db(db, sku, update)
        if not product_db:
            raise HTTPException(status_code=404, detail="Product not found")

        # Update metadata.json
        update_metadata(
            sku=sku,
            name=update.name,
            description=update.description,
            tags=update.tags,
            production=update.production  # <-- updated
        )
    finally:
        db.close()

    return {"sku": sku, "message": "Product updated successfully"}

@app.get("/products/")
def list_products():
    db: Session = SessionLocal()
    try:
        products = db.query(crud.models.Product).all()
        result = []
        for p in products:
            result.append({
                "sku": p.sku,
                "name": p.name,
                "description": p.description,
                "production": p.production,  # <-- updated
                "tags": [t.name for t in p.tags]
            })
    finally:
        db.close()
    return result
