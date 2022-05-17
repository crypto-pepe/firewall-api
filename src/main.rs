extern crate core;

use std::io;

use firewall_executor::ban_checker::redis::RedisBanChecker;
use firewall_executor::config;
use firewall_executor::redis::get_pool;
use firewall_executor::server::Server;
use pepe_log::info;

#[tokio::main]
async fn main() -> io::Result<()> {
    info!("start application");

    let cfg = match config::Config::load() {
        Ok(a) => a,
        Err(e) => panic!("can't read config {:?}", e),
    };

    info!("config loaded"; "config" => &cfg);

    let redis_pool = match get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let redis_svc = match RedisBanChecker::new(redis_pool, cfg.redis.timeout_sec).await {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = Server::new(&cfg.server, redis_svc)?;
    srv.run().await
}
