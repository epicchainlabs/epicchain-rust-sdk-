//! # Secp256r1 Cryptographic Module
//!
//! This module provides cryptographic functionalities for the secp256r1 elliptic curve.
//! It includes implementations for handling public keys, private keys, and signatures,
//! as well as utilities for signing and verifying data.
//!
//! ## Features
//!
//! - Generation of public and private keys.
//! - Conversion between different formats and representations of keys and signatures.
//! - Signing data with a private key and verifying signatures with a public key.
//! - Integration with external libraries like `neo-codec`, `p256`, and `rand_core` for cryptographic operations.
//!
//! ## Usage
//!
//! - `Secp256r1PublicKey`: Represents a public key on the secp256r1 curve. It can be created
//!   from raw coordinates, existing `PublicKey` instances, or from byte slices.
//!   It provides functionalities to verify signatures and to encode the key in various formats.
//!
//! - `Secp256r1PrivateKey`: Represents a private key on the secp256r1 curve. It can be randomly
//!   generated or created from a byte slice. It provides methods to sign data and to retrieve
//!   the associated public key.
//!
//! - `Secp256r1Signature`: Represents a digital signature generated using a secp256r1 private key.
//!   It can be created from scalar values, `U256` representations, or from raw bytes. It offers
//!   a method to convert the signature back into a byte array.
//!
//! ## Examples
//!
//! Basic usage involves creating a private key, generating a signature for a message, and then
//! using the corresponding public key to verify the signature. Public and private keys can be
//! converted to and from various formats for storage or transmission.
//!
//! ```
//! use rand_core::OsRng;
//! use neo_rs::prelude::Secp256r1PrivateKey;
//!
//! // Generate a new private key
//! let private_key = Secp256r1PrivateKey::random(&mut OsRng);
//!
//! // Sign a message
//! let message = b"Example message";
//! let signature = private_key.sign_tx(message).expect("Failed to sign message");
//!
//! // Obtain the public key
//! let public_key = private_key.to_public_key();
//!
//! // Verify the signature
//! assert!(public_key.verify(message, &signature).is_ok());
//! ```
//!
//! Note: Error handling is crucial for cryptographic operations. Ensure proper error handling
//! in real-world applications.

use core::fmt;
use std::{
	cmp::Ordering,
	hash::{Hash, Hasher},
};

use p256::{
	ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
	elliptic_curve::{
		sec1::{FromEncodedPoint, ToEncodedPoint},
		Field,
	},
	EncodedPoint, FieldBytes, PublicKey, SecretKey,
};
use primitive_types::U256;
use rand_core::OsRng;
use rustc_serialize::hex::{FromHex, ToHex};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use signature::{SignerMut, Verifier};

use neo::prelude::{CryptoError, Decoder, Encoder, NeoConstants, NeoSerializable};

#[cfg_attr(feature = "substrate", serde(crate = "serde_substrate"))]
#[derive(Debug, Clone)]
pub struct Secp256r1PublicKey {
	inner: PublicKey,
}

#[derive(Debug, Clone)]
pub struct Secp256r1PrivateKey {
	inner: SecretKey,
}

#[derive(Debug, Clone)]
pub struct Secp256r1Signature {
	inner: Signature,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Secp256r1SignedMsg<T: Serialize> {
	pub msg: T,
	pub signature: Secp256r1Signature,
}

impl Secp256r1PublicKey {
	/// Constructs a new `Secp256r1PublicKey` from the given x and y coordinates.
	///
	/// This function attempts to create a public key from uncompressed x and y coordinates.
	/// It returns `None` if the provided coordinates do not correspond to a valid point on the curve.
	///
	/// - Parameters:
	///     - gx: The x coordinate of the public key.
	///     - gy: The y coordinate of the public key.
	///
	/// - Returns: An `Option<Secp256r1PublicKey>`.
	pub fn new(gx: [u8; 32], gy: [u8; 32]) -> Option<Self> {
		let mut uncompressed_point = Vec::with_capacity(65);
		uncompressed_point.push(0x04);
		uncompressed_point.extend_from_slice(&gx);
		uncompressed_point.extend_from_slice(&gy);

		let encoded_point = EncodedPoint::from_bytes(&uncompressed_point).ok()?;
		let public_key = PublicKey::from_encoded_point(&encoded_point).unwrap();

		Some(Secp256r1PublicKey { inner: public_key })
	}

