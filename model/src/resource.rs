//! Resource models for grant requests
//!
//! The Resource model can be represented as either a single string, or as an
//! object.  Because the AccessToken model contains an array of resources (as
//! the "access" attriute), we need to manually serialize and deserialize it.
//!
//! This file should be used as a pattern whenever there is a Vec<String_or_Struct>
//! variant pattern.

use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::{str::FromStr};
use uuid::Uuid;
use void::Void;

use crate::grant::AccessRequest;


#[derive(Serialize ,Deserialize)]
pub struct GnapRegisterResourceServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server_name: Option<String>, 
    pub resource_server_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_set: Option<Vec<AccessRequest>>,
}

#[derive(Serialize ,Deserialize, Debug)]
pub struct GnapResourceServer {
    pub resource_server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server_name: Option<String>, 
    pub resource_server_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_set: Option<Vec<AccessRequest>>,
}

impl GnapResourceServer {
    pub fn create(register: GnapRegisterResourceServer) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            resource_server: id,
            resource_server_name: register.resource_server_name,
            resource_server_key: register.resource_server_key,
            resource_set: register.resource_set
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ResourceEntitlement {
    // If set, this is a mapped entitlement.  If not, it is a template that can be mapped
    client_id: Option<Uuid>,
    // Entitlements can be referenced by name
    name: Option<String>,
    resource_type: String,
    actions: Option<Vec<String>>,
    locations: Option<Vec<String>>,
    data_types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ResourceRequest {
    #[serde(rename = "type")]
    resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_types: Option<Vec<String>>,
}

impl FromStr for ResourceRequest {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ResourceRequest {
            resource_type: s.to_string(),
            actions: None,
            locations: None,
            data_types: None,
        })
    }
}

impl ToRedisArgs for &ResourceRequest {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(
            serde_json::to_string(self).expect("Can't serialize ResourceRequest as string"),
        )
    }
}