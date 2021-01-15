/*
 * Provides gay through an easy-to-use REST API!
*/

use actix_web::*;
use serde::Deserialize;

mod errors;
mod flags;
mod sexualities;
mod useful;
use errors::Errors;
use sexualities as sx;

#[derive(Deserialize)]
struct GayInfo {
    gender: String,
    sexuality: String,
}

#[get("/")]
async fn hi(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Hello")
}

#[get("/info/{gender}/{sexuality}")]
async fn gayinfo(info: web::Path<GayInfo>) -> impl Responder {
    let response: HttpResponse;

    if sx::is_valid(&info.sexuality) {
        response = HttpResponse::Ok().json(map![
            "gender" => flags!(&info.gender[..]),
            "sexuality" => flags!(&info.sexuality[..])
        ]);
    } else {
        response = HttpResponse::NotFound().json(map![
            "error" => Errors::SexualityNotFound
        ]);
    };

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting gAyPI server");
    HttpServer::new(|| App::new().service(hi).service(gayinfo))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
