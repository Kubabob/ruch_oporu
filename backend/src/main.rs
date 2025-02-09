// src/main.rs
use actix_web::{App, HttpServer};
use backend::handlers;
use dotenv;
use std::env;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::from_path(".env").expect("Failed to load .env file");

    let backend_url = env::var("BACKEND_URL").expect("BACKEND_URL must be set");

    HttpServer::new(|| {
        let cors = Cors::default()
            // .allowed_origin(&env::var("FRONTEND_URL").expect("FRONTEND_URL must be set")) // Your frontend origin
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);
        
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(handlers::form_submission::submit_form)
            .service(handlers::queries::get_total_submissions)
            .service(handlers::queries::get_submissions_by_status)
            .service(handlers::queries::get_submissions_with_status)
            .service(handlers::queries::get_submissions_count_with_status)
            .service(handlers::queries::get_submission_by_id)
    })
    .bind(backend_url)?
    .run()
    .await
}