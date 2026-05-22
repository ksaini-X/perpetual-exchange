use axum::Router;
use axum::extract::Path;
use axum::routing::{get, post};
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = Router::new().route("/positions/open/{asset}", get(get_open_positions));

    axum::serve(listener, router);
}

async fn get_open_positions(Path(asset): Path<String>) {}
