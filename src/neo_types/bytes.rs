use std::ops::BitXor;

use bs58::encode as bs58_encode;
use derive_more::{AsRef, Deref, Index, IndexMut, IntoIterator};
use hex::encode as hex_encode;
use num_bigint::BigInt;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// `Bytes` is a wrapper around a vector of bytes (`Vec<u8>`) providing utility methods
/// for encoding, decoding, and other common operations on byte arrays.
#[derive(Debug, Serialize, Deserialize, AsRef, Deref, IntoIterator, Index, IndexMut)]
struct Bytes(Vec<u8>);

impl Bytes {
	fn b_int(&self) -> BigInt {
		let bytes = i128::from_be_bytes(self.0.as_slice().try_into().unwrap());
		BigInt::from_i128(bytes).unwrap()
	}

	fn base64_encoded(&self) -> String {
		base64::encode(&self.0)
	}

	fn base58_encoded(&self) -> String {
		bs58_encode(self.0.as_slice()).into_string()
	}

	fn base58_check_encoded(&self) -> String {
		let checksum = &Sha256::digest(&Sha256::digest(&self.0))[..4];
		let mut bytes = self.0.clone();
		bytes.extend_from_slice(checksum);
		bs58_encode(&bytes).into_string()
	}

	fn no_prefix_hex(&self) -> String {
		hex_encode(self.0.as_slice()).trim_start_matches("0x").to_string()
	}

	fn var_size(&self) -> usize {
		std::mem::size_of::<usize>() + self.0.len()
	}

	fn scripthash_to_address(&self) -> String {
		let mut script = vec![0x17];
		script.extend_from_slice(&self.0.iter().rev().copied().collect::<Vec<_>>());

		let mut hasher = Sha256::new();
		hasher.update(&script);
		let checksum = &hasher.finalize()[..4];

		let mut address = script;
		address.extend_from_slice(checksum);

		bs58_encode(&address).into_string()
	}

	fn to_padded(&self, length: usize, trailing: bool) -> Result<Bytes, &'static str> {
		let bytes_len = self.0.len();
		if bytes_len > length {
			return Err("Input is too large")
		}

		let mut padded = vec![0u8; length];
		let offset = if self.0.first() == Some(&0) { 1 } else { 0 };

		if trailing {
			padded[..bytes_len - offset].copy_from_slice(&self.0[offset..]);
		} else {
			padded[length - bytes_len + offset..].copy_from_slice(&self.0[offset..]);
		}

		Ok(Bytes(padded))
	}

	fn trim_trailing_bytes(&mut self, byte: u8) {
		while self.0.last() == Some(&byte) {
			self.0.pop();
		}
	}
}

impl BitXor for Bytes {
	type Output = Result<Self, &'static str>;

	fn bitxor(self, rhs: Self) -> Self::Output {
		if self.0.len() != rhs.0.len() {
			return Err("Arrays do not have the same length")
		}

		let bytes = self.0.iter().zip(rhs.0.iter()).map(|(x, y)| x ^ y).collect();

		Ok(Bytes(bytes))
	}
}

pub trait ReverseTrait {
	fn reverse(&self) -> Self;
}

impl ReverseTrait for Vec<u8> {
	fn reverse(&self) -> Self {
		self.iter().rev().copied().collect()
	}
}
