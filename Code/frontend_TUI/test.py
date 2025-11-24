#!/usr/bin/env python3
"""
Simple test script for the TUI frontend
"""

import sys
import os

# Add the frontend_TUI directory to Python path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))


def test_imports():
    """Test that all modules can be imported"""
    try:
        from config import Config
        from app import App, Tab
        from services.api_service import APIService
        import handlers
        import ui

        print("✓ All imports successful")
        return True
    except ImportError as e:
        print(f"✗ Import failed: {e}")
        return False


def test_config():
    """Test configuration loading"""
    try:
        from config import Config

        config = Config()

        assert config.api_base_url == "http://localhost:8000"
        assert "products" in config.api_urls
        assert "search" in config.api_urls

        print("✓ Config test passed")
        return True
    except Exception as e:
        print(f"✗ Config test failed: {e}")
        return False


def test_app_initialization():
    """Test App initialization (without API calls)"""
    try:
        from config import Config
        from app import App, Tab

        config = Config()

        # Mock the API service to avoid network calls during testing
        class MockAPIService:
            def __init__(self, config=None):
                pass

            def get_categories(self):
                return []

            def get_tags(self):
                return []

            def search_products(self, q=""):
                return []

        # Temporarily replace the API service
        import app

        original_api_init = app.APIService
        app.APIService = MockAPIService

        try:
            app_instance = App(config)
            assert app_instance.running == True
            assert app_instance.current_tab == Tab.SEARCH
            print("✓ App initialization test passed")
            return True
        except Exception as e:
            print(f"Debug: Exception during App init: {e}")
            import traceback

            traceback.print_exc()
            raise
        finally:
            # Restore original API service
            app.APIService = original_api_init

    except Exception as e:
        print(f"✗ App initialization test failed: {e}")
        return False


def main():
    """Run all tests"""
    print("Testing 3D Print Database TUI...")
    print()

    tests = [
        test_imports,
        test_config,
        test_app_initialization,
    ]

    passed = 0
    total = len(tests)

    for test in tests:
        if test():
            passed += 1
        print()

    print(f"Results: {passed}/{total} tests passed")

    if passed == total:
        print("🎉 All tests passed! The TUI frontend is ready.")
        return 0
    else:
        print("❌ Some tests failed. Please check the implementation.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
