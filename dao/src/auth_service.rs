use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use model::instances::InstanceRequest;

use super::auth::AuthDb;
use super::db::GnapDB;
use super::cache::GnapCache;
use errors::AuthError;
use log::trace;
use model::credentials::Credentials;
use model::users::User;
use rand;

pub struct AuthService {
    pub db_client: AuthDb,
    pub db_gnap: GnapDB,
    pub cache_client: GnapCache,
}

impl AuthService {
    pub async fn create() -> Self {
        let db = AuthDb::new().await;
        let cache_client = GnapCache::new().await;
        let gnap = GnapDB::new().await;

        Self {
            cache_client,
            db_client: db,
            db_gnap: gnap
        }
    }

    pub async fn validate_account(&self, credentials: Credentials, instance: InstanceRequest) -> Result<bool, AuthError> {
        trace!("Fetching User from database");
        let user = self.db_client.fetch_account(credentials.username).await?;
        if user.is_some() {
            match validate_password(user.clone().unwrap().password, credentials.password) {
                Ok(_) => {
                    match self.db_gnap.authenticate_tx(instance.instance_id, user.unwrap()).await {
                        Ok(_) => Ok(true),
                        Err(_) => Err(AuthError::DatabaseNotFound)
                    }
                },
                Err(_) => 
                {   
                    Ok(false)
                },
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
            id,
            username: credentials.username,
            password: password_hash,
            access: None,
        };

        match self.db_client.add_user(user).await {
            Ok(_) => Ok(Some(true)),
            Err(err) => Err(err),
        }
    }
}

fn validate_password(expected_hash: String, candidate_password: String) -> Result<(), AuthError> {
    let expected_hash =
        PasswordHash::new(expected_hash.as_str()).expect("Failed to get password hash");

    let res = Argon2::default().verify_password(candidate_password.as_bytes(), &expected_hash);

    match res {
        Ok(v) => Ok(v),
        Err(_) => Err(AuthError::HashError),
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
        let password = String::from("soSecretPassword");
        let hash1 = compute_hash(password).expect("Failed to hash");
        assert!(hash1.starts_with("$argon2id$"))
    }

    #[test]
    fn compute_and_validate_my_hashes() {
        let password = String::from("soSecretPassword");

        let hash = compute_hash(password).expect("Failed to hash");

        let res =
            validate_password(hash, String::from("soSecretPassword")).expect("Failed to hash");
        assert_eq!(res, ());
    }

    #[test]
    fn check_validate_password() {
        let password = String::from("password");

        let hash = String::from("$argon2id$v=19$m=1500,t=2,p=1$SQ7OGnJMWaiUVfo1lOd8Iw$my2NzNZkr3h3phXr0cjtiNPTc2vLIrRmWMHxlDRouCI");
        let res = validate_password(hash, password).expect("it not to fail");

        assert_eq!(res, ());
    }
}
