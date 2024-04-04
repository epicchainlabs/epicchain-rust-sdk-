use std::hash::{Hash, Hasher};

use primitive_types::H160;
use serde::{Deserialize, Serialize};

use neo::prelude::{
	deserialize_script_hash, deserialize_vec_script_hash_option, serialize_script_hash,
	serialize_vec_script_hash_option, Decoder, Encoder, NeoConstants, NeoSerializable,
	Secp256r1PublicKey, SignerTrait, SignerType, TransactionError, VarSizeTrait, WitnessRule,
	WitnessScope,
};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionSigner {
	#[serde(rename = "account")]
	#[serde(serialize_with = "serialize_script_hash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	pub account: H160,

	#[serde(rename = "scopes")]
	pub scopes: Vec<WitnessScope>,

	#[serde(rename = "allowedcontracts")]
	#[serde(serialize_with = "serialize_vec_script_hash_option")]
	#[serde(deserialize_with = "deserialize_vec_script_hash_option")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allowed_contracts: Option<Vec<H160>>,

	#[serde(rename = "allowedgroups")]
	pub allowed_groups: Option<Vec<Secp256r1PublicKey>>,

	#[serde(rename = "rules")]
	pub rules: Option<Vec<WitnessRule>>,
}

impl Hash for TransactionSigner {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.account.hash(state);
		self.scopes.hash(state);
		self.allowed_contracts.hash(state);
		self.allowed_groups.hash(state);
		self.rules.hash(state);
	}
}

impl TransactionSigner {
	pub fn new(account: H160, scopes: Vec<WitnessScope>) -> Self {
		Self { account, scopes, allowed_contracts: None, allowed_groups: None, rules: None }
	}

	pub fn new_full(
		account: H160,
		scopes: Vec<WitnessScope>,
		allowed_contracts: Vec<H160>,
		allowed_groups: Vec<Secp256r1PublicKey>,
		rules: Vec<WitnessRule>,
	) -> Self {
		Self {
			account,
			scopes,
			allowed_contracts: Some(allowed_contracts),
			allowed_groups: Some(allowed_groups),
			rules: Some(rules),
		}
	}
}

impl SignerTrait for TransactionSigner {
	fn get_type(&self) -> SignerType {
		SignerType::Transaction
	}

	fn get_signer_hash(&self) -> &H160 {
		&self.account
	}

	fn set_signer_hash(&mut self, signer_hash: H160) {
		self.account = signer_hash;
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
		panic!("Not implemented")
	}

	fn get_allowed_contracts_mut(&mut self) -> &mut Vec<H160> {
		panic!("Not implemented")
	}

	fn get_allowed_groups(&self) -> &Vec<Secp256r1PublicKey> {
		panic!("Not implemented")
		// &self.allowed_groups
	}

	fn get_allowed_groups_mut(&mut self) -> &mut Vec<Secp256r1PublicKey> {
		panic!("Not implemented")
	}

	fn get_rules(&self) -> &Vec<WitnessRule> {
		panic!("Not implemented")
	}

	fn get_rules_mut(&mut self) -> &mut Vec<WitnessRule> {
		panic!("Not implemented")
	}
}

impl NeoSerializable for TransactionSigner {
	type Error = TransactionError;

	fn size(&self) -> usize {
		let mut size = (NeoConstants::HASH160_SIZE + 1) as usize;
		if self.scopes.contains(&WitnessScope::CustomContracts) {
			size += &self.allowed_contracts.clone().unwrap().var_size();
		}
		if self.scopes.contains(&WitnessScope::CustomGroups) {
			size += &self.allowed_groups.clone().unwrap().var_size();
		}

		if self.scopes.contains(&WitnessScope::WitnessRules) {
			size += &self.rules.clone().unwrap().var_size();
		}

		size
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_serializable_fixed(self.get_signer_hash());
		writer.write_u8(WitnessScope::combine(self.scopes.as_slice()));
		if self.scopes.contains(&WitnessScope::CustomContracts) {
			writer.write_serializable_variable_list(self.allowed_contracts.as_ref().unwrap());
		}
		if self.scopes.contains(&WitnessScope::CustomGroups) {
			writer.write_serializable_variable_list(self.allowed_groups.as_ref().unwrap());
		}
		if self.scopes.contains(&WitnessScope::WitnessRules) {
			writer.write_serializable_variable_list(self.rules.as_ref().unwrap());
		}
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		let mut signer = TransactionSigner::default();
		signer.set_signer_hash(reader.read_serializable().unwrap());
		let scopes = WitnessScope::split(reader.read_u8());
		signer.set_scopes(scopes);
		if signer.get_scopes().contains(&WitnessScope::CustomContracts) {
			signer.allowed_contracts = Some(reader.read_serializable_list().unwrap());
		}
		if signer.get_scopes().contains(&WitnessScope::CustomGroups) {
			signer.allowed_groups = Some(reader.read_serializable_list().unwrap());
		}
		if signer.get_scopes().contains(&WitnessScope::WitnessRules) {
			signer.rules = Some(reader.read_serializable_list().unwrap());
		}
		Ok(signer)
	}

	fn to_array(&self) -> Vec<u8> {
		let writer = &mut Encoder::new();
		self.encode(writer);
		writer.to_bytes()
	}
}
