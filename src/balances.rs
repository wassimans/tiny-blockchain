use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord,
	Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
		self.balances.insert(who, amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Zero::zero())
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		from: AccountId,
		to: AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(&from);
        let to_balance = self.balance(&to);

        let new_from_balance = from_balance.checked_sub(&amount).ok_or("Not enough funds!")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
	}
}

#[cfg(test)]
mod tests {
    use crate::balances::Pallet;

	#[test]
	fn init_balances() {
		let mut balances: Pallet<String, u32> = super::Pallet::new();

		assert_eq!(balances.balance(&"Alice".to_string()), 0);
		balances.set_balance("Alice".to_string(), 100);
		assert_eq!(balances.balance(&"Alice".to_string()), 100);
		assert_eq!(balances.balance(&"Bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances: Pallet<String, u32> = super::Pallet::new();
		balances.set_balance("Alice".to_string(), 100);

		balances.transfer("Alice".to_string(), "Bob".to_string(), 50).unwrap();
		assert_eq!(balances.balance(&"Alice".to_string()), 50);
		assert_eq!(balances.balance(&"Bob".to_string()), 50);

		let result = balances.transfer("Alice".to_string(), "Bob".to_string(), 60);
		assert_eq!(result, Err("Not enough funds!"));
		assert_eq!(balances.balance(&"Alice".to_string()), 50);
		assert_eq!(balances.balance(&"Bob".to_string()), 50);
	}
}
