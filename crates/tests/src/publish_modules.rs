use move_core_types::account_address::AccountAddress;
use move_vm_types::gas::UnmeteredGasMeter;

use fabric_vm::vm::FabricVM;

use crate::helpers::compiler::{compile_code, get_module};
use crate::mocks::storage::InMemoryStorage;

const CODE: &'static str = r#"
    module {{ADDR}}::test {
        struct Foo has key { a: bool }
        struct Bar has key { a: u64 }

        public fun get(addr: address): bool acquires Foo {
            borrow_global<Foo>(addr).a
        }

        public fun flip(addr: address) acquires Foo {
            let f_ref = borrow_global_mut<Foo>(addr);
            f_ref.a = !f_ref.a;
        }

        public entry fun publish(addr: &signer) {
            move_to(addr, Foo { a: true } );
            move_to(addr, Bar { a: 543 } )
        }
    }
"#;

fn format_code(address: AccountAddress) -> String {
    CODE.replace("{{ADDR}}", &format!("{}", address.to_hex_literal()))
}

#[test]
fn publish_single_module() {
    let address = AccountAddress::random();
    let compiled_unit = get_module(compile_code(format_code(address)).unwrap().pop().unwrap());

    let mut modules = vec![];
    compiled_unit.serialize(&mut modules).unwrap();

    let vm = FabricVM::new();
    let mut storage = InMemoryStorage::default();
    let mut session = vm.new_session(&storage);

    session
        .publish_module(modules.clone(), address, &mut UnmeteredGasMeter)
        .unwrap();

    let results = session.finish().unwrap();
    storage.apply_results(results);

    let mut session = vm.new_session(&storage);
    session
        .publish_module(modules, address, &mut UnmeteredGasMeter)
        .unwrap();

    let results = session.finish().unwrap();
    storage.apply_results(results);

    println!("{:?}", storage)
}
