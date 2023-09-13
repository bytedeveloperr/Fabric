use std::sync::Arc;

use crate::stores::{KeyCodec, Store, ValueCodec};
use anyhow::{Ok, Result};

pub struct DB {
    inner: Arc<sled::Db>,
}

impl DB {
    pub fn get<S: Store>(&self, key: &S::Key) -> Result<Option<S::Value>> {
        let tree = self.inner.open_tree(S::STORE_NAME)?;
        let k = <S::Key as KeyCodec<S>>::encode(key)?;

        tree.get(k)?
            .map(|v| <S::Value as ValueCodec<S>>::decode(&v).map_err(Into::into))
            .transpose()
    }

    pub fn insert<S: Store>(&self, key: &S::Key, value: S::Value) -> Result<()> {
        let tree = self.inner.open_tree(S::STORE_NAME)?;
        let k = <S::Key as KeyCodec<S>>::encode(key)?;

        tree.insert(k, <S::Value as ValueCodec<S>>::encode(&value)?)?;
        Ok(())
    }

    pub fn remove<S: Store>(&self, key: &S::Key) -> Result<Option<S::Value>> {
        let tree = self.inner.open_tree(S::STORE_NAME)?;
        let k = <S::Key as KeyCodec<S>>::encode(key)?;

        tree.remove(k)?
            .map(|v| <S::Value as ValueCodec<S>>::decode(&v).map_err(Into::into))
            .transpose()
    }
}

impl Default for DB {
    fn default() -> Self {
        let config = sled::Config::default().temporary(true);
        let db = config.open().expect("Failed to open db");

        Self {
            inner: Arc::new(db),
        }
    }
}
