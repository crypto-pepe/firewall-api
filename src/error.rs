use bb8::RunError;
use redis::RedisError;
use std::fmt::Display;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckBanError {
    #[error(transparent)]
    Error(#[from] Redis),
}

#[derive(Error, Debug)]
pub enum Redis {
    #[error("key '{0}' not found")]
    KeyNotExist(String),

    #[error("key '{0}' has not ttl")]
    NoTTL(String),

    #[error("bad ttl")]
    BadTTL,

    #[error("execute '{1}': {0:?}")]
    Cmd(Arc<RedisError>, String),

    #[error("get connection: {0:?}")]
    GetConnection(Arc<RunError<RedisError>>),

    #[error("build pool: {0:?}")]
    BuildPool(Arc<RedisError>),

    #[error("create connection manager: {0:?}")]
    CreateConnManager(Arc<RedisError>),

    #[error("timeout")]
    Timeout,
}

#[derive(Debug)]
pub struct ExecutorError {
    pub executor_name: String,
    pub error_desc: String,
}

#[derive(Debug, PartialEq)]
pub enum BanTargetConversionError {
    EmptyRequest,
    TargetRequired,
    PatternUnsupported,
}

impl Display for BanTargetConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BanTargetConversionError::EmptyRequest => {
                "at least on field required: 'ip', 'user_agent'"
            }
            BanTargetConversionError::PatternUnsupported => "\"*\" is only allowed pattern",
            BanTargetConversionError::TargetRequired => "'target' field is required",
        })
    }
}
