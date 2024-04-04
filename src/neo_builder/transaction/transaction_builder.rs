use std::{
	collections::HashSet,
	fmt::Debug,
	hash::{Hash, Hasher},
	iter::Iterator,
	str::FromStr,
};

/// This module contains the implementation of the `TransactionBuilder` struct, which is used to build and configure transactions.
///
/// The `TransactionBuilder` struct has various fields that can be set using its methods. Once the fields are set, the `get_unsigned_tx` method can be called to obtain an unsigned transaction.
///
/// The `TransactionBuilder` struct implements various traits such as `Debug`, `Clone`, `Eq`, `PartialEq`, and `Hash`.
///
/// # Example
///
/// ```
///
/// use neo_rs::prelude::TransactionBuilder;
/// let mut tx_builder = TransactionBuilder::new();
/// tx_builder.version(0)
///           .nonce(1)
///           .valid_until_block(100)
///           .set_script(vec![0x01, 0x02, 0x03])
///           .get_unsigned_tx();
/// ```
use getset::{CopyGetters, Getters, MutGetters, Setters};
use once_cell::sync::Lazy;
use primitive_types::H160;
use rustc_serialize::hex::ToHex;

use neo::prelude::*;

#[derive(Getters, Setters, MutGetters, CopyGetters, Default)]
pub struct TransactionBuilder<P: JsonRpcClient + 'static> {
	provider: Option<&'static Provider<P>>,
	version: u8,
	nonce: u32,
	valid_until_block: Option<u32>,
	// setter and getter
	#[getset(get = "pub", set = "pub")]
	signers: Vec<Signer>,
	additional_network_fee: u64,
	additional_system_fee: u64,
	attributes: Vec<TransactionAttribute>,
	script: Option<Bytes>,
	fee_consumer: Option<Box<dyn Fn(u64, u64)>>,
	fee_error: Option<TransactionError>,
}

impl<P: JsonRpcClient> Debug for TransactionBuilder<P> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TransactionBuilder")
			.field("version", &self.version)
			.field("nonce", &self.nonce)
			.field("valid_until_block", &self.valid_until_block)
			.field("signers", &self.signers)
			.field("additional_network_fee", &self.additional_network_fee)
			.field("additional_system_fee", &self.additional_system_fee)
			.field("attributes", &self.attributes)
			.field("script", &self.script)
			// .field("fee_consumer", &self.fee_consumer)
			.field("fee_error", &self.fee_error)
			.finish()
	}
}

impl<P: JsonRpcClient> Clone for TransactionBuilder<P> {
	fn clone(&self) -> Self {
		Self {
			provider: self.provider,
			version: self.version,
			nonce: self.nonce,
			valid_until_block: self.valid_until_block,
			signers: self.signers.clone(),
			additional_network_fee: self.additional_network_fee,
			additional_system_fee: self.additional_system_fee,
			attributes: self.attributes.clone(),
			script: self.script.clone(),
			// fee_consumer: self.fee_consumer.clone(),
			fee_consumer: None,
			fee_error: None,
		}
	}
}

impl<P: JsonRpcClient> Eq for TransactionBuilder<P> {}

impl<P: JsonRpcClient> PartialEq for TransactionBuilder<P> {
	fn eq(&self, other: &Self) -> bool {
		self.version == other.version
			&& self.nonce == other.nonce
			&& self.valid_until_block == other.valid_until_block
			&& self.signers == other.signers
			&& self.additional_network_fee == other.additional_network_fee
			&& self.additional_system_fee == other.additional_system_fee
			&& self.attributes == other.attributes
			&& self.script == other.script
	}
}

impl<P: JsonRpcClient> Hash for TransactionBuilder<P> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.version.hash(state);
		self.nonce.hash(state);
		self.valid_until_block.hash(state);
		self.signers.hash(state);
		self.additional_network_fee.hash(state);
		self.additional_system_fee.hash(state);
		self.attributes.hash(state);
		self.script.hash(state);
	}
}

