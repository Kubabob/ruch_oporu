use actix_web::{http::header::ContentDisposition, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::NaiveDateTime;
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Ankieta {
    pub ankieta_id: i32,
    pub user_id: Option<i32>,
    pub historia: Option<String>,
    pub tytul_wpisu: Option<String>,
    pub wybrany_cytat: Option<String>,
    pub czy_grafika: Option<bool>,
    pub czy_osoby_trzecie: Option<bool>,
    pub nazwa_grafiki: Option<String>,
    pub nazwa_zgody_na_wizerunek: Option<String>,
    pub czy_zgoda_na_opublikowanie_grafiki: Option<bool>,
    pub czy_anonimowo: Option<bool>,
    pub podpis: Option<String>,
    pub czy_autentyczny_wpis: Option<bool>,
    pub czy_zgoda_na_publikacje_wpisu: Option<bool>,
    pub czy_zgoda_na_wykorzystywanie_fragmentow: Option<bool>,
    pub czy_zgoda_na_regulamin: Option<bool>,
    pub czy_zgoda_na_rodo: Option<bool>,
    pub czas_dodania: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DodajAnkiete {
    pub user_id: i32,
    pub historia: String,
    pub tytul_wpisu: String,
    pub wybrany_cytat: String,
    pub czy_grafika: bool,
    pub czy_osoby_trzecie: bool,
    pub nazwa_grafiki: String,
    pub nazwa_zgody_na_wizerunek: String,
    pub czy_zgoda_na_opublikowanie_grafiki: bool,
    pub czy_anonimowo: bool,
    pub podpis: String,
    pub czy_autentyczny_wpis: bool,
    pub czy_zgoda_na_publikacje_wpisu: bool,
    pub czy_zgoda_na_wykorzystywanie_fragmentow: bool,
    pub czy_zgoda_na_regulamin: bool,
    pub czy_zgoda_na_rodo: bool,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PlikiAnkiety {
    pub pliki_ankiety_id: i32,
    pub ankieta_id: Option<i32>,
    pub nazwa_grafiki: Option<String>,
    pub grafika: Option<Vec<u8>>,
    pub nazwa_zgody_na_wizerunek: Option<String>,
    pub zgoda_na_wizerunek: Option<Vec<u8>>,
    pub czas_dodania: Option<NaiveDateTime>
}


#[derive(Serialize)]
pub struct SubmissionResult {
    message: String
}

#[derive(Serialize)]
pub struct SurveyNumber {
    number: i64
}


#[actix_web::post("/ankieta")]
pub async fn submit_survey(
    pool: web::Data<PgPool>,
    form: web::Json<DodajAnkiete>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO ankieta (user_id, historia, tytul_wpisu, wybrany_cytat, czy_grafika, czy_osoby_trzecie, nazwa_grafiki, nazwa_zgody_na_wizerunek, czy_zgoda_na_opublikowanie_grafiki, czy_anonimowo, podpis, czy_autentyczny_wpis, czy_zgoda_na_publikacje_wpisu, czy_zgoda_na_wykorzystywanie_fragmentow, czy_zgoda_na_regulamin, czy_zgoda_na_rodo) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING ankieta_id",
        form.user_id,
        form.historia,
        form.tytul_wpisu,
        form.wybrany_cytat,
        form.czy_grafika,
        form.czy_osoby_trzecie,
        form.nazwa_grafiki,
        form.nazwa_zgody_na_wizerunek,
        form.czy_zgoda_na_opublikowanie_grafiki,
        form.czy_anonimowo,
        form.podpis,
        form.czy_autentyczny_wpis,
        form.czy_zgoda_na_publikacje_wpisu,
        form.czy_zgoda_na_wykorzystywanie_fragmentow,
        form.czy_zgoda_na_regulamin,
        form.czy_zgoda_na_rodo,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(SubmissionResult {
            message: "Odpowiedź została zapisana!".to_string(),
        }),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się zapisać odpowiedzi.")
        }
    }
}


#[actix_web::post("/ankieta/upload")]
pub async fn send_survey_file(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> impl Responder {
    let mut ankieta_id = 0;
    let mut nazwa_grafiki = String::new();
    let mut grafika = Vec::new();
    let mut nazwa_zgody_na_wizerunek = String::new();
    let mut zgoda_na_wizerunek = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => continue,
        };

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };

        let name = match content_disposition.get_name() {
            Some(name) => name,
            None => continue,
        };

        match name {
            "ankieta_id" => {
                if let Some(chunk) = field.next().await {
                    let ankieta_id_bytes = chunk.unwrap().to_vec();
                    ankieta_id = String::from_utf8(ankieta_id_bytes).unwrap().trim().parse::<i32>().unwrap();
                }
            }
            "nazwa_grafiki" => {
                if let Some(chunk) = field.next().await {
                    let nazwa_grafiki_bytes = chunk.unwrap().to_vec();
                    nazwa_grafiki = String::from_utf8(nazwa_grafiki_bytes).unwrap().trim().to_string();
                }
            }
            "grafika" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    grafika.extend_from_slice(&data);
                }
            }
            "nazwa_zgody_na_wizerunek" => {
                if let Some(chunk) = field.next().await {
                    let nazwa_zgody_na_wizerunek_bytes = chunk.unwrap().to_vec();
                    nazwa_zgody_na_wizerunek = String::from_utf8(nazwa_zgody_na_wizerunek_bytes).unwrap().trim().to_string();
                }
            }
            "zgoda_na_wizerunek" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    zgoda_na_wizerunek.extend_from_slice(&data);
                }
            }
            _ => {}
        }
    }

    // Insert into the database
    let result = sqlx::query!(
        "INSERT INTO pliki_ankiety (ankieta_id, nazwa_grafiki, grafika, nazwa_zgody_na_wizerunek, zgoda_na_wizerunek) VALUES ($1, $2, $3, $4, $5)",
        ankieta_id,
        nazwa_grafiki,
        grafika,
        nazwa_zgody_na_wizerunek,
        zgoda_na_wizerunek,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("File uploaded successfully"),
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().body("Failed to upload file")
        }
    }
}


#[actix_web::get("/ankieta")]
async fn get_survey(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Ankieta,
        "SELECT * FROM ankieta"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(responses) => HttpResponse::Ok().json(responses),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się pobrać odpowiedzi.")
        }
    }
}

#[actix_web::get("/ankieta/upload")]
async fn get_survey_uploads(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        PlikiAnkiety,
        "SELECT * FROM pliki_ankiety"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(responses) => HttpResponse::Ok().json(responses),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się pobrać odpowiedzi.")
        }
    }
}



#[actix_web::get("/ankieta/number")]
async fn get_survey_number(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT COUNT(*) FROM ankieta")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(response) => HttpResponse::Ok().json(SurveyNumber {
            number: response.count.unwrap().into()
        }),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się pobrać odpowiedzi.")
        }
    }   
}

#[actix_web::get("/ankieta/upload/number")]
async fn get_survey_uploads_number(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT COUNT(*) FROM pliki_ankiety")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(response) => HttpResponse::Ok().json(SurveyNumber {
            number: response.count.unwrap().into()
        }),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się pobrać odpowiedzi.")
        }
    }   
}

