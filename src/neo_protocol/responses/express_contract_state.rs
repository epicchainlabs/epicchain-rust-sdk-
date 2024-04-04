use neo::prelude::{deserialize_script_hash, serialize_script_hash, ContractManifest, ScriptHash};
use primitive_types::H160;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Debug, Clone)]
pub struct ExpressContractState {
	#[serde(serialize_with = "serialize_script_hash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	pub hash: ScriptHash,
	pub manifest: ContractManifest,
}

impl ExpressContractState {
	pub fn new(hash: H160, manifest: ContractManifest) -> Self {
		Self { hash, manifest }
	}
}

impl PartialEq for ExpressContractState {
	fn eq(&self, other: &Self) -> bool {
		self.hash == other.hash && self.manifest == other.manifest
	}
}