	/// Constructs a `Secp256r1PublicKey` from an existing `PublicKey`.
	///
	/// This method can be used to convert a `PublicKey` from the `p256` crate into a `Secp256r1PublicKey`.
	///
	/// - Parameter public_key: A `PublicKey` instance.
	///
	/// - Returns: A `Secp256r1PublicKey` instance.
	pub fn from_public_key(public_key: PublicKey) -> Self {
		Secp256r1PublicKey { inner: public_key }
	}

	/// Constructs a `Secp256r1PublicKey` from a byte slice.
	///
	/// Attempts to parse a byte slice as an encoded elliptic curve point and create a public key.
	/// Returns a `CryptoError` if the byte slice does not represent a valid public key.
	///
	/// - Parameter bytes: A byte slice representing an encoded elliptic curve point.
	///
	/// - Returns: A `Result<Secp256r1PublicKey, CryptoError>`.
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
		let encoded_point = match EncodedPoint::from_bytes(bytes) {
			Ok(v) => v,
			Err(_) => return Err(CryptoError::InvalidPublicKey),
		};
		let public_key = if PublicKey::from_encoded_point(&encoded_point).is_some().into() {
			PublicKey::from_encoded_point(&encoded_point).unwrap()
		} else {
			return Err(CryptoError::InvalidPublicKey)
		};
		Ok(Secp256r1PublicKey { inner: public_key })
	}

	/// Verifies a digital signature against a message using this public key.
	///
	/// This method checks if the provided signature is valid for the given message under this public key.
	/// Returns a `CryptoError` if the signature verification fails.
	///
	/// - Parameters:
	///     - message: The message that was signed.
	///     - signature: The signature to verify.
	///
	/// - Returns: A `Result<(), CryptoError>`.
	pub fn verify(
		&self,
		message: &[u8],
		signature: &Secp256r1Signature,
	) -> Result<(), CryptoError> {
		let verifying_key = VerifyingKey::from(&self.inner);

		verifying_key
			.verify(message, &signature.inner)
			.map_err(|_| CryptoError::SignatureVerificationError)
	}

	/// Gets this public key's elliptic curve point encoded as defined in section 2.3.3 of [SEC1](http://www.secg.org/sec1-v2.pdf).
	///
	/// - Parameter compressed: If the EC point should be encoded in compressed or uncompressed format
	///
	/// - Returns: The encoded public key
	pub fn get_encoded(&self, compressed: bool) -> Vec<u8> {
		self.inner.to_encoded_point(compressed).as_bytes().to_vec()
	}

	/// Gets this public key's elliptic curve point encoded as defined in section 2.3.3 of [SEC1](http://www.secg.org/sec1-v2.pdf)
	/// in compressed format as hexadecimal.
	///
	/// - Returns: The encoded public key in compressed format as hexadecimal without a prefix
	pub fn get_encoded_compressed_hex(&self) -> String {
		let encoded = self.get_encoded(true);
		encoded.to_hex()
	}

	/// Constructs a `Secp256r1PublicKey` from a hexadecimal string representation.
	///
	/// This method attempts to parse a hexadecimal string as an encoded elliptic curve point.
	/// Returns `None` if the string is not a valid encoding or does not represent a valid public key.
	///
	/// - Parameter encoded: A hexadecimal string representing an encoded elliptic curve point.
	///
	/// - Returns: An `Option<Secp256r1PublicKey>`.
	pub fn from_encoded(encoded: &str) -> Option<Self> {
		let encoded = &encoded.replace("0x", "");
		let encoded = match encoded.from_hex() {
			Ok(v) => v,
			Err(_) => return None,
		};

		match Secp256r1PublicKey::from_bytes(encoded.as_slice()) {
			Ok(v) => Some(v),
			Err(_) => None,
		}
	}
}

