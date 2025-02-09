use actix_web::http::header::ContentType;
// src/handlers/form_submission.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::db::{create_pool, get_conn};
use uuid::Uuid;
use mysql::params;
use mysql::prelude::Queryable;

#[derive(Debug, Deserialize)]
pub struct FormData {
    pub status: String,
    pub history: String,
    pub title: Option<String>,
    pub quote: Option<String>,
    pub is_graphic: bool,
    pub graphic_file: Option<String>, // Stores filename
    pub is_another: bool,
    pub image_consent_file: Option<String>, // Stores filename
    pub is_public_image: bool,
    pub is_nonanonymous: bool,
    pub signature: Option<String>,
    pub is_authentic: bool,
    pub is_public: bool,
    pub usage_consent: bool,
    pub rules_consent: bool,
    pub rodo_consent: bool,
}

#[post("/submit")]
pub async fn submit_form(form: web::Json<FormData>) -> impl Responder {
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let mut conn = match get_conn(&pool) {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection error: {}", e)),
    };

    // File handling
    let uuid = Uuid::new_v4().to_string();
    let mut graphic_path = None;
    let mut consent_path = None;

    if let Some(ref filename) = form.graphic_file {
        let new_filename = format!("{}_{}", uuid, filename);
        graphic_path = Some(new_filename.clone());
        // Save file to filesystem (pseudo-code)
        // fs::write(format!("./uploads/{}", new_filename), file_content)?;
    }

    if let Some(ref filename) = form.image_consent_file {
        let new_filename = format!("{}_{}", uuid, filename);
        consent_path = Some(new_filename.clone());
        // Save file to filesystem
    }

    // Insert into database
    let query = r"
        INSERT INTO form_submissions (
            status, history, title, quote, is_graphic, graphic_file,
            is_another, image_consent_file, is_public_image, is_nonanonymous,
            signature, is_authentic, is_public, usage_consent, rules_consent, rodo_consent
        ) VALUES (
            :status, :history, :title, :quote, :is_graphic, :graphic_file,
            :is_another, :image_consent_file, :is_public_image, :is_nonanonymous,
            :signature, :is_authentic, :is_public, :usage_consent, :rules_consent, :rodo_consent
        )";

    let params = params! {
        "status" => &form.status,
        "history" => &form.history,
        "title" => &form.title,
        "quote" => &form.quote,
        "is_graphic" => form.is_graphic,
        "graphic_file" => graphic_path,
        "is_another" => form.is_another,
        "image_consent_file" => consent_path,
        "is_public_image" => form.is_public_image,
        "is_nonanonymous" => form.is_nonanonymous,
        "signature" => &form.signature,
        "is_authentic" => form.is_authentic,
        "is_public" => form.is_public,
        "usage_consent" => form.usage_consent,
        "rules_consent" => form.rules_consent,
        "rodo_consent" => form.rodo_consent,
    };

    match conn.exec_drop(query, params) {
        Ok(_) => HttpResponse::Ok().insert_header(ContentType::json()).json("Form submitted successfully"),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Failed to submit form")
        }
    }
}