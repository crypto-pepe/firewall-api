use actix_web::{post, HttpResponse, Responder, ResponseError, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::api::http_error;
use crate::api::http_error::ErrorResponse;
use crate::model::UnBanEntity;
use crate::unban::UnBanner;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnBanRequest {
    pub target: UnBanEntity,
}

#[tracing::instrument(skip(unban))]
#[post("/api/unban")]
pub async fn process_unban(
    unban_req: web::Json<UnBanRequest>,
    unban: Data<Box<dyn UnBanner + Sync + Send>>,
) -> Result<impl Responder, impl ResponseError> {
    match unban.unban(unban_req.0.target).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            let err_resp: ErrorResponse = e.into();
            Err(err_resp)
        }
    }
}
