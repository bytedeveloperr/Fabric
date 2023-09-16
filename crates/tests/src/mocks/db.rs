use std::collections::BTreeMap;
use std::sync::Mutex;

use anyhow::{Ok, Result};
use lazy_static::lazy_static;

use fabric_storage::stores::{KeyCodec, Store, ValueCodec};

#[derive(Default)]
pub struct MockDB {
    state: BTreeMap<Vec<u8>, Vec<u8>>,
}

lazy_static! {
    static ref MOCK_DB: Mutex<MockDB> = Mutex::new(MockDB::default());
}

//
// impl MockDB {
//     fn remove<S: Store>(&self, key: &S::Key) -> Result<Option<S::Value>>;
// }

impl MockDB {
    pub fn get<S: Store>(&self, key: &S::Key) -> Result<Option<S::Value>> {
        let key = <S::Key as KeyCodec<S>>::encode(key)?;

        let db = MOCK_DB.lock().unwrap();

        db.state
            .get(&key)
            .map(|v| <S::Value as ValueCodec<S>>::decode(v).map_err(Into::into))
            .transpose()
    }

    pub fn insert<S: Store>(&self, key: &S::Key, value: S::Value) -> Result<()> {
        let key = <S::Key as KeyCodec<S>>::encode(key)?;
        let value = <S::Value as ValueCodec<S>>::encode(&value)?;

        let mut db = MOCK_DB.lock().unwrap();

        db.state.insert(key, value);
        Ok(())
    }
}
