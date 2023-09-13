use crate::account_state::AccountState;

#[derive(Default, Clone, Debug)]
pub struct RawAccountState {
    pub state: Vec<u8>,
}

impl TryFrom<&AccountState> for RawAccountState {
    type Error = bcs::Error;

    fn try_from(value: &AccountState) -> Result<Self, Self::Error> {
        Ok(Self {
            state: bcs::to_bytes(value)?,
        })
    }
}

impl From<&[u8]> for RawAccountState {
    fn from(value: &[u8]) -> Self {
        Self {
            state: value.to_vec(),
        }
    }
}

impl From<&RawAccountState> for Vec<u8> {
    fn from(value: &RawAccountState) -> Self {
        value.state.clone()
    }
}

impl From<RawAccountState> for Vec<u8> {
    fn from(value: RawAccountState) -> Self {
        Self::from(&value)
    }
}

impl From<Vec<u8>> for RawAccountState {
    fn from(value: Vec<u8>) -> Self {
        RawAccountState { state: value }
    }
}

impl AsRef<[u8]> for RawAccountState {
    fn as_ref(&self) -> &[u8] {
        &self.state
    }
}
