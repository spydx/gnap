use super::cache::GnapCache;
use super::token::TokenDb;
use errors::TokenError;
use model::tokens::Token;

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

    pub async fn revoke_token(&self, token: &Token) -> Result<bool, TokenError> {
        match self.db_client.remove_token(&token).await {
            Ok(_) => Ok(true),
            Err(_) => Err(TokenError::NotFound),
        }
    }

    pub async fn rotate_token(&self, token: Token) -> Result<Token, TokenError> {


        match self.db_client.fetch_token(&token).await {
            Ok(newtoken) => {
                let _ = self
                        .db_client
                        .remove_token(&token)
                        .await;

                match  self.db_client.add_token(&newtoken).await {
                    Ok(_) => Ok(newtoken),
                    Err(_) => Err(TokenError::RotateToken)
                }
            },
            Err(_) => Err(TokenError::RotateToken)
        }
    }
}
