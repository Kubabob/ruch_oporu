use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures_util::stream::StreamExt as _;
use std::fs::File;
use std::io::Write;

#[post("/upload")]
async fn upload_file(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                let content_disposition = field.content_disposition();
                if let Some(filename) = content_disposition.unwrap().get_filename() {
                    let filepath = format!("./uploaded_surveys/{}", sanitize_filename::sanitize(filename));
                    let mut f = File::create(filepath).unwrap();

                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        f.write_all(&data).unwrap();
                    }
                    println!("Plik zapisany!");
                }
            }
            Err(err) => {
                println!("Błąd: {:?}", err);
                return HttpResponse::InternalServerError().body("Błąd podczas przesyłania pliku");
            }
        }
    }
    HttpResponse::Ok().body("Plik przesłany")
}
