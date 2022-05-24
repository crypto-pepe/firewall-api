use firewall_api::api::Server;
use firewall_api::ban_checker::redis::RedisBanChecker;
use firewall_api::redis::get_pool;
use firewall_api::{config, telemetry};

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

    let ban_checker = match RedisBanChecker::new(
        redis_pool,
        cfg.redis_query_timeout_secs,
        cfg.redis_keys_prefix.clone(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = Server::new(&cfg.server, ban_checker)?;
    srv.run().await
}
