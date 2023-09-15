use anyhow::Ok;
use fabric_types::access_path::AccessPath;
use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, ResourceKey, StructTag},
    resolver::{ModuleResolver, ResourceResolver},
};
use std::collections::BTreeMap;

use crate::readers::state::StateReader;

pub struct DataCache<'s, S: StateReader> {
    pub reader: &'s S,
    pub data_map: BTreeMap<AccessPath, Option<Vec<u8>>>,
}

impl<'s, S: StateReader> DataCache<'s, S> {
    pub fn new(reader: &'s S) -> Self {
        Self {
            reader,
            data_map: BTreeMap::default(),
        }
    }
}

impl<'s, S: StateReader> ResourceResolver for DataCache<'s, S> {
    type Error = anyhow::Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        let resource_key = ResourceKey::new(address.clone(), typ.clone());
        let access_path = AccessPath::from(resource_key);
        Ok(self.reader.get(&access_path)?)
    }
}

impl<'s, S> ModuleResolver for DataCache<'s, S>
where
    S: StateReader,
{
    type Error = anyhow::Error;

    fn get_module(&self, id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        let access_path = AccessPath::from(id.clone());

        Ok(self.reader.get(&access_path)?)
    }
}
