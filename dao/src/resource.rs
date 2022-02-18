#[allow(unused_imports)]
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use log::trace;
use errors::ResourceError;
use model::resource::{GnapResourceServer};

#[derive(Clone)]
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

    pub async fn add_resource(&self, resource: GnapResourceServer) -> Result<(), ResourceError> {
        trace!("Adding resource server");

        let collection = self.database.collection::<GnapResourceServer>(COLLECTION);
        match collection.insert_one(resource, None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(ResourceError::DatabaseError(err))
        }
    }

    pub async fn add_access_to_resources(&self, access: GnapResourceServer) -> Result<(), ResourceError> {
        trace!("updating access sets");

        let collection = self.database.collection::<GnapResourceServer>(COLLECTION);
        let filter = doc! { "resource_server": &access.resource_server};

        match collection.find_one_and_replace(filter, access, None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(ResourceError::DatabaseError(err))
        }
    }

    pub async fn fetch_resource_server(&self, resource_server: String) -> Result<Option<GnapResourceServer>, ResourceError> {
        trace!("looking for resource server");
        let collection = self.database.collection::<GnapResourceServer>(COLLECTION);

        let filter = doc! { "resource_server": &resource_server };

        match collection.find_one(filter, None).await {
            Ok(data) => Ok(data),
            Err(_) => {
                trace!("Not found");
                Ok(None)
            }
        }
    }
}