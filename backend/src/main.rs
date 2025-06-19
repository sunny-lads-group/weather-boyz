use axum::{
    Router,
    routing::get,
};
use tower_http::cors::CorsLayer;



#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/hello", get(hello_world))
        .layer(cors);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
