use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use void::Void;

use crate::grant::AccessRequest;

#[derive(Serialize, Deserialize, Clone)]
pub struct IntrospectRequest {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proof: Option<String>,
    resource_server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<AccessRequest>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InstrospectResponse {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl FromStr for IntrospectRequest {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IntrospectRequest {
            access_token: s.to_string(),
            proof: None,
            resource_server: s.to_string(),
            access: None,
        })
    }
}

impl ToRedisArgs for &IntrospectRequest {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(
            serde_json::to_string(self).expect("Can't serialize IntrospectRequest as string"),
        )
    }
}
