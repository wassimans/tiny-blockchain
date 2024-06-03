use std::collections::BTreeMap;

use num::{CheckedAdd, Zero};


/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, NonceNumber> {
	/// The current block number.
	pub block_number: BlockNumber,
	/// A map from an account to their nonce.
	pub nonce: BTreeMap<AccountId, NonceNumber>,
}

impl<BlockNumber, AccountId, NonceNumber>  Pallet<BlockNumber, AccountId, NonceNumber>
where
	BlockNumber: Zero + CheckedAdd<Output = BlockNumber> + Copy + From<u8>,
	AccountId: Ord,
	NonceNumber: Zero + CheckedAdd<Output = NonceNumber> + Copy + From<u8>,
{
	/// Create a new instance of the System Pallet.
pub fn new() -> Self {
	Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number = self.block_number + 1.into();
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: AccountId) {
		self.nonce.entry(who).and_modify(|curr| *curr = *curr + 1.into()).or_insert(1.into());
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		use super::*;

		let mut pallet : Pallet< u32,String, u32>= Pallet::new();
		pallet.nonce.insert("Wassim".to_string(), 0);
		assert_eq!(pallet.block_number, 0);
		pallet.inc_block_number();
		assert_eq!(pallet.block_number, 1);
		pallet.inc_nonce("Wassim".to_string());
		assert_eq!(pallet.nonce.get("Wassim").unwrap(), &1);
	}
}
