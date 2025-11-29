# backend/app/main.py
from fastapi import FastAPI, HTTPException, Path, Query
from sqlalchemy.orm import Session, joinedload
from sqlalchemy.exc import OperationalError
from typing import List, Dict, Any
from . import crud, schemas, models
from .database import SessionLocal, create_tables
from . import tag_utils
from ensure_file_structure import (
    create_product_folder,
    update_metadata,
)  # <-- fixed import

app = FastAPI()

# Create tables on startup - this will retry if DB isn't ready
create_tables()


    category_details = None
                "name": category_obj.name,
                "sku_initials": category_obj.sku_initials,
                "description": category_obj.description,
            }

    # Create folder and metadata.json
    create_product_folder(
        sku=crud.create_product_db(db, product),
        name=product.name,
        description=product.description or "",
        tags=tag_names,
        production=product.production,
        materials=material_names,
        category=category_details,  # Pass category details
    )

    # Update metadata with additional fields (using materials array)
    update_metadata(
        sku=result["sku"],
    Handle side effects after product update: metadata.
    """
    # Get tag and material names for metadata (from IDs) if provided
    tag_names = None
    material_names = None
        sku=result["sku"],
        name=update.name,
        description=update.description,
        tags=tag_names,
        production=update.production,
        stock_quantity=inventory.stock_quantity,
        reorder_point=inventory.reorder_point,
        unit_cost=inventory.unit_cost,
        selling_price=inventory.selling_price,
    )


@app.post("/products/")
def save_product(product: schemas.ProductBase):
    """
    Unified product save function - replaces separate create/update endpoints.
    Handles both create and update based on product_id.

    FRONTEND: Use this unified endpoint for both create and update operations.
    DEPRECATED: Old separate create/update endpoints removed in v2.0
    """
    db: Session = SessionLocal()
    try:
                result["product_id"],
                result["sku"],
                schemas.ProductUpdate(**product.dict(exclude_unset=True)),
                db,
            )
    finally:
        db.close()

    return result


@app.get("/products/{product_id}")
def get_product(product_id: int = Path(...)):
    """
    Get a product by product_id
    """
    # type: ignore  # SQLAlchemy typing issues
    db: Session = SessionLocal()
    try:
        product_db = crud.get_product_by_id(db, product_id)
        if product_db is None:
            raise HTTPException(status_code=404, detail="Product not found")

        # Create Product schema with nested response objects
        product = schemas.Product(
            name=product_db.name,
            description=product_db.description,
            production=product_db.production,
            category_id=product_db.category.id if product_db.category else None,
            color=product_db.color,
            print_time=product_db.print_time,
            weight=product_db.weight,
            stock_quantity=product_db.stock_quantity,
            reorder_point=product_db.reorder_point,
            unit_cost=product_db.unit_cost,
            selling_price=product_db.selling_price,
            product_id=product_db.id,  # Required field in response schema
            id=product_db.id,
            sku=product_db.sku,
            folder_path=product_db.folder_path,
            # Nested response objects with both ID and name
            tags=[
                schemas.TagResponse(id=tag.id, name=tag.name) for tag in product_db.tags
            ],
            materials=[
                schemas.MaterialResponse(id=material.id, name=material.name)
                for material in product_db.materials
            ],
            category=schemas.CategoryResponse(
                id=product_db.category.id,
                name=product_db.category.name,
                sku_initials=product_db.category.sku_initials,
                description=product_db.category.description,
            )
            if product_db.category
            else None,
        )
        return product
    finally:
        db.close()


# DEPRECATED: Replaced by unified /products/ endpoint
# Old separate create/update endpoints removed in v2.0
# FRONTEND: Use unified save_product() function instead

# Temporarily removed decorator to debug


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

        db.close()


@app.put("/tags/{tag_name}")
def update_tag(tag_name: str, tag_update: schemas.TagUpdate):
    """
    Update an existing tag
    """
    db: Session = SessionLocal()
    try:
        tag = db.query(models.Tag).filter(models.Tag.name == tag_name).first()
        if not tag:
            raise HTTPException(status_code=404, detail="Tag not found")

        if tag_update.name:
            # Check if new name exists
            existing = (
                db.query(models.Tag).filter(models.Tag.name == tag_update.name).first()
            )
            if existing:

        return {
            "name": tag.name,
            "usage_count": len(tag.products),
        }
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        db.close()


@app.delete("/tags/{tag_name}")
def delete_tag(tag_name: str):
    """
    Delete a tag if it's not used by any products
    """
    db: Session = SessionLocal()
    try:
        # Find the tag
        tag = db.query(models.Tag).filter(models.Tag.name == tag_name).first()
        if not tag:
            raise HTTPException(status_code=404, detail="Tag not found")

        # Check if tag is used by any products
        usage_count = len(tag.products)
        if usage_count > 0:
            raise HTTPException(
                status_code=400,
                detail=f"Tag is used by {usage_count} product(s) and cannot be deleted",
            )

        db.close()


@app.post("/materials")
def create_material(material: schemas.MaterialCreate):
    """
    Create a new material
    """
    db: Session = SessionLocal()
    try:
        # Check if material exists
        existing = (
            db.query(models.Material)
            .filter(models.Material.name == material.name)
            .first()
        )
        if existing:
            raise HTTPException(status_code=400, detail="Material already exists")

        db_material = models.Material(name=material.name)
        db.add(db_material)
        db.commit()
        db.refresh(db_material)

        return {
            "name": db_material.name,
            "usage_count": 0,
        }
    except HTTPException:
        raise
                    status_code=400, detail="Material name already exists"
                )

            material.name = material_update.name

        db.commit()
        db.refresh(material)

        return {
            "name": material.name,
            "usage_count": len(material.products),
        }
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        db.close()


@app.delete("/materials/{material_name}")
def delete_material(material_name: str):
    """
    Delete a material if it's not used by any products
    """
    db: Session = SessionLocal()
    try:
        # Find material
        material = (
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
        )
    finally:
        db.close()


@app.put("/categories/{category_id}")
def update_category(category_id: int, category_update: schemas.CategoryUpdate):
    """
    Update an existing category
    """
    db: Session = SessionLocal()
    try:
        # Check if category exists
        existing_category = (
            db.query(crud.models.Category)
            .filter(crud.models.Category.id == category_id)
            .first()
        )
        if not existing_category:
            raise HTTPException(status_code=404, detail="Category not found")

        # Validate sku_initials if provided
        if category_update.sku_initials is not None:
            if (
                len(category_update.sku_initials) != 3
                or not category_update.sku_initials.isalpha()
                or not category_update.sku_initials.isupper()
            ):
                raise HTTPException(
                    status_code=400,
                    detail="SKU initials must be exactly 3 uppercase letters",
                )

        # Check for conflicts with other categories
        if category_update.name or category_update.sku_initials:
            conflict_query = db.query(crud.models.Category).filter(
                crud.models.Category.id != category_id
            )
            if category_update.name:
                conflict_query = conflict_query.filter(
                    crud.models.Category.name == category_update.name
                )
            if category_update.sku_initials:
                conflict_query = conflict_query.filter(
                    crud.models.Category.sku_initials == category_update.sku_initials
                )
            existing = conflict_query.first()
            if existing:
                if existing.name == category_update.name:
                    raise HTTPException(
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


@app.delete("/products/{product_id}")
def delete_product(
    product_id: int = Path(..., description="Product ID to delete"),
    delete_files: bool = Query(False, description="Also delete files from filesystem"),
):
    """
    Delete a product by product_id
    Options: Delete from database only, or database + filesystem
    """
    db: Session = SessionLocal()
    try:
        # Find product
        product = crud.delete_product_by_id(db, product_id)
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")

        # Store folder path for file deletion
        folder_path = product.folder_path

        # Delete files if requested
        if delete_files and folder_path:
            try:
                import shutil

                product_tags = [t.name for t in p.tags]
                product_materials = [m.name for m in p.materials]
                results.append(
                    {
                        "id": p.id,
                        "sku": p.sku,
                        "name": p.name,
                        "description": p.description,
                        "production": p.production,
                        "tags": product_tags,
                        "material": product_materials,
                        "color": p.color,
                        "print_time": p.print_time,
                        "weight": p.weight,
                        "stock_quantity": p.stock_quantity,
                        "reorder_point": p.reorder_point,
                        "unit_cost": p.unit_cost,
                        "selling_price": p.selling_price,
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


        return {"sku": sku, "message": "Inventory updated successfully"}
    finally:
        db.close()


@app.get("/inventory/status")
def get_inventory_status():
    """
    Get inventory status for all products
    Returns products with calculated inventory status (in stock, low stock, out of stock)
    """
    db: Session = SessionLocal()
    try:
        products = db.query(crud.models.Product).all()
        result = []

        for p in products:
            # Calculate inventory status
            if p.stock_quantity == 0:
                status = "out_of_stock"
            elif p.stock_quantity <= p.reorder_point:
                status = "low_stock"
            else:
                status = "in_stock"

            # Calculate total value (stock_quantity * unit_cost in cents)
            total_value = None
            if p.stock_quantity and p.unit_cost:
                total_value = p.stock_quantity * p.unit_cost

            # Calculate profit margin if both costs are available
            profit_margin = None
            if p.unit_cost and p.selling_price and p.unit_cost > 0:
                profit_margin = ((p.selling_price - p.unit_cost) / p.unit_cost) * 100

            result.append(
                {
                    "sku": p.sku,
                    "name": p.name,
                    "stock_quantity": p.stock_quantity,
                    "reorder_point": p.reorder_point,
                    "unit_cost": p.unit_cost,
                    "selling_price": p.selling_price,
                    "total_value": total_value,
                    "profit_margin": profit_margin,
                    "status": status,
                    "production": p.production,
                }
            )
