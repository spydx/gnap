use crate::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/introspect")
            .route(web::post().to(handlers::resources::introspect_validate)),
    )
    .service(web::resource("/resource").route(web::post().to(handlers::resources::resource)));
}
