CREATE TABLE pliki_ankiety (
    pliki_ankiety_id SERIAL PRIMARY KEY,
    ankieta_id INT,
    nazwa_grafiki VARCHAR(50),
    grafika BYTEA,
    nazwa_zgody_na_wizerunek VARCHAR(50),
    zgoda_na_wizerunek BYTEA, -- Przechowywanie pliku
    czas_dodania TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_ankieta_id
        FOREIGN KEY (ankieta_id)
            REFERENCES ankieta (ankieta_id)
);
