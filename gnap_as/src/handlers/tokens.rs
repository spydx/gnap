//! Token API Handlers

use actix_web::{web, HttpResponse};
use dao::tokenservice::TokenService;
use log::trace;
use mongodb::bson::doc;
use model::tokens::Token;

pub async fn revoke_token(
    service: web::Data<TokenService>,
    token: web::Path<String>,
) -> HttpResponse {
    trace!("revoke token");
    let token_id = token.into_inner();
    let token = Token::from_string(token_id);
    match service.revoke_token(&token).await {
        Ok(_) => {
            trace!("Succesfully revoked");
            HttpResponse::Ok().json("")
        }
        Err(_) => {
            trace!("Can't find the token");
            HttpResponse::NoContent().json(doc! { "error": "unkown token" })
        }
    }
}

pub async fn rotate_token(
    _service: web::Data<TokenService>,
    id: web::Path<String>,
) -> HttpResponse {
    trace!("rotate token");
    HttpResponse::Ok().json(id.into_inner())
}
