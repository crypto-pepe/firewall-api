extern crate core;

use std::io;

use firewall_executor::ban_checker::redis::RedisBanChecker;
use firewall_executor::redis::get_pool;
use firewall_executor::server::Server;
use firewall_executor::{config, telemetry};

#[tokio::main]
async fn main() -> io::Result<()> {
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

    let ban_checker = match RedisBanChecker::new(redis_pool, cfg.redis.timeout_sec).await {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = Server::new(&cfg.server, ban_checker)?;
    srv.run().await
}
