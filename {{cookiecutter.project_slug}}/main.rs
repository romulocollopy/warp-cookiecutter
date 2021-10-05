mod webapp;

#[tokio::main]
async fn main() {
    webapp::server::serve().await
}
