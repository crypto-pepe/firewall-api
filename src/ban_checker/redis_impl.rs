use std::sync::Arc;

use async_trait::async_trait;
use num_traits::ToPrimitive;
use redis::AsyncCommands;

use crate::ban_checker::BanChecker;
use crate::errors;
use crate::errors::CheckBanError;
use crate::redis::Service;

#[async_trait]
impl BanChecker for Service {
    async fn ban_ttl(&self, bt: String) -> Result<Option<u64>, CheckBanError> {
        return match get_ttl(self, bt).await {
            Ok(ttl) => Ok(ttl),
            Err(e) => match e {
                errors::Redis::KeyNotExist(_) => Ok(None),
                _ => Err(CheckBanError::Error(e)),
            },
        };
    }
}

async fn get_ttl(redis: &Service, key: String) -> Result<Option<u64>, errors::Redis> {
    tokio::time::timeout(redis.timeout, _get_ttl(redis, key))
        .await
        .map_err(|_| errors::Redis::Timeout)?
}

async fn _get_ttl<'a>(redis: &Service, key: String) -> Result<Option<u64>, errors::Redis> {
    let pool = redis.pool.clone();

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
