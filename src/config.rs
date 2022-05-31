use pepe_config::{ConfigError, FileFormat};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::telemetry;
use crate::{api, executor};

pub const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub redis: pepe_config::redis::Config,
    pub server: api::Config,
    pub telemetry: telemetry::Config,
    pub redis_keys_prefix: String,
    #[serde(default = "default_redis_query_timeout")]
    pub redis_query_timeout: duration_string::DurationString,
    pub executors: Vec<executor::Config>,
}

fn default_redis_query_timeout() -> duration_string::DurationString {
    duration_string::DurationString::new(Duration::from_secs(5))
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        pepe_config::load(DEFAULT_CONFIG, FileFormat::Yaml)
    }
}
