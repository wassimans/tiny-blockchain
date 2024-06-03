use std::collections::BTreeMap;

use num::{CheckedAdd, Zero};

pub trait Config {
	type BlockNumber: Zero + CheckedAdd<Output = Self::BlockNumber> + Copy + From<u8>;
	type AccountId: Ord;
	type Nonce: Zero + CheckedAdd<Output = Self::Nonce> + Copy + From<u8>;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	pub block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number = self.block_number + 1.into();
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: T::AccountId) {
		self.nonce
			.entry(who)
			.and_modify(|curr| *curr = *curr + 1.into())
			.or_insert(1.into());
	}
}

#[cfg(test)]
mod tests {

	struct TestConfig;

	impl super::Config for TestConfig {
		type BlockNumber = u32;

		type AccountId = String;

		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		use super::*;

		let mut pallet = Pallet::<TestConfig>::new();
		pallet.nonce.insert("Wassim".to_string(), 0);
		assert_eq!(pallet.block_number, 0);
		pallet.inc_block_number();
		assert_eq!(pallet.block_number, 1);
		pallet.inc_nonce("Wassim".to_string());
		assert_eq!(pallet.nonce.get("Wassim").unwrap(), &1);
	}
}
