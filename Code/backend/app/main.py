# backend/app/main.py
from fastapi import FastAPI, HTTPException, Path, Query
from sqlalchemy.orm import Session
from typing import List, Dict, Any
from . import crud, schemas
from .database import SessionLocal
from . import tag_utils
from ensure_file_structure import (
    create_product_folder,
    update_metadata,
)  # <-- fixed import

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
            production=product.production,  # <-- updated
        )
    finally:
        db.close()

    return {"sku": sku, "message": "Product created successfully"}


@app.put("/products/{sku}")
def update_product(sku: str = Path(...), update: schemas.ProductUpdate = None):
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
            production=update.production,  # <-- updated
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
            result.append(
                {
                    "sku": p.sku,
                    "name": p.name,
                    "description": p.description,
                    "production": p.production,  # <-- updated
                    "tags": [t.name for t in p.tags],
                }
            )
    finally:
        db.close()
    return result


@app.get("/tags/suggest")
def suggest_tags(q: str = Query(..., min_length=1, max_length=50)):
    """
    Suggest existing tags based on partial input
    Query params: q=partial_tag_name
    Returns: [{"name": "tag_name", "usage_count": 5}, ...]
    """
    db: Session = SessionLocal()
    try:
        return tag_utils.suggest_tags(db, q, limit=10)
    finally:
        db.close()


@app.get("/tags/stats")
def get_tag_stats():
    """
    Get usage statistics for all tags
    Returns: {"tag_name": count, ...}
    """
    db: Session = SessionLocal()
    try:
        return tag_utils.get_tag_stats(db)
    finally:
        db.close()


@app.get("/tags")
def list_all_tags():
    """
    Get all unique tags with their usage counts
    Returns: [{"name": "tag_name", "usage_count": 5}, ...]
    """
    db: Session = SessionLocal()
    try:
        stats = tag_utils.get_tag_stats(db)
        return [
            {"name": tag_name, "usage_count": count}
            for tag_name, count in stats.items()
        ]
    finally:
        db.close()


@app.get("/products/search")
def search_products(
    name: str = Query(None, description="Search by product name (partial match)"),
    sku: str = Query(None, description="Search by SKU (exact or partial match)"),
    tags: str = Query(None, description="Search by tag names (comma-separated)"),
    production: bool = Query(None, description="Filter by production status"),
):
    """
    Search products by name, SKU, tags, or production status
    Tags can be comma-separated for multiple tag search
    Results ordered by tag match count (most matches first), then by SKU
    Returns list of matching products with match_count for multi-tag searches
    """
    db: Session = SessionLocal()
    try:
        # Parse tags if provided
        search_tags = []
        if tags:
            search_tags = [tag.strip() for tag in tags.split(",") if tag.strip()]

        # Build base query
        query = db.query(crud.models.Product)

        # Apply filters
        if name:
            query = query.filter(crud.models.Product.name.ilike(f"%{name}%"))
        if sku:
            query = query.filter(crud.models.Product.sku.ilike(f"%{sku}%"))

        # Handle tag filtering
        if search_tags:
            # For multiple tags, we want products that have ANY of the tags
            # We'll filter and then sort by match count
            query = (
                query.join(crud.models.Product.tags)
                .filter(crud.models.Tag.name.in_(search_tags))
                .distinct()
            )

        if production is not None:
            query = query.filter(crud.models.Product.production == production)

        # Get results
        products = query.all()

        # Process results and add match count for multi-tag searches
        result = []
        for p in products:
            product_tags = [t.name for t in p.tags]
            match_count = 0
            if search_tags:
                match_count = len(set(product_tags) & set(search_tags))

            result.append(
                {
                    "id": p.id,
                    "sku": p.sku,
                    "name": p.name,
                    "description": p.description,
                    "production": p.production,
                    "tags": product_tags,
                    "match_count": match_count,
                }
            )

        # Sort results: most tag matches first, then by SKU alphabetically
        if search_tags:
            result.sort(key=lambda x: (-x["match_count"], x["sku"]))
        else:
            result.sort(key=lambda x: x["sku"])

        return result
    finally:
        db.close()
