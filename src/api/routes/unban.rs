use actix_web::web::Data;
use actix_web::{delete, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::error::BanTargetConversionError;
use crate::executor::Client;
use crate::model::UnBanEntity;

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

#[tracing::instrument(skip(client))]
#[delete("/api/bans")]
pub async fn process_unban(
    unban_req: web::Json<UnBanRequest>,
    client: Data<Client>,
) -> Result<impl Responder, impl ResponseError> {
    if let Err(e) = unban_req.verify() {
        return Err(e.into());
    }

    match client.unban(unban_req.0).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            let err_resp: ErrorResponse = e.into();
            Err(err_resp)
        }
    }
}
