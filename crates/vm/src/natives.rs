use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_stdlib::natives::GasParameters;
use move_vm_runtime::native_functions::NativeFunctionTable;

pub fn move_stdlib_natives() -> NativeFunctionTable {
    move_stdlib::natives::all_natives(CORE_CODE_ADDRESS, GasParameters::zeros())
}

pub fn all_natives() -> NativeFunctionTable {
    move_stdlib_natives()
}
