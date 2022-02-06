use redis::{RedisWrite, ToRedisArgs};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use void::Void;

#[derive(Serialize, Deserialize, Clone)]
pub struct IntrospectRequest {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proof: Option<String>,
    resource_server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Vec<String>>,
}

impl FromStr for IntrospectRequest {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            IntrospectRequest {
                access_token: s.to_string(),
                proof: None,
                resource_server: s.to_string(),
                access: None
            }
        )
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