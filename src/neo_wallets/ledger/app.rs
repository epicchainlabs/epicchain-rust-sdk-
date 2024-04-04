#![allow(unused)]

use std::convert::TryFrom;

use coins_ledger::{
	common::{APDUAnswer, APDUCommand, APDUData},
	transports::{Ledger, LedgerAsync},
};
use futures_executor::block_on;
use futures_util::lock::Mutex;
use primitive_types::U256;
use thiserror::Error;

use neo::prelude::{Address, Secp256r1Signature, Transaction};

use super::types::*;

/// A Ledger Neo App.
///
/// This is a simple wrapper around the [Ledger transport](Ledger)
#[derive(Debug)]
pub struct LedgerNeo {
	transport: Mutex<Ledger>,
	derivation: DerivationType,
	pub(crate) network: u64,
	pub(crate) address: Address,
}

impl std::fmt::Display for LedgerNeo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"LedgerApp. Key at index {} with address {:?} on network {}",
			self.derivation, self.address, self.network
		)
	}
}

impl LedgerNeo {
	/// Instantiate the application by acquiring a lock on the ledger device.
	///
	///
	/// ```
	/// use neo_rs::prelude::{HDPath, Ledger};
	///  async fn foo() -> Result<(), Box<dyn std::error::Error>> {
	///
	/// let ledger = Ledger::new(HDPath::LedgerLive(0), 1).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn new(derivation: DerivationType, network: u64) -> Result<Self, LedgerError> {
		let transport = Ledger::init().await?;
		let address = Self::get_address_with_path_transport(&transport, &derivation).await?;

		Ok(Self { transport: Mutex::new(transport), derivation, network, address })
	}

	/// Consume self and drop the ledger mutex
	pub fn close(self) {}

	/// Get the account which corresponds to our derivation path
	pub async fn get_address(&self) -> Result<Address, LedgerError> {
		self.get_address_with_path(&self.derivation).await
	}

	/// Gets the account which corresponds to the provided derivation path
	pub async fn get_address_with_path(
		&self,
		derivation: &DerivationType,
	) -> Result<Address, LedgerError> {
		let data = APDUData::new(&Self::path_to_bytes(derivation));
		let transport = self.transport.lock().await;
		Self::get_address_with_path_transport(&transport, derivation).await
	}

	#[tracing::instrument(skip(transport))]
	async fn get_address_with_path_transport(
		transport: &Ledger,
		derivation: &DerivationType,
	) -> Result<Address, LedgerError> {
		let data = APDUData::new(&Self::path_to_bytes(derivation));

		let command = APDUCommand {
			ins: INS::GET_PUBLIC_KEY as u8,
			p1: P1::NON_CONFIRM as u8,
			p2: P2::NO_CHAINCODE as u8,
			data,
			response_len: None,
		};

		tracing::debug!("Dispatching get_address request to ethereum app");
		let answer = block_on(transport.exchange(&command))?;
		let result = answer.data().ok_or(LedgerError::UnexpectedNullResponse)?;

		let address = {
			// extract the address from the response
			let offset = 1 + result[0] as usize;
			let address_str = &result[offset + 1..offset + 1 + result[offset] as usize];
			let mut address = [0; 20];
			address.copy_from_slice(&hex::decode(address_str)?);
			Address::from(address)
		};
		tracing::debug!(?address, "Received address from device");
		Ok(address)
	}

	/// Returns the semver of the Neo ledger app
	pub async fn version(&self) -> Result<String, LedgerError> {
		let transport = self.transport.lock().await;

		let command = APDUCommand {
			ins: INS::GET_APP_CONFIGURATION as u8,
			p1: P1::NON_CONFIRM as u8,
			p2: P2::NO_CHAINCODE as u8,
			data: APDUData::new(&[]),
			response_len: None,
		};

		tracing::debug!("Dispatching get_version");
		let answer = block_on(transport.exchange(&command))?;
		let result = answer.data().ok_or(LedgerError::UnexpectedNullResponse)?;
		if result.len() < 4 {
			return Err(LedgerError::ShortResponse { got: result.len(), at_least: 4 })
		}
		let version = format!("{}.{}.{}", result[1], result[2], result[3]);
		tracing::debug!(version, "Retrieved version from device");
		Ok(version)
	}

	/// Signs a Neo transaction (requires confirmation on the ledger)
	pub async fn sign_tx(&self, tx: &Transaction) -> Result<Secp256r1Signature, LedgerError> {
		Ok(signature)
	}

	/// Signs an ethereum personal message
	pub async fn sign_message<S: AsRef<[u8]>>(
		&self,
		message: S,
	) -> Result<Secp256r1Signature, LedgerError> {
		let message = message.as_ref();

		let mut payload = Self::path_to_bytes(&self.derivation);
		payload.extend_from_slice(&(message.len() as u32).to_be_bytes());
		payload.extend_from_slice(message);

		self.sign_payload(INS::SIGN_PERSONAL_MESSAGE, &payload).await
	}

	#[tracing::instrument(err, skip_all, fields(command = %command, payload = hex::encode(payload)))]
	// structs
	pub async fn sign_payload(
		&self,
		command: INS,
		payload: &Vec<u8>,
	) -> Result<Secp256r1Signature, LedgerError> {
		if payload.is_empty() {
			return Err(LedgerError::EmptyPayload)
		}
		let transport = self.transport.lock().await;
		let mut command = APDUCommand {
			ins: command as u8,
			p1: P1_FIRST,
			p2: P2::NO_CHAINCODE as u8,
			data: APDUData::new(&[]),
			response_len: None,
		};

		let mut answer = None;
		// workaround for https://github.com/LedgerHQ/app-ethereum/issues/409
		// TODO: remove in future version
		let chunk_size =
			(0..=255).rev().find(|i| payload.len() % i != 3).expect("true for any length");

		// Iterate in 255 byte chunks
		let span = tracing::debug_span!("send_loop", index = 0, chunk = "");
		let guard = span.entered();
		for (index, chunk) in payload.chunks(chunk_size).enumerate() {
			guard.record("index", index);
			guard.record("chunk", hex::encode(chunk));
			command.data = APDUData::new(chunk);

			tracing::debug!("Dispatching packet to device");
			answer = Some(block_on(transport.exchange(&command))?);

			let data = answer.as_ref().expect("just assigned").data();
			if data.is_none() {
				return Err(LedgerError::UnexpectedNullResponse)
			}
			tracing::debug!(
				response = hex::encode(data.expect("just checked")),
				"Received response from device"
			);

			// We need more data
			command.p1 = P1::MORE as u8;
		}
		drop(guard);
		let answer = answer.expect("payload is non-empty, therefore loop ran");
		let result = answer.data().expect("check in loop");
		if result.len() < 65 {
			return Err(LedgerError::ShortResponse { got: result.len(), at_least: 65 })
		}
		let v = result[0] as u64;
		let r = U256::from_big_endian(&result[1..33]);
		let s = U256::from_big_endian(&result[33..]);
		let sig = Secp256r1Signature { r, s, v };
		tracing::debug!(sig = %sig, "Received signature from device");
		Ok(sig)
	}

	// helper which converts a derivation path to bytes
	fn path_to_bytes(derivation: &DerivationType) -> Vec<u8> {
		let derivation = derivation.to_string();
		let elements = derivation.split('/').skip(1).collect::<Vec<_>>();
		let depth = elements.len();

		let mut bytes = vec![depth as u8];
		for derivation_index in elements {
			let hardened = derivation_index.contains('\'');
			let mut index = derivation_index.replace('\'', "").parse::<u32>().unwrap();
			if hardened {
				index |= 0x80000000;
			}

			bytes.extend(index.to_be_bytes());
		}

		bytes
	}
}

