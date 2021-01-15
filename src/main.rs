// Provides gay through an easy-to-use REST API!
use actix_web::*;
use serde::Serialize;

#[derive(Serialize)]
struct Message(&'static str);

#[get("/")]
async fn hi(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(Message("Hello"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting gAyPI server");
    HttpServer::new(|| App::new().service(hi))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
