//! Transaction API Handlers

use actix_web::http::header::HeaderMap;
use actix_web::{HttpRequest, HttpResponse, web};
use dao::auth_service::AuthService;
use errors::AuthError;
use log::trace;
use model::credentials::Credentials;
use model::instances::{InstanceRequest, InstanceResponse};

// TODO:
// GET <as>/gnap/auth/:instance:
pub async fn auth(
    service: web::Data<AuthService>,
    request: HttpRequest,
    instance: web::Path<String>,
) -> HttpResponse {
    trace!("Auth");
    let login = basic_authentication(request.headers());
    match login {
        Ok(credentials) => {
            let instance = InstanceRequest::create(instance.into_inner());
            match service
                .validate_account(credentials, instance)
                .await
            {
                Ok(b) => {
                    let body = InstanceResponse::create(b);
                    HttpResponse::Ok().json(body)
                }
                Err(_) => {
                    let json = InstanceResponse::create(false);
                    HttpResponse::Unauthorized().json(json)
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid data"),
    }
}

// TODO:
// POST <as>/gnap/auth
pub async fn create(service: web::Data<AuthService>, request: HttpRequest) -> HttpResponse {
    trace!("User create");
    let account = basic_authentication(request.headers());

    match account {
        Ok(user) => match service.create_account(user).await {
            Ok(b) => {
                if b.unwrap() {
                    trace!("Created status {:?}", b);
                    HttpResponse::Ok().json("ok")
                } else {
                    trace!("Created status {:?}", b);
                    HttpResponse::InternalServerError().body("failed to create user")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("failed to create user"),
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid data"),
    }
}

fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, AuthError> {
    let header_values = headers.get("Authorization");

    if let Some(header_values) = header_values {
        let base64encoded = header_values
            .to_str()
            .expect("Failed to get headervalue")
            .strip_prefix("Basic ")
            .unwrap();

        let decoded_bytes =
            base64::decode_config(base64encoded, base64::STANDARD).expect("Failed to decode");
        let credentials = String::from_utf8(decoded_bytes)
            .expect("Failed to turn bytes to string for credentials");
        let mut cred = credentials.splitn(2, ':');
        let username = cred
            .next()
            .ok_or(AuthError::UserNameError)?
            .to_string();
        let password = cred
            .next()
            .ok_or(AuthError::PasswordError)?
            .to_string();
        Ok(Credentials {
            username,
            password,
        })
    } else {
        Err(AuthError::BasicFailed)
    }
}
