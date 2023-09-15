use std::fmt::Debug;
use anyhow::Result;
use crate::{auth::authenticator::Authenticator, transaction::raw_transaction::RawTransaction};
use crate::auth::auth_session::AuthSession;

#[derive(Debug)]
pub struct SignedTransaction {
    raw_txn: RawTransaction,
    auth_session: AuthSession,
}

impl SignedTransaction
{
    pub fn new(auth_session: AuthSession, raw_txn: RawTransaction) -> Self {
        Self { auth_session, raw_txn }
    }

    pub fn verify(&self) -> Result<()> {
        unimplemented!()
    }
}
