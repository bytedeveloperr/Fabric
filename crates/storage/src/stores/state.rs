use std::sync::Arc;

use move_core_types::account_address::AccountAddress;
use move_core_types::effects::ChangeSet;

use fabric_types::account_state::{AccountChanges, AccountState};
use fabric_types::raw_account_state::RawAccountState;

use crate::db::DB;

use super::{KeyCodec, Store, ValueCodec};

pub struct StateStore {
    db: Arc<DB>,
}

impl StateStore {
    pub fn new(db: DB) -> Self {
        Self { db: Arc::new(db) }
    }

    pub fn apply_change_set(&self, change_set: ChangeSet) -> anyhow::Result<()> {
        let accounts_change_set = change_set.into_inner();

        for (address, account_change_set) in accounts_change_set.into_iter() {
            let raw_account = self.get(&address)?.unwrap_or(RawAccountState::default());
            let mut account_state = AccountState::try_from(&raw_account)?;

            let (modules, resources) = account_change_set.into_inner();
            account_state.insert_modules(address, modules)?;
            account_state.insert_resources(resources)?;
        }

        Ok(())
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
