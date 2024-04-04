use rustc_serialize::hex::ToHex;

use neo::prelude::{
	CryptoError, PrivateKeyExtension, PublicKeyExtension, Secp256r1PrivateKey, Secp256r1PublicKey,
};

/// Convert a private key to a public key.
pub fn private_key_to_public_key(private_key: &Secp256r1PrivateKey) -> Secp256r1PublicKey {
	private_key.to_public_key()
}

/// Convert a private key to hex format.
///
/// Returns the private key as a hex encoded string.
pub fn private_key_to_hex(private_key: &Secp256r1PrivateKey) -> String {
	private_key.to_raw_bytes().to_vec().to_hex()
}

/// Convert a private key in hex format to a Secp256r1PrivateKey.
///
/// # Errors
///
/// Will return an error if the hex decoding fails
pub fn private_key_from_hex(hex: &str) -> Result<Secp256r1PrivateKey, CryptoError> {
	let bytes = hex::decode(hex)?;
	let secret_key = Secp256r1PrivateKey::from_slice(&bytes)?;
	Ok(secret_key)
}

/// Convert a public key to hex format.
///
/// Returns the public key as a hex encoded string.
pub fn public_key_to_hex(public_key: &Secp256r1PublicKey) -> String {
	public_key.to_vec().to_hex()
}

/// Convert a public key in hex format to a Secp256r1PublicKey.
///
/// # Errors
///
/// Will return an error if hex decoding fails
pub fn public_key_from_hex(hex: &str) -> Result<Secp256r1PublicKey, CryptoError> {
	let bytes = hex::decode(hex)?;
	let public_key = Secp256r1PublicKey::from_slice(&bytes)?;
	Ok(public_key)
}

pub trait ToArray32 {
	fn to_array32(&self) -> Result<[u8; 32], CryptoError>;
}

macro_rules! impl_to_array32 {
	($type:ty) => {
		impl ToArray32 for $type {
			fn to_array32(&self) -> Result<[u8; 32], CryptoError> {
				if self.len() != 32 {
					return Err(CryptoError::InvalidFormat(
						"Vector does not contain exactly 32 elements".to_string(),
					))
				}

				let mut array = [0u8; 32];
				let bytes = &self[..array.len()]; // Take a slice of the vec
				array.copy_from_slice(bytes); // Copy the slice into the array
				Ok(array)
			}
		}
	};
}

impl_to_array32!(Vec<u8>);
impl_to_array32!(&[u8]);
