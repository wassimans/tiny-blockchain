use support::Dispatch;

use crate::types::Block;

mod balances;
mod proof_of_existence;
mod support;
mod system;

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type BlockNumber = u32;

	type AccountId = String;

	type Nonce = u32;
}

impl balances::Config for Runtime {
	type Balance = u128;
}

impl proof_of_existence::Config for Runtime {
    type Content = &'static str;
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


fn main() {
	let mut runtime = Runtime::new();
	let alice = &"alice".to_string();
	let bob = &"bob".to_string();
	let charlie = &"charlie".to_string();
	runtime.balances.set_balance(alice.into(), 100);

	let block_1 = Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![support::Extrinsic {
			caller: "alice".to_string(),
			call: RuntimeCall::balances(balances::Call::transfer {
				to: "bob".to_string(),
				amount: 69,
			}),
		}],
	};

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

	runtime
		.execute_block(block_1)
		.expect("All blocks being executed must be valid.");

	runtime
		.execute_block(block_2)
		.expect("All blocks being executed must be valid.");

	runtime
		.execute_block(block_3)
		.expect("All blocks being executed must be valid.");

	// inspect the runtime state
	println!("{:#?}", runtime);
}
