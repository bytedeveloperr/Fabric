use anyhow::{self, Result};
use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::authenticator::Authenticator, transaction::signed_transaction::SignedTransaction,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTransaction {
    payload: RawTransactionPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawTransactionPayload {
    MoveCall(MoveCall),
    Publish(Vec<Module>),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MoveCall {
    module_id: ModuleId,
    function: Identifier,
    ty_args: Vec<TypeTag>,
    args: Vec<Vec<u8>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Module {
    code: Vec<u8>,
}

impl MoveCall {
    pub fn new(
        module_id: ModuleId,
        function: Identifier,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> Self {
        MoveCall {
            module_id,
            function,
            ty_args,
            args,
        }
    }

    pub fn function(&self) -> &Identifier {
        &self.function
    }

    pub fn module_id(&self) -> &ModuleId {
        &self.module_id
    }

    pub fn type_args(&self) -> &Vec<TypeTag> {
        &self.ty_args
    }

    pub fn call_args(&self) -> &Vec<Vec<u8>> {
        &self.args
    }
}

impl Module {
    pub fn new(code: Vec<u8>) -> Self {
        Self { code }
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }
}

impl RawTransaction {
    pub fn new_move_call(move_call: MoveCall) -> Self {
        Self {
            payload: RawTransactionPayload::MoveCall(move_call),
        }
    }

    pub fn new_publish(modules: Vec<Module>) -> Self {
        Self {
            payload: RawTransactionPayload::Publish(modules),
        }
    }

    pub fn payload(&self) -> &RawTransactionPayload {
        &self.payload
    }

    pub fn to_signed<Auth: Authenticator>(
        self,
        credential: &Auth::Credential,
    ) -> Result<SignedTransaction> {
        let identity = Auth::validate_credential(credential).unwrap();
        // TODO: Prevent double TX

        Ok(SignedTransaction::new(identity, self))
    }
}
