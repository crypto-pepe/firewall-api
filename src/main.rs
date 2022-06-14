mod api;
mod ban_checker;
mod config;
mod error;
mod executor;
mod model;
mod redis;
mod telemetry;

use crate::api::auth::ApiKeyChecker;
use crate::redis::get_pool;
use api::Server;
use ban_checker::redis::RedisBanChecker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("start application");

    let cfg = config::Config::load()?;

    tracing::info!("config loaded; config={:?}", &cfg);

    let subscriber = telemetry::get_subscriber(&cfg.telemetry);
    telemetry::init_subscriber(subscriber);

    let redis_pool = get_pool(&cfg.redis).await?;

    let redis_query_timeout: std::time::Duration = cfg.redis_query_timeout.into();
    let ban_checker = RedisBanChecker::new(
        redis_pool,
        redis_query_timeout,
        cfg.redis_keys_prefix.clone(),
    );

    let api_key_checker = ApiKeyChecker::new(cfg.api_key);
    let executor_client = executor::Pool::new(cfg.executors.clone());
    let srv = Server::new(
        &cfg.server,
        Box::new(ban_checker),
        executor_client,
        api_key_checker,
    )?;
    srv.run().await
}
