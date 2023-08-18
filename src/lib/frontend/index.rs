use askama::Template;
use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;
