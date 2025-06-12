use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;




#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let app: Router = Router::new()
        .route("/", get(handle_request))
        .route("/hello", get(handle_request))
        .layer(cors);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request() -> &'static str {
    "Hello, World!"
}