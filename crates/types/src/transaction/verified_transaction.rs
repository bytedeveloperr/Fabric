use std::fmt::Debug;

use crate::auth::auth_session::AuthSession;
use crate::transaction::raw_transaction::RawTransaction;

#[derive(Debug)]
pub struct VerifiedTransaction {
    raw_txn: RawTransaction,
    auth_session: AuthSession,
}

impl VerifiedTransaction {
    pub fn new(auth_session: AuthSession, raw_txn: RawTransaction) -> Self {
        Self {
            auth_session,
            raw_txn,
        }
    }
}
