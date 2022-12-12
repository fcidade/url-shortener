use axum::{
    routing::{get, post},
    Router, Json, http::StatusCode, response::IntoResponse,
};
use nanoid::nanoid;
use sync_wrapper::SyncWrapper;

#[shuttle_service::main]
async fn main() -> shuttle_service::ShuttleAxum {
    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/version", get(version))
        .route("/u/:id", get(redirect_url))
        .route("/shorten", post(shorten_url));

    let sync_wrapper = SyncWrapper::new(app);
    Ok(sync_wrapper)
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
    let response_body = shorten_url::Response{
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