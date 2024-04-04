use std::fmt;

use ethereum_types::Address;
use primitive_types::H256;
use signature::hazmat::PrehashSigner;

use neo::{
	crypto::Secp256r1Signature,
	prelude::{Transaction, WalletError},
};

/// An Ethereum private-public key pair which can be used for signing messages.
///
/// # Examples
///
/// ## Signing and Verifying a message
///
/// The wallet can be used to produce ECDSA [`Secp256r1Signature`] objects, which can be
/// then verified. Note that this uses [`hash_message`] under the hood which will
/// prefix the message being hashed with the `Ethereum Signed Message` domain separator.
///
/// ```
///
/// # use rand::thread_rng;
/// use neo_rs::prelude::LocalSigner;
///  async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// let wallet = LocalSigner::new(&mut thread_rng());
///
/// let wallet = wallet.with_network(1337u64);
///
/// // The wallet can be used to sign messages
/// let message = b"hello";
/// let signature = wallet.sign_message(message).await?;
/// assert_eq!(signature.recover(&message[..]).unwrap(), wallet.address());
///
/// // LocalSigner is clonable:
/// let wallet_clone = wallet.clone();
/// let signature2 = wallet_clone.sign_message(message).await?;
/// assert_eq!(signature, signature2);
/// # Ok(())
/// # }
/// ```
///
/// [`Secp256r1Signature`]: ethers_core::types::Secp256r1Signature
/// [`hash_message`]: fn@ethers_core::utils::hash_message
#[derive(Clone)]
pub struct WalletSigner<D: PrehashSigner<Secp256r1Signature>> {
	/// The WalletSigner's private Key
	pub(crate) signer: D,
	/// The wallet's address
	pub(crate) address: Address,
	pub(crate) network: Option<u64>,
}

impl<D: PrehashSigner<Secp256r1Signature>> WalletSigner<D> {
	/// Creates a new `WalletSigner` instance using an external `Signer` and associated Ethereum address.
	///
	/// # Arguments
	///
	/// * `signer` - An implementation of the `PrehashSigner` trait capable of signing messages.
	/// * `address` - The Ethereum address associated with the signer's public key.
	///
	/// # Returns
	///
	/// A new `WalletSigner` instance.
	pub fn new_with_signer(signer: D, address: Address) -> Self {
		WalletSigner { signer, address, network: None }
	}
}

impl<D: Sync + Send + PrehashSigner<Secp256r1Signature>> WalletSigner<D> {
	/// Signs a given `Transaction`, using the wallet's private key.
	///
	/// # Arguments
	///
	/// * `tx` - A reference to the transaction to be signed.
	///
	/// # Returns
	///
	/// A `Result` containing the `Secp256r1Signature` of the transaction, or a `WalletError` on failure.
	async fn sign_transaction(&self, tx: &Transaction) -> Result<Secp256r1Signature, WalletError> {
		let mut tx_with_network = tx.clone();
		if tx_with_network.network().is_none() {
			// in the case we don't have a network, let's use the signer chain id instead
			tx_with_network.set_network(self.network.unwrap() as u32);
		}
		self.signer
			.sign_prehash(&tx_with_network.get_hash_data().unwrap())
			.map_err(|_| WalletError::SignHashError)
	}

	/// Signs a given hash directly, without performing any additional hashing.
	///
	/// # Arguments
	///
	/// * `hash` - A `H256` hash to be signed.
	///
	/// # Returns
	///
	/// A `Result` containing the `Secp256r1Signature` of the hash, or a `WalletError` on failure.
	pub fn sign_hash(&self, hash: H256) -> Result<Secp256r1Signature, WalletError> {
		self.signer.sign_prehash(hash.as_ref()).map_err(|_| WalletError::SignHashError)
	}

	/// Returns a reference to the wallet's signer.
	///
	/// # Returns
	///
	/// A reference to the `D`, the signer.
	pub fn signer(&self) -> &D {
		&self.signer
	}

	/// Returns the wallet's associated address.
	///
	/// # Returns
	///
	/// The `Address` of the wallet.
	fn address(&self) -> Address {
		self.address
	}

	/// Gets the wallet's chain id
	fn network(&self) -> Option<u64> {
		self.network
	}

	/// Associates the wallet with a specific network ID.
	///
	/// # Arguments
	///
	/// * `network` - The network ID to associate with the wallet.
	///
	/// # Returns
	///
	/// The `WalletSigner` instance with the network ID set.
	fn with_network<T: Into<u64>>(mut self, network: T) -> Self {
		self.network = Some(network.into());
		self
	}
}

// do not log the signer
impl<D: PrehashSigner<Secp256r1Signature>> fmt::Debug for WalletSigner<D> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("WalletSigner")
			.field("address", &self.address)
			.field("chain_Id", &self.network)
			.finish()
	}
}
