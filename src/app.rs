use axum::{
  Router,
  http::StatusCode,
  response::{Html, IntoResponse},
  routing::get
};

use crate::{config::AppConfig, database::initialize_dbs};
use crate::{database::Databases, handlers};


#[derive(Clone)]
pub struct AppState {
  pub env: String,
  pub databases: Databases,
}

/** Initialise API */
pub async fn initialize_app() -> Router {
  return Router::new().nest(
    "/api",
    Router::new()
      .route("/", get(|| async { Html("Hello World!") }))
      .fallback(handler_404), // 👈 catch-all handler
  );
}

async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Route not found")
}
