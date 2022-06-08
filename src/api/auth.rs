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
        let key = key.to_str().map_err(|_| ErrorResponse {
            code: 400,
            reason: format!("can't convert {} header to string", API_KEY_HEADER),
            details: None,
        })?;
        if self.key != key {
            return Err(ErrorResponse {
                code: 403,
                reason: format!("{} header is invalid", API_KEY_HEADER),
                details: None,
            });
        }
        Ok(())
    }
}
