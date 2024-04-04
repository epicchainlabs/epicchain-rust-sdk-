use neo::prelude::{
	deserialize_h256, deserialize_h256_option, serialize_h256, serialize_h256_option, NeoWitness,
	TransactionResult,
};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct NeoBlock {
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub hash: H256,
	pub size: i32,
	pub version: i32,
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub prev_block_hash: H256,
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub merkle_root_hash: H256,
	pub time: i32,
	pub index: i32,
	pub primary: Option<i32>,
	pub next_consensus: String,
	pub witnesses: Option<Vec<NeoWitness>>,
	pub transactions: Option<Vec<TransactionResult>>,
	pub confirmations: i32,
	#[serde(serialize_with = "serialize_h256_option")]
	#[serde(deserialize_with = "deserialize_h256_option")]
	pub next_block_hash: Option<H256>,
}
