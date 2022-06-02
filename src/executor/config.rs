use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExecutorConfig {
    pub name: String,
    pub base_url: String,
}

pub type Config = Vec<ExecutorConfig>;
