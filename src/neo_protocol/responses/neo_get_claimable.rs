use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Claimables {
	#[serde(rename = "claimable")]
	pub claims: Vec<Claim>,
	pub address: String,
	#[serde(rename = "unclaimed")]
	pub total_unclaimed: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Claim {
	#[serde(rename = "txid")]
	pub tx_id: String,
	#[serde(rename = "n")]
	pub index: u64,
	#[serde(rename = "value")]
	pub neo_value: u64,
	#[serde(rename = "start_height")]
	pub start_height: u64,
	#[serde(rename = "end_height")]
	pub end_height: u64,
	#[serde(rename = "generated")]
	pub generated_gas: String,
	#[serde(rename = "sysfee")]
	pub system_fee: String,
	#[serde(rename = "unclaimed")]
	pub unclaimed_gas: String,
}
