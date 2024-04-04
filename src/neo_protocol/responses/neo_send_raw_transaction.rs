use neo::prelude::{deserialize_h256, serialize_h256};
use primitive_types::H256;

#[derive(Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct RawTransaction {
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub hash: H256,
}
