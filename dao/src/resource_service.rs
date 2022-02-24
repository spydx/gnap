use model::{resource::{GnapResourceServer, GnapRegisterResourceServer}, introspect::InstrospectResponse, grant::GrantRequest};
use log::{trace, debug};
use crate::{resource::ResourceDB, cache::GnapCache, token_service::TokenService, service::Service};
use errors::ResourceError;
use model::introspect::IntrospectRequest;

#[derive(Clone)]
pub struct ResourceService {
    pub db_client: ResourceDB,
    pub cache_client: GnapCache,
    pub token_service: TokenService,
    pub tx_service: Service,
}

impl ResourceService {
    pub async fn create() -> Self {
        let db_client = ResourceDB::new().await;
        let cache_client = GnapCache::new().await;
        let token_service = TokenService::create().await;
        let tx_service = Service::create().await;

        Self {
            db_client, 
            cache_client,
            token_service,
            tx_service,
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

        
        // 1. Get RS
        let target = ir.resource_server.clone();
        let resource_server = match self
            .db_client
            .fetch_resource_server(target)
            .await
            {
                Ok(data) => data,
                Err(_) => None

            };
        
        
        let resource_server = if resource_server.is_none() {
            return Err(ResourceError::NotFound)
        } else {
            resource_server.unwrap()
        };

        debug!("We be here");
        // 2. Get token
        let token = match self.token_service.db_client.fetch_token_by_id(ir.access_token.clone()).await {
            Ok(t) => Some(t),
            Err(_) => None,
        };

        debug!("We there");
        let token = if token.is_none() {
            return Err(ResourceError::TokenError)
        } else {
            token.unwrap()
        };

        debug!("lastly be here");
        // 3. Get access from token
        let ac = match self.tx_service.get_transaction(token.tx.unwrap()).await {
            Ok(d) => Some(d),
            Err(_) => None,
        };

        let access_request = if ac.is_none() {
            return Err(ResourceError::AccessNotFound)
        } else {
            ac.unwrap().request.unwrap()
        };
        



        // bug: IR does not contain any access, and hence fail.
        // need to look up token and tx before validating.


        match validate_access_request(access_request, resource_server) {
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


fn validate_access_request(grant_request: GrantRequest , rs: GnapResourceServer) -> Result<(), ResourceError> {
    debug!("req: {:#?}", grant_request);

    for wanted_access in rs.access {
        for access in wanted_access {
            for access_request in grant_request.access_token.clone() {
                for user_access in access_request.access {
                    let res = user_access.eq(&access);
                    debug!("Access: {:#?}", access);
                    debug!("Validated: {:#?}", res);
                    if res {
                        return Ok(())
                    }
                }
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