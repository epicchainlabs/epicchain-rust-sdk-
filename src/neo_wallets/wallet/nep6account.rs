use std::collections::HashMap;

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use neo::prelude::{
	Account, Address, AddressOrScriptHash, Base64Encode, ContractParameterType, NEP6Contract,
	NEP6Parameter, NeoSerializable, StringExt, VerificationScript, WalletError,
};

/// Represents an account in the NEP-6 format.
#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
pub struct NEP6Account {
	/// The address of the account.
	#[getset(get = "pub", set = "pub")]
	#[serde(rename = "address")]
	pub address: Address,

	/// An optional label for the account.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "label")]
	pub label: Option<String>,

	/// Indicates whether the account is set as default.
	#[serde(default)]
	#[serde(rename = "isDefault")]
	pub is_default: bool,

	/// Indicates whether the account is locked.
	#[serde(rename = "lock")]
	pub lock: bool,

	/// An optional private key associated with the account.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "key")]
	pub key: Option<String>,

	/// An optional NEP-6 contract associated with the account.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "contract")]
	pub contract: Option<NEP6Contract>,

	/// An optional additional data associated with the account.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "extra")]
	pub extra: Option<HashMap<String, String>>,
}

impl NEP6Account {
	/// Creates a new NEP-6 account with the given parameters.
	///
	/// # Arguments
	///
	/// * `address` - The address of the account.
	/// * `label` - An optional label for the account.
	/// * `is_default` - Indicates whether the account is set as default.
	/// * `lock` - Indicates whether the account is locked.
	/// * `key` - An optional private key associated with the account.
	/// * `contract` - An optional NEP-6 contract associated with the account.
	/// * `extra` - An optional additional data associated with the account.
	///
	/// # Example
	///
	/// ```
	/// use std::collections::HashMap;
	/// use neo_rs::prelude::{Address, NEP6Account, NEP6Contract};
	///
	/// let address = Address::from("example_address");
	/// let label = Some("My Account".to_string());
	/// let is_default = true;
	/// let lock = false;
	/// let key = Some("example_private_key".to_string());
	/// let contract = Some(NEP6Contract::new());
	/// let extra = Some(HashMap::new());
	///
	/// let account = NEP6Account::new(address, label, is_default, lock, key, contract, extra);
	/// ```
	pub fn new(
		address: Address,
		label: Option<String>,
		is_default: bool,
		lock: bool,
		key: Option<String>,
		contract: Option<NEP6Contract>,
		extra: Option<HashMap<String, String>>,
	) -> Self {
		Self { address, label, is_default, lock, key, contract, extra }
	}

	/// Converts an `Account` into a `NEP6Account`.
	///
	/// # Arguments
	///
	/// * `account` - The account to convert.
	///
	/// # Errors
	///
	/// Returns a `WalletError` if there is an issue converting the account.
	///
	/// # Example
	///
	/// ```
	/// use neo_rs::prelude::{Account, NEP6Account};
	///
	/// let account = Account::default();
	/// let nep6_account = NEP6Account::from_account(&account);
	/// ```
	pub fn from_account(account: &Account) -> Result<NEP6Account, WalletError> {
		if account.key_pair.is_some() && account.encrypted_private_key.is_none() {
			return Err(WalletError::AccountState(
				"Account private key is available but not encrypted.".to_string(),
			))
		}

		let mut parameters = Vec::new();
		if let Some(verification_script) = &account.verification_script {
			if verification_script.is_multi_sig() {
				for i in 0..verification_script.get_nr_of_accounts()? {
					parameters.push(NEP6Parameter {
						param_name: format!("signature{}", i),
						param_type: ContractParameterType::Signature,
					});
				}
			} else if verification_script.is_single_sig() {
				parameters.push(NEP6Parameter {
					param_name: "signature".to_string(),
					param_type: ContractParameterType::Signature,
				});
			}
		}

		let contract = if !parameters.is_empty() {
			Some(NEP6Contract {
				script: Some(account.clone().verification_script.unwrap().to_array().to_base64()),
				is_deployed: false,
				nep6_parameters: parameters,
			})
		} else {
			None
		};

		Ok(NEP6Account {
			address: account.address_or_scripthash.address().clone(),
			label: account.label.clone(),
			is_default: account.is_default,
			lock: account.is_locked,
			key: account.encrypted_private_key.clone(),
			contract,
			extra: None,
		})
	}

