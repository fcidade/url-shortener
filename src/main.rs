use std::{env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{mysql::MySqlPoolOptions};

mod redirect_url;
mod shorten_url;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(env::var("DATABASE_URL").ok().expect("DATABASE_URL should not be empty").as_str())
        .await
        .unwrap();

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/version", get(version))
        .route("/u/:id", get(redirect_url::handler))
        .route("/shorten", post(shorten_url::handler))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on: {}", addr);

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
