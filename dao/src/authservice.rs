use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};

use log::trace;
use model::credentials::Credentials;
use rand;
use errors::AuthError;
use model::users::User;
use super::auth::AuthDb;
use super::cache::GnapCache;

pub struct AuthService {
    pub db_client: AuthDb,
    pub cache_client: GnapCache
}

impl AuthService {
    pub async fn create() -> Self {
        let db = AuthDb::new().await;
        let cache_client = GnapCache::new().await;
        Self {
            cache_client: cache_client,
            db_client: db,
        }
    }

    pub async fn validate_account(
        &self,
        credentials: Credentials,
    ) -> Result<bool, AuthError> {

        trace!("Fetching User from database");
        let result = self.db_client.fetch_account(credentials.username).await?;
        if result.is_some() {
            match validate_password(result.unwrap().password, credentials.password) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub async fn create_account(
        &self,
        credentials: Credentials,
    ) -> Result<Option<bool>, AuthError> {
        let password_hash = compute_hash(credentials.password).expect("Failed to hash password");
        let id = User::create_id().to_string();
        let user = User {
            id: id,
            username: credentials.username,
            password: password_hash
        };

        match self.db_client.add_user(user).await {
            Ok(_) => Ok(Some(true)),
            Err(err) => Err(err)
        }
    }
}

fn validate_password(
    expected_hash: String,
    candidate_password: String,
) -> Result<(), AuthError> {
    let expected_hash =
        PasswordHash::new(expected_hash.as_str()).expect("Failed to get password hash");

    let res = Argon2::default()
        .verify_password(candidate_password.as_bytes(), &expected_hash);
    
    match res {
        Ok(v) => Ok(v),
        Err(_) => Err(AuthError::HashError)
    }

}
fn compute_hash(password: String) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(1500, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)
    .expect("Failed to hash")
    .to_string();
    Ok(hash)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compute_my_compute_hash() {
        let password= String::from("soSecretPassword");
        let hash1= compute_hash(password).expect("Failed to hash");
        assert!(hash1.starts_with("$argon2id$"))

    }

    #[test]
    fn compute_and_validate_my_hashes() {
        let password = String::from("soSecretPassword");
      
        let hash= compute_hash(password).expect("Failed to hash");
    
        let res = validate_password(hash, String::from("soSecretPassword")).expect("Failed to hash");
        assert_eq!(res, ());
    
    }
}