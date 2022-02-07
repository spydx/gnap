//! Token API Handlers

use actix_web::{web, HttpResponse};
use dao::service::Service;
use log::trace;
pub async fn revoke_token(_service: web::Data<Service>, id: web::Path<String>) -> HttpResponse {
    trace!("revoke token");

    HttpResponse::Ok().json(id.into_inner())
}

pub async fn rotate_token(_service: web::Data<Service>, id: web::Path<String>) -> HttpResponse {
    trace!("rotate token");
    HttpResponse::Ok().json(id.into_inner())
}
