use move_core_types::account_address::AccountAddress;
use crate::identifier::Identifier;

#[derive(Debug)]
pub struct AuthSession {
    expires_at: u64,
    metadata: Option<Vec<u8>>,
    account: AuthSessionAccount
}

#[derive(Clone, Debug)]
pub struct AuthSessionAccount {
    identifier: Identifier,
    address: AccountAddress,
}

impl AuthSession {
    pub fn new(expires_at: u64, account: AuthSessionAccount, metadata: Option<Vec<u8>>) -> Self{
        Self {
            expires_at,
            account,
            metadata
        }
    }

    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }

    pub fn account(&self) -> &AuthSessionAccount {
        &self.account
    }
}

impl AuthSessionAccount {
    pub fn new(address: AccountAddress, identifier: Identifier) -> Self {
        Self { address, identifier }
    }

    pub fn address(&self) -> &AccountAddress {
        &self.address
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl Default for AuthSession {
    fn default() -> Self {
        Self { expires_at: 0, account: AuthSessionAccount::default(), metadata: None }
    }
}

impl Default for AuthSessionAccount {
    fn default() -> Self {
        Self {
            address: AccountAddress::random(),
            identifier: Identifier::u64(0)
        }
    }
}

