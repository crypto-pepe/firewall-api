use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{dev, error, post, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use mime;
use tokio::io;
use tracing_actix_web::TracingLogger;

use crate::ban_checker::redis::RedisBanChecker;
use crate::ban_checker::BanChecker;
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

    pub async fn run(self) -> io::Result<()> {
        self.srv.await
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
) -> impl Responder {
    if let Err(e) = ban_req.verify() {
        return e.error_response();
    }

    let target = ban_req.target.to_string();

    match checker.ban_ttl(target).await {
        Ok(o) => match o {
            None => response_free(),
            Some(ttl) => response_ban(ttl),
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            response_error(e.to_string())
        }
    }
}
