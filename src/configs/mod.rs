use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct MongoConfig {
  pub uri: String,
  pub db_name: String,
}

// #[derive(Debug, Clone)]
pub struct AppConfig {
  pub env: String,
  pub mongo: MongoConfig,
}