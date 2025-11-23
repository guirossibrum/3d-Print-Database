from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, declarative_base
from . import models

DATABASE_URL = "postgresql://admin:admin@db:5432/products"

engine = create_engine(DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()


def create_tables():
    """Create all tables if they don't exist"""
    Base.metadata.create_all(bind=engine)


def ensure_tables_exist():
    """Ensure tables exist, create if they don't"""
    try:
        create_tables()
    except Exception:
        # Tables might already exist or DB not ready
        pass


def get_db():
    ensure_tables_exist()
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
