use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

mod db;
use db::user_queries::create_user;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Test the database connection
    let pool = db::pool::get_pool().await.unwrap();

    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/createUser", post(create_user))
        .layer(cors)
        .layer(axum::extract::Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
