# backend/app/tag_utils.py
import re
from difflib import SequenceMatcher
from typing import List, Dict, Any
from sqlalchemy.orm import Session
from sqlalchemy import func
from . import models


def normalize_tag(tag: str) -> str:
    """
    Normalize tag: lowercase, strip whitespace, replace spaces with hyphens
    Examples: "Toy Car" -> "toy-car", "  TOY  " -> "toy"
    """
    if not tag:
        return ""
    # Convert to lowercase, strip whitespace, replace spaces/underscores with hyphens
    normalized = tag.lower().strip()
    normalized = re.sub(r"[\s_]+", "-", normalized)
    # Remove any other special characters except hyphens and alphanumeric
    normalized = re.sub(r"[^a-z0-9-]", "", normalized)
    # Remove multiple consecutive hyphens
    normalized = re.sub(r"-+", "-", normalized)
    # Remove leading/trailing hyphens
    normalized = normalized.strip("-")
    return normalized


def validate_tag(tag: str) -> bool:
    """
    Validate tag format and length
    - Must be 1-50 characters
    - Only alphanumeric, hyphens, underscores allowed
    - Cannot be empty after normalization
    """
    if not tag or len(tag) > 50:
        return False

    normalized = normalize_tag(tag)
    if not normalized:
        return False

    # Allow alphanumeric, hyphens, underscores
    return bool(re.match(r"^[a-zA-Z0-9_-]+$", normalized))


def get_similarity(a: str, b: str) -> float:
    """
    Calculate similarity ratio between two strings (case-insensitive)
    Returns float between 0.0 and 1.0
    """
    return SequenceMatcher(None, a.lower(), b.lower()).ratio()


def find_similar_tags(
    db: Session, tag_name: str, threshold: float = 0.8, limit: int = 5
) -> List[Dict[str, Any]]:
    """
    Find tags similar to input using fuzzy matching
    Returns list of dicts with 'name' and 'similarity' keys
    """
    normalized_input = normalize_tag(tag_name)

    # Get all existing tags
    all_tags = db.query(models.Tag.name).all()

    similar = []
    for tag_row in all_tags:
        existing_tag = tag_row.name
        similarity = get_similarity(normalized_input, existing_tag)
        if similarity >= threshold:
            similar.append({"name": existing_tag, "similarity": similarity})

    # Sort by similarity (highest first), then by name
    similar.sort(key=lambda x: (-x["similarity"], x["name"]))
    return similar[:limit]


def suggest_tags(db: Session, query: str, limit: int = 10) -> List[Dict[str, Any]]:
    """
    Suggest tags based on partial input with usage counts
    Returns tags that start with or contain the query, ordered by usage
    """
    if not query or len(query.strip()) < 1:
        return []

    search_term = f"%{query.strip().lower()}%"

    # Get tags with usage counts that match the search
    tags_with_counts = (
        db.query(
            models.Tag.name,
            func.count(models.product_tags.c.product_id).label("usage_count"),
        )
        .join(models.product_tags)
        .filter(func.lower(models.Tag.name).like(search_term))
        .group_by(models.Tag.name)
        .order_by(
            func.count(models.product_tags.c.product_id).desc(),  # Most used first
            models.Tag.name,  # Then alphabetical
        )
        .limit(limit)
        .all()
    )

    return [
        {"name": tag.name, "usage_count": tag.usage_count} for tag in tags_with_counts
    ]


def get_tag_stats(db: Session) -> Dict[str, int]:
    """
    Get usage statistics for all tags
    Returns dict mapping tag name to usage count
    """
    stats = (
        db.query(
            models.Tag.name, func.count(models.product_tags.c.product_id).label("count")
        )
        .join(models.product_tags)
        .group_by(models.Tag.name)
        .order_by(func.count(models.product_tags.c.product_id).desc())
        .all()
    )

    return {tag.name: tag.count for tag in stats}
