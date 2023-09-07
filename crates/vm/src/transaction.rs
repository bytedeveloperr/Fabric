use std::marker::PhantomData;

use move_core_types::account_address::AccountAddress;

use crate::resolvers::address::AddressResolver;

pub struct RawTransaction<R>
where
    R: AddressResolver,
{
    sender: AccountAddress,
    action: TransactionAction,
    _marker: PhantomData<R>,
}

pub enum TransactionAction {
    Empty,
    PublishPackage(Vec<Vec<u8>>),
}

impl<R> RawTransaction<R>
where
    R: AddressResolver,
{
    pub fn new<T: Clone>(sender: T, resolver: R) -> Self {
        let sender = resolver.get_address(sender).unwrap();

        Self {
            sender,
            _marker: PhantomData,
            action: TransactionAction::Empty,
        }
    }

    pub fn publish_package<T: Clone>(&self, modules: Vec<Vec<u8>>) {
        self.action = TransactionAction::PublishPackage(modules)
    }
}
