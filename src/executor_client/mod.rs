use serde::{Deserialize, Serialize};
pub mod client;
pub use client::ExecutorClient;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExecutorConfig {
    pub name: String,
    pub base_url: String,
}
