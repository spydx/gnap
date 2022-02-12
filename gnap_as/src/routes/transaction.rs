use crate::handlers;
use actix_web::{http, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/gnap")
            .service(
                web::resource("/tx")
                    .route(web::post().to(handlers::transaction::grant_request))
                    .route(
                        web::method(http::Method::OPTIONS).to(handlers::transaction::grant_options),
                    ),
            )
            .service(
                web::resource("/introspect").route(web::post().to(handlers::resources::introspect)),
            )
            .service(
                web::resource("/resource").route(web::post().to(handlers::resources::resource)),
            )
            .service(
                web::resource("/auth")
                    .route(web::get().to(handlers::auth::auth))
                    .route(web::post().to(handlers::auth::create)),
            ),
    );
}
