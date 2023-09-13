use std::fmt::Debug;

use crate::{authenticator::Authenticator, raw_transaction::RawTransaction};

#[derive(Debug)]
pub struct SignedTransaction<A: Authenticator> {
    raw_txn: RawTransaction,
    authenticator: A,
}

impl<A> SignedTransaction<A>
where
    A: Authenticator,
{
    pub fn new(raw_txn: RawTransaction, authenticator: A) -> Self {
        Self {
            raw_txn,
            authenticator,
        }
    }
}
