"""
API service for communicating with the 3D Print Database backend
"""

import requests
import json
from typing import List, Dict, Any, Optional
from config import Config


class APIService:
    """Service for API communication"""

    def __init__(self, config: Config):
        self.config = config
        self.session = requests.Session()
        # Set a reasonable timeout
        self.timeout = 10

    def _get(
        self, url: str, params: Optional[Dict[str, Any]] = None
    ) -> List[Dict[str, Any]]:
        """Make a GET request"""
        try:
            response = self.session.get(url, params=params, timeout=self.timeout)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise Exception(f"API request failed: {e}")

    def _post(self, url: str, data: Dict[str, Any]) -> Dict[str, Any]:
        """Make a POST request"""
        try:
            response = self.session.post(url, json=data, timeout=self.timeout)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise Exception(f"API request failed: {e}")

    def _put(self, url: str, data: Dict[str, Any]) -> Dict[str, Any]:
        """Make a PUT request"""
        try:
            response = self.session.put(url, json=data, timeout=self.timeout)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise Exception(f"API request failed: {e}")

    def _delete(self, url: str) -> bool:
        """Make a DELETE request"""
        try:
            response = self.session.delete(url, timeout=self.timeout)
            response.raise_for_status()
            return True
        except requests.exceptions.RequestException as e:
            raise Exception(f"API request failed: {e}")

    def get_products(self) -> List[Dict[str, Any]]:
        """Get all products"""
        return self._get(self.config.api_urls["products"])

    def search_products(self, query: str = "") -> List[Dict[str, Any]]:
        """Search products with optional query"""
        params = {"q": query} if query else {}
        return self._get(self.config.api_urls["search"], params)

    def create_product(self, product_data: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Create a new product"""
        return self._post(self.config.api_urls["products"], product_data)

    def update_product(self, sku: str, updates: Dict[str, Any]) -> bool:
        """Update an existing product"""
        url = f"{self.config.api_urls['products']}{sku}"
        result = self._put(url, updates)
        return result is not None

    def delete_product(self, sku: str, delete_files: bool = False) -> bool:
        """Delete a product"""
        params = {"delete_files": delete_files}
        url = f"{self.config.api_urls['products']}{sku}"
        # Add query parameters for delete_files
        if delete_files:
            url += "?delete_files=true"
        return self._delete(url)

    def get_categories(self) -> List[Dict[str, Any]]:
        """Get all categories"""
        return self._get(self.config.api_urls["categories"])

    def create_category(
        self, category_data: Dict[str, Any]
    ) -> Optional[Dict[str, Any]]:
        """Create a new category"""
        return self._post(self.config.api_urls["categories"], category_data)

    def update_category(self, category_id: int, updates: Dict[str, Any]) -> bool:
        """Update an existing category"""
        url = f"{self.config.api_urls['categories']}/{category_id}"
        result = self._put(url, updates)
        return result is not None

    def delete_category(self, category_id: int) -> bool:
        """Delete a category"""
        url = f"{self.config.api_urls['categories']}/{category_id}"
        return self._delete(url)

    def get_tags(self) -> List[Dict[str, Any]]:
        """Get all tags with usage counts"""
        return self._get(self.config.api_urls["tags"])

    def suggest_tags(self, query: str, limit: int = 10) -> List[Dict[str, Any]]:
        """Get tag suggestions"""
        params = {"q": query, "limit": limit}
        return self._get(self.config.api_urls["tag_suggest"], params)

    def delete_tag(self, tag_name: str) -> bool:
        """Delete a tag"""
        url = f"{self.config.api_urls['tags']}/{tag_name}"
        return self._delete(url)

    def get_inventory_status(self) -> List[Dict[str, Any]]:
        """Get inventory status for all products"""
        return self._get(self.config.api_urls["inventory"])

    def update_inventory(self, sku: str, updates: Dict[str, Any]) -> bool:
        """Update inventory for a product"""
        url = f"{self.config.api_urls['products']}{sku}/inventory"
        result = self._put(url, updates)
        return result is not None
