use neo::prelude::*;
use primitive_types::H160;
use rustc_serialize::hex::ToHex;
use serde_derive::{Deserialize, Serialize};
use signature::{hazmat::PrehashSigner, Error};
use std::{
	fmt::Debug,
	hash::{Hash, Hasher},
	str::FromStr,
};

pub trait AccountTrait: Sized + PartialEq + Send + Sync + Debug + Clone {
	type Error: Sync + Send + Debug + Sized;

	// Methods to access the fields
	fn key_pair(&self) -> &Option<KeyPair>;
	fn address_or_scripthash(&self) -> &AddressOrScriptHash;
	fn label(&self) -> &Option<String>;
	fn verification_script(&self) -> &Option<VerificationScript>;
	fn is_locked(&self) -> bool;
	fn encrypted_private_key(&self) -> &Option<String>;
	fn signing_threshold(&self) -> &Option<u32>;
	fn nr_of_participants(&self) -> &Option<u32>;
	fn set_key_pair(&mut self, key_pair: Option<KeyPair>);
	fn set_address_or_scripthash(&mut self, address_or_scripthash: AddressOrScriptHash);
	fn set_label(&mut self, label: Option<String>);
	fn set_verification_script(&mut self, verification_script: Option<VerificationScript>);
	fn set_locked(&mut self, is_locked: bool);
	fn set_encrypted_private_key(&mut self, encrypted_private_key: Option<String>);

	fn set_signing_threshold(&mut self, signing_threshold: Option<u32>);
	fn set_nr_of_participants(&mut self, nr_of_participants: Option<u32>);

	fn new(
		address: AddressOrScriptHash,
		label: Option<String>,
		verification_script: Option<VerificationScript>,
		signing_threshold: Option<u32>,
		nr_of_participants: Option<u32>,
	) -> Self;

	fn from_key_pair(
		key_pair: KeyPair,
		signing_threshold: Option<u32>,
		nr_of_participants: Option<u32>,
	) -> Result<Self, Self::Error>;

	fn from_key_pair_opt(
		key_pair: Option<KeyPair>,
		address: AddressOrScriptHash,
		label: Option<String>,
		verification_script: Option<VerificationScript>,
		is_locked: bool,
		is_default: bool,
		encrypted_private_key: Option<String>,
		signing_threshold: Option<u32>,
		nr_of_participants: Option<u32>,
	) -> Self;

	fn from_wif(wif: &str) -> Result<Self, Self::Error>;

	fn decrypt_private_key(&mut self, password: &str) -> Result<(), Self::Error>;

	fn encrypt_private_key(&mut self, password: &str) -> Result<(), Self::Error>;

	fn get_script_hash(&self) -> ScriptHash;

	fn get_signing_threshold(&self) -> Result<u32, Self::Error>;

	fn get_nr_of_participants(&self) -> Result<u32, Self::Error>;

	fn from_verification_script(script: &VerificationScript) -> Result<Self, Self::Error>;

	fn from_public_key(public_key: &Secp256r1PublicKey) -> Result<Self, Self::Error>;

	fn multi_sig_from_public_keys(
		public_keys: &mut [Secp256r1PublicKey],
		signing_threshold: u32,
	) -> Result<Self, Self::Error>;
	fn multi_sig_from_addr(
		address: String,
		signing_threshold: u8,
		nr_of_participants: u8,
	) -> Result<Self, Self::Error>;

	fn from_address(address: &str) -> Result<Self, Self::Error>;

	fn from_script_hash(script_hash: &H160) -> Result<Self, Self::Error>;

	fn create() -> Result<Self, Self::Error>;

	fn is_multi_sig(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Account {
	#[serde(skip)]
	pub key_pair: Option<KeyPair>,
	#[serde(
		serialize_with = "serialize_address_or_script_hash",
		deserialize_with = "deserialize_address_or_script_hash"
	)]
	pub address_or_scripthash: AddressOrScriptHash,
	pub label: Option<String>,
	pub verification_script: Option<VerificationScript>,
	pub is_default: bool,
	pub is_locked: bool,
	pub encrypted_private_key: Option<String>,
	pub signing_threshold: Option<u32>,
	pub nr_of_participants: Option<u32>,
}

