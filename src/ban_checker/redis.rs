use std::sync::Arc;
use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use num_traits::ToPrimitive;
use redis::AsyncCommands;

use crate::ban_checker::BanChecker;
use crate::errors;
use crate::errors::CheckBanError;

#[async_trait]
impl BanChecker for RedisBanChecker {
    async fn ban_ttl(&self, bt: String) -> Result<Option<u64>, CheckBanError> {
        return match self.get_ttl(bt).await {
            Ok(ttl) => Ok(ttl),
            Err(e) => match e {
                errors::Redis::KeyNotExist(_) => Ok(None),
                _ => Err(CheckBanError::Error(e)),
            },
        };
    }
}

pub struct RedisBanChecker {
    pub pool: Pool<RedisConnectionManager>,
    pub timeout: time::Duration,
}

impl RedisBanChecker {
    pub async fn new(
        pool: Pool<RedisConnectionManager>,
        timeout_secs: u64,
    ) -> Result<Self, errors::Redis> {
        let timeout = time::Duration::from_secs(timeout_secs);
        Ok(RedisBanChecker { pool, timeout })
    }
    pub async fn get_ttl(&self, key: String) -> Result<Option<u64>, errors::Redis> {
        tokio::time::timeout(self.timeout, self._get_ttl(key))
            .await
            .map_err(|_| errors::Redis::Timeout)?
    }

    async fn _get_ttl<'a>(&self, key: String) -> Result<Option<u64>, errors::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(errors::Redis::GetConnection(Arc::new(e)));
            }
        };

        let ttl: i128 = conn
            .ttl(&key)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "TTL".to_string()))?;
        match ttl {
            -2 => Err(errors::Redis::KeyNotExist(key)),
            -1 => Err(errors::Redis::NoTTL(key)),
            _ => Ok(Some(ttl.to_u64().unwrap())),
        }
    }
}