	/// Converts a `NEP6Account` into an `Account`.
	///
	/// # Errors
	///
	/// Returns a `WalletError` if there is an issue converting the account.
	///
	/// # Example
	///
	/// ```
	/// use neo_rs::prelude::NEP6Account;
	/// let nep6_account = NEP6Account::default();
	/// let account = nep6_account.to_account();
	/// ```
	pub fn to_account(&self) -> Result<Account, WalletError> {
		let mut verification_script: Option<VerificationScript> = None;
		let mut signing_threshold: Option<u8> = None;
		let mut nr_of_participants: Option<u8> = None;

		if let Some(contract) = &self.contract {
			if contract.script.is_some() {
				verification_script = Some(VerificationScript::from(
					contract.script.clone().unwrap().base64_decoded().unwrap(),
				));

				if verification_script.as_ref().unwrap().is_multi_sig() {
					signing_threshold =
						Some(verification_script.as_ref().unwrap().get_signing_threshold()? as u8);
					nr_of_participants =
						Some(verification_script.as_ref().unwrap().get_nr_of_accounts()? as u8);
				}
			}
		}

		Ok(Account {
			address_or_scripthash: AddressOrScriptHash::Address(self.clone().address),
			label: self.clone().label,
			verification_script,
			is_locked: self.clone().lock,
			encrypted_private_key: self.clone().key,
			signing_threshold: signing_threshold.map(|s| s as u32),
			nr_of_participants: nr_of_participants.map(|s| s as u32),
			..Default::default()
		})
	}
}

impl PartialEq for NEP6Account {
	/// Checks if two `NEP6Account` instances are equal based on their addresses.
	///
	/// # Example
	///
	/// ```
	///
	/// use neo_rs::prelude::NEP6Account;
	///
	/// let account1 = NEP6Account::default();
	/// let account2 = NEP6Account::default();
	/// assert_eq!(account1, account2);
	/// ```
	fn eq(&self, other: &Self) -> bool {
		self.address == other.address
	}
}

#[cfg(test)]
mod tests {
	use neo::prelude::{
		AccountTrait, NEP6Account, PrivateKeyExtension, Secp256r1PrivateKey, TestConstants,
	};

	#[test]
	fn test_decrypt_with_standard_scrypt_params() {
		let private_key = Secp256r1PrivateKey::from_bytes(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY).unwrap(),
		)
		.unwrap();

		let nep6_account = NEP6Account::new(
			"".to_string(),
			None,
			true,
			false,
			Some(TestConstants::DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY.to_string()),
			None,
			None,
		);

		let mut account = nep6_account.to_account().unwrap();

		account.decrypt_private_key(TestConstants::DEFAULT_ACCOUNT_PASSWORD).unwrap();

		assert_eq!(account.key_pair.clone().unwrap().private_key.to_vec(), private_key.to_vec());

		// Decrypt again
		account.decrypt_private_key(TestConstants::DEFAULT_ACCOUNT_PASSWORD).unwrap();
		assert_eq!(account.key_pair.clone().unwrap().private_key, private_key);
	}

	#[test]
	fn test_load_account_from_nep6() {
		let data = include_str!("../../../test_resources/wallet/account.json");
		let nep6_account: NEP6Account = serde_json::from_str(data).unwrap();

		let account = nep6_account.to_account().unwrap();

		assert!(!account.is_default);
		assert!(!account.is_locked);
		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
	}
}
