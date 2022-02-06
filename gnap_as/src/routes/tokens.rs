use crate::handlers;
use actix_web::{web};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tokens")
            .service(web::resource("/{id}").route(web::post().to(handlers::tokens::rotate_token)))
            .service(web::resource("/{id}").route(web::delete().to(handlers::tokens::revoke_token))),
    );
}

