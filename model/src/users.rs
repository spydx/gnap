use uuid::Uuid;
use crate::credentials::Credentials;
use super::CachePath;
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use secrecy::Secret;

#[derive(Serialize, Debug, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Secret<String>,
}


impl User {
    fn create_id() -> Uuid {
        Uuid::new_v4()
    }

    fn create_user(user: Credentials) -> Self {
        todo!()
    }

    fn validate_user(user: Credentials) -> bool {        
        false
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