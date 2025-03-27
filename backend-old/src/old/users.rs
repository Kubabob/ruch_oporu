use actix_web::{web, Responder,HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// Struktura reprezentująca użytkownika
#[derive(Serialize)]
struct User {
    user_id: i32,
    imie: Option<String>,
    nazwisko: Option<String>,
    email: Option<String>,
}

// Struktura dla dodawania/aktualizacji użytkownika
#[derive(Serialize, Deserialize)]
struct CreateUser {
    imie: String,
    nazwisko: String,
    email: String,
}

// CREATE - Dodawanie użytkownika
#[actix_web::post("/uzytkownicy")]
async fn create_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<CreateUser>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO uzytkownik (imie, nazwisko, email) VALUES ($1, $2, $3) RETURNING user_id",
        new_user.imie,
        new_user.nazwisko,
        new_user.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.user_id),
        Err(err) => {
            eprintln!("Błąd: {}", err);
            HttpResponse::InternalServerError().body("Nie udało się dodać użytkownika")
        }
    }
}

// READ - Pobieranie listy użytkowników
#[actix_web::get("/uzytkownicy")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        "SELECT user_id, imie, nazwisko, email FROM uzytkownik"
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
#[actix_web::put("/uzytkownicy/{id}")]
async fn update_user(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    updated_user: web::Json<CreateUser>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE uzytkownik SET imie = $1, nazwisko = $2, email = $3 WHERE user_id = $4",
        updated_user.imie,
        updated_user.nazwisko,
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
#[actix_web::delete("/uzytkownicy/{id}")]
async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM uzytkownik WHERE user_id = $1", *id)
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