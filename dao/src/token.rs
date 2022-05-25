use errors::TokenError;
use log::{debug, trace};
use model::tokens::Token;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;

#[derive(Clone)]
pub struct TokenDb {
    pub client: Client,
    pub database: Database,
}
const COLLECTION: &str = "tokens";

impl TokenDb {
    pub async fn new() -> Self {
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI missing");
        let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
        let app_name = env::var("MONGODB_APP_NAME").expect("MONGODB_APP_NAME missing");

        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to create client options");
        client_options.app_name = Some(app_name);

        let client = Client::with_options(client_options).expect("Failed to create db client");

        let db = client.database(&database);
        Self {
            client,
            database: db,
        }
    }

    pub async fn prune_db(&self) -> Result<(), TokenError> {
        debug!("Pruning database");
        let collection = self.database.collection::<Token>(COLLECTION);
        let expire_filter = doc! { "expire": "0"};
        let null_filter = doc! { "expire": null};

        let _new = collection
            .delete_many(expire_filter, None)
            .await
            .map_err(TokenError::DatabaseError);

        let _null = collection
            .delete_many(null_filter, None)
            .await
            .map_err(TokenError::DatabaseError);
        debug!("Done pruning");
        Ok(())
    }

    pub async fn add_token(&self, token: &Token) -> Result<bool, TokenError> {
        let collection = self.database.collection::<Token>(COLLECTION);
        match collection.insert_one(token, None).await {
            Ok(_) => Ok(true),
            Err(err) => Err(TokenError::DatabaseError(err)),
        }
    }

    pub async fn remove_token(&self, token: &Token) -> Result<bool, TokenError> {
        let cursor_result = self
            .database
            .collection::<Token>(COLLECTION)
            .delete_one(doc! { "id": &token.id}, None)
            .await
            .map_err(TokenError::DatabaseError);

        match cursor_result {
            Ok(_) => Ok(true),
            Err(err) => {
                trace!("Failed to revoke token {:?}", err);
                Ok(false)
            }
        }
    }

    pub async fn update_token(&self, token: Token) -> bool {
        let cursor_result = self
            .database
            .collection::<Token>(COLLECTION)
            .find_one_and_replace(doc! { "id": &token.id }, &token, None)
            .await
            .map_err(TokenError::DatabaseError);

        cursor_result.is_ok()
    }

    pub async fn fetch_token(&self, token: &Token) -> Result<Token, TokenError> {
        let cursor_result = self
            .database
            .collection::<Token>(COLLECTION)
            .find_one(doc! { "id": &token.id }, None)
            .await
            .map_err(TokenError::DatabaseError);

        let res_token = match cursor_result {
            Ok(stored_token) => {
                if stored_token.is_some() {
                    stored_token
                } else {
                    None
                }
            }
            Err(_) => None,
        };

        if res_token.is_none() {
            Err(TokenError::NotFound)
        } else {
            Ok(res_token.unwrap())
        }
    }

    pub async fn fetch_token_by_id(&self, token_id: String) -> Result<Token, TokenError> {
        let cursor_result = self
            .database
            .collection::<Token>(COLLECTION)
            .find_one(doc! { "id": &token_id }, None)
            .await
            .map_err(TokenError::DatabaseError);

        match cursor_result {
            Ok(token) => {
                if token.is_none() {
                    Err(TokenError::NotFound)
                } else {
                    Ok(token.unwrap())
                }
            }
            Err(_) => Err(TokenError::NotFound),
        }
    }

    pub async fn fetch_token_by_ac(&self, access_token: String) -> Result<Token, TokenError> {
        let filter = doc! { "access_token": access_token};

        let cursor_result = self
            .database
            .collection::<Token>(COLLECTION)
            .find_one(filter, None)
            .await
            .map_err(TokenError::DatabaseError);

        match cursor_result {
            Ok(token) => {
                if token.is_some() {
                    Ok(token.unwrap())
                } else {
                    Err(TokenError::NotFound)
                }
            }
            Err(_) => Err(TokenError::NotFound),
        }
    }
}
