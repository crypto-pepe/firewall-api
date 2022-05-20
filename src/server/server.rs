use std::sync::Arc;
use std::time::UNIX_EPOCH;

use actix_web::web::Data;
use actix_web::{dev, error, post, web, App, HttpResponse, HttpServer, ResponseError};
use anyhow::anyhow;
use mime;
use tokio::io;
use tracing_actix_web::TracingLogger;

use crate::ban_checker::redis::RedisBanChecker;
use crate::ban_checker::BanChecker;
use crate::http_error;
use crate::model::BanTargetRequest;
use crate::server::{response::*, Config};

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
        cfg.app_data(json_cfg).service(check_ban);
    })
}

#[tracing::instrument(skip(checker))]
#[post("/api/check-ban")]
async fn check_ban(
    ban_req: web::Json<BanTargetRequest>,
    checker: Data<RedisBanChecker>,
) -> Result<HttpResponse, impl ResponseError> {
    if let Err(e) = ban_req.verify() {
        return Err(e.into());
    }

    let target = ban_req.target.to_string();

    match checker.ban_ttl(target).await {
        Ok(o) => match o {
            None => Ok(BanStatus::Free.into()),
            Some(ttl) => {
                let expires_at = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(s) => s.as_secs() + ttl,
                    Err(e) => {
                        tracing::error!("{:?}", e);
                        return Err(http_error::ErrorResponse {
                            code: 500,
                            reason: e.to_string(),
                            details: None,
                        });
                    }
                };
                Ok(BanStatus::Banned(BannedBanStatus {
                    ban_expires_at: expires_at,
                })
                .into())
            }
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(http_error::ErrorResponse {
                code: 500,
                reason: e.to_string(),
                details: None,
            })
        }
    }
}
