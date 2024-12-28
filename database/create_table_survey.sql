CREATE TABLE IF NOT EXISTS survey (
    user_id SERIAL PRIMARY KEY,
    history TEXT,
    tytul_wpisy VARCHAR(32) DEFAULT NULL,
    wybrany_cytat VARCHAR(100) DEFAULT NULL,
    czy_grafika boolean, 
    czy_osoby_trzecie boolean DEFAULT false,
    link_do_zgody_na_wizerunek VARCHAR(200) DEFAULT NULL,
    czy_zgoda_na_opublikowanie_grafiki boolean DEFAULT true,
    czy_anonimowo boolean DEFAULT true,
    podpis VARCHAR(100) DEFAULT NULL,
    czy_autentyczny_wpis boolean,
    czy_zgoda_na_publikacje_wpisu boolean,
    czy_zgoda_na_wykorzystywanie_fragmentow boolean,
    czy_zgoda_na_regulamin boolean,
    czy_zgoda_na_rodo boolean,
    czas_dodania timestamp
)