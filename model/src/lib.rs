use errors::GnapError;
use uuid::Uuid;
pub mod account;
pub mod client;
pub mod gnap;
pub mod grant;
pub mod introspect;
pub mod oauth;
pub mod oidc;
pub mod resource;
pub mod transaction;
pub mod credentials;
pub mod users;
pub mod tokens;
/// CachePath ensures each model type that will be cached provides a
/// consistent path to cache objects
pub trait CachePath {
    fn cache_path() -> &'static str;
}

/// GnapID ensures any model that contains an "id", such as "client_id"
/// is generated and parsed in a consistent manner
pub trait GnapID {
    fn parse_id(&self) -> Result<Uuid, GnapError>;
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    #[test]
    fn it_works() {
        let my_uuid = Uuid::new_v4();
        println!("{}", my_uuid);
        assert_eq!(2 + 2, 4);
    }
}
