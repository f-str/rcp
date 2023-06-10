use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use reqwest::Client;
use std::env;
use env_logger::Builder;
use log::{info, LevelFilter, warn};

async fn cors_proxy(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse> {
    let url = match req.match_info().get("url") {
        Some(url) => url,
        None => return {
            warn!("Bad request: not valid url specified");
            Ok(HttpResponse::BadRequest().finish())
        },
    };

    info!("Forwarding request to {}", url);

    let client = Client::new();

    // Determine the HTTP method
    let method = match req.method() {
        &actix_web::http::Method::GET => reqwest::Method::GET,
        &actix_web::http::Method::POST => reqwest::Method::POST,
        &actix_web::http::Method::PUT => reqwest::Method::PUT,
        &actix_web::http::Method::DELETE => reqwest::Method::DELETE,
        _ => return {
            warn!("Bad request: not valid HTTP method specified");
            Ok(HttpResponse::MethodNotAllowed().finish())
        },
    };

    // Forward the request to the specified URL
    let response = client
        .request(method, url)
        .body(body.to_vec())
        .send()
        .await
        .unwrap();

    // Get the Content-Type header from the response
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map(|header| header.to_str().unwrap())
        .unwrap_or("application/json");

    // Create a new response with the response body and appropriate headers
    Ok(HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .append_header(("Access-Control-Max-Age", "3600"))
        .append_header(("Content-Type", content_type))
        .body(response.bytes().await.unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logger based on the environment variable
    let logging_enabled = env::var("LOGGING_ENABLED")
        .map(|val| val == "true")
        .unwrap_or(false);

    if logging_enabled {
        Builder::new()
            .filter_level(LevelFilter::Info)
            .init();
    }

    // Get the port from the environment variable or use the default value 8080
    let port = env::var("PORT")
        .map(|val| val.parse().unwrap_or(8080))
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{url:.+}")
                    .route(web::get().to(cors_proxy))
                    .route(web::post().to(cors_proxy))
                    .route(web::put().to(cors_proxy))
                    .route(web::delete().to(cors_proxy)),
            )
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}