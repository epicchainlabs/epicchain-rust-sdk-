use std::{
	collections::HashMap,
	hash::{Hash, Hasher},
};

use primitive_types::H160;
use serde::{Deserialize, Serialize};

use neo::prelude::{deserialize_script_hash, serialize_script_hash, ContractParameter, StackItem};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct InvocationResult {
	pub script: String,
	pub state: NeoVMStateType,
	pub gas_consumed: String,
	pub exception: Option<String>,
	pub notifications: Option<Vec<Notification>>,
	pub diagnostics: Option<Diagnostics>,
	pub stack: Vec<StackItem>,
	pub tx: Option<String>,
	pub pending_signature: Option<PendingSignature>,
	pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub enum NeoVMStateType {
	Halt,
	Fault,
	Break,
	StepInto,
	StepOut,
	StepOver,
	Exception,
}

impl InvocationResult {
	// constructor and helper methods
	pub fn new(
		script: String,
		state: NeoVMStateType,
		gas_consumed: String,
		exception: Option<String>,
		notifications: Option<Vec<Notification>>,
		diagnostics: Option<Diagnostics>,
		stack: Vec<StackItem>,
		tx: Option<String>,
		pending_signature: Option<PendingSignature>,
		session_id: Option<String>,
	) -> Self {
		Self {
			script,
			state,
			gas_consumed,
			exception,
			notifications,
			diagnostics,
			stack,
			tx,
			pending_signature,
			session_id,
		}
	}

	pub fn has_state_fault(&self) -> bool {
		matches!(self.state, NeoVMStateType::Fault)
	}

	pub fn get_first_stack_item(&self) -> Result<&StackItem, &str> {
		self.stack.first().ok_or("Stack is empty")
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PendingSignature {
	pub typ: String,
	pub data: String,
	pub items: HashMap<String, Item>,
	pub network: u32,
}

impl Hash for PendingSignature {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.typ.hash(state);
		self.data.hash(state);
		// self.items.hash(state);
		self.network.hash(state);
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Item {
	pub script: String,
	pub parameters: Vec<ContractParameter>,
	pub signatures: HashMap<String, String>,
}

impl Hash for Item {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.script.hash(state);
		self.parameters.hash(state);
		// self.signatures.hash(state);
	}
}

// Other structs like Diagnostics, Notification

// Diagnostics
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Diagnostics {
	pub invoked_contracts: InvokedContract,
	pub storage_changes: Vec<StorageChange>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct InvokedContract {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub hash: H160,
	pub invoked_contracts: Option<Vec<InvokedContract>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StorageChange {
	pub state: String,
	pub key: String,
	pub value: String,
}

// Notification
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Notification {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub contract: H160,
	pub event_name: String,
	pub state: NotificationState,
	pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub enum NotificationState {
	Failure,
	Success,
	Halt,
	Fault,
	StepInto,
	StepOut,
	StepOver,
	Break,
}
