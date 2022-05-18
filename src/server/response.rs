use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum CheckBanResponse {
    Free(BanResponseFree),
    Ban(BanResponseBan),
    Error(BanResponseError),
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BanResponseError {
    Error(String),
}

#[derive(Serialize)]
#[serde(tag = "status", content = "ban_expires_at")]
pub enum BanResponseBan {
    #[serde(rename = "banned")]
    Ban(u64),
}

#[derive(Serialize)]
#[serde(tag = "status")]
#[serde(rename_all = "kebab-case")]
pub enum BanResponseFree {
    Free,
}

impl CheckBanResponse {
    pub fn response(&self) -> HttpResponse {
        match self {
            CheckBanResponse::Free(_) => HttpResponse::Ok(),
            CheckBanResponse::Ban(_) => HttpResponse::Ok(),
            CheckBanResponse::Error(_) => HttpResponse::InternalServerError(),
        }
            .json(self)
    }
}
