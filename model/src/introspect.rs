use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use void::Void;

use crate::grant::AccessRequest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntrospectRequest {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
    pub resource_server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstrospectResponse {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<AccessRequest>>,
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
