use crate::api::UnBanRequest;
use futures::future::join_all;
use reqwest::StatusCode;
use serde::Serialize;

use crate::error::ExecutorError;
use crate::executor::config::ExecutorConfig;
use crate::executor::Config;

pub struct Pool {
    executors: Vec<ExecutorConfig>,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ExecutorConfigRequest {
    dry_run: bool,
}

impl Pool {
    pub fn new(cfg: Config) -> Self {
        let client = reqwest::Client::new();
        Pool {
            client,
            executors: cfg as Vec<ExecutorConfig>,
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
        let handles = self.executors.iter().map(|e| {
            let mut b = self
                .client
                .request(method.clone(), format!("{}{}", &e.base_url, path));
            if let Some(payload) = &payload {
                b = b.json(payload)
            }
            b.send()
        });

        let executor_errors: Vec<ExecutorError> = join_all(handles)
            .await
            .iter()
            .zip(&self.executors)
            .filter_map(|(resp, executor)| match resp {
                Err(e) => {
                    tracing::error!("{:?}", e);
                    Some(ExecutorError {
                        executor_name: executor.name.clone(),
                        error_desc: e.to_string(),
                    })
                }
                Ok(resp) => {
                    if resp.status() != expected_status {
                        Some(ExecutorError {
                            executor_name: executor.name.clone(),
                            error_desc: resp
                                .status()
                                .canonical_reason()
                                .unwrap_or("internal error")
                                .to_string(),
                        })
                    } else {
                        None
                    }
                }
            })
            .collect();

        if !executor_errors.is_empty() {
            return Err(executor_errors);
        }
        Ok(())
    }
}