pub static GAS_TOKEN_HASH: Lazy<ScriptHash> =
	Lazy::new(|| ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf").unwrap());

impl<P: JsonRpcClient> TransactionBuilder<P> {
	// const GAS_TOKEN_HASH: ScriptHash = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf").unwrap();
	pub const BALANCE_OF_FUNCTION: &'static str = "balanceOf";
	pub const DUMMY_PUB_KEY: &'static str =
		"02ec143f00b88524caf36a0121c2de09eef0519ddbe1c710a00f0e2663201ee4c0";

	// Constructor
	pub fn new() -> Self {
		Self {
			provider: None,
			version: 0,
			nonce: 0,
			valid_until_block: None,
			signers: Vec::new(),
			additional_network_fee: 0,
			additional_system_fee: 0,
			attributes: Vec::new(),
			script: None,
			fee_consumer: None,
			fee_error: None,
		}
	}

	pub fn with_provider(provider: &'static Provider<P>) -> Self {
		Self {
			provider: Some(provider),
			version: 0,
			nonce: 0,
			valid_until_block: None,
			signers: Vec::new(),
			additional_network_fee: 0,
			additional_system_fee: 0,
			attributes: Vec::new(),
			script: None,
			fee_consumer: None,
			fee_error: None,
		}
	}

	// Configuration
	pub fn version(&mut self, version: u8) -> &mut Self {
		self.version = version;
		self
	}

	pub fn nonce(&mut self, nonce: u32) -> Result<&mut Self, TransactionError> {
		// Validate
		if nonce >= u32::MAX {
			return Err(TransactionError::InvalidNonce)
		}

		self.nonce = nonce;
		Ok(self)
	}

	// Other methods

	// Set valid until block
	pub fn valid_until_block(&mut self, block: u32) -> Result<&mut Self, TransactionError> {
		if block == 0 {
			return Err(TransactionError::InvalidBlock)
		}

		self.valid_until_block = Some(block);
		Ok(self)
	}

	// Set script
	pub fn set_script(&mut self, script: Bytes) -> &mut Self {
		self.script = Some(script);
		self
	}

	// Get unsigned transaction
	pub async fn get_unsigned_tx(&mut self) -> Result<Transaction<P>, TransactionError> {
		// Validate configuration
		if self.signers.is_empty() {
			return Err(TransactionError::NoSigners)
		}

		if self.script.is_none() {
			return Err(TransactionError::NoScript)
		}
		let len = self.signers.len();
		self.signers.dedup();

		// Validate no duplicate signers
		if len != self.signers.len() {
			return Err(TransactionError::DuplicateSigner)
		}

		// Check signer limits
		if self.signers.len() > NeoConstants::MAX_SIGNER_SUBITEMS as usize {
			return Err(TransactionError::TooManySigners)
		}

		// Validate script
		if let Some(script) = &self.script {
			if script.is_empty() {
				return Err(TransactionError::EmptyScript)
			}
		} else {
			return Err(TransactionError::NoScript)
		}

		// Get fees
		let system_fee = self.get_system_fee().await.unwrap();
		let network_fee = self.get_network_fee(&tx).await.unwrap();

		// Check sender balance if needed
		if let Some(fee_consumer) = &self.fee_consumer {
			let sender_balance = 0; // self.get_sender_balance().await.unwrap();
			if network_fee + system_fee > sender_balance {
				fee_consumer(network_fee + system_fee, sender_balance);
			}
		}

		Ok(Transaction {
			provider: None,
			version: self.version,
			nonce: self.nonce,
			valid_until_block: self.valid_until_block.unwrap(),
			size: 0,
			sys_fee: 0,
			net_fee: 0,
			signers: self.signers.clone(),
			attributes: self.attributes.clone(),
			script: self.script.clone().unwrap(), // We've already checked for None case above
			witnesses: vec![],
			block_time: None,
		})
	}

	// async fn get_system_fee(&self) -> Result<u64, TransactionError> {
	// 	let script = self.script.as_ref().unwrap();
	//
	// 	let response = NEO_INSTANCE
	// 		.read()
	// 		.unwrap()
	// 		.invoke_script(script.to_hex(), vec![self.signers[0].clone()])
	// 		.request()
	// 		.await
	// 		.unwrap();
	// 	Ok(u64::from_str(response.gas_consumed.as_str()).unwrap()) // example
	// }

	async fn get_network_fee(&mut self, tx: &Transaction<P>) -> Result<u64, TransactionError> {
		let fee = self.provider.unwrap().calculate_network_fee(tx.to_array().to_hex()).await?;
		Ok(fee)
	}

	async fn get_sender_balance(&self) -> Result<u64, TransactionError> {
		// Call network
		let sender = &self.signers[0];

		if Self::is_account_signer(sender) {
			let balance = self
				.provider
				.unwrap()
				.invoke_function(
					&GAS_TOKEN_HASH,
					Self::BALANCE_OF_FUNCTION.to_string(),
					vec![ContractParameter::from(sender.get_signer_hash())],
					None,
				)
				.await?
				.stack[0]
				.clone();
			return Ok(balance.as_int().unwrap() as u64)
		}
		Err(TransactionError::InvalidSender)
	}

	fn is_account_signer(signer: &Signer) -> bool {
		// let sig = <T as Signer>::SignerType;
		if signer.get_type() == SignerType::Account {
			return true
		}
		return false
	}

	// Sign transaction
	pub async fn sign(&mut self) -> Result<&mut Self, BuilderError> {
		let mut transaction =
			self.get_unsigned_tx().await.unwrap().with_provider(self.provider.unwrap());
		let tx_bytes = transaction.get_hash_data().await?;

		let mut witnesses_to_add = Vec::new();

		for signer in &mut transaction.signers {
			if Self::is_account_signer(signer) {
				let account_signer = signer.as_account_signer().unwrap();
				let acc = &account_signer.account;
				if acc.is_multi_sig() {
					return Err(BuilderError::IllegalState(
						"Transactions with multi-sig signers cannot be signed automatically."
							.to_string(),
					))
				}

				let key_pair = acc.key_pair().as_ref().ok_or_else(|| {
					BuilderError::InvalidConfiguration(
						"Cannot create transaction signature because account does not hold a private key.".to_string(),
					)
				})?;

				witnesses_to_add.push(Witness::create(tx_bytes.clone(), key_pair)?);
			} else {
				let contract_signer = signer.as_contract_signer().unwrap();
				witnesses_to_add
					.push(Witness::create_contract_witness(contract_signer.verify_params.clone())?);
			}
		}

		for witness in witnesses_to_add {
			transaction.add_witness(witness);
		}

		Ok(transaction)
	}

	fn signers_contain_multi_sig_with_committee_member(&self, committee: &HashSet<H160>) -> bool {
		for signer in &self.signers {
			if let Some(account_signer) = signer.as_account_signer() {
				if account_signer.is_multi_sig() {
					if let Some(script) = &account_signer.account().verification_script() {
						for pubkey in script.get_public_keys().unwrap() {
							let hash = public_key_to_script_hash(&pubkey);
							if committee.contains(&hash) {
								return true
							}
						}
					}
				}
			}
		}

		false
	}

	pub fn is_high_priority(&self) -> bool {
		self.attributes
			.iter()
			.any(|attr| matches!(attr, TransactionAttribute::HighPriority))
	}

	// async fn can_send_cover_fees(&self, fees: u64) -> Result<bool, BuilderError> {
	// 	let balance = self.get_sender_gas_balance().await?;
	// 	Ok(balance >= fees)
	// }

	// async fn get_sender_gas_balance(&self) -> Result<u64, BuilderError> {
	// 	let sender_hash = self.signers[0].get_signer_hash();
	// 	let result = NEO_INSTANCE
	// 		.read()
	// 		.unwrap()
	// 		.invoke_function(
	// 			&H160::from(Self::GAS_TOKEN_HASH),
	// 			Self::BALANCE_OF_FUNCTION.to_string(),
	// 			vec![sender_hash.into()],
	// 			vec![],
	// 		)
	// 		.request()
	// 		.await?;
	//
	// 	Ok(result.stack[0].as_int().unwrap() as u64)
	// }
}

#[cfg(test)]
mod tests {
	use std::{
		ops::Deref,
		str::FromStr,
		sync::{
			atomic::{AtomicBool, Ordering},
			Arc,
		},
	};

	use lazy_static::lazy_static;
	use openssl::rand;
	use primitive_types::{H160, H256};

	use neo::prelude::{
		Account, AccountSigner, AccountTrait, ContractSigner, Http, Middleware, NeoConstants,
		Provider, ScriptBuilder, TransactionAttribute, TransactionBuilder, Witness,
	};
	use rand::random;

	lazy_static! {
		pub static ref ACCOUNT1: Account = Account::from_private_key(
			"e6e919577dd7b8e97805151c05ae07ff4f752654d6d8797597aca989c02c4cb3"
		);
		pub static ref ACCOUNT2: Account = Account::from_private_key(
			"b4b2b579cac270125259f08a5f414e9235817e7637b9a66cfeb3b77d90c8e7f9"
		);
		pub static ref TEST_PROVIDER: Provider<Http> =
			Provider::<Http>::try_from(NeoConstants::SEED_1).unwrap();
	}

	#[tokio::test]
	async fn test_build_transaction_with_correct_nonce() {
		let mut nonce = rand::random::<u32>();

		let mut tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
			.valid_until_block(1)
			.unwrap()
			.set_script(vec![1, 2, 3])
			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into()])
			.nonce(nonce)
			.unwrap()
			.get_unsigned_tx()
			.await
			.unwrap();

		assert_eq!(tx.nonce(), nonce);

		nonce = 0;
		tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
			.valid_until_block(1)
			.unwrap()
			.set_script(vec![1, 2, 3])
			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).into()])
			.nonce(nonce)
			.unwrap()
			.get_unsigned_tx()
			.await
			.unwrap();
		assert_eq!(tx.nonce(), nonce);

		nonce = u32::MAX;
		tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
			.valid_until_block(1)
			.unwrap()
			.set_script(vec![1, 2, 3])
			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).into()])
			.nonce(nonce)
			.unwrap()
			.get_unsigned_tx()
			.await
			.unwrap();
		assert_eq!(tx.nonce(), nonce);
	}

	#[tokio::test]
	async fn test_invoke_script() {
		let script = ScriptBuilder::new()
			.contract_call(
				H160::from_str("86d58778c8d29e03182f38369f0d97782d303cc0").unwrap(),
				"symbol",
				vec![],
				None,
			)
			.unwrap()
			.to_bytes();

		let result = TransactionBuilder::new()
			.set_script(&script)
			.call_invoke_script()
			.await
			.unwrap()
			.invocation_result
			.unwrap();

		assert_eq!(result.stack()[0].as_str().unwrap(), "NEO");
	}

	#[tokio::test]
	async fn test_build_without_setting_script() {
		let err = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
			.get_unsigned_tx()
			.await
			.err()
			.unwrap();

		assert_eq!(err.to_string(), "Cannot build a transaction without a script");
	}

	#[tokio::test]
	async fn test_sign_transaction_with_additional_signers() {
		let script = vec![0x01, 0x02, 0x03];

		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
			.set_script(&script)
			.set_signers(vec![
				AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into(),
				AccountSigner::called_by_entry(ACCOUNT2.deref()).unwrap().into(),
			])
			.valid_until_block(1000)
			.unwrap()
			.sign()
			.await
			.unwrap();

		assert_eq!(tx.witnesses().len(), 2);

		let signers = tx
			.witnesses()
			.iter()
			.map(|witness| witness.verification_script().get_public_keys()[0])
			.collect::<Vec<_>>();

		assert!(signers.contains(&ACCOUNT1.deref().key_pair.unwrap().public_key()));
		assert!(signers.contains(&ACCOUNT2.deref().key_pair.unwrap().public_key()));
	}

	// 	#[tokio::test]
	// 	async fn test_send_invoke_function() {
	// 		let script = ScriptBuilder::new()
	// 			.contract_call(
	// 				H160::from_str("f46719e2d16bf50cddcef9d4bbfece901f73cbb6").unwrap(),
	// 				"transfer",
	// 				vec![
	// 					ACCOUNT1.into(),
	// 					2_000_000u32.into(),
	// 					None.into(),
	// 				],
	// 				None,
	// 			)
	// 			.to_array();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into()])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		tx.send().await.unwrap();
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_transfer_neo_from_normal_account() {
	// 		let expected_verification_script = ACCOUNT1.deref().verification_script().unwrap();
	//
	// 		let script = ScriptBuilder::new()
	// 			.contract_call(
	// 				H160::from_str("f46719e2d16bf50cddcef9d4bbfece901f73cbb6").unwrap(),
	// 				"transfer",
	// 				vec![ACCOUNT1.to_array(), recipient.to_array(), 5u32.to_array(), None::to_array()],
	// 				None,
	// 			).unwrap()
	// 			.to_bytes();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into()])
	// 			.valid_until_block(100)
	// 			.unwrap()
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.get_script(), script);
	// 		assert_eq!(tx.witnesses().len(), 1);
	// 		assert_eq!(tx.witnesses()[0].verification_script(), expected_verification_script);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_tracking_transaction_should_return_correct_block() {
	// 		// Mock network calls
	//
	// 		let script = ScriptBuilder::new()
	// 			.contract_call(
	// 				H160::from_str("f46719e2d16bf50cddcef9d4bbfece901f73cbb6").unwrap(),
	// 				"transfer",
	// 				vec![ACCOUNT1.to_array(), recipient.to_array(), 5u32.to_array(), None::to_array()],
	// 				None,
	// 			)
	// 			.unwrap()
	// 			.to_bytes();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.nonce(0)
	// 			.unwrap()
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref())])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		tx.send().await.unwrap();
	//
	// 		let block_num = tx.track().await.unwrap();
	//
	// 		assert_eq!(block_num, 1002);
	// 	}
	// 	#[tokio::test]
	// 	async fn test_get_application_log() {
	// 		// Mock network calls
	//
	// 		let script = ScriptBuilder::new()
	// 			.contract_call(
	// 				H160::from_str("f46719e2d16bf50cddcef9d4bbfece901f73cbb6").unwrap(),
	// 				"transfer",
	// 				vec![ACCOUNT1.to_array(), ACCOUNT1.to_array(), 1u32.to_array(), None::to_array()],
	// 				None,
	// 			)
	// 			.unwrap()
	// 			.to_bytes();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into()])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		tx.send().await.unwrap();
	//
	// 		let log = tx.get_application_log().await.unwrap();
	//
	// 		assert_eq!(
	// 			log.transaction_id,
	// 			H256::from_str("eb52f99ae5cf923d8905bdd91c4160e2207d20c0cb42f8062f31c6743770e4d1")
	// 				.unwrap()
	// 		);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_build_with_invalid_script() {
	// 		let script = vec![
	// 			0x0c, 0x0e, 0x4f, 0x72, 0x61, 0x63, 0x6c, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61,
	// 			0x63, 0x74, 0x41, 0x1a, 0xf7, 0x7b, 0x67,
	// 		];
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into()]);
	//
	// 		let err = builder.get_unsigned_tx().await.err().unwrap();
	//
	// 		assert!(err.to_string().contains("Instruction out of bounds"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_invoke_script_vm_faults() {
	// 		let script = vec![0x0c, 0x00, 0x12, 0x0c, 0x14, 0x93, 0xad, 0x15, 0x72];
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).unwrap().into()]);
	//
	// 		let err = builder.get_unsigned_tx().await.err().unwrap();
	//
	// 		assert_eq!(err.to_string(),
	// 				   "The vm exited due to the following exception: Value was either too large or too small for an Int32.");
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_get_unsigned_tx() {
	// 		let script = vec![0x01, 0x02, 0x03];
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref())])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.version(), 0);
	// 		assert_eq!(tx.signers().len(), 1);
	// 		assert!(tx.witnesses().is_empty());
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_additional_network_fee() {
	// 		let script = vec![0x01, 0x02, 0x03];
	//
	// 		let base_tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		let base_network_fee = base_tx.network_fee();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.additional_network_fee(2000)
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.network_fee(), base_network_fee + 2000);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_adding_more_than_max_attributes_to_tx() {
	// 		let one_too_many = NeoConstants::MAX_TRANSACTION_ATTRIBUTES + 1;
	// 		let attributes = vec![TransactionAttribute::HighPriority; one_too_many as usize];
	//
	// 		let error = TransactionBuilder::with_provider(TEST_PROVIDER.deref()).attributes(&attributes).await.err().unwrap();
	//
	// 		assert_eq!(
	// 			error.to_string(),
	// 			format!(
	// 				"A transaction cannot have more than {} attributes",
	// 				NeoConstants::MAX_TRANSACTION_ATTRIBUTES
	// 			)
	// 		);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_adding_more_than_max_attributes_to_tx_with_signers() {
	// 		let mut builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref());
	// 		builder.set_signers(vec![
	// 			AccountSigner::called_by_entry(&Account::default()).unwrap().into(),
	// 			AccountSigner::called_by_entry(&Account::default()).unwrap().into(),
	// 			AccountSigner::called_by_entry(&Account::default()).unwrap().into(),
	// 		]);
	//
	// 		let attributes = vec![
	// 			TransactionAttribute::HighPriority;
	// 			(NeoConstants::MAX_TRANSACTION_ATTRIBUTES - 3) as usize
	// 		];
	//
	// 		let error = builder.attributes(&attributes).await.err().unwrap();
	//
	// 		assert!(error.to_string().contains("A transaction cannot have more than"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_automatic_setting_of_valid_until_block_variable() {
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.script(vec![1, 2, 3])
	// 			.set_signers(vec![AccountSigner::none(&Account::default())])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.valid_until_block(), 1000); // Assuming default is 1000
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_extend_script() {
	// 		let script1 = ScriptBuilder::new()
	// 			.contract_call(H160::default(), "method1", vec![])
	// 			.to_array();
	//
	// 		let script2 = ScriptBuilder::new()
	// 			.contract_call(H160::default(), "method2", vec![])
	// 			.to_array();
	//
	// 		let mut builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref()).script(&script1);
	//
	// 		assert_eq!(builder.script(), &script1);
	//
	// 		builder.extend_script(&script2);
	//
	// 		assert_eq!(builder.script(), &[&script1[..], &script2[..]].concat());
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_invoking_with_params_should_produce_correct_request() {
	// 		// Mock RPC call
	//
	// 		let result = TEST_PROVIDER
	// 			.deref()
	// 			.invoke_function(
	// 				&H160::default(),
	// 				"transfer".to_string(),
	// 				vec![ACCOUNT1.to_array(), recipient.to_array(), 5u32.to_array(), None::to_array()],
	// 				None,
	// 			)
	// 			.await
	// 			.unwrap();
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_do_if_sender_cannot_cover_fees() {
	// 		// Mock RPC calls
	//
	// 		let tested = Arc::new(AtomicBool::new(false));
	//
	// 		let script = vec![0x01];
	//
	// 		TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(ACCOUNT1.deref()).into()])
	// 			.valid_until_block(100)
	// 			.do_if_sender_cannot_cover_fees(|fee, balance| {
	// 				assert_eq!(fee, 1_000_000);
	// 				assert_eq!(balance, 100_000);
	// 				tested.store(true, Ordering::SeqCst);
	// 			})
	// 			.await
	// 			.unwrap();
	//
	// 		assert!(tested.load(Ordering::SeqCst));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_throw_if_sender_cannot_cover_fees() {
	// 		// Mock RPC calls
	//
	// 		let script = vec![0x01];
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(account1.clone()).unwrap().into()])
	// 			.valid_until_block(100);
	//
	// 		let error = builder
	// 			.throw_if_sender_cannot_cover_fees(neo_swift::Error::illegal_state("test error"))
	// 			.await
	// 			.err()
	// 			.unwrap();
	//
	// 		assert_eq!(error.to_string(), "test error");
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_sign_transaction_with_contract_witness() {
	// 		let contract_hash = H160::default();
	// 		let invocation_script = vec![0x01, 0x02, 0x03];
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![0x01])
	// 			.set_signers(vec![
	// 				AccountSigner::called_by_entry(&Account::default()).unwrap().into(),
	// 				ContractSigner::global(contract_hash, invocation_script).into(),
	// 			])
	// 			.valid_until_block(1000)
	// 			.unwrap()
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		assert!(tx.witnesses().contains(&Witness::from_scripts(invocation_script, vec![])));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_set_first_signer() {
	// 		let account1 = Account::default();
	// 		let account2 = Account::default();
	//
	// 		let mut builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref()).set_script(vec![]).set_signers(vec![
	// 			AccountSigner::global(account1.clone()).unwrap().into(),
	// 			AccountSigner::called_by_entry(account2.clone()).unwrap().into(),
	// 		]);
	//
	// 		builder.first_signer(account2.address_or_scripthash().script_hash());
	// 		assert_eq!(builder.signers()[0].get_signer_hash(), account2.address_or_scripthash().script_hash());
	//
	// 		builder.first_signer(account1.address_or_scripthash().script_hash());
	// 		assert_eq!(builder.signers()[0].get_signer_hash(), account1.get_script_hash());
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_transmission_on_fault() {
	// 		// Mock RPC call with VM fault
	//
	// 		neo_swift::allow_transmission_on_fault();
	// 		assert!(neo_swift::allows_transmission_on_fault());
	//
	// 		let script = vec![0x01]; // Faulting script
	//
	// 		let account = Account::default();
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::none(account).unwrap().into()]);
	//
	// 		let result = builder.call_invoke_script().await.unwrap();
	// 		assert!(result.has_state_fault());
	//
	// 		let gas_consumed = result.gas_consumed() as u64;
	// 		let tx = builder.get_unsigned_tx().await.unwrap();
	//
	// 		assert_eq!(tx.system_fee(), gas_consumed);
	//
	// 		TEST_PROVIDER.deref().prevent_transmission_on_fault();
	// 		assert!(!neo_swift::allows_transmission_on_fault());
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_prevent_transmission_on_fault() {
	// 		assert!(!neo_swift::allows_transmission_on_fault());
	//
	// 		let script = vec![0x01]; // Faulting script
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::none(&Account::default()).unwrap().into()]);
	//
	// 		let result = builder.get_unsigned_tx().await.err().unwrap();
	//
	// 		assert!(result.to_string().contains("The vm exited due to"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_get_application_log_tx_not_sent() {
	// 		// Mock RPC calls
	//
	// 		let script = ScriptBuilder::new()
	// 			.contract_call(&H160::default(), "transfer", vec![])
	// 			.unwrap()
	// 			.to_bytes();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(&Account::default()).unwrap().into()])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		let result = tx.get_application_log().await;
	//
	// 		assert!(result.is_err());
	// 		assert_eq!(
	// 			result.err().unwrap().to_string(),
	// 			"Cannot get the application log before transaction has been sent."
	// 		);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_get_application_log_not_existing() {
	// 		// Mock sending transaction and then mock RPC call returning null
	//
	// 		let script = ScriptBuilder::new()
	// 			.contract_call(H160::default(), "transfer", vec![])
	// 			.unwrap()
	// 			.to_bytes();
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(&script)
	// 			.set_signers(vec![AccountSigner::called_by_entry(Account::default()).unwrap().into()])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		tx.send().await.unwrap();
	//
	// 		let result = tx.get_application_log().await;
	//
	// 		assert!(result.is_err());
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_version() {
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.version(), 1);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_build_transaction_with_invalid_block_number() {
	// 		let result = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.valid_until_block(u32::MAX)
	// 			.set_script(vec![])
	// 			.set_signers(vec![AccountSigner::called_by_entry(Account::default()).unwrap().into()])
	// 			.get_unsigned_tx()
	// 			.await;
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("valid until block number cannot exceed"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_signing_with_account_without_ec_key_pair() {
	// 		let account = Account::from_verification_script(vec![0x01]);
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.set_signers(vec![AccountSigner::none(account).unwrap().into()]);
	//
	// 		let result = builder.sign().await;
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("does not have keys available to sign the transaction"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_tracking_transaction_tx_not_sent() {
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.set_signers(vec![AccountSigner::called_by_entry(&Account::default()).unwrap().into()])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		let result = tx.track().await;
	//
	// 		assert!(result.is_err());
	// 		assert_eq!(
	// 			result.err().unwrap().to_string(),
	// 			"Cannot subscribe before transaction has been sent."
	// 		);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_sending_transaction_because_it_doesnt_contain_the_right_number_of_witnesses()
	// 	{
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.set_signers(vec![AccountSigner::called_by_entry(&Account::default()).unwrap().into()])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		let result = tx.send().await;
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("does not have the same number of signers and witnesses"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_automatically_signing_with_multi_sig_account_signer() {
	// 		let multi_sig_account = Account::create_multi_sig(vec![], 1);
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.set_signers(vec![AccountSigner::none(multi_sig_account).unwrap().into()]);
	//
	// 		let result = builder.sign().await;
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("Transactions with multi-sig signers cannot be signed automatically"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_with_no_signing_account() {
	// 		let contract_account = Account::from_script_hash(H160::default());
	//
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.set_signers(vec![ContractSigner::called_by_entry(contract_account, vec![])]);
	//
	// 		let result = builder.sign().await;
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("requires at least one signing account"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_nonce() {
	// 		let account = Account::default();
	//
	// 		let mut tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.nonce(123)
	// 			.unwrap()
	// 			.set_signers(vec![AccountSigner::called_by_entry(account.clone()).unwrap().into()])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.nonce(), 123);
	//
	// 		tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.nonce(u32::MAX)
	// 			.unwrap()
	// 			.set_signers(vec![AccountSigner::called_by_entry(account).unwrap().into()])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.nonce(), u32::MAX);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_automatically_set_nonce() {
	// 		// Mock RPC call to get account nonce
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_signers(vec![AccountSigner::called_by_entry(Account::default()).unwrap().into()])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert!(tx.nonce() > 0 && tx.nonce() < u32::MAX);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_additional_system_fee() {
	// 		let base_tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref()).get_unsigned_tx().await.unwrap();
	//
	// 		let base_fee = base_tx.sys_fee;
	//
	// 		let tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref());
	// 			tx.additional_system_fee = 12345;
	// 			tx.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	//
	// 		assert_eq!(tx.sys_fee, base_fee + 12345);
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_fail_building_transaction_with_incorrect_nonce() {
	// 		let builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref());
	//
	// 		let result = builder.nonce(u32::MAX + 1);
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("nonce must be less than or equal to u32::MAX"));
	//
	// 		let result = builder.nonce(u32::MAX + 2);
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("nonce must be less than or equal to u32::MAX"));
	//
	// 		let result = builder.nonce(-1);
	//
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("nonce must be greater than or equal to 0"));
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_override_signer() {
	// 		let account1 = Account::default();
	// 		let account2 = Account::default();
	//
	// 		let mut builder = TransactionBuilder::with_provider(TEST_PROVIDER.deref());
	//
	// 		builder.set_signers(vec![AccountSigner::global(account1.clone()).unwrap().into()]);
	//
	// 		assert_eq!(builder.signers()[0].get_signer_hash(), account1.script_hash());
	//
	// 		builder.set_signers(vec![AccountSigner::global(account2.clone()).unwrap().into()]);
	//
	// 		assert_eq!(builder.signers()[0].get_signer_hash(), account2.address_or_scripthash().script_hash());
	// 	}
	//
	// 	#[tokio::test]
	// 	async fn test_update_no_signed_transaction_attr_after_siging() {
	// 		let mut tx = TransactionBuilder::with_provider(TEST_PROVIDER.deref())
	// 			.set_script(vec![])
	// 			.set_signers(vec![AccountSigner::called_by_entry(&Account::default()).unwrap().into()])
	// 			.sign()
	// 			.await
	// 			.unwrap();
	//
	// 		assert!(tx.no_signed_transaction());
	//
	// 		let result = tx.no_signed_transaction(true);
	// 		assert!(result.is_err());
	// 		assert!(result
	// 			.err()
	// 			.unwrap()
	// 			.to_string()
	// 			.contains("no_signed_transaction attribute can only be set before signing"));
	// 	}
	//
	// 	#[tokio::test]
	// 	#[should_panic]
	// 	async fn test_build_with_no_signers() {
	// 		TransactionBuilder::<Http>::default()
	// 			.set_script(vec![])
	// 			.get_unsigned_tx()
	// 			.await
	// 			.unwrap();
	// 	}
}
