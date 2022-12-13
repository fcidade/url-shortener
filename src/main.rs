use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use nanoid::nanoid;

#[tokio::main]
async fn main() {
    // pool.execute(include_str!("../schema.sql"))
    //     .await
    //     .unwrap();
    let port = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/version", get(version))
        .route("/u/:id", get(redirect_url))
        .route("/shorten", post(shorten_url));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on: {}", addr);
    tracing::debug!("listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn healthcheck() -> &'static str {
    "OK"
}

async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

async fn redirect_url() {}

async fn shorten_url(Json(payload): Json<shorten_url::Request>) -> impl IntoResponse {
    let generated_id = nanoid!(6);
    // Save into database
    // Return generated uri
    let response_body = shorten_url::Response {
        generated_id: generated_id.clone(),
        uri: format!("/u/{}", generated_id),
    };
    (StatusCode::CREATED, Json(response_body))
}

mod shorten_url {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct Request {
        pub url: String,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub generated_id: String,
        pub uri: String,
    }
}
