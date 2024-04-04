use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use neo::prelude::{NEP6Account, ScryptParamsDef};

/// Represents a NEP-6 wallet.
#[derive(Serialize, Deserialize, Clone, Getters, CopyGetters)]
#[getset(get = "pub", set = "pub")]
pub struct NEP6Wallet {
	/// The name of the wallet.
	#[serde(rename = "name")]
	pub(crate) name: String,
	/// The version of the wallet.
	#[serde(rename = "version")]
	pub(crate) version: String,
	/// The scrypt parameters used for encryption.
	#[serde(rename = "scrypt")]
	pub(crate) scrypt: ScryptParamsDef,
	/// The accounts associated with the wallet.
	#[serde(rename = "accounts")]
	pub(crate) accounts: Vec<NEP6Account>,
	/// Additional data associated with the wallet.
	#[serde(rename = "extra")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) extra: Option<HashMap<String, String>>,
}

impl NEP6Wallet {
	/// Creates a new NEP-6 wallet with the given parameters.
	///
	/// # Arguments
	///
	/// * `name` - The name of the wallet.
	/// * `version` - The version of the wallet.
	/// * `scrypt` - The scrypt parameters used for encryption.
	/// * `accounts` - The accounts associated with the wallet.
	/// * `extra` - Additional data associated with the wallet.
	///
	/// # Example
	///
	/// ```
	/// use std::collections::HashMap;
	/// use neo_rs::prelude::{NEP6Wallet, ScryptParamsDef};
	///
	/// let name = "MyWallet".to_string();
	/// let version = "1.0".to_string();
	/// let scrypt = ScryptParamsDef::default();
	/// let accounts = vec![];
	/// let extra = Some(HashMap::new());
	///
	/// let wallet = NEP6Wallet::new(name, version, scrypt, accounts, extra);
	/// ```
	pub fn new(
		name: String,
		version: String,
		scrypt: ScryptParamsDef,
		accounts: Vec<NEP6Account>,
		extra: Option<HashMap<String, String>>,
	) -> Self {
		Self { name, version, scrypt, accounts, extra }
	}
}

#[cfg(test)]
mod tests {
	use neo::prelude::{ContractParameterType, NEP6Wallet, ScryptParamsDef};

	#[test]
	fn test_read_wallet() {
		let data = include_str!("../../../test_resources/wallet/wallet.json");
		let wallet: NEP6Wallet = serde_json::from_str(data).unwrap();

		assert_eq!(wallet.clone().name, "Wallet");
		assert_eq!(wallet.clone().version, "1.0");
		assert_eq!(wallet.clone().scrypt, ScryptParamsDef::default());
		assert_eq!(wallet.clone().accounts.len(), 2);

		let account1 = &wallet.accounts[0];

		assert_eq!(account1.address, "NLnyLtep7jwyq1qhNPkwXbJpurC4jUT8ke");
		assert_eq!(account1.label, Some("Account1".to_string()));
		assert!(account1.is_default);
		assert!(!account1.lock);
		assert_eq!(
			account1.key,
			Some("6PYVEi6ZGdsLoCYbbGWqoYef7VWMbKwcew86m5fpxnZRUD8tEjainBgQW1".to_string())
		);
		assert!(account1.extra.is_none());

		let contract1 = account1.contract.as_ref().unwrap();

		assert_eq!(
			contract1.script,
			Some("DCECJJQloGtaH45hM/x5r6LCuEML+TJyl/F2dh33no2JKcULQZVEDXg=".to_string())
		);
		assert!(!contract1.is_deployed);

		let parameter1 = &contract1.nep6_parameters[0];
		assert_eq!(parameter1.param_name, "signature".to_string());
		assert_eq!(parameter1.param_type, ContractParameterType::Signature);

		let account2 = &wallet.accounts[1];

		assert_eq!(account2.address, "NWcx4EfYdfqn5jNjDz8AHE6hWtWdUGDdmy".to_string());
		assert_eq!(account2.label, Some("Account2".to_string()));
		assert!(!account2.is_default);
		assert!(!account2.lock);
		assert_eq!(
			account2.key,
			Some("6PYSQWBqZE5oEFdMGCJ3xR7bz6ezz814oKE7GqwB9i5uhtUzkshe9B6YGB".to_string())
		);
		assert!(account2.extra.is_none());

		let contract2 = account2.contract.as_ref().unwrap();

		assert_eq!(
			contract2.script,
			Some("DCEDHMqqRt98SU9EJpjIwXwJMR42FcLcBCy9Ov6rpg+kB0ALQZVEDXg=".to_string())
		);
		assert!(!contract2.is_deployed);

		let parameter2 = &contract2.nep6_parameters[0];
		assert_eq!(parameter2.param_name, "signature".to_string());
		assert_eq!(parameter2.param_type, ContractParameterType::Signature);
	}
}
