use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    StatusCode,
};
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

    let fetch: reqwest::Response = client
        .get(&path)
        .header(USER_AGENT, "Mythril / 0.1")
        .send()
        .await
        .expect("Couldn't send request!");

    //lists out the header values cause we might need them
    for (key, value) in fetch.headers().iter() {
        println!("{:?}: {:?}", key, value);
    }

    let content_type = fetch
        .headers()
        .get(CONTENT_TYPE)
        .expect("No content type found.")
        .clone();

    if content_type.to_str().unwrap().contains("image") {
        let image_data = fetch.bytes().await.expect("Couldnt get image.");
        // let image = image::load_from_memory(&image_data).expect("Couldnt load image.");

        return HttpResponse::build(StatusCode::OK)
            .content_type(content_type)
            .body(image_data);
    }

    let data = fetch.text().await.unwrap();

    let raw: Value = serde_json::from_str(&data).unwrap();
    return HttpResponse::Ok().json(raw);

    // return web::Json(raw);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut port: u16;
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().expect("Invalid PORT"),
        Err(_) => 3000,
    };

    println!("running on: {}", format!("http://127.0.0.1:{}", port));

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("https://nascent.dev")
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().ends_with(b".nascent.dev")
                    })
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(base)
            .default_service(web::to(catch_all))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
