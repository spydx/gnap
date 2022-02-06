use actix_web::{web, HttpResponse};
use dao::service::Service;
use errors::GnapError;
use log::trace;
use model::oidc::OpenIDConfiguration;

pub async fn openid_config(_service: web::Data<Service>) -> HttpResponse {
    trace!("openid_config");

    let issuer = "http://localhost:8000".to_owned();
    let authorization_endpoint = "http://localhost:8000/gnap/auth".to_owned();
    let token_endpoint = "http://localhost:8000/gnap/token".to_owned();
    let userinfo_endpoint = "http://localhost:8000/gnap/userinfo".to_owned();
    let jwks_uri = "http://localhost:8000/gnap/jwks".to_owned();

    let config: OpenIDConfiguration = OpenIDConfiguration::new(
        issuer,
        authorization_endpoint,
        token_endpoint,
        userinfo_endpoint,
        jwks_uri,
    );

    HttpResponse::Ok().json(config)
}

pub async fn gnap_config(service: web::Data<Service>) -> HttpResponse {
    let result = service.get_gnap_well_knowns().await;

    match result {
        Ok(response) => HttpResponse::Ok().json(&response),
        Err(err) => match err {
            GnapError::BadData => HttpResponse::BadRequest().body("Missing GNAP options"),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}
