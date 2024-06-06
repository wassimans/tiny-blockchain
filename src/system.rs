use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
	type BlockNumber: Zero + One + AddAssign + Copy;
	type AccountId: Ord;
	type Nonce: Zero + One + Copy;
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
		self.block_number += T::BlockNumber::one();
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: T::AccountId) {
		self.nonce
			.entry(who)
			.and_modify(|curr| *curr = *curr + T::Nonce::one())
			.or_insert(T::Nonce::one());
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
