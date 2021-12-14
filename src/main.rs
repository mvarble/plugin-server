pub mod libraries;
pub mod server;
pub mod testing;

#[tokio::main]
async fn main() {
    let server = server::build();
    server.run(([127, 0, 0, 1], 8000)).await
}
