/*
 * Provides gay through an easy-to-use REST API!
*/

use actix_web::*;
use serde::Deserialize;

mod errors;
mod flags;
mod useful;
use errors::Errors;

pub fn is_valid(sx: &str) -> bool {
    ["gay", "lesbian", "bi", "pan", "ace", "cis", "trans"].contains(&sx)
}

#[get("/")]
async fn hi(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(map![
        "docs" => "https://github.com/jaq-the-cat/gaypi"
    ])
}

#[get("/gender")]
async fn gender(gender: web::Json<String>) -> impl Responder {
    if !is_valid(&gender[..]) {
        return HttpResponse::NotFound().json(map![
            "error" => Errors::GenderNotFound
        ]);
    }
    HttpResponse::Ok().json(flags!(&gender[..]))
}

#[get("/sexuality")]
async fn sexuality(sexuality: web::Json<String>) -> impl Responder {
    if !is_valid(&sexuality[..]) {
        return HttpResponse::NotFound().json(map![
            "error" => Errors::GenderNotFound
        ]);
    }
    HttpResponse::Ok().json(flags!(&sexuality[..]))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting gAyPI server");
    HttpServer::new(|| App::new().service(hi).service(gender).service(sexuality))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
