use anyhow::Result;
use move_core_types::{language_storage::StructTag, move_resource::MoveResource};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{access_path::Path, raw_account_state::RawAccountState};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountState(BTreeMap<Vec<u8>, Vec<u8>>);

impl AccountState {
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.0.get(key)
    }

    pub fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.remove(key)
    }

    pub fn get_resource<T: MoveResource>(&self) -> Result<Option<T>> {
        self.get_resource_impl(&T::struct_tag().access_vector())
    }

    pub fn get_modules(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.0
            .iter()
            .filter_map(|(k, v)| match Path::try_from(k).expect("msg") {
                Path::Resource(_) => None,
                Path::Code(_) => Some(v),
            })
    }

    pub fn get_resources(&self) -> impl Iterator<Item = (StructTag, &[u8])> {
        self.0
            .iter()
            .filter_map(|(k, v)| match Path::try_from(k).expect("msg") {
                Path::Resource(t) => Some((t, v.as_ref())),
                Path::Code(_) => None,
            })
    }

    pub fn get_resource_impl<T: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<T>> {
        self.0
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
