// Provides gay through an easy-to-use REST API!

use actix_web::*;
use serde::{Deserialize, Serialize};

mod sexualities;

macro_rules! json {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

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
    return if sexualities::Sexualities::is_valid(&info.sexuality) {
        HttpResponse::Ok().json([info.gender.clone(), info.sexuality.clone()])
    } else {
        HttpResponse::NotFound()
            .status(http::StatusCode::NOT_FOUND)
            .json(json![
                "error" => "sexual orientation not found"
            ])
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting gAyPI server");
    HttpServer::new(|| App::new().service(hi).service(gayinfo))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
