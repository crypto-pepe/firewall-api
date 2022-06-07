use crate::api::http_error::ErrorResponse;
use actix_web::HttpRequest;

pub struct ApiKeyChecker {
    pub header: String,
    pub key: String,
}

impl ApiKeyChecker {
    pub fn new(header: String, key: String) -> Self {
        ApiKeyChecker { key, header }
    }

    pub fn check(&self, req: &HttpRequest) -> Result<(), ErrorResponse> {
        let key = req
            .headers()
            .get(&self.header)
            .ok_or_else(|| ErrorResponse {
                code: 401,
                reason: format!("{} header is required", self.header),
                details: None,
            })?;
        self.key.eq(key).then(|| ()).ok_or_else(|| ErrorResponse {
            code: 403,
            reason: "auth header is invalid".into(),
            details: None,
        })
    }
}
