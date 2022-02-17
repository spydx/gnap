use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use log::trace;
use errors::ResourceError;
use model::resource::ResourceEntitlement;

pub struct ResourceDB {
    pub client: Client,
    pub database: Database,
}

const COLLECTION: &str = "resources";


impl ResourceDB { 
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

    pub async fn add_resource(&self, resource: ResourceEntitlement) -> Result<(), ResourceError> {
        trace!("Adding resource server");

        let collection = self.database.collection::<ResourceEntitlement>(COLLECTION);
        match collection.insert_one(resource, None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(ResourceError::DatabaseError(err))
        }
    }
}