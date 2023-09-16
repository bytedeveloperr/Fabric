use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_core_types::effects::{ChangeSet, Event, Op};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::resolver::{ModuleResolver, ResourceResolver};

#[derive(Default, Debug)]
pub struct InMemoryStorage {
    accounts: BTreeMap<AccountAddress, InMemoryAccountStorageData>,
}

#[derive(Default, Debug)]
pub struct InMemoryAccountStorageData {
    resources: BTreeMap<StructTag, Vec<u8>>,
    modules: BTreeMap<Identifier, Vec<u8>>,
}

impl InMemoryStorage {
    pub fn apply_results(&mut self, results: (ChangeSet, Vec<Event>)) {
        let changeset = results.0;
        let _event = results.1;

        let inner = changeset.into_inner();

        for (address, account_changeset) in inner.into_iter() {
            let (modules, resources) = account_changeset.into_inner();

            match self.accounts.entry(address) {
                Entry::Vacant(account) => {
                    let mut storage = InMemoryAccountStorageData::default();
                    apply_changes(&mut storage.modules, modules);
                    apply_changes(&mut storage.resources, resources);

                    account.insert(storage);
                }
                Entry::Occupied(mut account) => {
                    let storage = account.get_mut();
                    apply_changes(&mut storage.modules, modules);
                    apply_changes(&mut storage.resources, resources);
                }
            }
        }
    }
}

impl ResourceResolver for InMemoryStorage {
    type Error = Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self
            .accounts
            .get(&address)
            .map(|storage| storage.resources.get(typ).cloned())
            .unwrap_or(None))
    }
}

impl ModuleResolver for InMemoryStorage {
    type Error = Error;

    fn get_module(&self, id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        let address = id.address();
        Ok(self
            .accounts
            .get(address)
            .map(|storage| storage.modules.get(id.name()).cloned())
            .unwrap_or(None))
    }
}

fn apply_changes<K, V>(tree: &mut BTreeMap<K, V>, data: BTreeMap<K, Op<V>>)
where
    K: Ord,
{
    for (key, op) in data {
        match op {
            Op::New(v) => {
                tree.insert(key, v);
            }
            Op::Modify(v) => {
                tree.entry(key).and_modify(|existing| *existing = v);
            }
            Op::Delete => {
                tree.remove(&key);
            }
        };
    }
}
