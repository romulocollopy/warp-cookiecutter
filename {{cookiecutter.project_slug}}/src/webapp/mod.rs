mod server;

pub async fn serve() {
    server::serve().await;
}
