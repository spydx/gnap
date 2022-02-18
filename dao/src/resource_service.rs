use model::resource::{GnapResourceServer, GnapRegisterResourceServer};
use log::trace;
use crate::{resource::ResourceDB, cache::GnapCache};
use errors::ResourceError;

#[derive(Clone)]
pub struct ResourceService {
    pub db_client: ResourceDB,
    pub cache_client: GnapCache,
}

impl ResourceService {
    pub async fn create() -> Self {
        let db_client = ResourceDB::new().await;
        let cache_client = GnapCache::new().await;

        Self {
            db_client, 
            cache_client
        }
    }

    pub async fn add_resource_server(&self, rs: GnapRegisterResourceServer) -> Result<(), ResourceError> {
        trace!("Registering resources");
        let rs = GnapResourceServer::create(rs);
        match self.db_client.add_resource(rs).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn register_resources_set(&self, rs: GnapResourceServer) -> Result<(), ResourceError> {
        trace!("Registering resources");
        match self.db_client.add_access_to_resources(rs).await {
            Ok(()) => Ok(()),
            Err(err) => Err(err)
        }
    }
}