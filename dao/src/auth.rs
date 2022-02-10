use argon2::{Argon2, PasswordHasher};
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use uuid::Uuid;

use argon2::{Algorithm, Version};
use errors::AuthError;
use log::{debug, trace};
use model::credentials::Credentials;
use rand;
use secrecy::Secret;
pub struct AuthDb {
    pub client: Client,
    pub database: Database,
}

impl AuthDb {
    pub async fn new() -> Self {
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI missing");
        let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
        let app_name = env::var("MONGODB_APP_NAME").expect("MONGODB_APP_NAME missing");

        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to create client options");
        client_options.app_name = Some(app_name);

        let client = Client::with_options(client_options).expect("Failed to create client");

        let db = client.database(&database);
        Self {
            client: client,
            database: db,
        }
    }

    pub async fn validate_account(
        &self,
        credentials: Credentials,
    ) -> Result<Option<bool>, AuthError> {
        todo!()
    }

    pub async fn create_account(
        &self,
        credentials: Credentials,
    ) -> Result<Option<bool>, AuthError> {
        let collection = self.database.collection::<Users>("users");
        todo!()
    }
}

fn compute_hash(password: Secret<String>) -> Result<Secret<String>, AuthError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(1500, 2, None).unwrap(),
        )
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .to_string();
    Ok(Secret::new(hash))
}
