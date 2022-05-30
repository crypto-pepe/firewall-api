use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{dev, error, web, App, HttpServer, ResponseError};
use anyhow::anyhow;
use mime;
use tokio::io;
use tracing_actix_web::TracingLogger;

use crate::api::http_error::ErrorResponse;
use crate::api::{routes, Config};
use crate::ban_checker::BanChecker;
use crate::unbanner::UnBanner;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(
        cfg: &Config,
        banchecker_svc: Box<dyn BanChecker + Sync + Send>,
        unban_svc: Box<dyn UnBanner + Sync + Send>,
    ) -> Result<Server, io::Error> {
        let bc = Data::from(Arc::new(banchecker_svc));
        let ub = Data::from(Arc::new(unban_svc));

        let srv = HttpServer::new(move || {
            App::new()
                .app_data(bc.clone())
                .app_data(ub.clone())
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
                let reason = err.to_string();
                error::InternalError::from_response(
                    err,
                    ErrorResponse {
                        code: 400,
                        reason,
                        details: None,
                    }
                    .error_response(),
                )
                .into()
            });
        cfg.app_data(json_cfg)
            .service(routes::check_ban)
            .service(routes::process_unban);
    })
}
