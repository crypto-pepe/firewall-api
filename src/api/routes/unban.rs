use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::error::BanTargetConversionError;
use crate::model::UnBanEntity;
use crate::unbanner::UnBanner;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnBanRequest {
    pub target: Option<UnBanEntity>,
}

impl UnBanRequest {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        if self.target.is_none() {
            Err(BanTargetConversionError::TargetRequired)
        } else {
            self.target.as_ref().unwrap().verify()
        }
    }
}

#[tracing::instrument(skip(unban))]
#[post("/api/unban")]
pub async fn process_unban(
    unban_req: web::Json<UnBanRequest>,
    unban: Data<Box<dyn UnBanner + Sync + Send>>,
) -> Result<impl Responder, impl ResponseError> {
    if let Err(e) = unban_req.verify() {
        return Err(e.into());
    }

    match unban.unban(unban_req.0).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            let err_resp: ErrorResponse = e.into();
            Err(err_resp)
        }
    }
}
