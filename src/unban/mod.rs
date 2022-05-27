mod noop_service;
mod service;
mod config;

use std::collections::HashMap;

use anyhow::Error;
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::model::{BanTarget, UnBanEntity};

#[async_trait]
pub trait UnBanner {
    async fn unban(&self, ut: UnBanEntity) -> Result<(), Vec<UnbanStatus>>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Executor {
    pub name: String,
    pub url: String,
}

pub struct Service {
    cli: reqwest::Client,
    executors: Vec<Executor>,
}

pub struct NoOpService {}

impl Service {
    pub fn new(executors: Vec<Executor>) -> Self {
        let cli = reqwest::Client::new();
        Service {
            cli,
            executors: executors.clone(),
        }
    }
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

#[async_trait]
impl UnBanner for Service {
    async fn unban(&self, ut: UnBanEntity) -> Result<(), Vec<UnbanStatus>> {
        let mut ubs = Vec::new();
        for exec in &self.executors {
            let resp = self
                .cli
                .delete(&exec.url)
                .body(serde_json::to_vec(&ut).expect("UnBanEntity derives Serialize"))
                .header(CONTENT_TYPE, "application/json".to_string())
                .send()
                .await
                .map_err(|e| anyhow::Error::from(e));
            if let Err(e) = resp {
                tracing::error!("{:?}",e);
                ubs.push(UnbanStatus::Error(exec.name.clone(), "internal error".to_string()));
                continue;
            }
            let resp = resp.unwrap();
            if resp.status() != StatusCode::NO_CONTENT {
                ubs.push(UnbanStatus::Error(exec.name.clone(), resp.status().to_string()))
            } else {
                ubs.push(UnbanStatus::Ok(exec.name.clone()))
            }
        }
        if ubs.iter().any(|a| matches!(a, UnbanStatus::Error(_,_))) {
            return Err(ubs);
        }
        Ok(())
    }
}

#[async_trait]
impl UnBanner for NoOpService {
    async fn unban(&self, _ut: UnBanEntity) -> Result<(), Vec<UnbanStatus>> {
        Ok(())
    }
}