impl From<H160> for Account {
	fn from(script_hash: H160) -> Self {
		Self {
			address_or_scripthash: AddressOrScriptHash::ScriptHash(script_hash),
			..Default::default()
		}
	}
}

impl From<&H160> for Account {
	fn from(script_hash: &H160) -> Self {
		Self {
			address_or_scripthash: AddressOrScriptHash::ScriptHash(script_hash.clone()),
			..Default::default()
		}
	}
}

impl PartialEq for Account {
	fn eq(&self, other: &Self) -> bool {
		self.address_or_scripthash == other.address_or_scripthash
			&& self.label == other.label
			&& self.verification_script == other.verification_script
			&& self.is_locked == other.is_locked
			&& self.encrypted_private_key == other.encrypted_private_key
			&& self.signing_threshold == other.signing_threshold
			&& self.nr_of_participants == other.nr_of_participants
	}
}

impl Hash for Account {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.address_or_scripthash.hash(state);
		self.label.hash(state);
		self.verification_script.hash(state);
		self.is_locked.hash(state);
		self.encrypted_private_key.hash(state);
		self.signing_threshold.hash(state);
		self.nr_of_participants.hash(state);
	}
}

impl AccountTrait for Account {
	type Error = ProviderError;

	fn key_pair(&self) -> &Option<KeyPair> {
		&self.key_pair
	}

	fn address_or_scripthash(&self) -> &AddressOrScriptHash {
		&self.address_or_scripthash
	}

	fn label(&self) -> &Option<String> {
		&self.label
	}

	fn verification_script(&self) -> &Option<VerificationScript> {
		&self.verification_script
	}

	fn is_locked(&self) -> bool {
		self.is_locked
	}

	fn encrypted_private_key(&self) -> &Option<String> {
		&self.encrypted_private_key
	}

	fn signing_threshold(&self) -> &Option<u32> {
		&self.signing_threshold
	}

	fn nr_of_participants(&self) -> &Option<u32> {
		&self.nr_of_participants
	}

	fn set_key_pair(&mut self, key_pair: Option<KeyPair>) {
		self.key_pair = key_pair;
	}

	fn set_address_or_scripthash(&mut self, address_or_scripthash: AddressOrScriptHash) {
		self.address_or_scripthash = address_or_scripthash;
	}

	fn set_label(&mut self, label: Option<String>) {
		self.label = label;
	}

	fn set_verification_script(&mut self, verification_script: Option<VerificationScript>) {
		self.verification_script = verification_script;
	}

	fn set_locked(&mut self, is_locked: bool) {
		self.is_locked = is_locked;
	}

	fn set_encrypted_private_key(&mut self, encrypted_private_key: Option<String>) {
		self.encrypted_private_key = encrypted_private_key;
	}

	fn set_signing_threshold(&mut self, signing_threshold: Option<u32>) {
		self.signing_threshold = signing_threshold;
	}

	fn set_nr_of_participants(&mut self, nr_of_participants: Option<u32>) {
		self.nr_of_participants = nr_of_participants;
	}

	fn new(
		address: AddressOrScriptHash,
		label: Option<String>,
		verification_script: Option<VerificationScript>,
		signing_threshold: Option<u32>,
		nr_of_participants: Option<u32>,
	) -> Self {
		Self {
			key_pair: None,
			address_or_scripthash: address,
			label,
			verification_script,
			is_default: false,
			is_locked: false,
			encrypted_private_key: None,
			signing_threshold,
			nr_of_participants,
		}
	}

	fn from_key_pair(
		key_pair: KeyPair,
		signing_threshold: Option<u32>,
		nr_of_participants: Option<u32>,
	) -> Result<Self, Self::Error> {
		let address = public_key_to_address(&key_pair.public_key);
		Ok(Self {
			key_pair: Some(key_pair.clone()),
			address_or_scripthash: AddressOrScriptHash::Address(address.clone()),
			label: Some(address),
			verification_script: Some(VerificationScript::from_public_key(
				&key_pair.clone().public_key(),
			)),
			is_default: false,
			is_locked: false,
			encrypted_private_key: None,
			signing_threshold,
			nr_of_participants,
		})
	}

