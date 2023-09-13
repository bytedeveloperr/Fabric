use super::{KeyCodec, Store, ValueCodec};
use crate::db::DB;
use fabric_types::raw_account_state::RawAccountState;
use move_core_types::account_address::AccountAddress;
use std::sync::Arc;

pub struct StateStore {
    db: Arc<DB>,
}

impl StateStore {
    pub fn new(db: DB) -> Self {
        Self { db: Arc::new(db) }
    }
}

impl Store for StateStore {
    const STORE_NAME: &'static str = "state";

    type Key = AccountAddress;

    type Value = RawAccountState;

    fn get(&self, key: &Self::Key) -> anyhow::Result<Option<Self::Value>> {
        self.db.get::<StateStore>(key)
    }

    fn insert(&self, key: &Self::Key, value: Self::Value) -> anyhow::Result<()> {
        self.db.insert::<StateStore>(key, value)
    }

    fn remove(&self, key: &Self::Key) -> anyhow::Result<Option<Self::Value>> {
        self.db.remove::<StateStore>(key)
    }
}

impl KeyCodec<StateStore> for AccountAddress {
    fn encode(&self) -> anyhow::Result<Vec<u8>> {
        bcs::to_bytes(&self).map_err(Into::into)
    }

    fn decode(data: &[u8]) -> anyhow::Result<Self> {
        AccountAddress::from_bytes(data).map_err(Into::into)
    }
}

impl ValueCodec<StateStore> for RawAccountState {
    fn encode(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self.state.clone())
    }

    fn decode(data: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::from(data))
    }
}
