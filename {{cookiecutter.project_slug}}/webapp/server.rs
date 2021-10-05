use crate::webapp::routes;
use axum::{routing::BoxRoute, Router};
use std::env;
use std::net::SocketAddr;

#[allow(dead_code)]
pub async fn serve() {
    // Attaches routes to the host address

    let host = get_host();
    let routes = app().into_make_service();
    axum::Server::bind(&host).serve(routes).await.unwrap();
}

pub fn app() -> Router<BoxRoute> {
    Router::new()
        .nest("/", routes::ssr::get_routes())
        .nest("/api", routes::api::get_routes())
        .boxed()
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
