from fastapi import FastAPI, HTTPException, Depends
import ankieta
from database import engine, SessionLocal
from sqlalchemy.orm import Session
from typing import Annotated
from pydantic import BaseModel
from datetime import datetime
from typing import Optional

class AnkietaBase(BaseModel):
    rola: str
    historia: str
    tytul_wpisu: str
    wybrany_cytat: str
    czy_grafika: bool
    grafika: Optional[str] = None
    czy_osoby_trzecie: bool
    zgoda_na_wizerunek: Optional[str] = None
    czy_zgoda_na_grafike: bool
    czy_anonimowo: bool
    podpis: Optional[str] = None
    czy_autentyczny_wpis: bool
    czy_zgoda_na_wykorzystanie_fragmentow: bool
    czy_zgoda_na_regulamin: bool


app = FastAPI()
ankieta.Base.metadata.create_all(bind=engine)


def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

db_dependency = Annotated[Session, Depends(get_db)]

@app.get("/")
async def root():
    return {"message": "Hello World"}


@app.post("/podziel-sie/")
async def post_form(ankietaBase: AnkietaBase, db: db_dependency):
    db_question = ankieta.Ankieta(
        rola=ankietaBase.rola,
        historia=ankietaBase.historia,
        tytul_wpisu=ankietaBase.tytul_wpisu,
        wybrany_cytat=ankietaBase.wybrany_cytat,
        czy_grafika=ankietaBase.czy_grafika,
        grafika=ankietaBase.grafika,
        czy_osoby_trzecie=ankietaBase.czy_osoby_trzecie,
        zgoda_na_wizerunek=ankietaBase.zgoda_na_wizerunek,
        czy_zgoda_na_grafike=ankietaBase.czy_zgoda_na_grafike,
        czy_anonimowo=ankietaBase.czy_anonimowo,
        podpis=ankietaBase.podpis,
        czy_autentyczny_wpis=ankietaBase.czy_autentyczny_wpis,
        czy_zgoda_na_wykorzystanie_fragmentow=ankietaBase.czy_zgoda_na_wykorzystanie_fragmentow,
        czy_zgoda_na_regulamin=ankietaBase.czy_zgoda_na_regulamin
    )
    db.add(db_question)
    db.commit()
    db.refresh(db_question)
    return db_question

@app.get("/podziel-sie/{id}")
async def get_form_by_id(question_id: int, db: db_dependency):
    result = db.query(ankieta.Ankieta).filter(ankieta.Ankieta.id == question_id).first()

    if not result:
        raise HTTPException(status_code=404, detail=f"Nie znaleziono historii z id {question_id}")
    return result