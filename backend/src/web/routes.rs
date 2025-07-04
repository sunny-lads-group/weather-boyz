use crate::{web::auth, web::services};
use axum::{
    Router,
    extract::Request,
    middleware,
    response::Response,
    routing::{get, post},
};
use std::future::Future;
use std::pin::Pin;
use tracing::{debug, info};

async fn logging_middleware(req: Request, next: axum::middleware::Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();

    info!("Incoming request: {} {}", method, uri);
    debug!("Request headers: {:?}", headers);

    let response = next.run(req).await;

    info!("Response status: {}", response.status());
    response
}

pub async fn app() -> Router {
    Router::new()
        .route("/signin", post(auth::sign_in))
        .route(
            "/tokenvalid/",
            get(services::hello).layer(middleware::from_fn(auth::authorization_middleware)),
        )
        .route(
            "/policy-templates",
            get(services::get_policy_templates).layer(middleware::from_fn(auth::authorization_middleware)),
        )
        .route(
            "/policies",
            post(services::create_policy).layer(middleware::from_fn(auth::authorization_middleware)),
        )
        .route(
            "/policies",
            get(services::get_user_policies).layer(middleware::from_fn(auth::authorization_middleware)),
        )
        .layer(middleware::from_fn(logging_middleware))
}
