use std::fmt::Debug;

use anyhow::{Error, Ok};
use fastcrypto::hash::Blake2b256;
use fastcrypto::hash::HashFunction;
use move_core_types::account_address::AccountAddress;

pub trait Authenticator: Debug + Clone {
    type Error: Debug;

    fn check(&self) -> Result<bool, Self::Error>;

    fn validate(&mut self) -> Result<(), Self::Error>;

    fn invalidate(&mut self) -> Result<(), Self::Error>;

    fn get_address(&self) -> Result<AccountAddress, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct MockAuthenticator {
    id: u64,
    is_valid: bool,
}

impl MockAuthenticator {
    pub fn new(id: u64) -> Self {
        Self { id, is_valid: true }
    }
}

impl Authenticator for MockAuthenticator {
    type Error = Error;

    fn validate(&mut self) -> Result<(), Self::Error> {
        self.is_valid = true;
        Ok(())
    }

    fn invalidate(&mut self) -> Result<(), Self::Error> {
        self.is_valid = false;
        Ok(())
    }

    fn check(&self) -> Result<bool, Self::Error> {
        Ok(self.is_valid)
    }

    fn get_address(&self) -> Result<AccountAddress, Self::Error> {
        let mut hasher = Blake2b256::default();
        hasher.update(b"FIL");
        hasher.update(bcs::to_bytes(&self.id)?);

        AccountAddress::from_bytes(hasher.finalize()).map_err(Into::into)
    }
}
