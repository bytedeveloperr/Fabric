use std::{str::FromStr, sync::Arc};

use fabric_types::{
    access_path::AccessPath, account_state::AccountState, raw_account_state::RawAccountState,
};
use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ResourceKey, StructTag},
    resolver::ResourceResolver,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::DB,
    resolver::StateResolver,
    stores::{state::StateStore, Store},
};

const TYPE_NAME: &'static str = "0x919b72c2271c0c8e165fb40356505042::module::Struct";

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResource {
    pub value: String,
}

impl TestResource {
    pub fn new(st: &str) -> Self {
        Self {
            value: st.to_string(),
        }
    }
}

#[test]
fn init() {
    let db = DB::default();
    let state = Arc::new(StateStore::new(db));
    let resolver = StateResolver::new(state.clone());
    let mut account_state = AccountState::default();

    let struct_tag = StructTag::from_str(TYPE_NAME).unwrap();
    let address = AccountAddress::random();

    let resource = ResourceKey::new(address, struct_tag.clone());
    let access_path = AccessPath::from(resource);

    let r1 = TestResource::new("hmmmmm1");
    let r2 = TestResource::new("heyy");
    account_state.insert(access_path.path.clone(), bcs::to_bytes(&r1).unwrap());
    let raw_account_state = RawAccountState::try_from(&account_state).unwrap();

    state.insert(&address, raw_account_state).unwrap();

    println!(
        "{:#?}",
        bcs::from_bytes::<TestResource>(
            &resolver
                .get_resource(&address, &struct_tag)
                .unwrap()
                .unwrap()
        )
        .unwrap()
    );

    account_state.insert(access_path.path.clone(), bcs::to_bytes(&r2).unwrap());
    let raw_account_state = RawAccountState::try_from(&account_state).unwrap();

    state.insert(&address, raw_account_state).unwrap();

    // let ret = account_state
    //     .get_resource_impl::<TestResource>(&access_path.path)
    //     .unwrap();

    // println!("{:#?}", state.get(&address).unwrap().unwrap());
    // println!("{:#?}", state.remove(&address).unwrap().unwrap());
    println!(
        "{:#?}",
        bcs::from_bytes::<TestResource>(
            &resolver
                .get_resource(&address, &struct_tag)
                .unwrap()
                .unwrap()
        )
        .unwrap()
    );
    // println!("{:?}", account_state);
    // println!("{:?}", raw_account_state);
}
