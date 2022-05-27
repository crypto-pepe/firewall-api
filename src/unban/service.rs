use crate::api::UnBanRequest;
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use crate::unban::{Executor, UnBanner, UnbanStatus};

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
    async fn unban(&self, ur: UnBanRequest) -> Result<(), Vec<UnbanStatus>> {
        let mut ubs = Vec::new();
        for exec in &self.executors {
            let resp = self
                .cli
                .delete(&exec.url)
                .body(serde_json::to_vec(&ur).expect("UnBanEntity derives Serialize"))
                .header(CONTENT_TYPE, "application/json".to_string())
                .send()
                .await;
            if let Err(e) = resp {
                tracing::error!("{:?}", e);
                ubs.push(UnbanStatus::Error(
                    exec.name.clone(),
                    "internal error".to_string(),
                ));
                continue;
            }
            let resp = resp.unwrap();
            if resp.status() != StatusCode::NO_CONTENT {
                ubs.push(UnbanStatus::Error(
                    exec.name.clone(),
                    resp.status()
                        .canonical_reason()
                        .unwrap_or("internal error")
                        .to_string(),
                ))
            } else {
                ubs.push(UnbanStatus::Ok(exec.name.clone()))
            }
        }
        if ubs.iter().any(|a| matches!(a, UnbanStatus::Error(_, _))) {
            return Err(ubs);
        }
        Ok(())
    }
}
