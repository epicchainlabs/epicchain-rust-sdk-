use bs58;
use hex;
use sha2::{Digest, Sha256};

use neo::prelude::ScriptHash;

pub trait StringExt {
	fn bytes_from_hex(&self) -> Result<Vec<u8>, hex::FromHexError>;

	fn base64_decoded(&self) -> Result<Vec<u8>, base64::DecodeError>;

	fn base64_encoded(&self) -> String;

	fn base58_decoded(&self) -> Option<Vec<u8>>;

	fn base58_check_decoded(&self) -> Option<Vec<u8>>;

	fn base58_encoded(&self) -> String;

	fn var_size(&self) -> usize;

	fn is_valid_address(&self) -> bool;

	fn is_valid_hex(&self) -> bool;

	fn address_to_scripthash(&self) -> Result<ScriptHash, &'static str>;

	fn reversed_hex(&self) -> String;
}

impl StringExt for String {
	fn bytes_from_hex(&self) -> Result<Vec<u8>, hex::FromHexError> {
		hex::decode(self.trim_start_matches("0x"))
	}

	fn base64_decoded(&self) -> Result<Vec<u8>, base64::DecodeError> {
		base64::decode(self)
	}

	fn base64_encoded(&self) -> String {
		base64::encode(self.as_bytes())
	}

	fn base58_decoded(&self) -> Option<Vec<u8>> {
		bs58::decode(self).into_vec().ok()
	}

	fn base58_check_decoded(&self) -> Option<Vec<u8>> {
		bs58::decode(self).into_vec().ok()
	}

	fn base58_encoded(&self) -> String {
		bs58::encode(self.as_bytes()).into_string()
	}

	fn var_size(&self) -> usize {
		let bytes = self.as_bytes();
		let len = bytes.len();
		if len < 0xFD {
			1
		} else if len <= 0xFFFF {
			3
		} else if len <= 0xFFFFFFFF {
			5
		} else {
			9
		}
	}

	fn is_valid_address(&self) -> bool {
		if let Some(data) = self.base58_decoded() {
			if data.len() == 25 && data[0] == 0x17 {
				let checksum = &Sha256::digest(&Sha256::digest(&data[..21]))[..4];
				checksum == &data[21..]
			} else {
				false
			}
		} else {
			false
		}
	}

	fn is_valid_hex(&self) -> bool {
		self.len() % 2 == 0 && self.chars().all(|c| c.is_ascii_hexdigit())
	}

	fn address_to_scripthash(&self) -> Result<ScriptHash, &'static str> {
		if self.is_valid_address() {
			let data = self.base58_decoded().ok_or("Invalid address").unwrap();
			let mut scripthash = data[1..21].to_vec();
			scripthash.reverse();
			Ok(ScriptHash::from_slice(&scripthash))
		} else {
			Err("Not a valid address")
		}
	}

	fn reversed_hex(&self) -> String {
		let mut bytes = self.bytes_from_hex().unwrap();
		bytes.reverse();
		hex::encode(bytes)
	}
}
