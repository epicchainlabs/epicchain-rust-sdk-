use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use neo::prelude::ContractParameter;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractParametersContext {
	pub type_: String,
	pub hash: String,
	pub data: String,
	pub items: HashMap<String, ContextItem>,
	pub network: u32,
}

impl ContractParametersContext {
	pub fn new(
		hash: String,
		data: String,
		items: Option<HashMap<String, ContextItem>>,
		network: u32,
	) -> Self {
		Self {
			type_: "Neo.Network.P2P.Payloads.Transaction".to_string(),
			hash,
			data,
			items: items.unwrap_or_default(),
			network,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextItem {
	pub script: String,
	pub parameters: Option<Vec<ContractParameter>>,
	pub signatures: HashMap<String, String>,
}

impl ContextItem {
	pub fn new(
		script: String,
		parameters: Option<Vec<ContractParameter>>,
		signatures: Option<HashMap<String, String>>,
	) -> Self {
		Self { script, parameters, signatures: signatures.unwrap_or_default() }
	}
}
