module fabric::account {
    struct Account has key {
        // TODO: add identifier?

        sequence_number: u64
    }

    const EAccountAlreadyExists: u64 = 0;

    public(friend) fun create(addr: address) {
        assert!(!exists<Account>(addr), EAccountAlreadyExists);
    }

    fun create_internal() {
        let new_account = Account { sequence_number: 0 };
    }
}