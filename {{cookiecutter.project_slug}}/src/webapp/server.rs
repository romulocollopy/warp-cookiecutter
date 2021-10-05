use axum::{handler::get, routing::Route, Router};
use http::{Request, Response, StatusCode, Uri};
use std::env;
use std::net::SocketAddr;

async fn get_slash() {
    // `GET /` called
}

async fn post_slash() {
    // `POST /` called
}

async fn get_foo() {
    // `GET /foo` called
}

pub async fn serve() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"

    let host = get_host();
    let app = get_routes();
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_routes() -> Router<Route> {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

fn get_host() -> SocketAddr {
    let host = match env::var("HOST") {
        Ok(val) => val,
        Err(_e) => "0.0.0.0".to_string(),
    };
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_e) => "8088".to_string(),
    };
    format!("{}:{}", host, port).parse().unwrap()
}
