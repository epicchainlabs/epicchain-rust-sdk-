use std::hash::{Hash, Hasher};

use primitive_types::H160;
use serde::{Deserialize, Serialize};

use neo::prelude::{deserialize_script_hash, serialize_script_hash};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TransactionSendToken {
	#[serde(rename = "asset")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub token: H160,
	pub value: i32,
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub address: H160,
}

impl TransactionSendToken {
	pub fn new(token: H160, value: i32, address: H160) -> Self {
		Self { token, value, address }
	}
}
