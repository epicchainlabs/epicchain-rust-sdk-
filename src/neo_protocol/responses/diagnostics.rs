use neo::prelude::{deserialize_script_hash, serialize_script_hash, ScriptHash};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Hash)]
pub struct Diagnostics {
	#[serde(rename = "invokedcontracts")]
	pub invoked_contracts: InvokedContract,
	#[serde(rename = "storagechanges")]
	pub storage_changes: Vec<StorageChange>,
}

impl Diagnostics {
	pub fn new(invoked_contracts: InvokedContract, storage_changes: Vec<StorageChange>) -> Self {
		Self { invoked_contracts, storage_changes }
	}
}

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct InvokedContract {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub hash: ScriptHash,
	pub invoked_contracts: Option<Vec<InvokedContract>>,
}

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct StorageChange {
	pub state: String,
	pub key: String,
	pub value: String,
}
