use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, ResponseError};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::api::response::*;
use crate::ban_checker::BanChecker;
use crate::error::BanTargetConversionError;

use crate::model::BanTarget;

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTargetRequest {
    pub target: BanTarget,
}

impl BanTargetRequest {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        if self.target.ip.is_none() && self.target.user_agent.is_none() {
            return Err(BanTargetConversionError::IPOrUserAgentRequired);
        }
        Ok(())
    }
}

#[tracing::instrument(skip(checker))]
#[post("/api/check-ban")]
pub async fn check_ban(
    ban_req: web::Json<BanTargetRequest>,
    checker: Data<Box<dyn BanChecker + Sync + Send>>,
) -> Result<HttpResponse, impl ResponseError> {
    if let Err(e) = ban_req.verify() {
        return Err(e.into());
    }

    let target = ban_req.target.to_string();

    match checker.ban_ttl(target).await {
        Ok(o) => match o {
            None => Ok(BanStatus::Free.into()),
            Some(ttl) => {
                let expires_at = Utc::now().timestamp() as u64 + ttl;
                Ok(BanStatus::Banned(BannedBanStatus {
                    ban_expires_at: expires_at,
                })
                .into())
            }
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(ErrorResponse {
                code: 500,
                reason: e.to_string(),
                details: None,
            })
        }
    }
}
