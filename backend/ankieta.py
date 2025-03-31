from pydantic import BaseModel
from datetime import datetime
from sqlalchemy import Boolean, Column, ForeignKey, Integer, String, DateTime, Text
from database import Base

class Ankieta(Base):
    __tablename__ = "Ankieta"

    id = Column(Integer, primary_key=True, index=True)
    rola = Column(String)
    historia = Column(Text)
    tytul_wpisu = Column(String)
    wybrany_cytat = Column(String)
    czy_grafika = Column(Boolean)
    grafika = Column(String)
    czy_osoby_trzecie = Column(Boolean)
    zgoda_na_wizerunek = Column(String)
    czy_zgoda_na_grafike = Column(Boolean)
    czy_anonimowo = Column(Boolean)
    podpis = Column(String)
    czy_autentyczny_wpis = Column(Boolean)
    czy_zgoda_na_wykorzystanie_fragmentow = Column(Boolean)
    czy_zgoda_na_regulamin = Column(Boolean)
    czas_dodania = Column(DateTime)
