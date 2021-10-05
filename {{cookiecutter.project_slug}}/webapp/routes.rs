pub mod ssr {
    use axum::{handler::get, handler::post, routing::BoxRoute, Router};

    pub fn get_routes() -> Router<BoxRoute> {
        Router::new()
            .route("/", get(get_slash))
            .route("/", post(post_slash))
            .boxed()
    }

    async fn get_slash() -> String {
        // `GET /` called
        String::from("Hello, World!")
    }

    async fn post_slash() -> String {
        // `POST /` called
        String::from("Post World, mdf!")
    }
}

pub mod api {
    use axum::{handler::get, handler::post, routing::BoxRoute, Router};

    pub fn get_routes() -> Router<BoxRoute> {
        Router::new()
            .route("/", get(fuuu))
            .route("/", post(fuuu))
            .boxed()
    }

    async fn fuuu() -> String {
        // `GET /foo` called
        String::from("Fuuuuuuuu")
    }

    mod tests {
        // #[tokio::test]
        // async fn handler() {
        //     let app = Router::new().route(
        //         "/",
        //         get(forever
        //             .layer(timeout())
        //             .handle_error(|_: BoxError| Ok::<_, Infallible>(StatusCode::REQUEST_TIMEOUT))),
        //     );
        //
        //     let client = TestClient::new(app);
        //
        //     let res = client.get("/").send().await;
        //     assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);
        // }
    }
}
