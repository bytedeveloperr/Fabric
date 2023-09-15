use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use anyhow::{Ok, Result};
use fabric_types::{access_path::AccessPath, account_state::AccountState};
use move_core_types::account_address::AccountAddress;
use parking_lot::RwLock;

use crate::stores::{state::StateStore, Store};

pub trait StateReader {
    fn get(&self, access_path: &AccessPath) -> Result<Option<Vec<u8>>>;
}

pub struct StoreStateReader<'s> {
    pub store: &'s Arc<StateStore>,
    pub accounts_cache: RwLock<HashMap<AccountAddress, AccountState>>,
}

impl<'s> StoreStateReader<'s> {
    pub fn new(store: &'s Arc<StateStore>) -> Self {
        Self {
            store,
            accounts_cache: RwLock::default(),
        }
    }
}

impl<'s> StateReader for StoreStateReader<'s> {
    fn get(&self, access_path: &AccessPath) -> Result<Option<Vec<u8>>> {
        let address = access_path.address;
        let path = &access_path.path;

        if let Some(account) = self.accounts_cache.read().get(&address) {
            return Ok(account.get(path).cloned());
        }

        let raw_account = self.store.get(&address)?;
        let account = raw_account
            .map(|account| AccountState::try_from(&account))
            .transpose()?
            .unwrap_or_default();

        let value = match self.accounts_cache.write().entry(address) {
            Entry::Occupied(value) => value.get().get(path).cloned(),
            Entry::Vacant(value) => value.insert(account).get(path).cloned(),
        };

        Ok(value)
    }
}
