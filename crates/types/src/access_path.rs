use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, ResourceKey, StructTag},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AccessPath {
    pub address: AccountAddress,
    pub path: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum Path {
    Code(ModuleId),
    Resource(StructTag),
}

impl AccessPath {
    pub fn new(address: AccountAddress, path: Vec<u8>) -> Self {
        Self { address, path }
    }

    pub fn to_path(&self) -> Path {
        bcs::from_bytes::<Path>(&self.path).expect("Failed to deserialize path")
    }
}

impl From<ResourceKey> for AccessPath {
    fn from(value: ResourceKey) -> Self {
        let path =
            bcs::to_bytes(&Path::Resource(value.type_)).expect("Failed to serialize resource key");

        Self {
            address: value.address,
            path,
        }
    }
}

impl From<ModuleId> for AccessPath {
    fn from(value: ModuleId) -> Self {
        let address = *value.address();
        let path = bcs::to_bytes(&Path::Code(value)).expect("Failed to serialize module id");

        Self { address, path }
    }
}

impl TryFrom<&[u8]> for Path {
    type Error = bcs::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes::<Path>(value)
    }
}

impl TryFrom<&Vec<u8>> for Path {
    type Error = bcs::Error;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        bcs::from_bytes::<Path>(value)
    }
}
