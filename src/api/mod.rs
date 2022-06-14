pub use config::Config;
pub use server::Server;

pub mod auth;
pub mod config;
mod http_error;
pub mod response;
mod routes;
pub mod server;

pub use routes::unban::UnbanRequest;

pub const API_KEY_HEADER: &str = "X-API-KEY";
