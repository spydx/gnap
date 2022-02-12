use super::CachePath;
use rand::Rng;
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
}

impl Token {
    pub fn create() -> Self {
        let access_token = generate_token();
        Self {
            access_token: access_token,
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

    let mut res = String::from("");
    let charlength = characters.len();
    for _n in 0..length {
        let pos = rng.gen_range(0..charlength);
        res.push(characters.chars().nth(pos).unwrap());
    }
    res
}

#[allow(dead_code)]
fn generate_usercode() -> String {
    let length = 64;
    let characters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    let mut res = String::from("");
    let charlength = characters.len();

    for _ in 0..length {
        let pos = rng.gen_range(0..charlength);
        res.push(characters.chars().nth(pos).unwrap());
    }

    res
}
/*
public generateRandomString(length: Number): String {
    var result = '';
    var characters =
      'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for (var i = 0; i < length; i++) {
      result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
  }
  */

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
}
