use model::{resource::{GnapResourceServer, GnapRegisterResourceServer}, introspect::InstrospectResponse};
use log::{trace, debug};
use crate::{resource::ResourceDB, cache::GnapCache, token_service::TokenService};
use errors::ResourceError;
use model::introspect::IntrospectRequest;

#[derive(Clone)]
pub struct ResourceService {
    pub db_client: ResourceDB,
    pub cache_client: GnapCache,
    pub token_service: TokenService,
}

impl ResourceService {
    pub async fn create() -> Self {
        let db_client = ResourceDB::new().await;
        let cache_client = GnapCache::new().await;
        let token_service = TokenService::create().await;

        Self {
            db_client, 
            cache_client,
            token_service,
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

    pub async fn introspect_token(&self, ir: IntrospectRequest) -> Result<InstrospectResponse, ResourceError> {

        let target = ir.resource_server.clone();
        let resource_server = match self
            .db_client
            .fetch_resource_server(target)
            .await
            {
                Ok(data) => data,
                Err(_) => None
            };

        if resource_server.is_none() {
            return Err(ResourceError::NotFound)
        }
        match validate_access_request(ir.clone(), resource_server.unwrap()) {
            Ok(_) => {
                let access = ir.access.clone();
                let key = Some(String::from("httpsig"));

                let token_active = match self.token_service.validate_token(ir.access_token).await {
                    Ok(_) => true, 
                    Err(_) => false,
                };

                let response = InstrospectResponse {
                    active: token_active,
                    access: access,
                    key:key 
                };
                Ok(response)
            },
            Err(err) => Err(err)
        }
    }
}


fn validate_access_request(ir: IntrospectRequest, rs: GnapResourceServer) -> Result<(), ResourceError> {
    let access_request = if ir.access.is_none() {
        return Err(ResourceError::AccessNotFound)
    } else {
        ir.access.unwrap().to_owned()
    };

    for wanted_access in rs.access {
        for access in wanted_access {
            let res = access_request.eq(&access);
            debug!("Access: {:#?}", access);
            debug!("Validated: {:#?}", res);
            if res {
                return Ok(())
            }
        }
    }
    Err(ResourceError::AccessNotFound)
}


#[cfg(test)]
mod test {
    #[test]
    fn validate_access_request_ok() {

    }

    #[test]
    fn validate_access_request_failed() {

    }

}