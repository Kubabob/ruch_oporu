CREATE TABLE IF NOT EXISTS Ankieta (
    id SERIAL PRIMARY KEY,
    rola VARCHAR(4),
    historia TEXT,
    tytul_wpisu VARCHAR(100) DEFAULT NULL,
    wybrany_cytat VARCHAR(100) DEFAULT NULL,
    czy_grafika boolean,
    grafika VARCHAR(200) DEFAULT NULL, 
    czy_osoby_trzecie boolean DEFAULT false,
    zgoda_na_wizerunek VARCHAR(200) DEFAULT NULL,
    czy_zgoda_na_opublikowanie_grafiki boolean DEFAULT true,
    czy_anonimowo boolean DEFAULT true,
    podpis VARCHAR(100) DEFAULT NULL,
    czy_autentyczny_wpis boolean,
    czy_zgoda_na_wykorzystywanie_fragmentow boolean,
    czy_zgoda_na_regulamin boolean,
    czy_zgoda_na_rodo boolean,
    czas_dodania timestamp DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE IF NOT EXISTS ankieta (
    ankieta_id SERIAL PRIMARY KEY,
    user_id INT,
    historia TEXT,
    tytul_wpisu VARCHAR(32) DEFAULT NULL,
    wybrany_cytat VARCHAR(100) DEFAULT NULL,
    czy_grafika boolean, 
    czy_osoby_trzecie boolean DEFAULT false,
    nazwa_grafiki VARCHAR(50) DEFAULT NULL,
    nazwa_zgody_na_wizerunek VARCHAR(50) DEFAULT NULL,
    czy_zgoda_na_opublikowanie_grafiki boolean DEFAULT true,
    czy_anonimowo boolean DEFAULT true,
    podpis VARCHAR(100) DEFAULT NULL,
    czy_autentyczny_wpis boolean,
    czy_zgoda_na_publikacje_wpisu boolean,
    czy_zgoda_na_wykorzystywanie_fragmentow boolean,
    czy_zgoda_na_regulamin boolean,
    czy_zgoda_na_rodo boolean,
    czas_dodania timestamp DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id
        FOREIGN KEY(user_id)
            REFERENCES uzytkownik(user_id)
);

