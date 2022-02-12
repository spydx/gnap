
use log::trace;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use errors::TokenError;
use model::tokens::Token;

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
            client: client,
            database: db,
        }
    }

    pub async fn add_token(&self, token: Token) -> Result<bool, TokenError>{
        let collection = self.database.collection::<Token>(COLLECTION);
        match collection.insert_one(token, None)
        .await {
            Ok(_) => Ok(true),
            Err(err) => Err(TokenError::DatabaseError(err))
        }
    }

    pub async fn remove_token(&self, access_token: String) -> Result<bool, TokenError> {
        let cursor_result = self.database.collection::<Token>(COLLECTION)
            .delete_one( doc! { "access_token": access_token}, None)
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

    pub async fn update_token(&self, token: Token ) -> bool {
        let cursor_result = self.database.collection::<Token>(COLLECTION)
            .find_one_and_replace(doc! { "access_token": &token.access_token }, &token, None)
            .await
            .map_err(TokenError::DatabaseError);
        
        match cursor_result { 
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
