# backend/app/main.py
from fastapi import FastAPI, HTTPException, Path, Query
from sqlalchemy.orm import Session
from sqlalchemy.exc import OperationalError
from typing import List, Dict, Any
from . import crud, schemas
from .database import SessionLocal, create_tables
from . import tag_utils
from ensure_file_structure import (
    create_product_folder,
    update_metadata,
)  # <-- fixed import

app = FastAPI()

# Try to create tables on startup, but don't fail if DB isn't ready
try:
    create_tables()
except OperationalError:
    # Database might not be ready yet, will retry on first request
    pass


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


@app.get("/categories")
def list_categories():
    """
    Get all categories
    Returns: [{"id": 1, "name": "Guitars", "sku_initials": "GUI", "description": "..."}, ...]
    """
    db: Session = SessionLocal()
    try:
        categories = (
            db.query(crud.models.Category).order_by(crud.models.Category.name).all()
        )
        return [
            {
                "id": c.id,
                "name": c.name,
                "sku_initials": c.sku_initials,
                "description": c.description,
            }
            for c in categories
        ]
    finally:
        db.close()


@app.post("/categories")
def create_category(category: schemas.CategoryCreate):
    """
    Create a new category
    """
    db: Session = SessionLocal()
    try:
        # Validate SKU initials (must be 3 uppercase letters)
        if (
            len(category.sku_initials) != 3
            or not category.sku_initials.isalpha()
            or not category.sku_initials.isupper()
        ):
            raise HTTPException(
                status_code=400,
                detail="SKU initials must be exactly 3 uppercase letters",
            )

        # Check for duplicates
        existing = (
            db.query(crud.models.Category)
            .filter(
                (crud.models.Category.name == category.name)
                | (crud.models.Category.sku_initials == category.sku_initials)
            )
            .first()
        )

        if existing:
            if existing.name == category.name:
                raise HTTPException(
                    status_code=400, detail="Category name already exists"
                )
            else:
                raise HTTPException(
                    status_code=400, detail="SKU initials already exist"
                )

        db_category = crud.models.Category(
            name=category.name,
            sku_initials=category.sku_initials,
            description=category.description,
        )
        db.add(db_category)
        db.commit()
        db.refresh(db_category)

        return {"id": db_category.id, "message": "Category created successfully"}
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(
            status_code=500, detail=f"Error creating category: {str(e)}"
        )
    finally:
        db.close()


@app.delete("/categories/{category_id}")
def delete_category(category_id: int):
    """
    Delete a category (only if no products are using it)
    """
    db: Session = SessionLocal()
    try:
        category = (
            db.query(crud.models.Category)
            .filter(crud.models.Category.id == category_id)
            .first()
        )
        if not category:
            raise HTTPException(status_code=404, detail="Category not found")

        # Check if any products use this category
        product_count = (
            db.query(crud.models.Product)
            .filter(crud.models.Product.category_id == category_id)
            .count()
        )
        if product_count > 0:
            raise HTTPException(
                status_code=400,
                detail=f"Cannot delete category: {product_count} products are using it",
            )

        db.delete(category)
        db.commit()

        return {"message": "Category deleted successfully"}
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(
            status_code=500, detail=f"Error deleting category: {str(e)}"
        )
    finally:
        db.close()


@app.delete("/products/{sku}")
def delete_product(
    sku: str = Path(..., description="SKU of the product to delete"),
    delete_files: bool = Query(False, description="Also delete files from filesystem"),
):
    """
    Delete a product by SKU
    Options: Delete from database only, or database + filesystem
    """
    db: Session = SessionLocal()
    try:
        # Find the product
        product = (
            db.query(crud.models.Product).filter(crud.models.Product.sku == sku).first()
        )
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")

        # Store folder path for file deletion
        folder_path = product.folder_path

        # Delete from database
        db.delete(product)
        db.commit()

        # Delete files if requested
        if delete_files and folder_path:
            try:
                import shutil
                import os

                if os.path.exists(folder_path):
                    shutil.rmtree(folder_path)
            except Exception as e:
                # Log error but don't fail the request
                print(f"Warning: Could not delete folder {folder_path}: {e}")

        return {
            "message": f"Product {sku} deleted successfully",
            "files_deleted": delete_files,
        }

    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=f"Error deleting product: {str(e)}")
    finally:
        db.close()


@app.get("/products/search")
def search_products(
    q: str = Query(
        "",
        description="Unified search query (searches name, SKU, and tags). Leave empty to show all products.",
    ),
    production: bool = Query(None, description="Filter by production status"),
):
    """
    Unified search across name, SKU, and tags
    If query is empty, returns all products ordered by SKU
    Returns products with match details showing where matches occurred
    Results ordered by total matches (descending), then by SKU (ascending)
    """
    db: Session = SessionLocal()
    try:
        search_term = q.strip() if q else ""

        # If no search term, return all products ordered by SKU
        if not search_term:
            all_products = (
                db.query(crud.models.Product).order_by(crud.models.Product.sku).all()
            )

            results = []
            for p in all_products:
                # Apply production filter if specified
                if production is not None and p.production != production:
                    continue

                results.append(
                    {
                        "id": p.id,
                        "sku": p.sku,
                        "name": p.name,
                        "description": p.description,
                        "production": p.production,
                        "tags": [t.name for t in p.tags],
                        "matches": {"total": 0, "name": 0, "sku": 0, "tags": 0},
                    }
                )
            return results

        # Perform search with query terms
        search_terms = [term.strip() for term in search_term.split() if term.strip()]

        # Get all products first, then filter in Python for complex matching
        all_products = db.query(crud.models.Product).all()

        results = []
        for p in all_products:
            product_tags = [t.name for t in p.tags]

            # Count matches in different fields
            name_matches = sum(
                1 for term in search_terms if term.lower() in p.name.lower()
            )
            sku_matches = sum(
                1 for term in search_terms if term.lower() in p.sku.lower()
            )
            tag_matches = sum(
                1
                for term in search_terms
                for tag in product_tags
                if term.lower() in tag.lower()
            )

            total_matches = name_matches + sku_matches + tag_matches

            # Only include if there's at least one match
            if total_matches > 0:
                # Apply production filter if specified
                if production is not None and p.production != production:
                    continue

                results.append(
                    {
                        "id": p.id,
                        "sku": p.sku,
                        "name": p.name,
                        "description": p.description,
                        "production": p.production,
                        "tags": product_tags,
                        "matches": {
                            "total": total_matches,
                            "name": name_matches,
                            "sku": sku_matches,
                            "tags": tag_matches,
                        },
                    }
                )

        # Sort by total matches (descending), then by SKU (ascending)
        results.sort(key=lambda x: (-x["matches"]["total"], x["sku"]))

        return results
    finally:
        db.close()
