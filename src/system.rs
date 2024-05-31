use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = BTreeMap<AccountId, u32>;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	pub block_number: BlockNumber,
	/// A map from an account to their nonce.
	pub nonce: Nonce,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Pallet { block_number: u32::default(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		self.nonce.entry(who.to_string()).and_modify(|curr| *curr += 1).or_insert(1);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		use super::*;

		let mut pallet = Pallet::new();
		pallet.nonce.insert("Wassim".to_string(), 0);
		assert_eq!(pallet.block_number, 0);
		pallet.inc_block_number();
		assert_eq!(pallet.block_number, 1);
		pallet.inc_nonce(&"Wassim".to_string());
		assert_eq!(pallet.nonce.get(&"Wassim".to_string()).unwrap(), &1);
	}
}
