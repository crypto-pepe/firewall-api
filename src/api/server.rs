use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{dev, error, web, App, HttpResponse, HttpServer};
use anyhow::anyhow;
use mime;
use tokio::io;
use tracing_actix_web::TracingLogger;

use crate::api::{routes, Config};
use crate::ban_checker::redis::RedisBanChecker;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(cfg: &Config, bc: RedisBanChecker) -> Result<Server, io::Error> {
        let bh = Data::from(Arc::new(bc));

        let srv = HttpServer::new(move || {
            App::new()
                .app_data(bh.clone())
                .configure(server_config())
                .wrap(TracingLogger::default())
        });

        let srv = srv.bind((cfg.host.clone(), cfg.port))?.run();
        Ok(Server { srv })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        self.srv.await.map_err(|e| anyhow!(e))
    }
}

fn server_config() -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg| {
        let json_cfg = web::JsonConfig::default()
            .content_type(|mime| mime == mime::APPLICATION_JSON)
            .error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
            });
        cfg.app_data(json_cfg).service(routes::check_ban);
    })
}
