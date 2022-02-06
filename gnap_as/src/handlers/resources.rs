//! Transaction API Handlers

use actix_web::{web, HttpResponse};
use dao::service::Service;
// use log::{error, trace};
use model::client::GnapClientRequest;

pub async fn introspect_validate(
    _service: web::Data<Service>,
    _client: web::Json<GnapClientRequest>,
) -> HttpResponse {
    HttpResponse::Ok().json("{OK intro validate}")
}

pub async fn resource(
    _service: web::Data<Service>,
    _client: web::Json<GnapClientRequest>,
) -> HttpResponse {
    HttpResponse::Ok().json("{OK resource}")
}
