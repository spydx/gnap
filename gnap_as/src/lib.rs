use actix_web::web;
use dao::auth_service::AuthService;
use dao::resource_service::ResourceService;
use dao::service::Service;
use dao::token_service::TokenService;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::env;
use std::net::SocketAddr;

mod utils;

/// Set up shared App state
///
/// Creates DB and Cache instances to be added to Actix App
pub async fn app_state() -> web::Data<Service> {
    // Init the database and cache services
    let dao_service = Service::create().await;

    // App::app_data will wrap the app state in an Arc, so it is sharable
    web::Data::new(dao_service)
}

pub async fn auth_state() -> web::Data<AuthService> {
    let authservice = AuthService::create().await;
    web::Data::new(authservice)
}

pub async fn token_state() -> web::Data<TokenService> {
    let tokenservice = TokenService::create().await;
    web::Data::new(tokenservice)
    
}

pub async fn rs_state() -> web::Data<ResourceService> {
    let rs_service = ResourceService::create().await;
    web::Data::new(rs_service)
    
}

/// Get addresses from ENV
///
/// This doesn't really havea ny value.  But fun to play with. We could just
/// as easily pass the string from env::var into the HttpServer.bind func.
pub fn get_ip_addresses() -> (SocketAddr, SocketAddr, String) {
    let api_address: SocketAddr = env::var("API_ADDRESS")
        .expect("API_ADDRESS is not set in env")
        .parse()
        .expect("API_ADDRESS is invalid");

    let tls_address: SocketAddr = env::var("TLS_ADDRESS")
        .expect("TLS_ADDRESS is not set in env")
        .parse()
        .expect("TLS_ADDRESS is invalid");

    // Get the local IP address of the non-loopback interface. This is just for
    // displaying at startup.
    let ip = utils::get_machine_ip();

    (api_address, tls_address, ip)
}

/*

To create a self-signed temporary cert for testing, copy&paste the following:

    openssl req -x509 \
    -newkey rsa:4096 \
    -keyout .keystore/key.pem \
    -out .keystore/cert.pem \
    -sha256 \
    -days 3650 \
    -noenc \
    -subj '/CN=localhost' \
    -addext "basicConstraints = critical, CA:true" \
    -addext "keyUsage = critical, Digital Signature, Certificate Sign" \
    -addext "subjectKeyIdentifier=hash"

*/

/*
        openssl req -x509 \
    -newkey rsa:4096 \
    -keyout .keystore/key.pem \
    -out .keystore/cert.pem \
    -sha256 \
    -days 3650 \
    -subj '/CN=localhost'
*/

/// SSL builder for HttpServer
pub fn tls_builder() -> SslAcceptorBuilder {
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(".keystore/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(".keystore/cert.pem")
        .unwrap();
    builder
}
