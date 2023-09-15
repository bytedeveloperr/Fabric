use std::{env::temp_dir, fs::File, io::Write, str::FromStr, sync::Arc};

use anyhow::{Ok, Result};
use move_binary_format::CompiledModule;
use move_compiler::{compiled_unit::AnnotatedCompiledUnit, Compiler};
use move_core_types::{identifier::Identifier, language_storage::ModuleId, value::MoveValue};
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas::UnmeteredGasMeter;

use fabric_types::{
    auth::authenticator::{Authenticator, MockAuthenticator},
    transaction::raw_transaction::{MoveCall, RawTransaction},
    transaction::signed_transaction::SignedTransaction,
};

use crate::{
    data_cache::DataCache, db::DB, readers::state::StoreStateReader, stores::state::StateStore,
};

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

#[test]
fn init() {
    let vm = MoveVM::new(vec![]).unwrap();
    let db = DB::default();
    let state = Arc::new(StateStore::new(db));

    let auth_session = MockAuthenticator::validate_credential(&vec![]).unwrap();
    let addr = auth_session.account().address().clone();
    let module = ModuleId::new(addr, Identifier::from_str("test").unwrap());
    let function = Identifier::from_str("publish").unwrap();

    let move_call = MoveCall::new(module, function, vec![], vec![]);
    let raw_txn = RawTransaction::new_move_call(move_call);

    // println!("{:#?}", raw_txn);

    let _signed_tx = SignedTransaction::new(auth_session, raw_txn);
    // println!("{:#?}", signed_tx);

    let code = CODE.replace("{{ADDR}}", &format!("{}", addr.to_hex_literal()));

    let mut compiled = compile(code).unwrap();
    let unit = get_module(compiled.pop().unwrap());

    let reader = StoreStateReader::new(&state);
    let cache = DataCache::new(&reader);
    let mut session = vm.new_session(&cache);
    let mut v = vec![];
    unit.serialize(&mut v).unwrap();

    session
        .publish_module(v, addr, &mut UnmeteredGasMeter)
        .unwrap();

    let module = ModuleId::new(addr, Identifier::from_str("test").unwrap());
    let function = Identifier::from_str("publish").unwrap();

    let signer = MoveValue::Signer(addr).simple_serialize().unwrap();
    session
        .execute_entry_function(
            &module,
            function.as_ident_str(),
            vec![],
            vec![signer],
            &mut UnmeteredGasMeter,
        )
        .unwrap();

    let res = session.finish().unwrap();

    println!("{:#?}", res);
}

fn compile(code: String) -> Result<Vec<AnnotatedCompiledUnit>> {
    let dir = temp_dir();

    let file_path = dir.as_path().join("module.move");
    {
        let mut file = File::create(&file_path).unwrap();
        write!(&mut file, "{}", code).unwrap();
    }

    let compiler = Compiler::from_files(
        vec![file_path.to_str().unwrap().to_string()],
        vec![],
        move_stdlib::move_stdlib_named_addresses(),
    );

    let (_, units) = compiler.build_and_report().unwrap();
    Ok(units)
}

fn get_module(unit: AnnotatedCompiledUnit) -> CompiledModule {
    match unit {
        AnnotatedCompiledUnit::Module(module) => module.named_module.module,
        _ => panic!(""),
    }
}
