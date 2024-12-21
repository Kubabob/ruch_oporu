use actix_web::{web, Responder,HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// Struktura reprezentująca użytkownika
#[derive(Serialize)]
struct User {
    id: i32,
    name: Option<String>,
    email: Option<String>,
}

// Struktura dla dodawania/aktualizacji użytkownika
#[derive(Serialize, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// CREATE - Dodawanie użytkownika
#[actix_web::post("/users")]
async fn create_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<CreateUser>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        new_user.name,
        new_user.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.id),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się dodać użytkownika")
        }
    }
}

// READ - Pobieranie listy użytkowników
#[actix_web::get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users"
    )
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się pobrać użytkowników")
        }
    }
}

// UPDATE - Aktualizacja użytkownika
#[actix_web::put("/users/{id}")]
async fn update_user(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    updated_user: web::Json<CreateUser>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        updated_user.name,
        updated_user.email,
        *id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Użytkownik zaktualizowany"),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się zaktualizować użytkownika")
        }
    }
}

// DELETE - Usuwanie użytkownika
#[actix_web::delete("/users/{id}")]
async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", *id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Użytkownik usunięty"),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się usunąć użytkownika")
        }
    }
}