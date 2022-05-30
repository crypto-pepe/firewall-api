use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub use service::Service;

use crate::api::UnBanRequest;

mod service;

#[async_trait]
pub trait UnBanner {
    async fn unban(&self, ur: UnBanRequest) -> Result<(), Vec<UnbanError>>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Executor {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct UnbanError {
    pub executor_name: String,
    pub error_desc: String,
}