	fn from_key_pair_opt(
		key_pair: Option<KeyPair>,
		address: AddressOrScriptHash,
		label: Option<String>,
		verification_script: Option<VerificationScript>,
		is_locked: bool,
		_is_default: bool,
		encrypted_private_key: Option<String>,
		signing_threshold: Option<u32>,
		nr_of_participants: Option<u32>,
	) -> Self {
		Self {
			key_pair,
			address_or_scripthash: address,
			label,
			verification_script,
			is_default: false,
			is_locked,
			encrypted_private_key,
			signing_threshold,
			nr_of_participants,
		}
	}

	fn from_wif(wif: &str) -> Result<Self, Self::Error> {
		let key_pair = KeyPair::from_secret_key(&private_key_from_wif(wif).unwrap());
		Self::from_key_pair(key_pair, None, None)
	}

	fn decrypt_private_key(&mut self, password: &str) -> Result<(), Self::Error> {
		if self.key_pair.is_some() {
			return Ok(())
		}

		let encrypted_private_key = self
			.encrypted_private_key
			.as_ref()
			.ok_or(Self::Error::IllegalState("No encrypted private key present".to_string()))
			.unwrap();
		let key_pair = get_private_key_from_nep2(encrypted_private_key, password).unwrap();
		self.key_pair =
			Some(KeyPair::from_private_key(&vec_to_array32(key_pair).unwrap()).unwrap());
		Ok(())
	}

	fn encrypt_private_key(&mut self, password: &str) -> Result<(), Self::Error> {
		let key_pair = self
			.key_pair
			.as_ref()
			.ok_or(Self::Error::IllegalState("No decrypted key pair present".to_string()))
			.unwrap();
		let encrypted_private_key = get_nep2_from_private_key(
			key_pair.private_key.to_raw_bytes().to_hex().as_str(),
			password,
		)
		.unwrap();
		self.encrypted_private_key = Some(encrypted_private_key);
		self.key_pair = None;
		Ok(())
	}

	fn get_script_hash(&self) -> ScriptHash {
		self.address_or_scripthash.script_hash()
	}

	fn get_signing_threshold(&self) -> Result<u32, Self::Error> {
		self.signing_threshold
			.ok_or_else(|| Self::Error::IllegalState("Account is not MultiSig".to_string()))
	}

	fn get_nr_of_participants(&self) -> Result<u32, Self::Error> {
		self.nr_of_participants
			.ok_or_else(|| Self::Error::IllegalState("Account is not MultiSig".to_string()))
	}

	fn from_verification_script(script: &VerificationScript) -> Result<Self, Self::Error> {
		let address = ScriptHash::from_script(&script.script());

		let (signing_threshold, nr_of_participants) = if script.is_multi_sig() {
			(
				Some(script.get_signing_threshold().unwrap()),
				Some(script.get_nr_of_accounts().unwrap()),
			)
		} else {
			(None, None)
		};

		Ok(Self {
			address_or_scripthash: AddressOrScriptHash::ScriptHash(address),
			label: Some(ScriptHashExtension::to_bs58_string(&address)),
			verification_script: Some(script.clone()),
			signing_threshold: signing_threshold.map(|x| x as u32),
			nr_of_participants: nr_of_participants.map(|x| x as u32),
			..Default::default()
		})
	}

	fn from_public_key(public_key: &Secp256r1PublicKey) -> Result<Self, Self::Error> {
		let script = VerificationScript::from_public_key(public_key);
		let address = ScriptHash::from_script(&script.script());

		Ok(Self {
			address_or_scripthash: AddressOrScriptHash::ScriptHash(address),
			label: Some(ScriptHashExtension::to_bs58_string(&address)),
			verification_script: Some(script),
			..Default::default()
		})
	}

	fn multi_sig_from_public_keys(
		public_keys: &mut [Secp256r1PublicKey],
		signing_threshold: u32,
	) -> Result<Self, Self::Error> {
		let script = VerificationScript::from_multi_sig(public_keys, signing_threshold as u8);
		let addr = ScriptHash::from_script(&script.script());

		Ok(Self {
			label: Some(script.script().to_base64()),
			verification_script: Some(script),
			signing_threshold: Some(signing_threshold),
			nr_of_participants: Some(public_keys.len() as u32),
			address_or_scripthash: AddressOrScriptHash::ScriptHash(addr),
			..Default::default()
		})
	}

