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
async fn is_good(what: web::Json<Vec<String>>) -> impl Responder {
    let mut results: Vec<Option<bool>> = Vec::new();
    for thing in what.iter() {
        results.push(match &thing.to_lowercase()[..] {
            "lgbt" => Some(true),
            "lgbt+" => Some(true),
            "lgbtq" => Some(true),
            "lgbtq+" => Some(true),
            "lgbtqi" => Some(true),
            "lgbtqi+" => Some(true),
            "lgbtqia" => Some(true),
            "lgbtqia+" => Some(true),
            "gay" => Some(true),
            "lesbian" => Some(true),
            "bi" => Some(true),
            "pan" => Some(true),
            "ace" => Some(true),
            "trans" => Some(true),
            "cis" => Some(true),
            "lgbtphobia" => Some(false),
            "homophobia" => Some(false),
            "homophobes" => Some(false),
            "transphobia" => Some(false),
            "transphobes" => Some(false),
            "biphobia" => Some(false),
            "biphobes" => Some(false),
            "trump" => Some(false),
            _ => None,
        });
    }
    HttpResponse::Ok().json(results)
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
            .service(is_good)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
