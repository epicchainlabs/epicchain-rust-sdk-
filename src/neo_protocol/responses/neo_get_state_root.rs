use neo::prelude::{deserialize_h256, serialize_h256, Witness};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StateRoot {
	pub version: u32,
	pub index: u32,
	#[serde(rename = "roothash")]
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub root_hash: H256,
	pub witnesses: Vec<Witness>,
}
