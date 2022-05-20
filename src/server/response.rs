use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "status")]
enum BanStatus {
    Free,
    Banned(BannedBanStatus),
}

pub struct BannedBanStatus {
    pub ban_expires_at: u64,
}

#[derive(Serialize)]
struct CheckBanResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BanStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_expires_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn response_free() -> HttpResponse {
    CheckBanResponse {
        status: Some(BanStatus::Free),
        ban_expires_at: None,
        error: None,
    }
        .response()
}

pub fn response_ban(expiration_time: u64) -> HttpResponse {
    CheckBanResponse {
        status: Some(BanStatus::Banned(BannedBanStatus { ban_expires_at: expiration_time })),
        ban_expires_at: Some(expiration_time),
        error: None,
    }
        .response()
}

pub fn response_error(error: String) -> HttpResponse {
    CheckBanResponse {
        status: None,
        ban_expires_at: None,
        error: Some(error),
    }
        .response()
}

impl CheckBanResponse {
    pub fn response(&self) -> HttpResponse {
        if self.error.is_none() {
            HttpResponse::Ok()
        } else {
            HttpResponse::InternalServerError()
        }
            .json(self)
    }
}
