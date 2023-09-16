use std::collections::BTreeMap;

use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_core_types::effects::Op;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::{language_storage::StructTag, move_resource::MoveResource};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{access_path::Path, raw_account_state::RawAccountState};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountState {
    address: AccountAddress,
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

pub trait AccountChanges {
    fn insert_modules(&mut self, modules: BTreeMap<Identifier, Op<Vec<u8>>>) -> Result<()>;

    fn insert_resources(&mut self, resources: BTreeMap<StructTag, Op<Vec<u8>>>) -> Result<()>;
}

impl AccountState {
    pub fn new(address: AccountAddress) -> Self {
        Self {
            address,
            data: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.data.insert(key, value)
    }

    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.remove(key)
    }

    pub fn get_resource<T: MoveResource>(&self) -> Result<Option<T>> {
        self.get_resource_impl(&T::struct_tag().access_vector())
    }

    pub fn get_modules(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.data
            .iter()
            .filter_map(|(k, v)| match Path::try_from(k).expect("msg") {
                Path::Resource(_) => None,
                Path::Code(_) => Some(v),
            })
    }

    pub fn get_resources(&self) -> impl Iterator<Item = (StructTag, &[u8])> {
        self.data
            .iter()
            .filter_map(|(k, v)| match Path::try_from(k).expect("msg") {
                Path::Resource(t) => Some((t, v.as_ref())),
                Path::Code(_) => None,
            })
    }

    pub fn get_resource_impl<T: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<T>> {
        self.data
            .get(key)
            .map(|value| bcs::from_bytes(value))
            .transpose()
            .map_err(Into::into)
    }
}

impl Into<Vec<u8>> for AccountState {
    fn into(self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

impl From<Vec<u8>> for AccountState {
    fn from(value: Vec<u8>) -> Self {
        bcs::from_bytes(&value).unwrap()
    }
}

impl TryFrom<&RawAccountState> for AccountState {
    type Error = bcs::Error;

    fn try_from(value: &RawAccountState) -> Result<Self, Self::Error> {
        bcs::from_bytes(&value.state)
    }
}

impl TryFrom<RawAccountState> for AccountState {
    type Error = bcs::Error;

    fn try_from(value: RawAccountState) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl AccountChanges for AccountState {
    fn insert_modules(&mut self, modules: BTreeMap<Identifier, Op<Vec<u8>>>) -> Result<()> {
        for (id, operation) in modules.into_iter() {
            let module_id = ModuleId::new(self.address, id).access_vector();

            match operation {
                Op::New(code) | Op::Modify(code) => {
                    self.insert(module_id.clone(), code.clone());
                }
                Op::Delete => {
                    self.remove(&module_id);
                }
            }
        }

        Ok(())
    }

    fn insert_resources(&mut self, resources: BTreeMap<StructTag, Op<Vec<u8>>>) -> Result<()> {
        for (tag, operation) in resources.into_iter() {
            let tag = tag.access_vector();

            match operation {
                Op::New(content) | Op::Modify(content) => {
                    self.insert(tag, content.clone());
                }
                Op::Delete => {
                    self.remove(&tag);
                }
            }
        }

        Ok(())
    }
}
