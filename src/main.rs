use support::Dispatch;

use crate::types::Block;

mod balances;
mod support;
mod system;
mod proof_of_existence;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type BlockNumber = u32;

	type AccountId = String;

	type Nonce = u32;
}

impl balances::Config for Runtime {
	type Balance = u128;
}

impl Runtime {
	pub fn new() -> Self {
		Runtime { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

impl Default for Runtime {
	fn default() -> Self {
		Self::new()
	}
}

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
}

impl Runtime {
	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		// Increment the system's block number.
		self.system.inc_block_number();

		// - Check that the block number of the incoming block matches the current block number,
		// or return an error.
		if block.header.block_number != self.system.block_number {
			return Err("Incoming and current block numbers dont match!");
		}

		// Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			// Increment the nonce of the caller.
			self.system.inc_nonce(caller.clone());
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}

		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {

			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = &"alice".to_string();
	let bob = &"bob".to_string();
	let charlie = &"charlie".to_string();
	runtime.balances.set_balance(alice.into(), 100);

	let new_block = Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![support::Extrinsic {
			caller: "alice".to_string(),
			call: RuntimeCall::Balances(balances::Call::Transfer { to: "bob".to_string(), amount: 69 }),
		}],
	};

	runtime.execute_block(new_block).expect("All blocks being executed must be valid.");

	// inspect the runtime state
	println!("{:#?}", runtime);
}