#[cfg(all(test, feature = "ledger"))]
mod tests {
	use signature::digest::Mac;

	use neo::prelude::RawTransaction;

	use super::*;

	#[tokio::test]
	#[ignore]
	// Replace this with your ETH addresses.
	async fn test_get_address() {
		// Instantiate it with the default ledger derivation path
		let ledger = LedgerNeo::new(DerivationType::LedgerLive(0), 1).await.unwrap();
		assert_eq!(
			ledger.get_address().await.unwrap(),
			"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse().unwrap()
		);
		assert_eq!(
			ledger.get_address_with_path(&DerivationType::Legacy(0)).await.unwrap(),
			"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse().unwrap()
		);
	}

	#[tokio::test]
	#[ignore]
	async fn test_sign_tx() {
		let ledger = LedgerNeo::new(DerivationType::LedgerLive(0), 1).await.unwrap();

		// approve uni v2 router 0xff
		let data = hex::decode("095ea7b30000000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();

		let tx_req = RawTransaction::new()
			.to("2ed7afa17473e17ac59908f088b4371d28585476".parse::<Address>().unwrap())
			.gas(1000000)
			.gas_price(400e9 as u64)
			.nonce(5)
			.data(data)
			.value(utils::parse_ether(100).unwrap())
			.into();
		let tx = ledger.sign_transaction(&tx_req).await.unwrap();
	}

	#[tokio::test]
	#[ignore]
	async fn test_version() {
		let ledger = LedgerNeo::new(DerivationType::LedgerLive(0), 1).await.unwrap();

		let version = ledger.version().await.unwrap();
		assert_eq!(version, "1.3.7");
	}

	#[tokio::test]
	#[ignore]
	async fn test_sign_message() {
		let ledger = LedgerNeo::new(DerivationType::Legacy(0), 1).await.unwrap();
		let message = "hello world";
		let sig = ledger.sign_message(message).await.unwrap();
		let addr = ledger.get_address().await.unwrap();
		sig.verify(message, addr).unwrap();
	}
}
