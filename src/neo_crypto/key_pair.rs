//! # KeyPair
//!
//! `KeyPair` is a module that provides an implementation for Elliptic Curve Key Pairs using the `p256` crate.
//!
//! This structure can be used to manage and manipulate EC key pairs,
//! including generating new pairs, importing them from raw bytes,
//! and converting them to various formats.

use rand::rngs::OsRng;

use neo::prelude::{
	wif_from_private_key, CryptoError, PublicKeyExtension, Secp256r1PrivateKey, Secp256r1PublicKey,
};

/// Represents an Elliptic Curve Key Pair containing both a private and a public key.

#[derive(Debug, Clone)]
pub struct KeyPair {
	/// The private key component of the key pair.
	pub private_key: Secp256r1PrivateKey,

	/// The public key component of the key pair.
	pub public_key: Secp256r1PublicKey,
}

impl KeyPair {
	/// Creates a new `KeyPair` instance given a private key and its corresponding public key.
	///
	/// # Arguments
	///
	/// * `private_key` - A `Secp256r1PrivateKey` representing the private key.
	/// * `public_key` - A `Secp256r1PublicKey` representing the public key.
	pub fn new(private_key: Secp256r1PrivateKey, public_key: Secp256r1PublicKey) -> Self {
		Self { private_key, public_key }
	}

	pub fn private_key(&self) -> Secp256r1PrivateKey {
		self.private_key.clone()
	}

	pub fn public_key(&self) -> Secp256r1PublicKey {
		self.public_key.clone()
	}

	/// Derives a new `KeyPair` instance from just a private key.
	/// The public key is derived from the given private key.
	///
	/// # Arguments
	///
	/// * `private_key` - A `Secp256r1PrivateKey` representing the private key.
	pub fn from_secret_key(private_key: &Secp256r1PrivateKey) -> Self {
		let public_key = private_key.clone().to_public_key();
		Self::new(private_key.clone(), public_key)
	}

	/// Returns the 32-byte representation of the private key.
	pub fn private_key_bytes(&self) -> [u8; 32] {
		self.private_key.to_raw_bytes()
	}

	/// Returns the 64-byte uncompressed representation of the public key.
	pub fn public_key_bytes(&self) -> [u8; 64] {
		let mut buf = [0u8; 64];
		// Convert the Secp256r1PublicKey to its byte representation
		let vec_bytes: Vec<u8> = self.public_key.to_vec(); // uncompressed form
		buf.copy_from_slice(&vec_bytes[0..64]);

		buf
	}
}

impl KeyPair {
	/// Generates a new random `KeyPair`.
	pub fn new_random() -> Self {
		let mut rng = OsRng; // A cryptographically secure random number generator
		let secret_key = Secp256r1PrivateKey::random(&mut rng);
		Self::from_secret_key(&secret_key)
	}

	/// Creates an `KeyPair` from a given 32-byte private key.
	///
	/// # Arguments
	///
	/// * `private_key` - A 32-byte slice representing the private key.
	pub fn from_private_key(private_key: &[u8; 32]) -> Result<Self, CryptoError> {
		let secret_key = Secp256r1PrivateKey::from_bytes(private_key)?;
		Ok(Self::from_secret_key(&secret_key))
	}

	/// Creates an `KeyPair` from a given 65-byte public key.
	/// This will use a dummy private key internally.
	///
	/// # Arguments
	///
	/// * `public_key` - A 65-byte slice representing the uncompressed public key.
	pub fn from_public_key(public_key: &[u8; 64]) -> Result<Self, CryptoError> {
		let public_key = Secp256r1PublicKey::from_slice(public_key)?;
		let secret_key = Secp256r1PrivateKey::from_bytes(&[0u8; 32]).unwrap(); // dummy private key
		Ok(Self::new(secret_key, public_key))
	}

	/// Exports the key pair as a Wallet Import Format (WIF) string
	///
	/// Returns: The WIF encoding of this key pair
	pub fn export_as_wif(&self) -> String {
		wif_from_private_key(&self.private_key())
	}
}

#[cfg(test)]
mod tests {
	use rustc_serialize::hex::FromHex;

	use neo::prelude::KeyPair;

	#[test]
	fn test_public_key_wif() {
		let private_key = "c7134d6fd8e73d819e82755c64c93788d8db0961929e025a53363c4cc02a6962"
			.from_hex()
			.unwrap();
		let private_key_arr: &[u8; 32] = private_key.as_slice().try_into().unwrap();
		let key_pair = KeyPair::from_private_key(private_key_arr).unwrap();
		assert_eq!(
			key_pair.export_as_wif(),
			"L3tgppXLgdaeqSGSFw1Go3skBiy8vQAM7YMXvTHsKQtE16PBncSU"
		);
	}
}
