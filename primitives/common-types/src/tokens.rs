#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use common_traits::TokenMetadata;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::format_runtime_string;
use sp_std::vec::Vec;

#[derive(
	Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	// Relaychain token - Rococo token symbol
	Roc,
	Native,
	Dora,
}

impl TokenMetadata for CurrencyId {
	fn name(&self) -> Vec<u8> {
		match self {
			CurrencyId::Roc => b"Rococo coin".to_vec(),
			CurrencyId::Native => b"Native currency".to_vec(),
			CurrencyId::Dora => b"DORA stable coin".to_vec(),
		}
	}

	fn symbol(&self) -> Vec<u8> {
		match self {
			CurrencyId::Native => b"FF".to_vec(),
			CurrencyId::Dora => b"DORA".to_vec()
		}
	}

	fn decimals(&self) -> u8 {
		match self {
			CurrencyId::Native => 18,
			CurrencyId::Dora => 12,
		}
	}
}
