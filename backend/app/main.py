from fastapi import FastAPI, Depends
from sqlalchemy.orm import Session

from .database import Base, engine, get_db
from . import schemas, models, crud

Base.metadata.create_all(bind=engine)

app = FastAPI()


@app.post("/products/")
def create_product(product: schemas.ProductCreate, db: Session = Depends(get_db)):
    return crud.create_product(db, product)


@app.get("/products/")
def list_products(db: Session = Depends(get_db)):
    return db.query(models.Product).all()