impl Secp256r1PrivateKey {
	/// Generates a new private key using the provided random number generator (RNG).
	///
	/// - Parameter rng: A mutable reference to an `OsRng` instance.
	///
	/// - Returns: A new instance of the private key.
	pub fn random(rng: &mut OsRng) -> Self {
		Self { inner: SecretKey::random(rng) }
	}

	/// Creates a private key from a byte slice.
	///
	/// This method attempts to construct a private key from a given byte array.
	/// Returns an error if the byte slice does not represent a valid private key.
	///
	/// - Parameter bytes: A byte slice representing the private key.
	///
	/// - Returns: A `Result` with the private key or a `CryptoError`
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
		if bytes.len() != 32 {
			return Err(CryptoError::InvalidPrivateKey)
		}
		SecretKey::from_slice(bytes)
			.map(|inner| Self { inner })
			.map_err(|_| CryptoError::InvalidPrivateKey)
	}

	/// Returns the raw byte representation of the private key.
	///
	/// - Returns: A 32-byte array representing the private key.
	pub fn to_raw_bytes(&self) -> [u8; 32] {
		self.inner.clone().to_bytes().as_slice().try_into().unwrap()
	}

	/// Converts the private key to its corresponding public key.
	///
	/// - Returns: The corresponding `Secp256r1PublicKey`.
	pub fn to_public_key(&self) -> Secp256r1PublicKey {
		Secp256r1PublicKey::from_public_key(self.inner.public_key())
	}

	/// Signs a transaction with the private key.
	///
	/// This method signs the provided message (transaction) using the private key
	/// and returns the signature.
	///
	/// - Parameter message: A byte slice representing the message to be signed.
	///
	/// - Returns: A `Result` with the `Secp256r1Signature` or a `CryptoError`.
	pub fn sign_tx(&self, message: &[u8]) -> Result<Secp256r1Signature, CryptoError> {
		let signing_key = SigningKey::from_slice(&self.inner.to_bytes().as_slice())
			.map_err(|_| CryptoError::InvalidPrivateKey)?;
		let (signature, _) =
			signing_key.try_sign(message).map_err(|_| CryptoError::SigningError)?;

		Ok(Secp256r1Signature { inner: signature })
	}
}

impl Secp256r1Signature {
	/// Creates a signature from the scalar values of `r` and `s`.
	///
	/// This method constructs a signature from the provided `r` and `s` values,
	/// which are expected to be 32-byte arrays each.
	///
	/// - Parameters:
	///     - r: The r scalar value as a 32-byte array.
	///     - s: The s scalar value as a 32-byte array.
	///
	/// - Returns: An `Option<Secp256r1Signature>`. Returns `None` if the values
	///   do not form a valid signature.
	pub fn from_scalars(r: [u8; 32], s: [u8; 32]) -> Option<Self> {
		let r_arr: FieldBytes = r.into();
		let s_arr: FieldBytes = s.into();

		Signature::from_scalars(r_arr, s_arr)
			.ok()
			.map(|inner| Secp256r1Signature { inner })
	}

	/// Creates a signature from `U256` representations of `r` and `s`.
	///
	/// Converts the `U256` values of `r` and `s` into byte arrays and attempts
	/// to create a signature. Assumes `r` and `s` are big-endian.
	///
	/// - Parameters:
	///     - r: The r value as a `U256`.
	///     - s: The s value as a `U256`.
	///
	/// - Returns: A `Secp256r1Signature`.
	pub fn from_u256(r: U256, s: U256) -> Self {
		let mut x = [0u8; 32];
		let mut y = [0u8; 32];
		r.to_big_endian(&mut x);
		s.to_big_endian(&mut y);
		Secp256r1Signature::from_scalars(x, y).unwrap()
	}

