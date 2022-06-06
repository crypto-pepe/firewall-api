use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::error::{BanTargetConversionError, ExecutorError};
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub(crate) code: u16,
    pub(crate) reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) details: Option<HashMap<String, String>>, // field name -> description,
}

impl From<BanTargetConversionError> for ErrorResponse {
    fn from(btce: BanTargetConversionError) -> Self {
        ErrorResponse {
            code: 400,
            reason: "Provided request does not match the constraints".into(),
            details: Some(HashMap::from([("target".into(), btce.to_string())])),
        }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*serde_json::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<Vec<ExecutorError>> for ErrorResponse {
    fn from(uss: Vec<ExecutorError>) -> Self {
        let desc = HashMap::from_iter(
            uss.iter()
                .map(|us| (us.executor_name.clone(), us.error_desc.clone())),
        );
        ErrorResponse {
            code: 500,
            reason: "Some executors didn't response with success".to_string(),
            details: Some(desc),
        }
    }
}
