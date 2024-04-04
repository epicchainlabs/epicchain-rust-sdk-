use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Unspents {
	pub address: String,
	#[serde(rename = "balance")]
	pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Balance {
	#[serde(rename = "unspent")]
	pub(crate) unspent_transactions: Vec<UnspentTransaction>,
	#[serde(rename = "assethash")]
	pub(crate) asset_hash: String,
	#[serde(rename = "asset")]
	pub(crate) asset_name: String,
	#[serde(rename = "asset_symbol")]
	pub(crate) asset_symbol: String,
	pub(crate) amount: f64,
}

impl Eq for Balance {}

impl PartialEq for Balance {
	fn eq(&self, other: &Self) -> bool {
		self.asset_hash == other.asset_hash
			&& self.asset_name == other.asset_name
			&& self.asset_symbol == other.asset_symbol
			&& self.amount == other.amount
	}
}

impl Hash for Balance {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.asset_hash.hash(state);
		self.asset_name.hash(state);
		self.asset_symbol.hash(state);
		// self.amount.hash(state);
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnspentTransaction {
	#[serde(rename = "txid")]
	pub tx_id: String,
	#[serde(rename = "n")]
	pub index: u32,
	pub value: f64,
}
impl PartialEq for UnspentTransaction {
	fn eq(&self, other: &Self) -> bool {
		self.tx_id == other.tx_id && self.index == other.index && self.value == other.value
	}
}

impl Hash for UnspentTransaction {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.tx_id.hash(state);
		// self.index.hash(state);
		// self.value.hash(state);
	}
}
