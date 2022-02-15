//! Wrapper for MongoDB connections.
//!
use core::result::Result;
use errors::GnapError;
use futures::stream::TryStreamExt;
use log::{debug, trace};
use model::transaction::{TransactionOptions, GnapTransaction, GnapTransactionState};
use model::{
    account::{Account, AccountRequest},
    client::{GnapClient, GnapClientRequest},
    gnap::GnapOptions,
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

    pub async fn find_transaction(&self, tx_id: String) -> Result<Option<GnapTransaction>, GnapError> {
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

    pub async fn authenticate_tx(&self, tx_id: String) -> Result<(), GnapError> {
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
                    let mut update = trans.unwrap();
                    update.state = GnapTransactionState::Authorized;
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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
