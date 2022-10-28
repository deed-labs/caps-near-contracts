use near_sdk::Balance;

pub const YOCTO_PER_BYTE: Balance = 10_000_000_000_000_000_000;

pub const NO_DEPOSIT: Balance = 0;

pub mod gas {
    use near_sdk::Gas;

    const fn gas(n: u64) -> Gas {
        Gas(n * 10u64.pow(12))
    }

    pub const CREATE_SOULBOUND: Gas = gas(70);
    pub const UPDATE_SOULBOUND: Gas = gas(10);

    pub const ON_CREATE_CALLBACK: Gas = gas(10);
}

pub mod storage_bytes {
    use near_sdk::StorageUsage;

    pub const SOULBOUND: StorageUsage = 550_000;

    pub const COMMON: StorageUsage = 80;
}

pub mod storage_cost {
    use near_sdk::Balance;
    use super::YOCTO_PER_BYTE;

    const fn bytes_to_cost(bytes: u64) -> Balance {
        (bytes as Balance) * YOCTO_PER_BYTE
    }

    pub const STORE: Balance = bytes_to_cost(super::storage_bytes::SOULBOUND);
}