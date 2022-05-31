use serde::{Deserialize, Serialize};
pub mod client;
pub use client::Client;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub base_url: String,
}
