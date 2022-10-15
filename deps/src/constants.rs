use near_sdk::Balance;

pub const ONE_YOCTO: Balance = 1;
pub const NO_DEPOSIT: Balance = 0;

pub mod gas {
    use near_sdk::Gas;

    const fn gas(n: u64) -> Gas {
        Gas(n * 10u64.pow(12))
    }

    pub const CREATE_SOULBOUND: Gas = gas(70);

    pub const ON_CREATE_CALLBACK: Gas = gas(10);
}
