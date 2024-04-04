use getset::Getters;
use neo::prelude::{deserialize_script_hash, serialize_script_hash, ScriptHash};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep11Balances {
	pub address: String,
	#[serde(rename = "balance")]
	pub balances: Vec<Nep11Balance>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep11Balance {
	pub name: String,
	pub symbol: String,
	pub decimals: String,
	pub tokens: Vec<Nep11Token>,
	#[serde(rename = "assethash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub asset_hash: ScriptHash,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep11Token {
	#[serde(rename = "tokenid")]
	pub token_id: String,
	pub amount: String,
	#[serde(rename = "lastupdatedblock")]
	pub last_updated_block: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep17Balances {
	pub address: String,
	#[serde(rename = "balance")]
	pub balances: Vec<Nep17Balance>,
}

#[derive(Getters, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep17Balance {
	pub name: Option<String>,
	pub symbol: Option<String>,
	pub decimals: Option<String>,
	pub amount: String,
	#[serde(rename = "lastupdatedblock")]
	pub last_updated_block: u32,
	#[serde(rename = "assethash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub asset_hash: ScriptHash,
}
