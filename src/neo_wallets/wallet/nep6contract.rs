use serde::{Deserialize, Serialize};

use neo::prelude::ContractParameterType;

/// Represents a NEP-6 contract.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NEP6Contract {
	/// The script associated with the contract.
	#[serde(rename = "script")]
	pub script: Option<String>,

	/// Indicates whether the contract is deployed.
	#[serde(rename = "deployed")]
	pub is_deployed: bool,

	/// The NEP-6 parameters associated with the contract.
	#[serde(rename = "parameters")]
	pub nep6_parameters: Vec<NEP6Parameter>,
}

/// Represents a NEP-6 parameter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NEP6Parameter {
	/// The name of the parameter.
	#[serde(rename = "name")]
	pub param_name: String,

	/// The type of the parameter.
	#[serde(rename = "type")]
	pub param_type: ContractParameterType,
}

impl PartialEq for NEP6Contract {
	/// Checks if two `NEP6Contract` instances are equal.
	///
	/// # Example
	///
	/// ```
	/// use neo_rs::prelude::NEP6Contract;
	///
	/// let contract1 = NEP6Contract::default();
	/// let contract2 = NEP6Contract::default();
	/// assert_eq!(contract1, contract2);
	/// ```
	fn eq(&self, other: &Self) -> bool {
		self.script == other.script
			&& self.nep6_parameters == other.nep6_parameters
			&& self.is_deployed == other.is_deployed
	}
}
