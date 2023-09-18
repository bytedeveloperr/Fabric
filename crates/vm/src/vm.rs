use std::sync::Arc;

use move_core_types::resolver::MoveResolver;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_runtime::session::Session;

use fabric_types::transaction::verified_transaction::VerifiedTransaction;

use crate::natives::all_natives;

pub struct FabricVM {
    move_vm: Arc<MoveVM>,
}

impl FabricVM {
    pub fn new() -> Self {
        let move_vm =
            MoveVM::new(all_natives()).expect("Failure: MoveVM must be able to be created");

        Self {
            move_vm: Arc::new(move_vm),
        }
    }

    pub fn new_session<'r, R: MoveResolver>(&self, resolver: &'r R) -> Session<'r, '_, R> {
        self.move_vm.new_session(resolver)
    }
}

impl VMValidator for FabricVM {
    fn validate_transaction(&self, transaction: VerifiedTransaction) -> anyhow::Result<()> {
        // TODO: check Gas and run prologu

        Ok(())
    }
}

pub trait VMValidator {
    fn validate_transaction(&self, transaction: VerifiedTransaction) -> anyhow::Result<()>;
}