	/// Constructs a `Secp256r1Signature` from a byte slice.
	///
	/// This method attempts to parse a 64-byte slice as an ECDSA signature.
	/// The first 32 bytes represent `r` and the following 32 bytes represent `s`.
	/// Returns an error if the slice is not 64 bytes long or does not represent
	/// a valid signature.
	///
	/// - Parameter bytes: A 64-byte slice representing the signature.
	///
	/// - Returns: A `Result<Secp256r1Signature, CryptoError>`.
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
		if bytes.len() != 64 {
			return Err(CryptoError::InvalidFormat("Invalid signature length".to_string()))
		}
		Ok(Secp256r1Signature { inner: Signature::from_slice(bytes).unwrap() })
	}

	/// Converts the signature into a 64-byte array.
	///
	/// This method returns a byte array representation of the signature,
	/// with the first 32 bytes representing `r` and the last 32 bytes representing `s`.
	///
	/// - Returns: A 64-byte array representing the signature.
	pub fn to_bytes(&self) -> [u8; 64] {
		let r_bytes: FieldBytes = self.inner.r().into();
		let s_bytes: FieldBytes = self.inner.s().into();

		let mut bytes = [0u8; 64];
		bytes[..32].copy_from_slice(r_bytes.as_ref());
		bytes[32..].copy_from_slice(s_bytes.as_ref());

		bytes
	}
}

impl fmt::Display for Secp256r1PrivateKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Secp256r1PrivateKey: {}\n", hex::encode(self.inner.to_bytes().as_slice()))
	}
}

impl fmt::Display for Secp256r1PublicKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Secp256r1PublicKey: {}\n",
			hex::encode(self.inner.to_encoded_point(false).as_bytes())
		)
	}
}

impl fmt::Display for Secp256r1Signature {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Secp256r1Signature\n").unwrap();
		write!(f, "x: {}\n", hex::encode(&self.to_bytes()))
	}
}

impl Serialize for Secp256r1PublicKey {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_bytes(&self.get_encoded(true))
	}
}

impl Serialize for Secp256r1PrivateKey {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_bytes(&self.to_raw_bytes())
	}
}

impl Serialize for Secp256r1Signature {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_bytes(&self.to_bytes())
	}
}

impl<'de> Deserialize<'de> for Secp256r1PublicKey {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let bytes = <Vec<u8>>::deserialize(deserializer)?;
		Secp256r1PublicKey::from_bytes(&bytes)
			.map_err(|_| serde::de::Error::custom("Invalid public key"))
	}
}

impl<'de> Deserialize<'de> for Secp256r1PrivateKey {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let bytes = <Vec<u8>>::deserialize(deserializer)?;
		Secp256r1PrivateKey::from_bytes(&bytes)
			.map_err(|_| serde::de::Error::custom("Invalid private key"))
	}
}

impl<'de> Deserialize<'de> for Secp256r1Signature {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let bytes = <Vec<u8>>::deserialize(deserializer)?;
		Secp256r1Signature::from_bytes(&bytes)
			.map_err(|_| serde::de::Error::custom("Invalid signature"))
	}
}

impl PartialEq for Secp256r1PublicKey {
	fn eq(&self, other: &Secp256r1PublicKey) -> bool {
		self.get_encoded(true) == other.get_encoded(true)
	}
}

impl PartialOrd for Secp256r1PublicKey {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let self_bytes = self.get_encoded(true);
		let other_bytes = other.get_encoded(true);
		self_bytes.partial_cmp(&other_bytes)
	}
}

impl Eq for Secp256r1PublicKey {}

impl Ord for Secp256r1PublicKey {
	fn cmp(&self, other: &Self) -> Ordering {
		let self_bytes = self.get_encoded(true);
		let other_bytes = other.get_encoded(true);
		self_bytes.cmp(&other_bytes)
	}
}

