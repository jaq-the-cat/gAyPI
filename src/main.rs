/*
 * Provides gay through an easy-to-use API
*/

use actix_web::*;
use std::collections::HashMap;

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
async fn gender(genders: web::Json<Vec<String>>) -> impl Responder {
    let mut response: Vec<Vec<&str>> = Vec::new();
    for gender in genders.iter() {
        if !is_valid(&gender[..]) {
            return HttpResponse::NotFound().json(Errors::GenderNotFound);
        }
        response.push(flags!(&gender[..]));
    }
    HttpResponse::Ok().json(response)
}

#[get("/sexuality")]
async fn sexuality(sxs: web::Json<Vec<String>>) -> impl Responder {
    let mut response: Vec<Vec<&str>> = Vec::new();
    for sexuality in sxs.iter() {
        if !is_valid(&sexuality[..]) {
            return HttpResponse::NotFound().json(Errors::SexualityNotFound);
        }
        response.push(flags!(&sexuality[..]));
    }
    HttpResponse::Ok().json(response)
}

#[get("/rights")]
async fn rights(who: web::Json<Vec<String>>) -> impl Responder {
    let mut json: HashMap<&str, bool> = HashMap::new();
    for el in who.iter() {
        if is_valid(el) {
            json.insert(&el[..], true);
        } else {
            return HttpResponse::NotFound().json(Errors::GenderOrSexualityNotFound);
        }
    }
    HttpResponse::Ok().json(json)
}

#[get("/isgood")]
async fn is_good(what: web::Json<String>) -> impl Responder {
    return HttpResponse::Ok().json(match &what[..] {
        "lgbt" => "yes",
        "lgbt+" => "yes",
        "lgbtq" => "yes",
        "lgbtq+" => "yes",
        "lgbtqi" => "yes",
        "lgbtqi+" => "yes",
        "lgbtqia" => "yes",
        "lgbtqia+" => "yes",
        "gay" => "yes",
        "lesbian" => "yes",
        "bi" => "yes",
        "pan" => "yes",
        "ace" => "yes",
        "trans" => "yes",
        "cis" => "yes",
        _ => "idk",
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting gAyPI server");
    HttpServer::new(|| {
        App::new()
            .service(hi)
            .service(gender)
            .service(sexuality)
            .service(rights)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
