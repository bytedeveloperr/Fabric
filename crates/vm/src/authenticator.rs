use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub struct Identifier {}

pub trait Authenticator {
    type Error: Debug;

    fn revoke_token(&self, token: Vec<u8>) -> Result<(), Self::Error>;

    fn validate_token(&self, token: Vec<u8>) -> Result<(), Self::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {}
