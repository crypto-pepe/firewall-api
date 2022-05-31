use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExecutorInfo {
    pub name: String,
    pub base_url: String,
}

pub type Config = Vec<ExecutorInfo>;
