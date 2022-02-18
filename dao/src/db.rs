//! Wrapper for MongoDB connections.
//!
use core::result::Result;
use errors::GnapError;
use futures::stream::TryStreamExt;
use log::{debug, trace};
use model::transaction::{TransactionOptions, GnapTransaction, GnapTransactionState};
use model::{
    users::User,
    account::{Account, AccountRequest},
    client::{GnapClient, GnapClientRequest},
    gnap::GnapOptions,
    tokens::Token,
};
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct GnapDB {
    pub client: Client,
    pub database: Database,
}

const COL_TRANSACTION: &str = "transaction";
const COL_TRANSACTIONOPTIONS: &str = "transaction_options";
const COL_GNAPOPTIONS: &str = "service_config";
const COL_ACCOUNTS: &str = "accounts";
const COL_CLIENTS: &str = "clients";
const COL_TOKEN: &str = "tokens";


//const MONGO_URI: &str = "mongodb://127.0.0.1:27017";

impl GnapDB {
    pub async fn new() -> Self {
        // Read the config from either the environment or a .env file.
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI missing");
        let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
        let app_name = env::var("MONGODB_APP_NAME").expect("MONGODB_APP_NAME missing");

        // Create the ClientOptions and set the app_name
        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to create client options");
        client_options.app_name = Some(app_name);

        // Create the client and grab a database handle
        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");
        let db = client.database(&database);
        
        Self {
            client: client,
            database: db,
        }
    }

