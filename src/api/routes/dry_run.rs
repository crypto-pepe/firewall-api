use actix_web::web::Data;
use actix_web::{patch, web, HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::executor::Pool;
use crate::ApiKeyChecker;

#[derive(Debug, Serialize, Deserialize)]
pub struct DryRunModeRequest {
    pub enabled: bool,
}

#[tracing::instrument(skip(req, api_key_checker, client))]
#[patch("/api/dry-run-mode")]
pub async fn dry_run_mode(
    req: HttpRequest,
    api_key_checker: Data<ApiKeyChecker>,
    dry_run_req: web::Json<DryRunModeRequest>,
    client: Data<Pool>,
) -> Result<HttpResponse, impl ResponseError> {
    api_key_checker.check(&req)?;
    match client.enable_dry_run_mode(dry_run_req.enabled).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("enable dry run mode: {:?}", e);
            let err_resp: ErrorResponse = e.into();
            Err(err_resp)
        }
    }
}
