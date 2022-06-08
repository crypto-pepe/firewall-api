use crate::api::http_error::ErrorResponse;
use crate::api::API_KEY_HEADER;
use actix_web::HttpRequest;

pub struct ApiKeyChecker {
    pub key: String,
}

impl ApiKeyChecker {
    pub fn new(key: String) -> Self {
        ApiKeyChecker { key }
    }

    pub fn check(&self, req: &HttpRequest) -> Result<(), ErrorResponse> {
        let key = req.headers().get(API_KEY_HEADER).ok_or(ErrorResponse {
            code: 401,
            reason: format!("{} header is required", API_KEY_HEADER),
            details: None,
        })?;
        self.key.eq(key).then(|| ()).ok_or(ErrorResponse {
            code: 403,
            reason: format!("{} header is invalid", API_KEY_HEADER),
            details: None,
        })
    }
}
