use fabric_storage::storage::FabricStorage;
use move_core_types::account_address::AccountAddress;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas::UnmeteredGasMeter;

pub struct FabricVM {
    move_vm: MoveVM,
    storage: FabricStorage,
}

impl FabricVM {
    pub fn publish_package(&self, tx: Raw) {
        let sender = AccountAddress::from_hex_literal("0x1").unwrap();

        let mut session = self.move_vm.new_session(self.storage.get_resolver());
        session
            .publish_module_bundle(vec![], sender, &mut UnmeteredGasMeter)
            .unwrap();

        let _result = session.finish();
    }
}

impl Default for FabricVM {
    fn default() -> Self {
        Self {
            storage: FabricStorage::new(),
            move_vm: MoveVM::new(vec![]).unwrap(),
        }
    }
}
