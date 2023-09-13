use std::sync::Arc;

use crate::{
    db::DB,
    stores::{state::StateStore, Store},
};

use fabric_types::account_state::AccountState;
use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag},
    resolver::{ModuleResolver, ResourceResolver},
};

pub struct StateResolver {
    store: Arc<StateStore>,
}

impl ModuleResolver for StateResolver {
    type Error = anyhow::Error;

    fn get_module(&self, id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        let raw_account = self.store.get(id.address())?;

        Ok(match raw_account {
            Some(raw_account) => {
                let account_state = AccountState::try_from(&raw_account)?;
                account_state.get(&id.access_vector()).cloned()
            }
            None => None,
        })
    }
}

impl ResourceResolver for StateResolver {
    type Error = anyhow::Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        let raw_account = self.store.get(address)?.unwrap_or_default();
        let account_state = AccountState::try_from(&raw_account)?;
        Ok(account_state.get(&typ.access_vector()).cloned())
    }
}

impl StateResolver {
    pub fn new(store: Arc<StateStore>) -> Self {
        Self { store }
    }

    pub fn new_inmemory() -> Self {
        let db = DB::default();

        Self {
            store: Arc::new(StateStore::new(db)),
        }
    }
}
