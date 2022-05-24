use std::fmt::Display;
use std::time::UNIX_EPOCH;

use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::http_error::ErrorResponse;
use crate::api::response::*;
use crate::ban_checker::redis::RedisBanChecker;
use crate::ban_checker::BanChecker;
use crate::model::BanTarget;

#[derive(Debug, PartialEq)]
pub enum BanTargetConversionError {
    FieldRequired,
}

impl Display for BanTargetConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("at least on field required: 'ip', 'user_agent'")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTargetRequest {
    pub target: BanTarget,
}

impl BanTargetRequest {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        if self.target.ip.is_none() && self.target.user_agent.is_none() {
            return Err(BanTargetConversionError::FieldRequired);
        }
        Ok(())
    }
}

#[tracing::instrument(skip(checker))]
#[post("/api/check-ban")]
pub async fn check_ban(
    ban_req: web::Json<BanTargetRequest>,
    checker: Data<RedisBanChecker>,
) -> Result<HttpResponse, impl ResponseError> {
    if let Err(e) = ban_req.verify() {
        return Err(e.into());
    }

    let target = ban_req.target.to_string();

    match checker.ban_ttl(target).await {
        Ok(o) => match o {
            None => Ok(BanStatus::Free.into()),
            Some(ttl) => {
                let expires_at = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(s) => s.as_secs() + ttl,
                    Err(e) => {
                        tracing::error!("{:?}", e);
                        return Err(ErrorResponse {
                            code: 500,
                            reason: e.to_string(),
                            details: None,
                        });
                    }
                };
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
