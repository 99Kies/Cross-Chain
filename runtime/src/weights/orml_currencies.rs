#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_currencies.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_currencies::WeightInfo for WeightInfo<T> {
    // Storage: Tokens Accounts (r:2 w:2)
    // Storage: EvmAccounts EvmAddresses (r:1 w:0)
    // Storage: EVM Accounts (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: EvmAccounts Accounts (r:0 w:1)
    fn transfer_non_native_currency() -> Weight {
        (75_114_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: EvmAccounts EvmAddresses (r:1 w:0)
    // Storage: EVM Accounts (r:1 w:1)
    // Storage: EvmAccounts Accounts (r:0 w:1)
    fn transfer_native_currency() -> Weight {
        (72_672_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn update_balance_non_native_currency() -> Weight {
        (42_995_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    fn update_balance_native_currency_creating() -> Weight {
        (46_440_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: EvmAccounts EvmAddresses (r:1 w:0)
    // Storage: EVM Accounts (r:1 w:1)
    // Storage: EvmAccounts Accounts (r:0 w:1)
    fn update_balance_native_currency_killing() -> Weight {
        (52_750_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}
