//! Token API Handlers

use actix_web::{web, HttpResponse};
use dao::token_service::TokenService;
use log::{debug, trace};
use model::tokens::Token;
use mongodb::bson::doc;

pub async fn revoke_token(
    service: web::Data<TokenService>,
    token_id: web::Path<String>,
) -> HttpResponse {
    debug!("revoke token");
    let token_id = token_id.into_inner();
    let token = Token::from_string(token_id);
    match service.revoke_token(&token).await {
        Ok(_) => {
            trace!("Succesfully revoked");
            HttpResponse::Ok().json(doc! { "status": "revoked" })
        }
        Err(_) => {
            trace!("Can't find the token");
            HttpResponse::NoContent().json(doc! { "status": "error" })
        }
    }
}

pub async fn rotate_token(
    _service: web::Data<TokenService>,
    token_id: web::Path<String>,
) -> HttpResponse {
    trace!("rotate token");
    HttpResponse::Ok().json(token_id.into_inner())
}
