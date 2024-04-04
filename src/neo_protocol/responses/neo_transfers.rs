use neo::prelude::{
	deserialize_h256, deserialize_script_hash, serialize_h256, serialize_script_hash, ScriptHash,
};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep11Transfers {
	pub sent: Vec<Nep11Transfer>,
	pub received: Vec<Nep11Transfer>,
	#[serde(rename = "address")]
	pub transfer_address: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep11Transfer {
	#[serde(rename = "tokenid")]
	pub token_id: String,
	pub timestamp: u64,
	#[serde(rename = "assethash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub asset_hash: ScriptHash,
	#[serde(rename = "transferaddress")]
	pub transfer_address: String,
	pub amount: u64,
	#[serde(rename = "blockindex")]
	pub block_index: u32,
	#[serde(rename = "transfernotifyindex")]
	pub transfer_notify_index: u32,
	#[serde(rename = "txhash")]
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub tx_hash: H256,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep17Transfers {
	pub sent: Vec<Nep17Transfer>,
	pub received: Vec<Nep17Transfer>,
	#[serde(rename = "address")]
	pub transfer_address: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Nep17Transfer {
	pub timestamp: u64,
	#[serde(rename = "assethash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub asset_hash: ScriptHash,
	#[serde(rename = "transferaddress")]
	pub transfer_address: String,
	pub amount: u64,
	#[serde(rename = "blockindex")]
	pub block_index: u32,
	#[serde(rename = "transfernotifyindex")]
	pub transfer_notify_index: u32,
	#[serde(rename = "txhash")]
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub tx_hash: H256,
}
