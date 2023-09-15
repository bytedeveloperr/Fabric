use std::fmt::Debug;

use anyhow::Result;

use crate::auth::auth_session::AuthSession;
use crate::transaction::raw_transaction::RawTransaction;

#[derive(Debug)]
pub struct SignedTransaction {
    raw_txn: RawTransaction,
    auth_session: AuthSession,
}

impl SignedTransaction {
    pub fn new(auth_session: AuthSession, raw_txn: RawTransaction) -> Self {
        Self {
            auth_session,
            raw_txn,
        }
    }

    pub fn verify(&self) -> Result<()> {
        unimplemented!()
    }
}
