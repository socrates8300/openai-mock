use actix_web::web;
use crate::handlers::completions_handler;

pub fn configure_completion_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/completions", web::post().to(completions_handler)),
    );
}