impl Hash for Secp256r1PublicKey {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.get_encoded(false).hash(state);
	}
}

impl Hash for Secp256r1PrivateKey {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.to_raw_bytes().hash(state);
	}
}

impl Hash for Secp256r1Signature {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.to_bytes().hash(state);
	}
}

impl PartialEq for Secp256r1PrivateKey {
	fn eq(&self, other: &Self) -> bool {
		self.to_raw_bytes() == other.to_raw_bytes()
	}
}

impl PartialEq for Secp256r1Signature {
	fn eq(&self, other: &Self) -> bool {
		self.to_bytes() == other.to_bytes()
	}
}

impl From<Vec<u8>> for Secp256r1PublicKey {
	fn from(bytes: Vec<u8>) -> Self {
		match Secp256r1PublicKey::from_bytes(&bytes) {
			Ok(v) => v,
			Err(_) => panic!("Invalid public key"),
		}
	}
}

pub trait PrivateKeyExtension
where
	Self: Sized,
{
	fn to_vec(&self) -> Vec<u8>;

	fn from_slice(slice: &[u8]) -> Result<Self, CryptoError>;
}

impl PrivateKeyExtension for Secp256r1PrivateKey {
	fn to_vec(&self) -> Vec<u8> {
		self.to_raw_bytes().to_vec()
	}

	fn from_slice(slice: &[u8]) -> Result<Self, CryptoError> {
		if slice.len() != 32 {
			return Err(CryptoError::InvalidPublicKey)
		}

		let mut arr = [0u8; 32];
		arr.copy_from_slice(slice);
		Ok(Self::from_bytes(&arr).map_err(|_| CryptoError::InvalidPublicKey).unwrap())
	}
}

pub trait PublicKeyExtension
where
	Self: Sized,
{
	fn to_vec(&self) -> Vec<u8>;
	fn from_slice(slice: &[u8]) -> Result<Self, CryptoError>;
}

impl PublicKeyExtension for Secp256r1PublicKey {
	fn to_vec(&self) -> Vec<u8> {
		self.get_encoded(true)
	}

	fn from_slice(slice: &[u8]) -> Result<Self, CryptoError> {
		if slice.len() != 64 && slice.len() != 33 {
			return Err(CryptoError::InvalidPublicKey)
		}
		Self::from_slice(slice).map_err(|_| CryptoError::InvalidPublicKey)
	}
}

impl NeoSerializable for Secp256r1PublicKey {
	type Error = CryptoError;

	fn size(&self) -> usize {
		64
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_bytes(&self.to_vec());
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		let bytes = reader.read_bytes(NeoConstants::PUBLIC_KEY_SIZE_COMPRESSED as usize).unwrap();
		Secp256r1PublicKey::from_bytes(&bytes).map_err(|_| CryptoError::InvalidPublicKey)
	}

	fn to_array(&self) -> Vec<u8> {
		self.get_encoded(false)
	}
}

#[cfg(test)]
mod tests {
	use rustc_serialize::hex::{FromHex, ToHex};

	use neo::prelude::{
		HashableForVec, Secp256r1PrivateKey, Secp256r1PublicKey, Secp256r1Signature, ToArray32,
	};

	const ENCODED_POINT: &str =
		"03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816";

	#[test]
	fn test_new_public_key_from_point() {
		let mut public_key = Secp256r1PublicKey::from_encoded(ENCODED_POINT).unwrap();

		assert_eq!(public_key.get_encoded(true).as_slice(), ENCODED_POINT.from_hex().unwrap());
		assert_eq!(public_key.get_encoded_compressed_hex(), ENCODED_POINT);
	}

	#[test]
	fn test_new_public_key_from_uncompressed_point() {
		let uncompressed = "04b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e1368165f4f7fb1c5862465543c06dd5a2aa414f6583f92a5cc3e1d4259df79bf6839c9";
		assert_eq!(
			Secp256r1PublicKey::from_encoded(uncompressed)
				.unwrap()
				.get_encoded_compressed_hex(),
			ENCODED_POINT
		);
	}

