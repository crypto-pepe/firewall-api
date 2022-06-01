use crate::api::UnBanRequest;
use futures::future::join_all;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde::Serialize;

use crate::error::ExecutorError;
use crate::executor::config::ExecutorInfo;
use crate::executor::Config;

pub struct Client {
    executors: Vec<ExecutorInfo>,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ExecutorConfigRequest {
    dry_run: bool,
}

impl Client {
    pub fn new(cfg: Config) -> Self {
        let client = reqwest::Client::new();
        Client {
            client,
            executors: cfg as Vec<ExecutorInfo>,
        }
    }

    pub async fn enable_dry_run_mode(&self, enabled: bool) -> Result<(), Vec<ExecutorError>> {
        self.do_request(
            reqwest::Method::POST,
            "/config".to_string(),
            Some(&ExecutorConfigRequest { dry_run: enabled }),
            StatusCode::NO_CONTENT,
        )
            .await
    }

    pub async fn unban(&self, req: UnBanRequest) -> Result<(), Vec<ExecutorError>> {
        self.do_request(
            reqwest::Method::DELETE,
            "/bans".to_string(),
            Some(req),
            StatusCode::NO_CONTENT,
        )
            .await
    }

    async fn do_request<T: Serialize>(
        &self,
        method: reqwest::Method,
        path: String,
        payload: Option<T>,
        expected_status: StatusCode,
    ) -> Result<(), Vec<ExecutorError>> {
        let mut ubs = Vec::new();
        let handles = self.executors.iter().map(|e| {
            let mut b = self
                .client
                .request(method.clone(), format!("{}{}", &e.base_url, path));
            if let Some(payload) = &payload {
                b = b
                    .json(payload)
                    .header(CONTENT_TYPE, "application/json".to_string());
            }
            b.send()
        });

        for (resp, executor) in join_all(handles).await.iter().zip(&self.executors) {
            if let Err(e) = resp {
                tracing::error!("{:?}", e);
                ubs.push(ExecutorError {
                    executor_name: executor.name.clone(),
                    error_desc: e.to_string(),
                });
                continue;
            }
            let resp = resp.as_ref().unwrap();
            if resp.status() != expected_status {
                ubs.push(ExecutorError {
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
