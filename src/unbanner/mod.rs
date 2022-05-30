use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub use service::Service;

use crate::api::http_error::ErrorResponse;
use crate::api::UnBanRequest;

mod service;

#[async_trait]
pub trait UnBanner {
    async fn unban(&self, ur: UnBanRequest) -> Result<(), Vec<UnbanStatus>>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Executor {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub enum UnbanStatus {
    // Exec name
    Ok(String),
    // Exec name, error
    Error(String, String),
}
