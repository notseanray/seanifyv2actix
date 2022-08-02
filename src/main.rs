mod lib;

#[tokio::main]
async fn main() {
    lib::run().await;
}
