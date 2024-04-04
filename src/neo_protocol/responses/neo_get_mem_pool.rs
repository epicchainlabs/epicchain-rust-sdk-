use neo::prelude::{deserialize_vec_h256, serialize_vec_h256};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemPoolDetails {
	pub height: u32,
	#[serde(serialize_with = "serialize_vec_h256")]
	#[serde(deserialize_with = "deserialize_vec_h256")]
	pub verified: Vec<H256>,
	#[serde(serialize_with = "serialize_vec_h256")]
	#[serde(deserialize_with = "deserialize_vec_h256")]
	pub unverified: Vec<H256>,
}
