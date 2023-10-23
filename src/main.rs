use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest::header::USER_AGENT;
use serde_json::Value;

#[get("/{url}")]
async fn base() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn catch_all(request: HttpRequest) -> impl Responder {
    let client = reqwest::Client::new();

    let path = String::from(request.path())
        .strip_prefix("/")
        .unwrap()
        .to_string();

    // let old = reqwest::get(path).await.unwrap().text().await.unwrap();
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

    // HttpResponse::Ok().body()
    web::Json(raw)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("running on: {}", String::from("http://127.0.0.1:8080"));

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default())
            .service(base)
            .default_service(web::to(catch_all))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
