use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawTransaction {
    sender_id: Vec<u8>,
    payload: RawTransactionPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawTransactionPayload {
    MoveCall(MoveCall),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MoveCall {
    module: ModuleId,
    function: Identifier,
    ty_args: Vec<TypeTag>,
    // #[serde(with = "vec_bytes")]
    args: Vec<Vec<u8>>,
}

impl MoveCall {
    pub fn new(
        module: ModuleId,
        function: Identifier,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> Self {
        MoveCall {
            module,
            function,
            ty_args,
            args,
        }
    }
}

impl RawTransaction {
    pub fn new_move_call(sender_id: Vec<u8>, move_call: MoveCall) -> RawTransaction {
        RawTransaction {
            sender_id,
            payload: RawTransactionPayload::MoveCall(move_call),
        }
    }
}
