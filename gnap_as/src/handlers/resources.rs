//! Transaction API Handlers

use actix_web::{web, HttpResponse};
use dao::resource_service::ResourceService;
use dao::service::Service;
use log::{error, trace};
use model::introspect::{InstrospectResponse, IntrospectRequest};
use model::resource::{GnapRegisterResourceServer, GnapResourceServer};
use mongodb::bson::doc;

/// HTTP POST  <as>/gnap/introspect
pub async fn introspect(
    _service: web::Data<Service>,
    _introrequest: web::Json<IntrospectRequest>,
) -> HttpResponse {
    let ir = InstrospectResponse {
        active: true,
        access: None,
        key: None,
    };
    HttpResponse::Ok().json(ir)
}

/// HTTP POST  <as>/gnap/resource
pub async fn register_resources_set(
    service: web::Data<ResourceService>,
    rs: web::Json<GnapResourceServer>,
) -> HttpResponse {

    let rs = rs.into_inner();
    match service.register_resources_set(rs).await { 
        Ok(_) => {
            trace!("Registered");
            HttpResponse::Ok().json(doc! { "status": "registered"})
        }
        Err(_) => {
            error!("Something went horribly wrong");
            HttpResponse::InternalServerError().json(doc! {"status": "failed"})
        }
    }
}

pub async fn register_resource_server(
    service: web::Data<ResourceService>,
    rs: web::Json<GnapRegisterResourceServer>,
) -> HttpResponse {
    let rs = rs.into_inner();
    match service.add_resource_server(rs).await {
        Ok(_) => {
            trace!("Created");
            HttpResponse::Ok().json(doc! { "status": "created"})
        }
        Err(_) => {
            error!("Something went horribly wrong");
            HttpResponse::InternalServerError().json(doc! {"status": "failed"})
        }
    }
}
