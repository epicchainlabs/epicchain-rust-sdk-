use serde::{Deserialize, Serialize};

use neo::prelude::{ContractManifest, ContractNef};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeContractState {
	pub id: i32,
	pub nef: ContractNef,
	pub update_history: Vec<i32>,
	#[serde(flatten)]
	pub base: ExpressContractState,
}

impl NativeContractState {
	pub fn new(
		id: i32,
		hash: [u8; 20],
		nef: ContractNef,
		manifest: ContractManifest,
		update_history: Vec<i32>,
	) -> Self {
		Self { id, nef, update_history, base: ExpressContractState { hash, manifest } }
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExpressContractState {
	hash: [u8; 20],
	manifest: ContractManifest,
}
