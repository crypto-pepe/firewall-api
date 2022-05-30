use async_trait::async_trait;
use futures::future::join_all;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use crate::api::UnBanRequest;
use crate::unbanner::{Executor, UnBanner, UnbanError};

pub struct Service {
    cli: reqwest::Client,
    executors: Vec<Executor>,
}

impl Service {
    pub fn new(executors: Vec<Executor>) -> Self {
        let cli = reqwest::Client::new();
        Service { cli, executors }
    }
}

#[async_trait]
impl UnBanner for Service {
    async fn unban(&self, ur: UnBanRequest) -> Result<(), Vec<UnbanError>> {
        let mut ubs = Vec::new();
        let handles = self.executors.iter().map(|e| {
            self.cli
                .delete(&e.url)
                .body(serde_json::to_vec(&ur).expect("UnBanEntity derives Serialize"))
                .header(CONTENT_TYPE, "application/json".to_string())
                .send()
        });

        for (resp, executor) in join_all(handles).await.iter().zip(&self.executors) {
            if let Err(e) = resp {
                tracing::error!("{:?}", e);
                ubs.push(UnbanError {
                    executor_name: executor.name.clone(),
                    error_desc: e.to_string(),
                });
                continue;
            }
            let resp = resp.as_ref().unwrap();
            if resp.status() != StatusCode::NO_CONTENT {
                ubs.push(UnbanError {
                    executor_name: executor.name.clone(),
                    error_desc: resp
                        .status()
                        .canonical_reason()
                        .unwrap_or("internal error")
                        .to_string(),
                });
            }
        }
        if !ubs.is_empty() {
            return Err(ubs);
        }
        Ok(())
    }
}
