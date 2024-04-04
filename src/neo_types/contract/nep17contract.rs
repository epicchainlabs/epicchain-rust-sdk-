use primitive_types::H160;
use serde::{Deserialize, Serialize};

use neo::prelude::{deserialize_script_hash, serialize_script_hash};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct Nep17Contract {
	#[serde(serialize_with = "serialize_script_hash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	pub script_hash: H160,
	pub symbol: String,
	pub decimals: u8,
}

impl Nep17Contract {
	pub fn new(script_hash: H160, symbol: String, decimals: u8) -> Self {
		Self { script_hash, symbol, decimals }
	}
}
