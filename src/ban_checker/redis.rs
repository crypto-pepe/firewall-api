use std::sync::Arc;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use num_traits::ToPrimitive;
use redis::AsyncCommands;
use std::time::Duration;

use crate::ban_checker::BanChecker;
use crate::error;
use crate::error::CheckBanError;

pub struct RedisBanChecker {
    pub pool: Pool<RedisConnectionManager>,
    pub timeout: Duration,
    pub key_prefix: String,
}

#[async_trait]
impl BanChecker for RedisBanChecker {
    #[tracing::instrument(skip(self))]
    async fn ban_ttl(&self, bt: String) -> Result<Option<u64>, CheckBanError> {
        return match self.get_ttl(bt).await {
            Ok(ttl) => Ok(ttl),
            Err(e) => match e {
                error::Redis::KeyNotExist(_) => Ok(None),
                _ => Err(CheckBanError::Error(e)),
            },
        };
    }
}

impl RedisBanChecker {
    pub fn new(pool: Pool<RedisConnectionManager>, timeout: Duration, key_prefix: String) -> Self {
        Self {
            pool,
            timeout,
            key_prefix,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_ttl(&self, key: String) -> Result<Option<u64>, error::Redis> {
        tokio::time::timeout(
            self.timeout,
            self._get_ttl(format!("{}{}", self.key_prefix, key)),
        )
        .await
        .map_err(|_| error::Redis::Timeout)?
    }

    #[tracing::instrument(skip(self))]
    async fn _get_ttl<'a>(&self, key: String) -> Result<Option<u64>, error::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(error::Redis::GetConnection(Arc::new(e)));
            }
        };

        let ttl: i128 = conn
            .ttl(&key)
            .await
            .map_err(|re| error::Redis::Cmd(Arc::new(re), "TTL".to_string()))?;
        match ttl {
            -2 => Err(error::Redis::KeyNotExist(key)),
            -1 => Err(error::Redis::NoTTL(key)),
            _ => match ttl.to_u64() {
                Some(ttl) => Ok(Some(ttl)),
                None => Err(error::Redis::BadTTL),
            },
        }
    }
}
