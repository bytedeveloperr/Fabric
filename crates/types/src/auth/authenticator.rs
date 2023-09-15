use std::fmt::Debug;

use anyhow::{Error, Ok};
use serde::de::DeserializeOwned;

use crate::auth::auth_session::AuthSession;

pub trait Authenticator: Debug + Clone + Sized {
    type Error: Debug;

    type Credential: Clone + Debug + DeserializeOwned;

    // fn check(&self) -> Result<bool, Self::Error>;

    fn validate_credential(credential: &Self::Credential) -> Result<AuthSession, Self::Error>;

    // fn invalidate_credential(&self) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub struct MockAuthenticator;

impl Authenticator for MockAuthenticator {
    type Error = Error;

    type Credential = Vec<u8>;

    // fn check(&self) -> Result<bool, Self::Error> {
    //     Ok(self.is_valid)
    // }

    fn validate_credential(_credential: &Self::Credential) -> Result<AuthSession, Self::Error> {
        Ok(AuthSession::default())
    }

    // fn invalidate_credential(&self) -> Result<(), Self::Error> {
    //     Ok(())
    // }
}
