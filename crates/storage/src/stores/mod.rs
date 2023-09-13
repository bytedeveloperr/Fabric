use anyhow::Result;
use std::fmt::Debug;

pub mod state;

pub trait Store: Sized {
    const STORE_NAME: &'static str;

    type Key: KeyCodec<Self>;

    type Value: ValueCodec<Self>;

    fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>>;
    fn insert(&self, key: &Self::Key, value: Self::Value) -> Result<()>;
    fn remove(&self, key: &Self::Key) -> Result<Option<Self::Value>>;
}

pub trait KeyCodec<S: Store + ?Sized>: Sized + Debug {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(data: &[u8]) -> Result<Self>;
}

pub trait ValueCodec<S: Store + ?Sized>: Sized + Debug {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(data: &[u8]) -> Result<Self>;
}
