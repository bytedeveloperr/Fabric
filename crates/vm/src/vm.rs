use std::sync::Arc;

use move_core_types::resolver::MoveResolver;
use move_vm_runtime::session::Session;

use fabric_types::transaction::verified_transaction::VerifiedTransaction;
use fabric_types::validator::VMValidator;

use crate::vm_impl::FabricVmImpl;

pub struct FabricVM(Arc<FabricVmImpl>);

impl FabricVM {
    pub fn new() -> Self {
        Self(Arc::new(FabricVmImpl::new_with_move_vm()))
    }

    pub fn new_session<'r, R: MoveResolver>(&self, resolver: &'r R) -> Session<'r, '_, R> {
        self.0.new_session(resolver)
    }
}

impl VMValidator for FabricVM {
    fn validate_transaction(&self, _transaction: VerifiedTransaction) -> anyhow::Result<()> {
        // TODO: check Gas and run prologue

        Ok(())
    }
}