	fn multi_sig_from_addr(
		address: String,
		signing_threshold: u8,
		nr_of_participants: u8,
	) -> Result<Self, Self::Error> {
		Ok(Self {
			label: Option::from(address.clone()),
			signing_threshold: Some(signing_threshold as u32),
			nr_of_participants: Some(nr_of_participants as u32),
			address_or_scripthash: AddressOrScriptHash::Address(address),
			..Default::default()
		})
	}

	fn from_address(address: &str) -> Result<Self, Self::Error> {
		let address = Address::from_str(address).unwrap();
		Ok(Self {
			address_or_scripthash: AddressOrScriptHash::Address(address.clone()),
			label: Some(address),
			..Default::default()
		})
	}

	fn from_script_hash(script_hash: &H160) -> Result<Self, Self::Error> {
		let address = script_hash.to_address();
		Self::from_address(&address)
	}

	fn create() -> Result<Self, Self::Error> {
		let key_pair = KeyPair::new_random();
		Self::from_key_pair(key_pair, None, None)
	}

	fn is_multi_sig(&self) -> bool {
		self.signing_threshold.is_some() && self.nr_of_participants.is_some()
	}
}

impl PrehashSigner<Secp256r1Signature> for Account {
	fn sign_prehash(&self, _prehash: &[u8]) -> Result<Secp256r1Signature, Error> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use neo::prelude::{
		Account, AccountTrait, KeyPair, PrivateKeyExtension, ScriptHashExtension,
		Secp256r1PublicKey, TestConstants, ToArray32, VerificationScript,
	};
	use rustc_serialize::hex::FromHex;

	#[test]
	fn test_create_generic_account() {
		let account = Account::create().unwrap();
		assert!(account.verification_script.is_some());
		assert!(account.key_pair.is_some());
		assert!(account.label.is_some());
		assert!(account.encrypted_private_key.is_none());
		assert!(!account.is_locked);
		assert!(!account.is_default);
	}

	#[test]
	fn test_init_account_from_existing_key_pair() {
		let key_pair = KeyPair::from_private_key(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY)
				.unwrap()
				.to_array32()
				.unwrap(),
		)
		.unwrap();
		let account = Account::from_key_pair(key_pair, None, None).unwrap();

