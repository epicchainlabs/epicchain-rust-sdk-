use primitive_types::H160;
use serde::{Deserialize, Serialize};

use neo::prelude::{
	deserialize_script_hash, serialize_script_hash, ContractManifest, ContractNef,
	InvocationResult, StackItem,
};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ContractState {
	pub id: i32,
	pub nef: ContractNef,
	pub update_counter: i32,
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub hash: H160,
	pub manifest: ContractManifest,
}

impl ContractState {
	pub fn new(
		id: i32,
		update_counter: i32,
		hash: H160,
		nef: ContractNef,
		manifest: ContractManifest,
	) -> Self {
		Self { id, nef, update_counter, hash, manifest }
	}

	pub fn contract_identifiers(
		stack_item: &StackItem,
	) -> Result<ContractIdentifiers, &'static str> {
		match stack_item {
			StackItem::Struct { value } if value.len() >= 2 => {
				let id = value[0].as_int().unwrap();
				let mut v = value[1].as_bytes().unwrap();
				v.reverse();
				let hash = H160::from_slice(&v);
				Ok(ContractIdentifiers { id: id as i32, hash })
			},
			_ => Err("Could not deserialize ContractIdentifiers from stack item"),
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ContractIdentifiers {
	pub id: i32,
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub hash: H160,
}

impl From<InvocationResult> for ContractIdentifiers {
	fn from(result: InvocationResult) -> Self {
		let stack_item = &result.stack[0];
		ContractState::contract_identifiers(stack_item).unwrap()
	}
}
