use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "status")]
pub enum BanStatus {
    Free,
    Banned(BannedBanStatus),
}

impl Into<HttpResponse> for BanStatus {
    fn into(self) -> HttpResponse {
        HttpResponse::build(StatusCode::OK).json(self)
    }
}

#[derive(Serialize)]
pub struct BannedBanStatus {
    pub ban_expires_at: u64,
}
