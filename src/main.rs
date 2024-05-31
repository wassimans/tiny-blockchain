mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet<crate::types::AccountId, crate::types::Balance>,
}

impl Runtime {
	pub fn new() -> Self {
		Runtime { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = &"alice".to_string();
	let bob = &"bob".to_string();
	let charlie = &"charlie".to_string();
	runtime.balances.set_balance(alice.into(), 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number, 1);

	// first transaction
	runtime.system.inc_nonce(alice);
	let _res = runtime
		.balances
		.transfer(alice.into(), bob.into(), 30)
		.map_err(|e| eprintln!("Error: {}", e));

	// second transaction
	runtime.system.inc_nonce(alice);
	let _res = runtime
		.balances
		.transfer(alice.into(), charlie.into(), 20)
		.map_err(|e| eprintln!("Error: {}", e));

	// inspect the runtime state
	println!("{:#?}", runtime);
}
