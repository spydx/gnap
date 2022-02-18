use model::resource::GnapResourceServer;

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

    pub async fn add_resource_server(&self, rs: GnapResourceServer) -> Result<(), ResourceError> {
        match self.db_client.add_resource(rs).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}