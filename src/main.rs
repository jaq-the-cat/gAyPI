// Provides gay through an easy-to-use REST API!
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message(&'static str);

async fn hi(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(web::Json(Message("Hello")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hi)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
