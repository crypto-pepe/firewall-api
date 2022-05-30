mod api;
mod ban_checker;
mod config;
mod error;
mod model;
mod redis;
mod telemetry;
mod unbanner;

use crate::redis::get_pool;
use api::Server;
use ban_checker::redis::RedisBanChecker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("start application");

    let cfg = match config::Config::load() {
        Ok(a) => a,
        Err(e) => panic!("can't read config {:?}", e),
    };

    tracing::info!("config loaded; config={:?}", &cfg);

    let subscriber = telemetry::get_subscriber(&cfg.telemetry);
    telemetry::init_subscriber(subscriber);

    let redis_pool = match get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let redis_query_timeout: std::time::Duration = cfg.redis_query_timeout.into();
    let ban_checker =
        match RedisBanChecker::new(redis_pool, redis_query_timeout, cfg.redis_keys_prefix.clone()).await {
            Ok(r) => r,
            Err(e) => panic!("can't setup redis {:?}", e),
        };

    let unban_svc = unbanner::Service::new(cfg.executors);

    let srv = Server::new(&cfg.server, Box::new(ban_checker), Box::new(unban_svc))?;
    srv.run().await
}
