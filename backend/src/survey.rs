use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct SurveyResponse {
    name: String,
    email: String,
    answer: String,
}

#[derive(Serialize)]
pub struct SubmissionResult {
    message: String,
}

#[actix_web::post("/survey")]
pub async fn submit_survey(
    pool: web::Data<PgPool>,
    form: web::Json<SurveyResponse>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO survey_responses (name, email, answer) VALUES ($1, $2, $3)",
        form.name,
        form.email,
        form.answer
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
        SurveyResponse,
        "SELECT name, email, answer FROM survey_responses"
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