	#[test]
	fn test_new_public_key_from_string_with_invalid_size() {
		let too_small = &ENCODED_POINT[0..ENCODED_POINT.len() - 2];
		assert!(Secp256r1PublicKey::from_encoded(too_small).is_none());
	}

	#[test]
	fn test_new_public_key_from_point_with_hex_prefix() {
		let prefixed = format!("0x{}", ENCODED_POINT);
		let a = Secp256r1PublicKey::from_encoded(&prefixed);
		assert!(a.is_some());
	}

	#[test]
	fn test_serialize_public_key() {
		let mut public_key = Secp256r1PublicKey::from_encoded(ENCODED_POINT).unwrap();
		assert_eq!(public_key.get_encoded(true), ENCODED_POINT.from_hex().unwrap());
	}

	#[test]
	fn test_deserialize_public_key() {
		let data = "036b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296";
		let mut public_key = Secp256r1PublicKey::from_encoded(&data).unwrap();
		assert_eq!(public_key.get_encoded(true).to_hex(), data);
	}

	#[test]
	fn test_public_key_size() {
		let mut key = Secp256r1PublicKey::from_encoded(
			"036b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
		)
		.unwrap();
		assert_eq!(key.get_encoded(true).len(), 33);
	}

	#[test]
	fn test_public_key_comparable() {
		let encoded_key2 = "036b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296";
		let encoded_key1_uncompressed = "04b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e1368165f4f7fb1c5862465543c06dd5a2aa414f6583f92a5cc3e1d4259df79bf6839c9";

		let key1 = Secp256r1PublicKey::from_encoded(ENCODED_POINT).unwrap();
		let key2 = Secp256r1PublicKey::from_encoded(encoded_key2).unwrap();
		let key1_uncompressed =
			Secp256r1PublicKey::from_encoded(encoded_key1_uncompressed).unwrap();

		assert!(key1 > key2);
		assert!(key1 == key1_uncompressed);
		assert!(!(key1 < key1_uncompressed));
		assert!(!(key1 > key1_uncompressed));
	}

	#[test]
	fn test_sign_message() {
		let private_key_hex = "9117f4bf9be717c9a90994326897f4243503accd06712162267e77f18b49c3a3";
		let public_key_hex = "0265bf906bf385fbf3f777832e55a87991bcfbe19b097fb7c5ca2e4025a4d5e5d6";
		let test_message = "A test message";
		let expected_r = "147e5f3c929dd830d961626551dbea6b70e4b2837ed2fe9089eed2072ab3a655";
		let expected_s = "523ae0fa8711eee4769f1913b180b9b3410bbb2cf770f529c85f6886f22cbaaf";

		let private_key =
			Secp256r1PrivateKey::from_bytes(&hex::decode(private_key_hex).unwrap()).unwrap();
		let public_key =
			Secp256r1PublicKey::from_bytes(&hex::decode(public_key_hex).unwrap()).unwrap();

		assert_eq!(public_key.clone(), private_key.clone().to_public_key());

		// Hash the message
		let hashed_msg = test_message.as_bytes().hash256();

		let signature: Secp256r1Signature = private_key.clone().sign_tx(&hashed_msg).unwrap();

		let expected_signature = Secp256r1Signature::from_scalars(
			expected_r.from_hex().unwrap().to_array32().unwrap(),
			expected_s.from_hex().unwrap().to_array32().unwrap(),
		)
		.unwrap_or_else(|| panic!("Failed to create signature from scalars"));
		assert!(public_key.verify(&hashed_msg, &signature).is_ok());
		assert!(public_key.verify(&hashed_msg, &signature).is_ok());

		// Verification
		assert!(public_key.verify(&hashed_msg, &signature).is_ok());
		// TODO: check this verification
		// assert!(public_key.verify(&hashed_msg, &expected_signature).is_ok());
	}
}
