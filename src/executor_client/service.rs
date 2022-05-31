use crate::api::UnBanRequest;
use futures::future::join_all;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde::Serialize;

use crate::error::ExecutorError;
use crate::executor_client::Executor;

pub struct ExecutorClient {
    executors: Vec<Executor>,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ExecutorConfigRequest {
    dry_run: bool,
}

impl ExecutorClient {
    pub fn new(executors: Vec<Executor>) -> Self {
        let client = reqwest::Client::new();
        ExecutorClient { client, executors }
    }
    async fn _do_request<T: Serialize>(
        &self,
        method: String,
        path: String,
        payload: Option<T>,
        expected_status: StatusCode,
    ) -> Result<(), Vec<ExecutorError>> {
        let mut ubs = Vec::new();
        let handles = self.executors.iter().map(|e| {
            let mut b = match method.as_str() {
                "DELETE" => self.client.delete(format!("{}{}", &e.base_url, path)),
                "POST" => self.client.post(format!("{}{}", &e.base_url, path)),
                "GET" => self.client.post(format!("{}{}", &e.base_url, path)),
                _ => self.client.get(format!("{}{}", &e.base_url, path)), // todo
            };
            if payload.is_some() {
                b = b
                    .body(
                        serde_json::to_vec(&payload.as_ref().unwrap())
                            .expect("payload must derive Serialize"),
                    )
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

    pub async fn enable_dry_run_mode(&self, enabled: bool) -> Result<(), Vec<ExecutorError>> {
        self._do_request(
            "POST".to_string(),
            "/config".to_string(),
            Some(&ExecutorConfigRequest { dry_run: enabled }),
            StatusCode::NO_CONTENT,
        )
        .await
    }

    pub async fn unban(&self, req: UnBanRequest) -> Result<(), Vec<ExecutorError>> {
        self._do_request(
            "DELETE".to_string(),
            "/bans".to_string(),
            Some(req),
            StatusCode::NO_CONTENT,
        )
        .await
    }
}
