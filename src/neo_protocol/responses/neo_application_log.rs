use neo::prelude::{deserialize_h256, serialize_h256, LogNotification, StackItem, VMState};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ApplicationLog {
	#[serde(rename = "txid")]
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub transaction_id: H256,
	pub executions: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Execution {
	pub trigger: String,
	#[serde(rename = "vmstate")]
	pub state: VMState,
	pub exception: Option<String>,
	#[serde(rename = "gasconsumed")]
	pub gas_consumed: String,
	pub stack: Vec<StackItem>,
	pub notifications: Vec<LogNotification>,
}
