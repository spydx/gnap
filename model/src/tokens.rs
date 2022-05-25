use super::CachePath;
use rand::Rng;
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub id: String,
    pub access_token: Option<String>,
    pub tx: Option<String>,
    pub label: Option<String>,
    pub expire: Option<u32>,
}

#[derive(Default)]
pub struct TokenBuilder {
    pub id: String,
    pub access_token: Option<String>,
    pub tx: Option<String>,
    pub label: Option<String>,
    pub expire: Option<u32>,
}

impl Token {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }
    pub fn create(tx: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let access_token = generate_token();
        Self {
            id,
            access_token: Some(access_token),
            tx: Some(tx),
            label: None,
            expire: Some(0),
        }
    }
    pub fn from_string(s: String) -> Self {
        Self {
            id: s,
            access_token: None,
            tx: None,
            label: None,
            expire: None,
        }
    }
}

impl TokenBuilder {
    pub fn new(tx: String) -> TokenBuilder {
        let id = Uuid::new_v4().to_string();
        Self {
            id,
            access_token: None,
            tx: Some(tx),
            label: None,
            expire: Some(0),
        }
    }

    pub fn label(mut self, label: Option<String>) -> TokenBuilder {
        self.label = label;
        self
    }

    pub fn expire(mut self, expire: u32) -> TokenBuilder {
        self.expire = Some(expire);
        self
    }

    pub fn build(self) -> Token {
        let access_token = generate_token();
        Token {
            id: self.id,
            access_token: Some(access_token),
            tx: self.tx,
            label: self.label,
            expire: self.expire
        }
    }
}

impl CachePath for Token {
    fn cache_path() -> &'static str {
        "gnap:token"
    }
}

impl ToRedisArgs for &Token {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize token"))
    }
}

/*
Copyright SecureKey Technologies Inc. All Rights Reserved.
SPDX-License-Identifier: Apache-2.0
*/
fn generate_token() -> String {
    let length = 64;
    let characters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let mut token = String::from("");
    let charlength = characters.len();
    for _n in 0..length {
        let pos = rng.gen_range(0..charlength);
        token.push(characters.chars().nth(pos).unwrap());
    }

    token
}

#[allow(dead_code)]
fn generate_usercode() -> String {
    let length = 64;
    let characters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    let mut usercode = String::from("");
    let charlength = characters.len();

    for _ in 0..length {
        let pos = rng.gen_range(0..charlength);
        usercode.push(characters.chars().nth(pos).unwrap());
    }

    usercode
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn gen_a_token() {
        let token1 = generate_token();
        let token2 = generate_token();

        println!("Token1: {}", token1);
        println!("Token2: {}", token2);
        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);
        assert!(token1.ne(&token2));
    }

    #[test]
    fn gen_usercode() {
        let token = generate_usercode();
        println!("UserCode: {}", token);
        assert_eq!(token.len(), 64);
    }

    #[test]
    fn token_builder_ok() {
        let tx = Uuid::new_v4().to_string();
        let label = Some(String::from("kenneth"));
        let token  = TokenBuilder::new(tx.clone())
                    .label(label.clone())
                    .build();

        assert_eq!(token.label, label);
        assert!(token.access_token.is_some());
        assert_eq!(token.tx.unwrap(), tx);
        assert_eq!(token.expire, Some(0))
    }
}