    pub async fn prune_db(&self) -> Result<(), GnapError> {
        debug!("Pruning database");
        let collection = self.database.collection::<GnapTransaction>(COL_TRANSACTION);
        let new_filter = doc! { "state": "new"};
        let waiting_filter = doc! { "state": "waiting"};
        let _new = collection
            .delete_many(new_filter, None)
            .await
            .map_err(GnapError::DatabaseError);

        let _waiting = collection
            .delete_many(waiting_filter, None)
            .await
            .map_err(GnapError::DatabaseError);

        Ok(())
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, GnapError> {
        match self.client.list_database_names(None, None).await {
            Ok(v) => Ok(v),
            Err(e) => Err(GnapError::DatabaseError(e)),
        }
    }

    // Figure out how to break these out into separate mods, so this file
    // is manageable.
    pub async fn fetch_gnap_well_knowns(&self) -> Result<GnapOptions, GnapError> {
        //self.update_gnap_options().await?;
        let cursor_result = self
            .database
            .collection::<GnapOptions>(COL_GNAPOPTIONS)
            .find(None, None)
            .await
            .map_err(GnapError::DatabaseError);
        match cursor_result {
            Ok(mut cursor) => match cursor.try_next().await {
                Ok(Some(result)) => Ok(result),
                Ok(None) => {
                    trace!("GnapOptions not found");
                    Err(GnapError::NotFound)
                }
                Err(e) => {
                    trace!("{:?}", &e);
                    Err(GnapError::DatabaseError(e))
                }
            },
            Err(e) => {
                trace!("{:?}", &e);
                Err(e)
            }
        }
    }

    pub async fn update_gnap_options(&self) -> Result<GnapOptions, GnapError> {
        let collection = self.database.collection::<GnapOptions>(COL_GNAPOPTIONS);
        let options = GnapOptions::new("http://localhost:8000");
        match collection.insert_one(options.clone(), None).await {
            Ok(_) => {
                debug!("Added options: {:?}", &options);
                Ok(options)
            }
            Err(err) => {
                debug!("Error saving GnapOptions: {:?}", &err);
                Err(GnapError::DatabaseError(err))
            }
        }
    }
    // Figure out how to break these out into separate mods, so this file
    // is manageable.
    pub async fn fetch_grant_options(&self) -> Result<TransactionOptions, GnapError> {
        let mut cursor = self
            .database
            .collection::<TransactionOptions>(COL_TRANSACTIONOPTIONS)
            .find(None, None)
            .await
            .map_err(GnapError::DatabaseError)?;

        match cursor.try_next().await {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Ok(TransactionOptions::new()),
            Err(e) => Err(GnapError::DatabaseError(e)),
        }
    }

    // Client methods
    pub async fn fetch_client_by_id(&self, id: &Uuid) -> Result<Option<GnapClient>, GnapError> {
        trace!("Fetching client by ID: {}", id.to_string());
        let cursor_result = self
            .database
            .collection::<GnapClient>(COL_CLIENTS)
            .find_one(doc! {"client_id": &id.to_string()}, None)
            .await
            .map_err(GnapError::DatabaseError);
        match cursor_result {
            Ok(cursor) => match cursor {
                Some(result) => {
                    trace!("Fetched a client");
                    Ok(Some(result))
                }
                None => {
                    trace!("Client not found");
                    Err(GnapError::NotFound)
                }
            },
            Err(e) => {
                trace!("get_client returned en error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn add_client(&self, request: GnapClientRequest) -> Result<GnapClient, GnapError> {
        let collection = self.database.collection::<GnapClient>(COL_CLIENTS);
        let client = GnapClient::new(request.redirect_uris, request.client_name);
        match collection.insert_one(client.clone(), None).await {
            Ok(_) => {
                debug!("Added client: {:?}", &client);
                Ok(client)
            }
            Err(err) => {
                debug!("Error saving client: {:?}", &err);
                Err(GnapError::DatabaseError(err))
            }
        }
    }

    // Client methods
    pub async fn fetch_account_by_id(&self, id: &Uuid) -> Result<Option<Account>, GnapError> {
        trace!("Fetching account by ID: {}", id.to_string());
        let cursor_result = self
            .database
            .collection::<Account>(COL_ACCOUNTS)
            .find_one(doc! {"account_id": &id.to_string()}, None)
            .await
            .map_err(GnapError::DatabaseError);
        match cursor_result {
            Ok(cursor) => match cursor {
                Some(result) => {
                    trace!("Fetched an account");
                    Ok(Some(result))
                },
                None => {
                    trace!("Account not found");
                    Err(GnapError::NotFound)
                }
            },
            Err(e) => {
                trace!("get_account_by_id returned en error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn add_account(&self, request: AccountRequest) -> Result<Account, GnapError> {
        let collection = self.database.collection::<Account>(COL_ACCOUNTS);
        let account = Account::from(request);
        match collection.insert_one(&account, None).await {
            Ok(_) => {
                debug!("Added account: {:?}", &account);
                Ok(account)
            },
            Err(err) => {
                debug!("Error saving account: {:?}", &err);
                Err(GnapError::DatabaseError(err))
            }
        }
    }

    pub async fn add_transaction(&self, tx: GnapTransaction) -> Result<GnapTransaction, GnapError> {
        let collection = self.database.collection::<GnapTransaction>(COL_TRANSACTION);
        match collection.insert_one(&tx, None).await {
            Ok(_) => {
                debug!("Added account: {:?}", &tx);
                Ok(tx)
            },
            Err(err) => {
                debug!("Error saving tx: {:?}", &err);
                Err(GnapError::DatabaseError(err))
            }
        }        
    }

    pub async fn get_transaction(&self, tx_id: String) -> Result<Option<GnapTransaction>, GnapError> {
       let cursor_result = self
            .database
            .collection::<GnapTransaction>(COL_TRANSACTION)
            .find_one( doc! { "tx_id": &tx_id}, None)
            .await
            .map_err(GnapError::DatabaseError);
        
        match cursor_result {
            Ok(result) => {
                match result { 
                    Some(tx) => {
                        trace!("Fetched TX");
                        Ok(Some(tx))
                    },
                    None => {
                        trace!("Account not found");
                        Err(GnapError::NotFound)
                    }
                }
            },
            Err(e) => {
                trace!("get_account_by_id returned en error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn delete_transaction(&self, tx_id: String) -> Result<(), GnapError> {
        let collection = self.database.collection::<GnapTransaction>(COL_TRANSACTION);

        match collection.delete_one(doc! { "tx_id": &tx_id}, None).await {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(GnapError::DatabaseError(err))
            }
        }
    }

    pub async fn update_transaction(&self, tx: GnapTransaction) -> Result<GnapTransaction, GnapError> {
        let cursor_result = self
            .database
            .collection::<GnapTransaction>(COL_TRANSACTION)
            .find_one_and_replace(doc! {"tx_id": &tx.tx_id}, &tx,None)
            .await
            .map_err(GnapError::DatabaseError);

        match cursor_result {
            Ok(_) => Ok(tx),
            Err(err) => {
                Err(err)
            }
        }    
    }

    pub async fn authenticate_tx(&self, tx_id: String, user: User) -> Result<(), GnapError> {
        let filter = doc! {"tx_id": &tx_id };

        let collection = self
            .database
            .collection::<GnapTransaction>(COL_TRANSACTION);

        let cursor_result = collection
            .find_one(filter.clone(), None)
            .await
            .map_err(GnapError::DatabaseError);

    
        let tx = match cursor_result {
            Ok(trans) => {

                let tx_update = if trans.is_some() {
                    match validate_user_access(user.clone(), trans.clone().unwrap()) {
                        Ok(_) => {},
                        Err(err) => return Err(err)
                    }
                    let update = trans
                        .unwrap()
                        .update_state(GnapTransactionState::Authorized)
                        .update_grantrequest(user.id);
                    Some(update)
                } else {
                    None
                };
                tx_update
          }, 
            Err(_) => None,
        };
        
        if tx.is_some() {
            let res = collection
                .find_one_and_replace(filter.clone(), &tx.unwrap(), None)
                .await
                .map_err(GnapError::DatabaseError);
            match res {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        } else {
            debug!("Something went wrong, Transaction not found");
            Err(GnapError::NotFound)
        }
    }
    pub async fn add_token(&self, token: &Token) -> Result<bool, GnapError> {
        let collection = self.database.collection::<Token>(COL_TOKEN);
        match collection.insert_one(token, None).await {
            Ok(_) => Ok(true),
            Err(err) => Err(GnapError::DatabaseError(err)),
        }
    }
}

fn validate_user_access(user: User, tx: GnapTransaction) -> Result<(), GnapError> {
    let grant = tx.request.unwrap();
    let user_access = user.access.unwrap();
    debug!("Lets VALIDATE");
    for wanted_access_tokens in grant.access_token.into_iter() {
        for wanted_access in wanted_access_tokens.access.into_iter() {
            let b = user_access.contains(&wanted_access);
            debug!("Access: {:#?}", wanted_access);
            debug!("Validated: {:#?}", b);
            if b {
                return Ok(())
            }
    
        }
    }

    Err(GnapError::AccessMismatch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    const TX_DATA: &str = r#"{
        "tx_id": "32aabb1c-5e1e-4ca9-992c-67b1b6a9de08",
        "state": "new",
        "request": {
            "access_token": [
                {
                    "access": [
                        "foo",
                        {
                            "type": "bar",
                            "actions": [
                                "read",
                                "write"
                            ]
                        }
                    ],
                    "label": "my_label",
                    "flags": [
                        "bearer"
                    ]
                }
            ],
            "subject": null,
            "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
            "user": null,
            "interact": {
                "start": [
                    "redirect"
                ],
                "finish": {
                    "method": "redirect",
                    "uri": "localhost:8000/gnap/auth",
                    "nonce": "419b6c799164494bb04958d04152e2b4"
                }
            }
        }
    }
  "#;

  const TX_DATA_OK: &str = r#"{
    "tx_id": "32aabb1c-5e1e-4ca9-992c-67b1b6a9de08",
    "state": "new",
    "request": {
        "access_token": [
            {
                "access": [
                    {
                        "type": "waterbowl-access",
                        "actions": [
                            "read",
                            "create"
                        ],
                        "locations": [
                            "https://localhost:8080/bowls/"
                        ]
                    },
                    {
                        "type": "waterlevel-access",
                        "actions": [
                            "read",
                            "create"
                        ],
                        "locations": [
                            "https://localhost:8080/bowls/waterlevels/"
                        ]
                    }
                ]
            }
        ],
        "subject": null,
        "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
        "user": null,
        "interact": {
            "start": [
                "redirect"
            ],
            "finish": {
                "method": "redirect",
                "uri": "localhost:8000/gnap/auth",
                "nonce": "419b6c799164494bb04958d04152e2b4"
            }
        }
    }
}
  "#;

    const USER_DATA: &str = r#"{
        "id": "6785732c-682a-458b-8465-2986a77abf6a",
        "username": "kenneth",
        "password": "$argon2id$v=19$m=1500,t=2,p=1$SQ7OGnJMWaiUVfo1lOd8Iw$my2NzNZkr3h3phXr0cjtiNPTc2vLIrRmWMHxlDRouCI",
        "access": [
            {
                "type": "waterbowl-access",
                "actions": [
                    "read",
                    "create"
                ],
                "locations": [
                    "https://localhost:8080/bowls/"
                ]
            },
            {
                "type": "waterlevel-access",
                "actions": [
                    "read",
                    "create"
                ],
                "locations": [
                    "https://localhost:8080/bowls/waterlevels/"
                ]
            }
        ]
    }"#;

    #[test]
    fn parse_user() {
        let _: User = serde_json::from_str(USER_DATA).unwrap();
    }

    #[test]
    fn parse_tx() {
        let _: GnapTransaction= serde_json::from_str(TX_DATA).unwrap();
    }


    #[test]
    fn parse_tx_ok() {
        let _: GnapTransaction= serde_json::from_str(TX_DATA_OK).unwrap();
    }

    #[test]
    fn test_validate_user_access() {
        let user = serde_json::from_str(USER_DATA).unwrap();
        let tx = serde_json::from_str(TX_DATA).unwrap();

        let res = validate_user_access(user, tx);
        match res {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_validate_user_access_ok() {
        let user = serde_json::from_str(USER_DATA).unwrap();
        let tx = serde_json::from_str(TX_DATA_OK).unwrap();

        let res = validate_user_access(user, tx);
        match res {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }
}
