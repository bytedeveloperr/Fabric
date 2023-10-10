pub trait VMValidator {
    fn validate_transaction(&self, transaction: VerifiedTransaction) -> anyhow::Result<()>;
}
