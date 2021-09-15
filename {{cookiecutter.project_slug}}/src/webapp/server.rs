use std::env;
use std::net::SocketAddr;
use warp::Filter;

pub async fn serve() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"

    let hello = warp::path!("hello" / String).map(|name: String| {
        let message = format!("{}: {}", "hello", name);
        Ok(warp::reply::json(&message))
    });

    let bye = warp::path!("bye" / String).map(|name: String| {
        let message = format!("{}: {}", "bye", name);
        Ok(warp::reply::json(&message))
    });

    let routes = hello.or(bye);

    warp::serve(routes).run(get_host()).await;
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
