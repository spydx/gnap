use crate::handlers;
use actix_web::{http, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/gnap")
            .service(
                web::scope("/tx")
                .service(
                web::resource("")
                            .route(web::post().to(handlers::transaction::grant_request))
                            .route(web::method(http::Method::OPTIONS).to(handlers::transaction::grant_options),
                    ),
                )
                .service(
                    web::resource("/{tx_id}")
                        .route(web::post().to(handlers::transaction::continue_request))
                )
            )
            .service(
        web::resource("/introspect")
                    .route(web::post().to(handlers::resources::introspect)),
            )
            .service(
                web::scope("/resource")
                .service(
            web::resource("")
                        .route(web::post().to(handlers::resources::register_resources_set)))
                .service(
                    web::resource("/new")
                        .route(web::post().to(handlers::resources::register_resource_server))
                )
            )
            .service(
        web::resource("/auth")
                    .route(web::get().to(handlers::auth::auth))
                    .route(web::post().to(handlers::auth::create)),
            )
            .service(
            web::scope("/token")
                .service(
                web::resource("/{token_id}")
                    .route(web::post().to(handlers::tokens::rotate_token))
                    .route(web::delete().to(handlers::tokens::revoke_token)),
                )
            ),
    );
}
