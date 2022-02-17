use errors::AuthError;
use log::trace;
use model::users::User;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;

pub struct AuthDb {
    pub client: Client,
    pub database: Database,
}

const COLLECTION: &str = "users";

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

    pub async fn fetch_account(&self, username: String) -> Result<Option<User>, AuthError> {
        let cursor_result = self
            .database
            .collection::<User>(COLLECTION)
            .find_one(doc! { "username": &username}, None)
            .await
            .map_err(AuthError::DatabaseError);

        match cursor_result {
            Ok(cursor) => match cursor {
                Some(res) => Ok(Some(res)),
                None => {
                    trace!("Fetch user error");
                    Err(AuthError::DatabaseNotFound)
                }
            },
            Err(e) => {
                trace!("Fetch user error {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn add_user(&self, user: User) -> Result<bool, AuthError> {
        let collection = self.database.collection::<User>(COLLECTION);

        match collection.insert_one(user, None).await {
            Ok(_) => Ok(true),
            Err(err) => Err(AuthError::DatabaseError(err)),
        }
    }
}
