use crate::{
    cache::GnapCache, resource::ResourceDB, service::Service, token_service::TokenService,
};
use errors::ResourceError;
use log::{debug, trace};
use model::grant::AccessRequest;
use model::introspect::IntrospectRequest;
use model::{
    grant::GrantRequest,
    introspect::InstrospectResponse,
    resource::{GnapRegisterResourceServer, GnapResourceServer},
};

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

    pub async fn add_resource_server(
        &self,
        rs: GnapRegisterResourceServer,
    ) -> Result<(), ResourceError> {
        trace!("Registering resources");
        let rs = GnapResourceServer::create(rs);
        match self.db_client.add_resource(rs).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn register_resources_set(
        &self,
        rs: GnapResourceServer,
    ) -> Result<(), ResourceError> {
        trace!("Registering resources");
        match self.db_client.add_access_to_resources(rs).await {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn introspect_token(
        &self,
        ir: IntrospectRequest,
    ) -> Result<InstrospectResponse, ResourceError> {
        // 1. Get RS
        let target = ir.resource_server.clone();
        let resource_server = match self.db_client.fetch_resource_server(target).await {
            Ok(data) => data,
            Err(_) => None,
        };

        let resource_server = if resource_server.is_none() {
            return Err(ResourceError::NotFound);
        } else {
            resource_server.unwrap()
        };

        debug!("We be here");
        // 2. Get token
        let token = match self
            .token_service
            .fetch_token_by_accesstoken(ir.access_token.clone())
            .await
        {
            Ok(t) => Some(t),
            Err(_) => None,
        };

        debug!("We there");
        let token = if token.is_none() {
            return Err(ResourceError::TokenError);
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
            return Err(ResourceError::AccessNotFound);
        } else {
            ac.unwrap().request.unwrap()
        };

        match validate_access_request(&access_request, resource_server) {
            Ok(_) => {
                let key = Some(String::from("httpsig"));

                let token_active = self.token_service.validate_token(token.id).await.is_ok();
                let atr = access_request.access_token.first().to_owned();

                let response = InstrospectResponse {
                    active: token_active,
                    access: Some(atr.unwrap().access.to_owned()),
                    key,
                };
                println!("{:#?}", response);
                Ok(response)
            }
            Err(err) => Err(err),
        }
    }
}

fn validate_access_request(
    grant_request: &GrantRequest,
    rs: GnapResourceServer,
) -> Result<(), ResourceError> {
    debug!("req: {:#?}", grant_request);
    debug!("rs:  {:#?}", rs);

    for wanted_access in rs.resource_set {
        for wanted in wanted_access.into_iter() {
            let (w_rs, w_actions) = match wanted {
                model::grant::AccessRequest::Value { resource_type, actions, locations:_, data_types: _ } => (resource_type, actions.unwrap()),
                _ => return Err(ResourceError::AccessNotFound)
            };

            for granted_access_tokens in grant_request.clone().access_token {
                for gc in granted_access_tokens.clone().access {
                    let (granted_ac, granted_actions ) = match gc {
                        AccessRequest::Value { resource_type, actions, locations: _, data_types: _} => (resource_type, actions.unwrap()),
                        _ => return Err(ResourceError::AccessNotFound)
                    };

                    if w_rs.eq(&granted_ac) {
                        for a in w_actions.clone() {
                            if granted_actions.contains(&a) {
                                return Ok(())
                            }
                        }
                    }
                }
            }
        }
    }


    /*for wanted_access in rs.resource_set {
        for access in wanted_access {
            for access_request in grant_request.access_token.clone() {
                for user_access in access_request.access {
                    let res = user_access.eq(&access);
                    debug!("Access: {:#?}", access);
                    debug!("Validated: {:#?}", res);
                    if res {
                        debug!("Did this");
                        return Ok(());
                    }
                }
            }
        }
    }
    */
    Err(ResourceError::AccessNotFound)
}


#[cfg(test)]
mod test {
    use model::grant::GrantRequest;
    use super::*;

    const GRANT_REQUEST: &str = r#"
        {
            "access_token": [
              {
                "access": [
                  {
                    "type": "waterbowl-access",
                    "actions": [
                      "create"
                    ],
                    "locations": [
                      "http://localhost:8080/bowls/"
                    ]
                  }
                ],
                "label": "bowls",
                "flags": [
                  "bearer"
                ]
              }
            ],
            "subject": null,
            "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
            "user": "6785732c-682a-458b-8465-2986a77abf6a",
            "interact": {
              "start": [
                "redirect"
              ],
              "finish": {
                "method": "redirect",
                "uri": "localhost:8000/gnap/auth",
                "nonce": "1c7628ca-73f3-4fce-af52-05d028dab09a"
              }
        }
    }
    "#;

    const GRANT_REQUEST_DELETE: &str = r#"
        {
            "access_token": [
              {
                "access": [
                  {
                    "type": "waterbowl-access",
                    "actions": [
                      "delete"
                    ],
                    "locations": [
                      "http://localhost:8080/bowls/"
                    ]
                  }
                ],
                "label": "bowls",
                "flags": [
                  "bearer"
                ]
              }
            ],
            "subject": null,
            "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
            "user": "6785732c-682a-458b-8465-2986a77abf6a",
            "interact": {
              "start": [
                "redirect"
              ],
              "finish": {
                "method": "redirect",
                "uri": "localhost:8000/gnap/auth",
                "nonce": "1c7628ca-73f3-4fce-af52-05d028dab09a"
              }
        }
    }
    "#;

    const RESOURSE_SERVER: &str = r#"
    {
        "resource_server": "e8a2968a-f183-45a3-b63d-4bbbd1dad276",
        "resource_server_name": "simple-api",
        "resource_server_key": "httsig",
        "resource_set": [
            {
            "type": "waterbowl-access",
            "actions": [
                "read",
                "create"
            ],
            "locations": [
                "http://localhost:8080/bowls/"
            ]
            },
            {
            "type": "waterlevel-access",
            "actions": [
                "read",
                "create",
                "delete"
            ],
            "locations": [
                "https://localhost:8080/waterlevels/"
            ]
            }
        ]
        
    }
    "#;


    #[test]
    fn validate_access_request_ok() {
        let grant_request: GrantRequest = serde_json::from_str(GRANT_REQUEST).unwrap();
        let rs: GnapResourceServer = serde_json::from_str(RESOURSE_SERVER).unwrap();

        assert!(validate_access_request(&grant_request, rs).is_ok())

    }

    #[test]
    fn validate_access_request_failed() {
        let grant_request: GrantRequest = serde_json::from_str(GRANT_REQUEST_DELETE).unwrap();
        let rs: GnapResourceServer = serde_json::from_str(RESOURSE_SERVER).unwrap();

        assert_eq!(validate_access_request(&grant_request, rs).is_ok(), false)
    }
}
