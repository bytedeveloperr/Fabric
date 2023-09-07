use crate::state::{FabricState, MoveModule};
use anyhow::ensure;
use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag},
    resolver::{ModuleResolver, ResourceResolver},
};
use sled::Db;

pub struct FabricResolver {
    state: FabricState,
}

impl ModuleResolver for FabricResolver {
    type Error = anyhow::Error;

    fn get_module(&self, id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        self.state
            .resolve_module(id)?
            .map(|m| {
                // TODO: Some more validations here

                let module = bcs::from_bytes::<MoveModule>(&m.value).unwrap();
                Ok(module.byte_code)
            })
            .transpose()
    }
}

impl ResourceResolver for FabricResolver {
    type Error = anyhow::Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        self.state
            .resolve_resource(address, typ)?
            .map(|r| {
                ensure!(
                    r.validate_type(typ),
                    "Resource type mismatch, expected: {:?}, found: {:?}",
                    r.type_tag,
                    typ
                );

                Ok(r.value)
            })
            .transpose()
    }
}

impl FabricResolver {
    pub fn new(db: Db) -> Self {
        Self {
            state: FabricState::new(db),
        }
    }

    pub fn new_inmemory() -> Self {
        let db = sled::Config::new().temporary(true).open().unwrap();

        Self {
            state: FabricState::new(db),
        }
    }
}
