use std::fmt::Display;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>, // field name -> description,
}

#[derive(Debug, PartialEq)]
pub enum BanTargetConversionError {
    FieldRequired,
}

impl Display for BanTargetConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("at least on field required: 'ip', 'user-agent'")
    }
}

impl From<BanTargetConversionError> for HttpResponse {
    fn from(v: BanTargetConversionError) -> Self {
        v.error_response()
    }
}

impl ResponseError for BanTargetConversionError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(ErrorResponse {
            code: 100,
            reason: "Provided request does not match the constraints".into(),
            details: Some(self.to_string()),
        })
    }
}
