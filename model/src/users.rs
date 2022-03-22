use super::grant::AccessRequest;
use super::CachePath;
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<AccessRequest>>,
}

impl User {
    pub fn create_id() -> Uuid {
        Uuid::new_v4()
    }
}

impl CachePath for User {
    fn cache_path() -> &'static str {
        "gnap:user"
    }
}

impl ToRedisArgs for &User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize user as string"))
    }
}
