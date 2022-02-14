use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GnapError {
    #[error("mongodb error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("Cache error: {0}")]
    CacheError(#[from] redis::RedisError),
    #[error("Not found error")]
    NotFound,
    #[error("Bad data error")]
    BadData,
    #[error("General error")]
    GeneralError,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authorization field missing")]
    BasicFailed,
    #[error("Field error for username")]
    UserNameError,
    #[error("Datbase not Found")]
    DatabaseNotFound,
    #[error("Field error for password")]
    PasswordError,
    #[error("Hash error")]
    HashError,
    #[error("Can't compare two hashes")]
    HashMissmatch,
    #[error("Can't store a user in the database")]
    DatabaseError(#[from] mongodb::error::Error),
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("General error")]
    InvalidToken,
    #[error("Can't store a token in the database")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Can't find token in the database")]
    NotFound,
    #[error("Can't rotate token")]
    RotateToken
}

impl From<serde_json::Error> for GnapError {
    fn from(_source: serde_json::Error) -> Self {
        Self::GeneralError
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
