use axum::{
  Router,
  http::StatusCode,
  response::{Html, IntoResponse},
  routing::get,
};

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
