mod noop_service;
mod service;
use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::model::{UnBanEntity};
pub use service::Service;

#[async_trait]
pub trait UnBanner {
    async fn unban(&self, ut: UnBanEntity) -> Result<(), Vec<UnbanStatus>>;
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
    // Exec name, code
    Error(String, String),
}

impl From<Vec<UnbanStatus>> for ErrorResponse {
    fn from(uss: Vec<UnbanStatus>) -> Self {
        let mut desc = HashMap::new();
        for us in uss {
            match us {
                UnbanStatus::Ok(s) => desc.insert(s, "OK".to_string()),
                UnbanStatus::Error(s, c) => desc.insert(s, c),
            };
        }
        ErrorResponse {
            code: 500,
            reason: "Some executors didn't unban successfully".to_string(),
            details: Some(desc),
        }
    }
}
