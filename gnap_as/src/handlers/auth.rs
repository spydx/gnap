//! Transaction API Handlers

use actix_web::{web, HttpResponse,};
use actix_web::http::header::HeaderMap;
use dao::service::Service;
use log::trace;
use model::credentials::Credentials;
use errors::AuthError;

pub async fn auth(_service: web::Data<Service>, request: web::HttpRequest) -> HttpResponse {
    trace!("Auth");
    let _login = basic_authentication(request.headers());
    
    match _login {
        Ok(v) => {
            println!("{:#?}", v);
            HttpResponse::Ok().json("ok")
        },
        Err(_) => HttpResponse::BadRequest().body("ups"),
    }
}


fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, AuthError> {
    let header_values = headers
        .get("Authorization");

    if header_values.is_some() {
        let base64encoded = header_values
            .unwrap().to_str()
            .expect("Failed to get headervalue")
            .strip_prefix("Basic ").unwrap();

        let decoded_bytes = base64::decode_config(base64encoded, base64::STANDARD).expect("Failed to decode");
        let credentials= String::from_utf8(decoded_bytes).expect("Failed to turn bytes to string for credentials");
        let mut cred = credentials.splitn(2,':');
        let username = cred
            .next()
            .ok_or_else(|| AuthError::UserNameError)?
            .to_string();
        let password = cred
            .next()
            .ok_or_else(|| AuthError::PasswordError)?
            .to_string();
        Ok(Credentials { username: username, password: password})
    } else {
        Err(AuthError::BasicFailed)
    }
}
