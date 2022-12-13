use sqlx::{MySql, Pool};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

struct ShortenedURL {
    url: String,
}

pub async fn handler(State(pool): State<Pool<MySql>>, Path(id): Path<String>) -> impl IntoResponse {
    let res = sqlx::query_as!(
        ShortenedURL,
        "select url from shortened_urls where id = ?",
        id.clone()
    )
    .fetch_one(&pool)
    .await;
    match res {
        Ok(row) => {
            let url = String::from(row.url);
            let contains_protocol = url.contains("http://") || url.contains("https://");
            let url = if contains_protocol { url } else { format!{"https://{}", url} };
            return Redirect::permanent(&*url).into_response();
        }
        Err(e) => {
            if let sqlx::Error::RowNotFound = e {
                return (StatusCode::NOT_FOUND).into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }
}
