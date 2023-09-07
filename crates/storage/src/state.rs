use anyhow::{Ok, Result};
use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag, TypeTag},
};
use serde::{Deserialize, Serialize};
use sled::Db;

pub struct FabricState {
    db: Db,
}

#[derive(Serialize, Deserialize)]
pub struct MoveModule {
    pub name: String,
    pub byte_code: Vec<u8>,
    pub address: AccountAddress,
}

#[derive(Serialize, Deserialize)]
pub struct FabricStateValue {
    pub value: Vec<u8>,
    pub type_tag: TypeTag,
}

impl FabricStateValue {
    pub fn validate_type(&self, typ: &StructTag) -> bool {
        match &self.type_tag {
            TypeTag::Struct(tag) => tag.as_ref() == typ,
            _ => false,
        }
    }
}

impl FabricState {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn resolve_module(&self, id: &ModuleId) -> Result<Option<FabricStateValue>> {
        Ok(self
            .db
            .open_tree(bcs::to_bytes(id.address())?)?
            .get(id.name().as_bytes())?
            .map(|v| bcs::from_bytes::<FabricStateValue>(&v))
            .transpose()?)
    }

    pub fn resolve_resource(
        &self,
        address: &AccountAddress,
        key: &StructTag,
    ) -> Result<Option<FabricStateValue>> {
        Ok(self
            .db
            .open_tree(bcs::to_bytes(&address)?)?
            .get(bcs::to_bytes(&key)?)?
            .map(|v| bcs::from_bytes::<FabricStateValue>(&v))
            .transpose()?)
    }
}
