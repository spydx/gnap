use crate::handlers;
use actix_web::web;

// this function could be located in different module
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/db")
            .service(web::resource("/client/{id}").route(web::get().to(handlers::db::get_client)))
            .service(web::resource("/client").route(web::put().to(handlers::db::add_client))),
    );
}
