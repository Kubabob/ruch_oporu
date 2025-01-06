CREATE TABLE uploads (
    id SERIAL PRIMARY KEY,
    nazwa_zgody_na_wizerunek TEXT NOT NULL,
    zgoda_na_wizerunek BYTEA NOT NULL, -- Przechowywanie pliku
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
