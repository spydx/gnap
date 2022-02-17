//! Transaction API Handlers

use actix_web::{web, HttpResponse};
use dao::service::Service;
// use log::{error, trace};
use model::client::GnapClientRequest;
use model::introspect::{IntrospectRequest, InstrospectResponse};



/// HTTP POST  <as>/gnap/introspect
pub async fn introspect(
    _service: web::Data<Service>,
    _introrequest: web::Json<IntrospectRequest>,
) -> HttpResponse {

    let ir = InstrospectResponse {
        active: true,
        access: None,
        key: None
    };
    HttpResponse::Ok().json(ir)
}

/// HTTP POST  <as>/gnap/resource
pub async fn resource(
    _service: web::Data<Service>,
    _client: web::Json<GnapClientRequest>,
) -> HttpResponse {
    HttpResponse::Ok().json("{OK resource}")
}
