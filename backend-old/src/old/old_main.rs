use actix_web::{web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
//use actix_web::http::header::{HeaderName, HeaderValue};
//use actix_web::middleware::DefaultHeaders;
use actix_cors::Cors;

mod survey;
mod users;
mod hello;
mod upload_file;
//mod auth;

use survey::{get_survey, get_survey_number, submit_survey, send_survey_file, get_survey_uploads, get_survey_uploads_number};
use hello::hello_json;
use users::{create_user, delete_user, get_users, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL nie ustawiony!");
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Nie można połączyć się z bazą danych");

    println!("Serwer wystartował na http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            /*.wrap_fn(|req, srv| {
                let auth_middleware = auth::AuthMiddleware { service: srv };
                auth_middleware.call(req)
            })*/
            .app_data(web::Data::new(pool.clone()))
            .service(hello_json)
            .service(create_user)
            .service(get_users)
            .service(update_user)
            .service(delete_user)
            .service(submit_survey)
            .service(get_survey)
            .service(get_survey_number)
            .service(send_survey_file)
            .service(get_survey_uploads)
            .service(get_survey_uploads_number)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
