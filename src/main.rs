use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tracing::info;
use tower_serve_static::ServeDir;
use include_dir::{Dir, include_dir};

mod utils;
mod index;

static ASSET_DIR: Dir<'static> = include_dir!("assets");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index::root))
        .nest_service("/assets", ServeDir::new(&ASSET_DIR))
        .fallback(fallback);


    info!("Start sever on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "This is not a valid route")
}
