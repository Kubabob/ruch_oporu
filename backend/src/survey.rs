use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Survey {
    pub user_id: i32,
    pub history: Option<String>,
    pub tytul_wpisy: Option<String>,
    pub wybrany_cytat: Option<String>,
    pub czy_grafika: Option<bool>,
    pub czy_osoby_trzecie: Option<bool>,
    pub link_do_zgody_na_wizerunek: Option<String>,
    pub czy_zgoda_na_opublikowanie_grafiki: Option<bool>,
    pub czy_anonimowo: Option<bool>,
    pub podpis: Option<String>,
    pub czy_autentyczny_wpis: Option<bool>,
    pub czy_zgoda_na_publikacje_wpisu: Option<bool>,
    pub czy_zgoda_na_wykorzystywanie_fragmentow: Option<bool>,
    pub czy_zgoda_na_regulamin: Option<bool>,
    pub czy_zgoda_na_rodo: Option<bool>,
    pub czas_dodania: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AddSurvey {
    pub user_id: i32,
    pub history: String,
    pub tytul_wpisy: String,
    pub wybrany_cytat: String,
    pub czy_grafika: bool,
    pub czy_osoby_trzecie: bool,
    pub link_do_zgody_na_wizerunek: String,
    pub czy_zgoda_na_opublikowanie_grafiki: bool,
    pub czy_anonimowo: bool,
    pub podpis: String,
    pub czy_autentyczny_wpis: bool,
    pub czy_zgoda_na_publikacje_wpisu: bool,
    pub czy_zgoda_na_wykorzystywanie_fragmentow: bool,
    pub czy_zgoda_na_regulamin: bool,
    pub czy_zgoda_na_rodo: bool,
    pub czas_dodania: NaiveDateTime,
}



#[derive(Serialize)]
pub struct SubmissionResult {
    message: String
}

#[actix_web::post("/survey")]
pub async fn submit_survey(
    pool: web::Data<PgPool>,
    form: web::Json<AddSurvey>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO survey (user_id, history, tytul_wpisy, wybrany_cytat, czy_grafika, czy_osoby_trzecie, link_do_zgody_na_wizerunek, czy_zgoda_na_opublikowanie_grafiki, czy_anonimowo, podpis, czy_autentyczny_wpis, czy_zgoda_na_publikacje_wpisu, czy_zgoda_na_wykorzystywanie_fragmentow, czy_zgoda_na_regulamin, czy_zgoda_na_rodo, czas_dodania) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
        form.user_id,
        form.history,
        form.tytul_wpisy,
        form.wybrany_cytat,
        form.czy_grafika,
        form.czy_osoby_trzecie,
        form.link_do_zgody_na_wizerunek,
        form.czy_zgoda_na_opublikowanie_grafiki,
        form.czy_anonimowo,
        form.podpis,
        form.czy_autentyczny_wpis,
        form.czy_zgoda_na_publikacje_wpisu,
        form.czy_zgoda_na_wykorzystywanie_fragmentow,
        form.czy_zgoda_na_regulamin,
        form.czy_zgoda_na_rodo,
        form.czas_dodania,
    )
    .execute(pool.get_ref())
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


#[actix_web::get("/survey")]
async fn get_survey(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Survey,
        "SELECT * FROM survey"
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
