use std::hash::{Hash, Hasher};

use getset::{Getters, Setters};
use primitive_types::H160;
use serde::{Deserialize, Serialize};

use neo::prelude::{
	deserialize_script_hash, deserialize_vec_public_key, deserialize_vec_script_hash,
	serialize_script_hash, serialize_vec_public_key, serialize_vec_script_hash, Account,
	AccountTrait, Decoder, Encoder, NeoConstants, NeoSerializable, PublicKeyExtension,
	ScriptHashExtension, SignerTrait, SignerType, TransactionError, VarSizeTrait, WitnessRule,
	WitnessScope,
};

use crate::prelude::Secp256r1PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
pub struct AccountSigner {
	#[serde(
		serialize_with = "serialize_script_hash",
		deserialize_with = "deserialize_script_hash"
	)]
	pub(crate) signer_hash: H160,
	pub(crate) scopes: Vec<WitnessScope>,
	#[serde(
		serialize_with = "serialize_vec_script_hash",
		deserialize_with = "deserialize_vec_script_hash"
	)]
	pub(crate) allowed_contracts: Vec<H160>,
	#[serde(
		serialize_with = "serialize_vec_public_key",
		deserialize_with = "deserialize_vec_public_key"
	)]
	allowed_groups: Vec<Secp256r1PublicKey>,
	rules: Vec<WitnessRule>,
	#[getset(get = "pub")]
	pub account: Account,
}

impl NeoSerializable for AccountSigner {
	type Error = TransactionError;

	fn size(&self) -> usize {
		let mut size: usize = NeoConstants::HASH160_SIZE as usize;
		if self.scopes.contains(&WitnessScope::CustomContracts) {
			size += self.allowed_contracts.var_size();
		}
		if self.scopes.contains(&WitnessScope::CustomGroups) {
			size += self.allowed_groups.var_size();
		}
		if self.scopes.contains(&WitnessScope::WitnessRules) {
			size += self.rules.var_size();
		}
		size
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_serializable_fixed(&self.signer_hash);
		writer.write_u8(WitnessScope::combine(&self.scopes));
		if self.scopes.contains(&WitnessScope::CustomContracts) {
			writer.write_serializable_variable_list(&self.allowed_contracts);
		}
		if self.scopes.contains(&WitnessScope::CustomGroups) {
			writer.write_serializable_variable_list(&self.allowed_groups);
		}
		if self.scopes.contains(&WitnessScope::WitnessRules) {
			writer.write_serializable_variable_list(&self.rules);
		}
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		let signer_hash = reader.read_serializable::<H160>().unwrap();
		let scopes = WitnessScope::split(reader.read_u8());
		let mut allowed_contracts = vec![];
		let mut allowed_groups = vec![];
		let mut rules = vec![];
		if scopes.contains(&WitnessScope::CustomContracts) {
			allowed_contracts = reader.read_serializable_list::<H160>().unwrap();
		}
		if scopes.contains(&WitnessScope::CustomGroups) {
			allowed_groups = reader.read_serializable_list::<Secp256r1PublicKey>().unwrap();
		}
		if scopes.contains(&WitnessScope::WitnessRules) {
			rules = reader.read_serializable_list::<WitnessRule>().unwrap();
		}
		Ok(Self {
			signer_hash,
			scopes,
			allowed_contracts,
			allowed_groups,
			rules,
			account: Account::from_address(signer_hash.to_address().as_str()).unwrap(),
		})
	}

	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}

impl PartialEq for AccountSigner {
	fn eq(&self, other: &Self) -> bool {
		self.signer_hash == other.signer_hash
			&& self.scopes == other.scopes
			&& self.allowed_contracts == other.allowed_contracts
			&& self.allowed_groups == other.allowed_groups
			&& self.rules == other.rules
		// && self.account == other.account
	}
}

impl Hash for AccountSigner {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.signer_hash.hash(state);
		self.scopes.hash(state);
		self.allowed_contracts.hash(state);
		for group in self.allowed_groups.iter() {
			group.to_vec().hash(state);
		}
		// self.allowed_groups.to_vec().hash(state);
		self.rules.hash(state);
		// self.account.hash(state);
		// self.scope.hash(state);
	}
}

impl SignerTrait for AccountSigner {
	fn get_type(&self) -> SignerType {
		SignerType::Account
	}

	fn get_signer_hash(&self) -> &H160 {
		&self.signer_hash
	}

	fn set_signer_hash(&mut self, signer_hash: H160) {
		self.signer_hash = signer_hash;
	}

	fn get_scopes(&self) -> &Vec<WitnessScope> {
		&self.scopes
	}

	fn get_scopes_mut(&mut self) -> &mut Vec<WitnessScope> {
		&mut self.scopes
	}

	fn set_scopes(&mut self, scopes: Vec<WitnessScope>) {
		self.scopes = scopes;
	}

	fn get_allowed_contracts(&self) -> &Vec<H160> {
		&self.allowed_contracts
	}

	fn get_allowed_contracts_mut(&mut self) -> &mut Vec<H160> {
		&mut self.allowed_contracts
	}

	fn get_allowed_groups(&self) -> &Vec<Secp256r1PublicKey> {
		&self.allowed_groups
	}

	fn get_allowed_groups_mut(&mut self) -> &mut Vec<Secp256r1PublicKey> {
		&mut self.allowed_groups
	}

	fn get_rules(&self) -> &Vec<WitnessRule> {
		&self.rules
	}

	fn get_rules_mut(&mut self) -> &mut Vec<WitnessRule> {
		&mut self.rules
	}
}

impl AccountSigner {
	fn new(account: &Account, scope: WitnessScope) -> Self {
		Self {
			signer_hash: account.get_script_hash().clone(),
			scopes: vec![scope],
			allowed_contracts: vec![],
			allowed_groups: vec![],
			rules: vec![],
			account: account.clone(),
		}
	}

	pub fn none(account: &Account) -> Result<Self, TransactionError> {
		Ok(Self::new(account, WitnessScope::None))
	}

	pub fn none_hash160(account_hash: H160) -> Result<Self, TransactionError> {
		let account = Account::from_address(account_hash.to_address().as_str()).unwrap();
		Ok(Self::new(&account, WitnessScope::None))
	}

	pub fn called_by_entry(account: &Account) -> Result<Self, TransactionError> {
		Ok(Self::new(account, WitnessScope::CalledByEntry))
	}

	pub fn called_by_entry_hash160(account_hash: H160) -> Result<Self, TransactionError> {
		let account = Account::from_address(account_hash.to_address().as_str()).unwrap();
		Ok(Self::new(&account, WitnessScope::CalledByEntry))
	}

	pub fn global(account: Account) -> Result<Self, TransactionError> {
		Ok(Self::new(&account, WitnessScope::Global))
	}

	pub fn global_hash160(account_hash: H160) -> Result<Self, TransactionError> {
		let account = Account::from_address(account_hash.to_address().as_str()).unwrap();
		Ok(Self::new(&account, WitnessScope::Global))
	}

	pub fn is_multi_sig(&self) -> bool {
		matches!(&self.account.verification_script(), Some(script) if script.is_multi_sig())
	}
}
