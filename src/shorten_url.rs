use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, MySql, Pool};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn redirect_url() {}

pub async fn handler(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<Request>,
) -> impl IntoResponse {
    let generated_id = nanoid!(6);

    let query = sqlx::query("insert into shortened_urls (id, url) values (?, ?)")
        .bind(generated_id.clone())
        .bind(payload.url.clone());

    if let Err(e) = pool.execute(query).await {
        println!("{}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let response_body = Response {
        generated_id: generated_id.clone(),
        original_url: payload.url,
        uri: format!("/u/{}", generated_id),
    };

    (StatusCode::CREATED, Json(response_body)).into_response()
}

#[derive(Deserialize)]
pub struct Request {
    pub url: String,
}

#[derive(Serialize)]
pub struct Response {
    pub generated_id: String,
    pub uri: String,
    pub original_url: String,
}
