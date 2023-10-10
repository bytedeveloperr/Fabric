use move_core_types::resolver::MoveResolver;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_runtime::session::Session;

use crate::natives::all_natives;

pub struct FabricVmImpl {
    move_vm: MoveVM,
}

impl FabricVmImpl {
    pub fn new_with_move_vm() -> Self {
        let move_vm = MoveVM::new(all_natives()).expect("cannot create move natives");
        Self { move_vm }
    }

    pub fn new_session<'r, S: MoveResolver>(&self, remote: &'r S) -> Session<'r, '_, S> {
        self.move_vm.new_session(remote)
    }
}
