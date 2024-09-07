use axum::response::{Html, IntoResponse};
use askama::Template;
use crate::html;


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    app: String,
}

pub async fn root() -> impl IntoResponse {
    html!(IndexTemplate { app: "new app".to_string() })
}

