use serde::{Deserialize, Serialize};
pub mod service;
pub use service::ExecutorClient;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Executor {
    pub name: String,
    pub base_url: String,
}
