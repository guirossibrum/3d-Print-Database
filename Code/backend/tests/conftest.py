# tests/conftest.py
import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from app.database import Base
from app import models


# Test database URL (SQLite in-memory)
TEST_DATABASE_URL = "sqlite:///./test.db"


@pytest.fixture(scope="session")
def engine():
    """Create test database engine"""
    engine = create_engine(TEST_DATABASE_URL, connect_args={"check_same_thread": False})
    Base.metadata.create_all(bind=engine)
    yield engine
    Base.metadata.drop_all(bind=engine)


@pytest.fixture(scope="function")
def db_session(engine):
    """Create test database session"""
    TestingSessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
    db = TestingSessionLocal()
    try:
        yield db
    finally:
        db.rollback()
        db.close()


@pytest.fixture
def sample_category(db_session):
    """Create a sample category for testing"""
    category = models.Category(name="Test Toys Unique", sku_initials="TTU")
    db_session.add(category)
    db_session.commit()
    db_session.refresh(category)
    return category


@pytest.fixture
def sample_product(db_session, sample_category):
    """Create a sample product for testing"""
    product = models.Product(
        sku="TT-0001",
        name="Test Product",
        description="A test product",
        tags=["test", "sample"],
        production=False,
        active=True,
        category_id=sample_category.id,
        folder_path="/test/path",
    )
    db_session.add(product)
    db_session.commit()
    db_session.refresh(product)
    return product
