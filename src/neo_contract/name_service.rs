#![feature(const_trait_impl)]
use async_trait::async_trait;
use futures::FutureExt;
use neo::prelude::{
	deserialize_script_hash, deserialize_script_hash_option, serialize_script_hash,
	serialize_script_hash_option, AddressOrScriptHash, ContractError, ContractParameter,
	JsonRpcClient, Middleware, NNSName, NeoIterator, NonFungibleTokenTrait, Provider, ScriptHash,
	SmartContractTrait, StackItem, TokenTrait, TransactionBuilder,
};
use primitive_types::H160;
use serde::{Deserialize, Serialize};
use std::{string::ToString, sync::Arc};

#[repr(u8)]
enum RecordType {
	None = 0,
	Txt = 1,
	A = 2,
	Aaaa = 3,
	Cname = 4,
	Srv = 5,
	Url = 6,
	Oauth = 7,
	Ipfs = 8,
	Email = 9,
	Dnssec = 10,
	Tlsa = 11,
	Smimea = 12,
	Hippo = 13,
	Http = 14,
	Sshfp = 15,
	Onion = 16,
	Xmpp = 17,
	Magnet = 18,
	Tor = 19,
	I2p = 20,
	Git = 21,
	Keybase = 22,
	Briar = 23,
	Zcash = 24,
	Mini = 25,
}

// NameState struct

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NameState {
	pub name: String,
	pub expiration: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_script_hash_option")]
	#[serde(serialize_with = "serialize_script_hash_option")]
	pub admin: Option<ScriptHash>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeoNameService<'a, P: JsonRpcClient> {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	script_hash: ScriptHash,
	#[serde(skip)]
	provider: Option<&'a Provider<P>>,
}

impl<'a, P: JsonRpcClient> NeoNameService<'a, P> {
	const ADD_ROOT: &'static str = "addRoot";
	const ROOTS: &'static str = "roots";
	const SET_PRICE: &'static str = "setPrice";
	const GET_PRICE: &'static str = "getPrice";
	const IS_AVAILABLE: &'static str = "isAvailable";
	const REGISTER: &'static str = "register";
	const RENEW: &'static str = "renew";
	const SET_ADMIN: &'static str = "setAdmin";
	const SET_RECORD: &'static str = "setRecord";
	const GET_RECORD: &'static str = "getRecord";
	const GET_ALL_RECORDS: &'static str = "getAllRecords";
	const DELETE_RECORD: &'static str = "deleteRecord";
	const RESOLVE: &'static str = "resolve";
	const PROPERTIES: &'static str = "properties";

	const NAME_PROPERTY: &'static str = "name";
	const EXPIRATION_PROPERTY: &'static str = "expiration";
	const ADMIN_PROPERTY: &'static str = "admin";

	pub fn new(provider: Option<&'a Provider<P>>) -> Self {
		Self { script_hash: provider.unwrap().nns_resolver().clone(), provider }
	}

	// Implementation

	async fn add_root(&self, root: &str) -> Result<TransactionBuilder<P>, ContractError> {
		let args = vec![root.to_string().into()];
		self.invoke_function(Self::ADD_ROOT, args).await
	}

	async fn get_roots(&self) -> Result<NeoIterator<String, P>, ContractError> {
		let args = vec![];
		let roots = self
			.call_function_returning_iterator(
				Self::ROOTS,
				args,
				Arc::new(|item: StackItem| item.to_string()),
			)
			.await;

		Ok(roots)
	}

	async fn get_symbol(&self) -> Result<String, ContractError> {
		Ok("NNS".to_string())
	}

	async fn get_decimals(&self) -> Result<u8, ContractError> {
		Ok(0)
	}

	// Register a name

	pub async fn register(
		&self,
		name: &str,
		owner: H160,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.check_domain_name_availability(name, true).await.unwrap();

		let args = vec![name.into(), owner.into()];
		self.invoke_function(Self::REGISTER, args).await
	}

	// Set admin for a name

	pub async fn set_admin(
		&self,
		name: &str,
		admin: H160,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.check_domain_name_availability(name, true).await.unwrap();

		let args = vec![name.into(), admin.into()];
		self.invoke_function(Self::SET_ADMIN, args).await
	}

	// Set record

