use axum::{response::IntoResponse};


#[derive(Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

/** Register user */
pub async fn register() -> impl IntoResponse {
  println!("->> {:<12} - login", "HANDLER");
}

/** Login user */
pub async fn login() -> impl IntoResponse {
  println!("->> {:<12} - get_user_by_id", "HANDLER");

}