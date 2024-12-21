use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, types::{Json, JsonValue}};

// Struktura dla odpowiedzi JSON
#[derive(Serialize)]
struct Message {
    message: String,
}


// Endpoint zwracający JSON
#[get("/api/hello")]
async fn hello_json() -> impl Responder {
    let response = Message {
        message: "Cześć z backendu Actix!".to_string(),
    };
    HttpResponse::Ok().json(response)
}