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


def handle_product_creation_side_effects(sku: str, product: schemas.ProductCreate):
    """
    Handle side effects after product creation: folder and metadata.
    """
    # Create folder and metadata.json
    create_product_folder(
        sku=sku,
        name=product.name,
        description=product.description or "",
        tags=product.tags,
        production=product.production,
    )

    # Update metadata with additional fields
    update_metadata(
        sku=sku,
        material=product.materials[0] if product.materials else None,
        color=product.color,
        print_time=product.print_time,
        weight=product.weight,
        stock_quantity=product.stock_quantity,
        reorder_point=product.reorder_point,
        unit_cost=product.unit_cost,
        selling_price=product.selling_price,
    )


def handle_product_update_side_effects(sku: str, update: schemas.ProductUpdate):
    """
    Handle side effects after product update: metadata.
    """
    update_metadata(
        sku=sku,
        name=update.name,
        description=update.description,
        tags=update.tags,
        production=update.production,
        material=update.materials[0] if update.materials else None,
        color=update.color,
        print_time=update.print_time,
        weight=update.weight,
        stock_quantity=update.stock_quantity,
        reorder_point=update.reorder_point,
        unit_cost=update.unit_cost,
        selling_price=update.selling_price,
    )


def handle_inventory_update_side_effects(sku: str, inventory: schemas.InventoryUpdate):
    """
    Handle side effects after inventory update: metadata.
    """
    update_metadata(
        sku=sku,
        stock_quantity=inventory.stock_quantity,
        reorder_point=inventory.reorder_point,
        unit_cost=inventory.unit_cost,
        selling_price=inventory.selling_price,
    )


@app.post("/products/")
def create_product(product: schemas.ProductCreate):
    db: Session = SessionLocal()
    try:
        # Create product in DB and get SKU
        sku = crud.create_product_db(db, product)

        # Handle side effects: folder and metadata
        handle_product_creation_side_effects(sku, product)
    finally:
        db.close()

    return {"sku": sku, "message": "Product created successfully"}


@app.put("/products/{sku}")
def update_product(update: schemas.ProductUpdate, sku: str = Path(...)):
    db: Session = SessionLocal()
    try:
        product_db = crud.update_product_db(db, sku, update)
        if not product_db:
            raise HTTPException(status_code=404, detail="Product not found")

        # Handle side effects: metadata update
        handle_product_update_side_effects(sku, update)
    finally:
        db.close()

    return {"sku": sku, "message": "Product updated successfully"}

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
        # Get all tags from the tags table, even if not used
        all_tags = db.query(models.Tag).all()
        tag_names = [tag.name for tag in all_tags]

        # Get usage stats for used tags
        stats = tag_utils.get_tag_stats(db)

        # Return all tags with their usage counts (0 if not used)
        return [
            {"name": tag_name, "usage_count": stats.get(tag_name, 0)}
            for tag_name in sorted(tag_names)
        ]
    finally:
        db.close()


@app.post("/tags")
def create_tag(tag: schemas.TagCreate):
    """
    Create a new tag
    """
    db: Session = SessionLocal()
    try:
        # Check if tag exists
        existing = db.query(models.Tag).filter(models.Tag.name == tag.name).first()
        if existing:
            raise HTTPException(status_code=400, detail="Tag already exists")

        db_tag = models.Tag(name=tag.name)
        db.add(db_tag)
        db.commit()
        db.refresh(db_tag)

        return {
            "name": db_tag.name,
            "usage_count": 0,
        }
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=str(e))
    finally:
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
                raise HTTPException(status_code=400, detail="Tag name already exists")

            tag.name = tag_update.name

        db.commit()
        db.refresh(tag)

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

        # Delete the tag
        db.delete(tag)
        db.commit()

        return {"message": "Tag deleted successfully"}
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        db.close()


