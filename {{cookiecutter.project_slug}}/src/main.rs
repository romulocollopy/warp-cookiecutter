mod webapp;

#[tokio::main]
async fn main() {
    webapp::serve().await
}
