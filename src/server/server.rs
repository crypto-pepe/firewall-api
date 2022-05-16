use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{dev, error, post, web, App, HttpResponse, HttpServer, Responder};
use mime;
use pepe_log::error;
use serde_json::json;
use tokio::io;

use crate::ban_checker::BanChecker;
use crate::model::{target_to_key, BanTargetRequest};
use crate::redis::Service;
use crate::server::Config;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(cfg: &Config, bh: Service) -> Result<Server, io::Error> {
        let bh = Data::from(Arc::new(bh));

        let srv =
            HttpServer::new(move || App::new().app_data(bh.clone()).configure(server_config()));

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

enum CheckBanResponse {
    Free,
    Ban(u64),
    Error,
}

impl CheckBanResponse {
    fn response(self) -> HttpResponse {
        match self {
            CheckBanResponse::Free => HttpResponse::Ok().json(json!({"status":"free"})),
            CheckBanResponse::Ban(ttl) => {
                HttpResponse::Ok().json(json!({"status":"banned", "ban_expires_at":ttl}))
            }
            CheckBanResponse::Error => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[post("/api/check-ban")]
async fn check_ban(ban_req: web::Json<BanTargetRequest>, checker: Data<Service>) -> impl Responder {
    let target = match target_to_key(&ban_req.target) {
        Ok(t) => t,
        Err(e) => return e.into(),
    };

    match checker.ban_ttl(target).await {
        Ok(o) => match o {
            None => CheckBanResponse::Free.response(),
            Some(ttl) => CheckBanResponse::Ban(ttl).response(),
        },
        Err(e) => {
            error!("{:?}", e);
            CheckBanResponse::Error.response()
        }
    }
}
