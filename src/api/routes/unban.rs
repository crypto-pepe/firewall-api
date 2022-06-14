use actix_web::web::Data;
use actix_web::{delete, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::error::BanTargetConversionError;
use crate::executor::Pool;
use crate::model::UnbanEntity;
use crate::ApiKeyChecker;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnbanRequest {
    pub target: Option<UnbanEntity>,
}

impl UnbanRequest {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        match self.target.as_ref() {
            Some(ube) => ube.verify(),
            None => Err(BanTargetConversionError::TargetRequired),
        }
    }
}

#[tracing::instrument(skip(req, client, api_key_checker))]
#[delete("/api/bans")]
pub async fn process_unban(
    req: HttpRequest,
    api_key_checker: Data<ApiKeyChecker>,
    unban_req: web::Json<UnbanRequest>,
    client: Data<Pool>,
) -> Result<impl Responder, ErrorResponse> {
    api_key_checker.check(&req)?;

    if let Err(e) = unban_req.verify() {
        return Err(e.into());
    }

    match client.unban(unban_req.0).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            Err(e.into())
        }
    }
}
