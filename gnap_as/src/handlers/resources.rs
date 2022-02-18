//! Transaction API Handlers

use actix_web::{web, HttpResponse};
use dao::resource_service::ResourceService;
use dao::service::Service;
// use log::{error, trace};
use model::resource::GnapResourceServer;
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
pub async fn register_resources_set(
    _service: web::Data<ResourceService>,
    _client: web::Json<GnapResourceServer>,
) -> HttpResponse {
    HttpResponse::Ok().json("{OK resource}")
}



