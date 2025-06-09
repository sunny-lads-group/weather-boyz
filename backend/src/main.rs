use axum::{
	routing::get,
	Router,
	response::Json,
};
use serde_json::{json};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
tracing_subscriber::fmt::init();

let app = Router::new()
.route("/", get(root_handler));

let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
println!("Listening on {}", addr);
axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn root_handler() -> Json<serde_json::Value> {
Json(json!({ "message": "Hello from Axum API!" }))
}