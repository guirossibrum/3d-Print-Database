#!/usr/bin/env python3
"""
Database reset script for 3D Print Database
Drops and recreates all tables with updated schema
WARNING: This will delete all existing data!
"""

import sys
import os

sys.path.append(os.path.dirname(__file__))

from sqlalchemy import create_engine, text
from app.database import Base, engine
from app import models


def reset_database(force=False):
    """Reset the database by dropping and recreating all tables"""

    if not force:
        print("⚠️  WARNING: This will delete ALL existing data!")
        try:
            confirm = input(
                "Are you sure you want to continue? (type 'YES' to confirm): "
            )
            if confirm != "YES":
                print("Database reset cancelled.")
                return False
        except EOFError:
            # Running in non-interactive environment, skip confirmation
            print("Running in non-interactive mode, proceeding with database reset...")
            pass

    try:
        print("🔄 Dropping existing tables...")
        # Drop all tables
        Base.metadata.drop_all(bind=engine)

        print("📝 Creating new tables with updated schema...")
        # Create all tables with new schema
        Base.metadata.create_all(bind=engine)

        print("✅ Database reset complete!")
        print("📋 New tables created:")
        print("   - products (with inventory fields)")
        print("   - tags")
        print("   - categories")
        print("   - product_tags (association table)")

    except Exception as e:
        print(f"❌ Error resetting database: {e}")
        return False

    return True


if __name__ == "__main__":
    # Check if running in interactive mode
    import sys

    force = "--force" in sys.argv or len(sys.argv) > 1

    success = reset_database(force=force)
    if success:
        print("\n🎉 Database is ready with the new inventory schema!")
        print("You can now restart the backend and use the inventory features.")
    else:
        print("\n💥 Database reset failed. Check the error messages above.")
