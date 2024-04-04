use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeoAddress {
	pub address: String,
	#[serde(rename = "haskey")]
	pub has_key: bool,

	pub label: Option<String>,
	#[serde(rename = "watchonly")]
	pub watch_only: bool,
}