	pub async fn set_record(
		&self,
		name: &str,
		record_type: RecordType,
		data: &str,
	) -> Result<TransactionBuilder<P>, ContractError> {
		let args = vec![name.into(), (record_type as u8).into(), data.into()];

		self.invoke_function(Self::SET_RECORD, args).await
	}

	// Delete record

	pub async fn delete_record(
		&self,
		name: &str,
		record_type: RecordType,
	) -> Result<TransactionBuilder<P>, ContractError> {
		let args = vec![name.into(), (record_type as u8).into()];
		self.invoke_function(Self::DELETE_RECORD, args).await
	}

	pub async fn is_available(&self, name: &str) -> Result<bool, ContractError> {
		let args = vec![name.into()];
		self.call_function_returning_bool(Self::IS_AVAILABLE, args).await
	}
	pub async fn renew(
		&self,
		name: &str,
		years: u32,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.check_domain_name_availability(name, true).await.unwrap();

		let args = vec![name.into(), years.into()];
		self.invoke_function(Self::RENEW, args).await
	}

	async fn get_name_state(&self, name: &[u8]) -> Result<NameState, ContractError> {
		let args = vec![name.into()];
		let result = self
			.provider
			.unwrap()
			.invoke_function(&self.script_hash, Self::PROPERTIES.to_string(), args, None)
			.await
			.unwrap()
			.stack[0]
			.clone();

		let map = result.as_map().unwrap();
		let name = map
			.get(&StackItem::ByteString { value: Self::NAME_PROPERTY.to_string() })
			.unwrap()
			.as_string()
			.unwrap();
		let expiration = map
			.get(&StackItem::ByteString { value: Self::EXPIRATION_PROPERTY.to_string() })
			.unwrap()
			.as_int()
			.unwrap() as u32;
		let admin = map
			.get(&StackItem::ByteString { value: Self::ADMIN_PROPERTY.to_string() })
			.unwrap()
			.as_address()
			.unwrap();

		Ok(NameState {
			name,
			expiration,
			admin: Some(AddressOrScriptHash::from(admin).script_hash()),
		})
	}
	async fn check_domain_name_availability(
		&self,
		name: &str,
		should_be_available: bool,
	) -> Result<(), ContractError> {
		let is_available = self.is_available(name).await.unwrap();

		if should_be_available && !is_available {
			return Err(ContractError::DomainNameNotAvailable(
				"Domain name already taken".to_string(),
			))
		} else if !should_be_available && is_available {
			return Err(ContractError::DomainNameNotRegistered(
				"Domain name not registered".to_string(),
			))
		}

		Ok(())
	}
}

#[async_trait]
impl<'a, P: JsonRpcClient> TokenTrait<'a, P> for NeoNameService<'a, P> {
	fn total_supply(&self) -> Option<u64> {
		todo!()
	}

	fn set_total_supply(&mut self, _total_supply: u64) {
		todo!()
	}

	fn decimals(&self) -> Option<u8> {
		Some(0)
	}

	fn set_decimals(&mut self, _decimals: u8) {
		panic!("Cannot set decimals for NNS")
	}

	fn symbol(&self) -> Option<String> {
		Some("NNS".to_string())
	}

	fn set_symbol(&mut self, _symbol: String) {
		panic!("Cannot set symbol for NNS")
	}

	async fn resolve_nns_text_record(&self, name: &NNSName) -> Result<H160, ContractError> {
		let req = {
			self.provider()
				.unwrap()
				.invoke_function(
					&self.script_hash(),
					"resolve".to_string(),
					vec![
						ContractParameter::from(name.name()),
						ContractParameter::from(RecordType::Txt as u8),
					],
					None,
				)
				.await
		};

		let address = req.unwrap().stack.first().unwrap().clone();

		Ok(H160::from_slice(&address.as_bytes().unwrap()))
	}
}

impl<'a, P: JsonRpcClient> SmartContractTrait<'a> for NeoNameService<'a, P> {
	type P = P;

	fn set_name(&mut self, _name: String) {}

	fn script_hash(&self) -> H160 {
		self.script_hash
	}

	fn set_script_hash(&mut self, script_hash: H160) {
		self.script_hash = script_hash;
	}

	fn provider(&self) -> Option<&Provider<P>> {
		self.provider
	}
}

impl<'a, P: JsonRpcClient> NonFungibleTokenTrait<'a, P> for NeoNameService<'a, P> {}
