// src/handlers/queries.rs
use actix_web::{get, web, HttpResponse, http::header::ContentType};
use crate::db::{create_pool, get_conn};
use mysql::prelude::Queryable;
use serde::Serialize;
use mysql::prelude::FromRow;
use serde_json::json;

// Get total submissions
#[get("/stats/total")]
pub async fn get_total_submissions() -> HttpResponse {
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let mut conn = match get_conn(&pool) {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection error: {}", e)),
    };

    match conn.query_first::<(u64,), &str>("SELECT COUNT(*) FROM form_submissions") {
        Ok(Some((count,))) => HttpResponse::Ok().json(count),
        Ok(None) => HttpResponse::NotFound().body("No submissions found"),
        Err(e) => {
            log::error!("Query error: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve data")
        }
    }
}

// Get submissions by status
#[get("/stats/status")]
pub async fn get_submissions_by_status() -> HttpResponse {
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let mut conn = match get_conn(&pool) {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection error: {}", e)),
    };

    let query = r"
        SELECT status, COUNT(*) as count 
        FROM form_submissions 
        GROUP BY status";

    match conn.query_map::<(String, u64), _, &str, (String, u64)>(query, |(status, count)| (status, count)) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => {
            log::error!("Query error: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve data")
        }
    }
}

// Get submissions with a specific status
#[get("/stats/status/{status}")]
pub async fn get_submissions_with_status(status: web::Path<String>) -> HttpResponse {
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let mut conn = match get_conn(&pool) {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection error: {}", e)),
    };

    let query = "SELECT id, status FROM form_submissions WHERE status = ?";
    match conn.exec_map::<(u64, String), _, _, _, _>(query, (status.into_inner(),), |(id, status)| (id, status)) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => {
            log::error!("Query error: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve data")
        }
    }
}

// Get submission by ID
#[get("/submission/{id}")]
pub async fn get_submission_by_id(id: web::Path<u64>) -> HttpResponse {
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let mut conn = match get_conn(&pool) {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection error: {}", e)),
    };


    let query = "SELECT id, status, history, title, quote, is_graphic, graphic_file, is_another, \
                 image_consent_file, is_public_image, is_nonanonymous, signature, is_authentic, \
                 is_public, usage_consent, rules_consent, rodo_consent FROM form_submissions WHERE id = ?";

    match conn.exec_first::<Submission, _, _>(query, (id.into_inner(),)) {
        Ok(Some(submission)) => HttpResponse::Ok().json(submission),
        Ok(None) => HttpResponse::NotFound().body("Submission not found"),
        Err(e) => {
            log::error!("Query error: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve submission")
        }
    }
}

// Get submissions with a specific status counted
#[get("/stats/status/{status}/count")]
pub async fn get_submissions_count_with_status(status: web::Path<String>) -> HttpResponse {
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let mut conn = match get_conn(&pool) {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection error: {}", e)),
    };

    let query = "SELECT COUNT(*) FROM form_submissions WHERE status = ?";
    match conn.exec_first::<(u64,), _, _>(query, (status.into_inner(),)) {
        Ok(Some((count,))) => HttpResponse::Ok().insert_header(ContentType::json()).json(json!({"count": count})),
        Ok(None) => HttpResponse::NotFound().body("Status not found"),
        Err(e) => {
            log::error!("Query error: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve count")
        }
    }
}

#[derive(Serialize, FromRow)]
    struct Submission {
        id: u64,
        status: String,
        history: String,
        title: Option<String>,
        quote: Option<String>,
        is_graphic: bool,
        graphic_file: Option<String>,
        is_another: bool,
        image_consent_file: Option<String>,
        is_public_image: bool,
        is_nonanonymous: bool,
        signature: Option<String>,
        is_authentic: bool,
        is_public: bool,
        usage_consent: bool,
        rules_consent: bool,
        rodo_consent: bool,
    }