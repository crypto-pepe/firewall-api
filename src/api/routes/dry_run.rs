use actix_web::web::Data;
use actix_web::{patch, web, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::executor::Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct DryRunModeRequest {
    pub enabled: bool,
}

#[tracing::instrument(skip(client))]
#[patch("/api/dry-run-mode")]
pub async fn dry_run_mode(
    req: web::Json<DryRunModeRequest>,
    client: Data<Pool>,
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
