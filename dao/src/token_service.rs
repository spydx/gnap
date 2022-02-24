use super::cache::GnapCache;
use super::token::TokenDb;
use errors::TokenError;
use model::tokens::Token;
use log::debug;

#[derive(Clone)]
pub struct TokenService {
    pub db_client: TokenDb,
    pub cache_client: GnapCache,
}

impl TokenService {
    pub async fn create() -> TokenService {
        let db_client = TokenDb::new().await;
        let cache_client = GnapCache::new().await;

        //let _ = db_client.prune_db().await.expect("Failed to prune");
        
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
    pub async fn validate_token(&self, token_id: String) -> Result<(), TokenError> {
        match self.db_client.fetch_token_by_id(token_id).await {
            Ok(t) =>{
                debug!("Are we here");
                if t.expire.is_some() {
                    // this is not enterily correct, but will do for out poc
                    Ok(())
                } else {
                    Err(TokenError::InvalidToken)
                }
            },
            Err(_) => {
                Err(TokenError::InvalidToken)
            }
        }
    }

    pub async fn fetch_token_by_accesstoken(&self, access_token: String) -> Result<Token, TokenError> {
        match self.db_client.fetch_token_by_ac(access_token).await {
            Ok(t) => Ok(t),
            Err(e) => Err(e)
        }
    }
}