		assert!(!account.is_multi_sig());
		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
		assert_eq!(account.label, Some(TestConstants::DEFAULT_ACCOUNT_ADDRESS.to_string()));
		assert_eq!(
			account.verification_script.unwrap().script(),
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_VERIFICATION_SCRIPT).unwrap()
		);
	}

	#[test]
	fn test_from_verification_script() {
		let verification_script = VerificationScript::from(
			hex::decode(TestConstants::COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT).unwrap(),
		);
		let account = Account::from_verification_script(&verification_script).unwrap();

		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::COMMITTEE_ACCOUNT_ADDRESS
		);
		assert_eq!(
			account.verification_script.unwrap().script(),
			&hex::decode(TestConstants::COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT).unwrap()
		);
	}

	#[test]
	fn test_from_public_key() {
		let public_key = Secp256r1PublicKey::from_bytes(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PUBLIC_KEY).unwrap(),
		)
		.unwrap();
		let account = Account::from_public_key(&public_key).unwrap();

		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
		assert_eq!(
			account.verification_script.unwrap().script(),
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_VERIFICATION_SCRIPT).unwrap()
		);
	}

	#[test]
	fn test_create_multi_sig_account_from_public_keys() {
		let public_key = Secp256r1PublicKey::from_bytes(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PUBLIC_KEY).unwrap(),
		)
		.unwrap();
		let account = Account::multi_sig_from_public_keys(&mut vec![public_key], 1).unwrap();

		assert!(account.is_multi_sig());
		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::COMMITTEE_ACCOUNT_ADDRESS.to_string()
		);
		assert_eq!(account.label, Some(TestConstants::COMMITTEE_ACCOUNT_ADDRESS.to_string()));
		assert_eq!(
			account.verification_script.unwrap().script(),
			&hex::decode(TestConstants::COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT).unwrap()
		);
	}

	#[test]
	fn test_nil_values_when_not_multi_sig() {
		let account = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS).unwrap();
		assert!(account.signing_threshold.is_none());
		assert!(account.nr_of_participants.is_none());
	}

	#[test]
	fn test_encrypt_public_key() {
		let key_pair = KeyPair::from_private_key(
			&TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY
				.from_hex()
				.unwrap()
				.to_array32()
				.unwrap(),
		)
		.unwrap();
		let mut account = Account::from_key_pair(key_pair, None, None).unwrap();

		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
		account.encrypt_private_key(TestConstants::DEFAULT_ACCOUNT_PASSWORD).unwrap();

		assert_eq!(
			account.encrypted_private_key.unwrap(),
			TestConstants::DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY
		);
	}

	// #[test]
	// fn test_to_nep6_account_with_only_an_address() {
	// 	let account = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS).unwrap();
	//
	// 	let nep6_account =  account.to_nep6_account().unwrap();
	//
	// 	assert!(nep6_account.contract.is_none());
	// 	assert!(!nep6_account.is_default);
	// 	// ...
	// }

	#[test]
	fn test_create_account_from_wif() {
		let account = Account::from_wif(TestConstants::DEFAULT_ACCOUNT_WIF).unwrap();

		let expected_key_pair = KeyPair::from_private_key(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY)
				.unwrap()
				.to_array32()
				.unwrap(),
		)
		.unwrap();

		assert_eq!(
			account.key_pair.clone().unwrap().public_key.get_encoded(false),
			expected_key_pair.public_key.get_encoded(false)
		);
		assert_eq!(
			account.key_pair.clone().unwrap().private_key.to_vec(),
			expected_key_pair.private_key.to_vec()
		);
		let addr = account.address_or_scripthash();
		assert_eq!(addr.address(), TestConstants::DEFAULT_ACCOUNT_ADDRESS);
	}

	#[test]
	fn test_create_account_from_address() {
		let account = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS).unwrap();

		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
		assert_eq!(account.label, Some(TestConstants::DEFAULT_ACCOUNT_ADDRESS.to_string()));
		assert_eq!(
			&account.address_or_scripthash.script_hash().to_hex(),
			TestConstants::DEFAULT_ACCOUNT_SCRIPT_HASH
		);
		// assert!(!account.is_default);
		assert!(!account.is_locked);
		assert!(account.verification_script.is_none());
	}

	#[test]
	fn test_get_nep17_balances() {
		// Create mock HTTP client

		// let account = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS).unwrap();
		//
		// let balances = account.get_nep17_balances(&mock_client).await.unwrap();
		//
		// assert_eq!(balances.len(), 2);
		// assert!(balances.contains_key(&TestConstants::GAS_TOKEN_HASH));
		// assert!(balances.contains_key(&TestConstants::NEO_TOKEN_HASH));
		// assert!(balances.values().contains(&300000000));
		// assert!(balances.values().contains(&5));
	}

	#[test]
	fn test_is_multi_sig() {
		let a = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS).unwrap();
		assert!(!a.is_multi_sig());

		let a1 = Account::multi_sig_from_addr(
			TestConstants::COMMITTEE_ACCOUNT_ADDRESS.to_string(),
			1,
			1,
		)
		.unwrap();
		assert!(a1.is_multi_sig());

		let a2 = Account::from_verification_script(&VerificationScript::from(
			hex::decode(TestConstants::COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT).unwrap(),
		))
		.unwrap();
		assert!(a2.is_multi_sig());

		let a3 = Account::from_verification_script(&VerificationScript::from(
			hex::decode(TestConstants::DEFAULT_ACCOUNT_VERIFICATION_SCRIPT).unwrap(),
		))
		.unwrap();
		assert!(!a3.is_multi_sig());
	}

	#[test]
	fn test_unlock() {
		let mut account = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS).unwrap();
		account.is_locked = true;
		assert!(account.is_locked);

		account.is_locked = false;
		assert!(!account.is_locked);
	}
}