@app.get("/materials")
def list_all_materials():
    """
    Get all unique materials with their usage counts
    Returns: [{"name": "material_name", "usage_count": 5}, ...]
    """
    db: Session = SessionLocal()
    try:
        # Get all materials from materials table, even if not used
        all_materials = db.query(models.Material).all()
        material_names = [material.name for material in all_materials]

        # Get usage stats for used materials
        material_usage = {}
        products = db.query(models.Product).all()
        for product in products:
            if product.materials:
                for material in product.materials:
                    material_usage[material.name] = (
                        material_usage.get(material.name, 0) + 1
                    )

        # Return all materials with their usage counts (0 if not used)
        return [
            {"name": material_name, "usage_count": material_usage.get(material_name, 0)}
            for material_name in sorted(material_names)
        ]
    finally:
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
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        db.close()


@app.put("/materials/{material_name}")
def update_material(material_name: str, material_update: schemas.MaterialUpdate):
    """
    Update an existing material
    """
    db: Session = SessionLocal()
    try:
        material = (
            db.query(models.Material)
            .filter(models.Material.name == material_name)
            .first()
        )
        if not material:
            raise HTTPException(status_code=404, detail="Material not found")

        if material_update.name:
            # Check if new name exists
            existing = (
                db.query(models.Material)
                .filter(models.Material.name == material_update.name)
                .first()
            )
            if existing:
                raise HTTPException(
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
            db.query(models.Material)
            .filter(models.Material.name == material_name)
            .first()
        )
        if not material:
            raise HTTPException(status_code=404, detail="Material not found")

        # Check if material is used by any products
        usage_count = len(material.products)
        if usage_count > 0:
            raise HTTPException(
                status_code=400,
                detail=f"Material is used by {usage_count} product(s) and cannot be deleted",
            )

        # Delete material
        db.delete(material)
        db.commit()

        return {"message": "Material deleted successfully"}
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(status_code=500, detail=str(e))
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

        return {
            "id": db_category.id,
            "name": db_category.name,
            "sku_initials": db_category.sku_initials,
            "description": db_category.description,
        }
    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(
            status_code=500, detail=f"Error creating category: {str(e)}"
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
                        status_code=400, detail="Category name already exists"
                    )
                else:
                    raise HTTPException(
                        status_code=400, detail="SKU initials already exist"
                    )

        # Update fields
        if category_update.name is not None:
            existing_category.name = category_update.name
        if category_update.sku_initials is not None:
            existing_category.sku_initials = category_update.sku_initials
        if category_update.description is not None:
            existing_category.description = category_update.description

        db.commit()
        db.refresh(existing_category)

        return {
            "id": existing_category.id,
            "name": existing_category.name,
            "sku_initials": existing_category.sku_initials,
            "description": existing_category.description,
        }

    except HTTPException:
        raise
    except Exception as e:
        db.rollback()
        raise HTTPException(
            status_code=500, detail=f"Error updating category: {str(e)}"
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
                        "material": [m.name for m in p.materials],
                        "color": p.color,
                        "print_time": p.print_time,
                        "weight": p.weight,
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


@app.put("/products/{sku}/inventory")
def update_inventory(inventory: schemas.InventoryUpdate, sku: str = Path(...)):
    """
    Update inventory fields for a specific product
    """
    if inventory is None:
        raise HTTPException(status_code=400, detail="Inventory update data required")

    db: Session = SessionLocal()
    try:
        product = crud.update_product_inventory(db, sku, inventory)
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")

        # Handle side effects: metadata update
        handle_inventory_update_side_effects(sku, inventory)

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

        return result
    finally:
        db.close()


# Register the products GET route at the end
def list_products():
    db: Session = SessionLocal()
    try:
        products = (
            db.query(crud.models.Product)
            .options(
                joinedload(crud.models.Product.tags),
                joinedload(crud.models.Product.materials),
            )
            .all()
        )
        result = []
        for p in products:
            materials_list = [m.name for m in p.materials]
            result.append(
                {
                    "id": p.id,
                    "sku": p.sku,
                    "name": p.name,
                    "description": p.description,
                    "production": p.production,
                    "tags": [t.name for t in p.tags],
                    "category_id": p.category_id,
                    "material": materials_list,
                    "color": p.color,
                    "print_time": p.print_time,
                    "weight": p.weight,
                    "stock_quantity": p.stock_quantity,
                    "reorder_point": p.reorder_point,
                    "unit_cost": p.unit_cost,
                    "selling_price": p.selling_price,
                }
            )
        return result
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        db.close()


app.add_api_route("/products/", list_products, methods=["GET"])
