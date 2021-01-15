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

#[derive(Deserialize)]
struct GayInfo {
    gender: String,
    sexuality: String,
}

#[get("/")]
async fn hi(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("https://github.com/jaq-the-cat/gaypi")
}

#[get("/info")]
async fn gayinfo(info: web::Json<GayInfo>) -> impl Responder {
    if !is_valid(&info.gender) {
        return HttpResponse::NotFound().json(map![
            "error" => Errors::GenderNotFound
        ]);
    } else if !is_valid(&info.sexuality) {
        return HttpResponse::NotFound().json(map![
            "error" => Errors::SexualityNotFound
        ]);
    }

    return HttpResponse::Ok().json(map![
        "gender" => flags!(&info.gender[..]),
        "sexuality" => flags!(&info.sexuality[..])
    ]);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting gAyPI server");
    HttpServer::new(|| App::new().service(hi).service(gayinfo))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
