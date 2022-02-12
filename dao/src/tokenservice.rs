use super::cache::GnapCache;
use super::token::TokenDb;
use errors::TokenError;

#[derive(Clone)]
pub struct TokenService {
    pub db_client: TokenDb,
    pub cache_client: GnapCache,
}

impl TokenService {
    pub async fn create() -> TokenService {
        let db_client = TokenDb::new().await;
        let cache_client = GnapCache::new().await;

        TokenService {
            db_client,
            cache_client,
        }
    }

    pub async fn revoke_token(&self, token: String) -> Result<bool, TokenError> {
        match self.db_client.remove_token(token).await {
            Ok(_) => Ok(true),
            Err(_) => Err(TokenError::NotFound),
        }
    }
}
