use getset::{Getters, Setters};
use neo::prelude::*;
use primitive_types::H160;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
pub struct NeoURI<'a, P: JsonRpcClient> {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_url_option")]
	#[serde(serialize_with = "serialize_url_option")]
	#[getset(get = "pub", set = "pub")]
	uri: Option<Url>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_script_hash_option")]
	#[serde(serialize_with = "serialize_script_hash_option")]
	#[getset(get = "pub", set = "pub")]
	recipient: Option<ScriptHash>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_script_hash_option")]
	#[serde(serialize_with = "serialize_script_hash_option")]
	#[getset(get = "pub", set = "pub")]
	token: Option<ScriptHash>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[getset(get = "pub", set = "pub")]
	amount: Option<u64>,
	#[serde(skip)]
	provider: Option<&'a Provider<P>>,
}

impl<'a, P: JsonRpcClient> NeoURI<'a, P> {
	const NEO_SCHEME: &'static str = "neo";
	const MIN_NEP9_URI_LENGTH: usize = 38;
	const NEO_TOKEN_STRING: &'static str = "neo";
	const GAS_TOKEN_STRING: &'static str = "gas";

	pub fn new(provider: Option<&'a Provider<P>>) -> Self {
		Self { uri: None, recipient: None, token: None, amount: None, provider }
	}

	pub fn from_uri(uri_string: &str) -> Result<Self, ContractError> {
		let parts: Vec<&str> = uri_string.split(".unwrap()").collect();
		let base = parts[0];
		let query = if parts.len() > 1 { Some(parts[1]) } else { None };

		let base_parts: Vec<&str> = base.split(":").collect();
		if base_parts.len() != 2
			|| base_parts[0] != Self::NEO_SCHEME
			|| uri_string.len() < Self::MIN_NEP9_URI_LENGTH
		{
			return Err(ContractError::InvalidNeoName("Invalid NEP-9 URI".to_string()))
		}

		let mut neo_uri = Self::new(None);
		neo_uri.set_recipient(ScriptHash::from_address(base_parts[1]).ok());

		if let Some(query_str) = query {
			for part in query_str.split("&") {
				let kv: Vec<&str> = part.split("=").collect();
				if kv.len() != 2 {
					return Err(ContractError::InvalidNeoName("Invalid query".to_string()))
				}

				match kv[0] {
					"asset" if neo_uri.token().is_none() => {
						&neo_uri.set_token(H160::from_str(kv[1]).ok());
					},
					"amount" if neo_uri.amount.is_none() => {
						neo_uri.amount = Some(kv[1].parse().unwrap());
					},
					_ => {},
				}
			}
		}

		Ok(neo_uri)
	}

	// Getters

	pub fn uri_string(&self) -> Option<String> {
		self.uri.as_ref().map(|uri| uri.to_string())
	}

	pub fn recipient_address(&self) -> Option<String> {
		self.recipient.as_ref().map(H160::to_address)
	}

	pub fn token_string(&self) -> Option<String> {
		self.token.as_ref().map(|token| match token {
			token if *token == NeoToken::<P>::new(None).script_hash() =>
				Self::NEO_TOKEN_STRING.to_owned(),
			token if *token == GasToken::<P>::new(None).script_hash() =>
				Self::GAS_TOKEN_STRING.to_owned(),
			_ => ScriptHashExtension::to_bs58_string(token),
		})
	}

	// Builders

	pub async fn build_transfer_from(
		&self,
		sender: &Account,
	) -> Result<TransactionBuilder<P>, ContractError> {
		let recipient = self
			.recipient
			.ok_or(ContractError::InvalidStateError("Recipient not set".to_string()))
			.unwrap();
		let amount = self
			.amount
			.ok_or(ContractError::InvalidStateError("Amount not set".to_string()))
			.unwrap();
		let token_hash = self
			.token
			.ok_or(ContractError::InvalidStateError("Token not set".to_string()))
			.unwrap();

		let token = &mut FungibleTokenContract::new(&token_hash, self.provider);

		// Validate amount precision
		let amount_scale = (amount as f64).log10().floor() as u32 + 1; //amount.scale() as u8; //.scale();

		if Self::is_neo_token(&token_hash) && amount_scale > 0 {
			return Err(ContractError::from(ContractError::InvalidArgError(
				"NEO does not support decimals".to_string(),
			)))
		}

		if Self::is_gas_token(&token_hash)
			&& amount_scale > GasToken::<P>::new(None).decimals().unwrap() as u32
		{
			return Err(ContractError::from(ContractError::InvalidArgError(
				"Too many decimal places for GAS".to_string(),
			)))
		}

		let decimals = token.get_decimals().await.unwrap();
		if amount_scale > decimals as u32 {
			return Err(ContractError::from(ContractError::InvalidArgError(
				"Too many decimal places for token".to_string(),
			)))
		}

		let amt = token.to_fractions(amount, 0).unwrap();
		token
			.transfer_from_account(sender, &recipient, amt, None)
			.await
			.map_err(|e| ContractError::from(e))
	}

	// Helpers

	fn is_neo_token(token: &H160) -> bool {
		token == &NeoToken::<P>::new(None).script_hash()
	}

	fn is_gas_token(token: &H160) -> bool {
		token == &GasToken::<P>::new(None).script_hash()
	}

	// Setters

	pub fn token_str(&mut self, token_str: &str) {
		self.token = match token_str {
			Self::NEO_TOKEN_STRING => Some(NeoToken::new(self.provider).script_hash()),
			Self::GAS_TOKEN_STRING => Some(GasToken::new(self.provider).script_hash()),
			_ => Some(token_str.parse().unwrap()),
		};
	}

	// URI builder

	fn build_query(&self) -> String {
		let mut parts = Vec::new();

		if let Some(token) = &self.token {
			let token_str = match token {
				token if *token == NeoToken::new(self.provider).script_hash() =>
					Self::NEO_TOKEN_STRING.to_owned(),
				token if *token == GasToken::new(self.provider).script_hash() =>
					Self::GAS_TOKEN_STRING.to_owned(),
				_ => ScriptHashExtension::to_bs58_string(token),
			};

			parts.push(format!("asset={}", token_str));
		}

		if let Some(amount) = &self.amount {
			parts.push(format!("amount={}", amount));
		}

		parts.join("&")
	}

	pub fn build_uri(&mut self) -> Result<Url, ContractError> {
		let recipient = self
			.recipient
			.ok_or(ContractError::InvalidStateError("No recipient set".to_string()))
			.unwrap();

		let base = format!("{}:{}", Self::NEO_SCHEME, recipient.to_address());
		let query = self.build_query();
		let uri_str = if query.is_empty() { base } else { format!("{}.unwrap(){}", base, query) };

		self.uri = Some(uri_str.parse().unwrap());

		Ok(self.uri.clone().unwrap())
	}
}
