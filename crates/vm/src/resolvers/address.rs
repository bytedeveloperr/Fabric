use move_core_types::account_address::AccountAddress;
use std::fmt::Debug;

pub trait AddressResolver {
    type Error: Debug;

    fn get_address<T: Clone>(&self, name: T) -> Result<AccountAddress, Self::Error>;
}
