use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::executor_client::ExecutorClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct DryRunModeRequest {
    pub enabled: bool,
}

#[tracing::instrument(skip(client))]
#[post("/api/enable-dry-run")]
pub async fn dry_run_mode(
    req: web::Json<DryRunModeRequest>,
    client: Data<ExecutorClient>,
) -> Result<HttpResponse, impl ResponseError> {
    match client.enable_dry_run_mode(req.enabled).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("enable dry run mode: {:?}", e);
            let err_resp: ErrorResponse = e.into();
            Err(err_resp)
        }
    }
}
