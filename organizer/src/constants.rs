use near_sdk::Balance;
pub const YOCTO_PER_BYTE: Balance = 10_000_000_000_000_000_000;


pub mod storage_bytes {
    use near_sdk::StorageUsage;
    /// Storage bytes that a raw store occupies, about 499 KB.
    pub const STORE: StorageUsage = 550_000;

    /// Storage bytes for a maximum size token without any metadata and without
    /// any royalties.
    pub const TOKEN: StorageUsage = 360;

    /// Storage bytes for some common components:
    ///
    /// - a single royalty
    /// - a single approval
    /// - an entry in the `tokens_per_account` map
    /// - an entry in the `composeables` map
    pub const COMMON: StorageUsage = 80;
}

pub mod storage_stake {
    use near_sdk::Balance;
    use super::YOCTO_PER_BYTE;

    const fn bytes_to_stake(bytes: u64) -> Balance {
        (bytes as Balance) * YOCTO_PER_BYTE
    }

    /// Storage stake required to deploy a store.
    pub const STORE: Balance = bytes_to_stake(super::storage_bytes::STORE);

    /// Storage stake required to hold a maximum size token without any metadata
    /// and without any royalties.
    pub const TOKEN: Balance = bytes_to_stake(super::storage_bytes::TOKEN);

    /// Storage stake required for some common components:
    ///
    /// - adding a single royalty
    /// - adding a single approval
    /// - adding a new entry to the `tokens_per_account` map
    /// - adding a new entry to the `composeables` map
    pub const COMMON: Balance = bytes_to_stake(super::storage_bytes::COMMON);

    /// Require 0.1 NEAR of storage stake to remain unused.
    pub const CUSHION: Balance = 10u128.pow(23);
}