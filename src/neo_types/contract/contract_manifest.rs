use std::{
	collections::HashMap,
	hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use neo::prelude::{
	deserialize_wildcard, serialize_wildcard, ContractParameter, ContractParameterType,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContractManifest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(default)]
	pub groups: Vec<ContractGroup>,
	#[serde(skip_serializing)]
	pub features: Option<HashMap<String, serde_json::Value>>,
	#[serde(default)]
	#[serde(serialize_with = "serialize_wildcard")]
	#[serde(deserialize_with = "deserialize_wildcard")]
	#[serde(rename = "supportedstandards")]
	pub supported_standards: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub abi: Option<ContractABI>,
	#[serde(default)]
	pub permissions: Vec<ContractPermission>,
	#[serde(skip_serializing)]
	#[serde(serialize_with = "serialize_wildcard")]
	#[serde(deserialize_with = "deserialize_wildcard")]
	pub trusts: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub extra: Option<HashMap<String, serde_json::Value>>,
}

// impl Eq for ContractManifest
impl PartialEq for ContractManifest {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.groups == other.groups
			&& self.features == other.features
			&& self.supported_standards == other.supported_standards
			&& self.abi == other.abi
			&& self.permissions == other.permissions
			&& self.trusts == other.trusts
			&& self.extra == other.extra
	}
}

impl Hash for ContractManifest {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.groups.hash(state);
		// self.features.hash(state);
		self.supported_standards.hash(state);
		self.abi.hash(state);
		self.permissions.hash(state);
		self.trusts.hash(state);
		// self.extra.hash(state);
	}
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct ContractGroup {
	pub pub_key: String,
	pub signature: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct ContractABI {
	pub methods: Vec<ContractMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub events: Option<Vec<ContractEvent>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct ContractMethod {
	pub name: String,
	pub parameters: Vec<ContractParameter>,
	pub offset: usize,
	pub return_type: ContractParameterType,
	pub safe: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct ContractEvent {
	pub name: String,
	pub parameters: Vec<ContractParameter>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct ContractPermission {
	pub contract: String,
	#[serde(serialize_with = "serialize_wildcard")]
	#[serde(deserialize_with = "deserialize_wildcard")]
	pub methods: Vec<String>,
}
