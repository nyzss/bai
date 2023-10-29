use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest::header::USER_AGENT;
use serde_json::Value;
use std::env;

#[get("/")]
async fn base() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn catch_all(request: HttpRequest) -> impl Responder {
    let client = reqwest::Client::new();

    let base_url = String::from(request.path())
        .strip_prefix("/")
        .unwrap()
        .to_string();

    let queries = request.query_string();

    let path = format!("{base_url}?{queries}");
    println!("requested path: {}", path);

    let data = client
        .get(path)
        .header(USER_AGENT, "Mythril / 0.1")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let raw: Value = serde_json::from_str(&data).unwrap();

    web::Json(raw)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut port: u16;
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().expect("Invalid PORT"),
        Err(_) => 8080,
    };

    println!("running on: {}", String::from("http://127.0.0.1:8080"));

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(base)
            .default_service(web::to(catch_all))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
