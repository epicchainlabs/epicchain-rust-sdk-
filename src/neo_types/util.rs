// #![allow(unused_imports)]
// #![allow(dead_code)]
// This module provides utility functions for parsing strings into numeric types, encoding numeric types to strings,
// converting between different data representations (e.g., bytes to strings, H256 to U256), and implementing traits
// for encoding data into Base58 and Base64 formats. These utilities are particularly useful in blockchain development,
// where such conversions and encodings are frequently required.

use primitive_types::{H160, H256, U256};

use neo::prelude::TypeError;

use crate::prelude::ScriptHash;

/// Parses a string into a `u64`, supporting both decimal and hexadecimal (prefixed with "0x") formats.
///
/// # Examples
///
/// ```
/// use neo_rs::prelude::parse_string_u64;
/// let decimal = "12345";
/// assert_eq!(parse_string_u64(decimal), 12345);
///
/// let hex = "0x3039";
/// assert_eq!(parse_string_u64(hex), 12345);
/// ```
pub fn parse_string_u64(u64_str: &str) -> u64 {
	if u64_str.starts_with("0x") {
		u64::from_str_radix(u64_str, 16).unwrap()
	} else {
		u64::from_str_radix(u64_str, 10).unwrap()
	}
}

/// Parses a string into a `U256`, accepting both decimal and hex (prefixed with "0x") formats.
///
/// # Examples
///
/// ```
/// use primitive_types::U256;
/// use neo_rs::prelude::parse_string_u256;
/// let decimal = "123456789";
/// assert_eq!(parse_string_u256(decimal), U256::from(123456789));
///
/// let hex = "0x75bcd15";
/// assert_eq!(parse_string_u256(hex), U256::from(123456789));
/// ```
pub fn parse_string_u256(u256_str: &str) -> U256 {
	if u256_str.starts_with("0x") {
		U256::from_str_radix(u256_str, 16).unwrap()
	} else {
		U256::from_str_radix(u256_str, 10).unwrap()
	}
}

/// Converts a hexadecimal string representation of an address into a `ScriptHash`.
///
/// # Examples
///
/// ```
/// use neo_rs::prelude::{parse_address, ScriptHash};
/// let address_hex = "0xabcdef1234567890";
/// let script_hash = parse_address(address_hex);
/// assert_eq!(script_hash, ScriptHash::from_slice(&[0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90]));
/// ```
pub fn parse_address(address: &str) -> ScriptHash {
	let bytes = hex::decode(address.trim_start_matches("0x")).unwrap();
	let mut padded_bytes = [0_u8; 20];
	padded_bytes[20 - bytes.len()..].copy_from_slice(&bytes);
	ScriptHash::from_slice(&padded_bytes)
}

/// Encodes an `H160` hash into a string representation.
///
/// # Examples
///
/// ```
/// use primitive_types::H160;
/// use neo_rs::prelude::encode_string_h160;
/// let hash = H160::repeat_byte(0xab);
/// let encoded = encode_string_h160(&hash);
/// assert!(encoded.starts_with("H160"));
/// ```
pub fn encode_string_h160(h160: &H160) -> String {
	format!("{:?}", h160).to_owned()
}

/// Parses a hexadecimal string into an `H256` hash, padding with zeros if necessary.
///
/// # Examples
///
/// ```
/// use primitive_types::H256;
/// use neo_rs::prelude::parse_string_h256;
/// let hex_str = "0x123456";
/// let h256 = parse_string_h256(hex_str);
/// assert_eq!(h256, H256::from_low_u64_be(0x123456));
/// ```
pub fn parse_string_h256(h256_str: &str) -> H256 {
	let bytes = hex::decode(h256_str.trim_start_matches("0x")).unwrap();
	// pad the bytes to 32bytes
	let mut padded_bytes = [0_u8; 32];
	padded_bytes[32 - bytes.len()..].copy_from_slice(&bytes);

	H256::from_slice(&padded_bytes)
}

/// Encodes an `H256` hash into a string representation.
///
/// # Examples
///
/// ```
/// use primitive_types::H256;
/// use neo_rs::prelude::encode_string_h256;
/// let hash = H256::repeat_byte(0xab);
/// let encoded = encode_string_h256(&hash);
/// assert!(encoded.starts_with("H256"));
/// ```
pub fn encode_string_h256(h256: &H256) -> String {
	format!("{:?}", h256).to_owned()
}

/// Encodes a `U256` value into a hexadecimal string prefixed with "0x".
///
/// # Examples
///
/// ```
/// use primitive_types::U256;
/// use neo_rs::prelude::encode_string_u256;
/// let value = U256::from(255);
/// let encoded = encode_string_u256(&value);
/// assert_eq!(encoded, "0xff");
/// ```
pub fn encode_string_u256(u256: &U256) -> String {
	format!("0x{:x}", u256).to_owned()
}

/// Encodes a vector of `U256` values into a vector of hexadecimal strings.
///
/// # Examples
///
/// ```
/// use primitive_types::U256;
/// use neo_rs::prelude::encode_vec_string_vec_u256;
/// let values = vec![U256::from(1), U256::from(2)];
/// let encoded_values = encode_vec_string_vec_u256(values);
/// assert_eq!(encoded_values, vec!["0x1", "0x2"]);
/// ```
pub fn encode_vec_string_vec_u256(item: Vec<U256>) -> Vec<String> {
	item.iter().map(|x| encode_string_u256(&x)).collect()
}

/// Parses a vector of hexadecimal string representations into a vector of `U256` values.
///
/// # Examples
///
/// ```
/// use primitive_types::U256;
/// use neo_rs::prelude::parse_vec_string_vec_u256;
/// let strings = vec!["0x1".to_string(), "0x2".to_string()];
/// let u256_values = parse_vec_string_vec_u256(strings);
/// assert_eq!(u256_values, vec![U256::from(1), U256::from(2)]);
/// ```
pub fn parse_vec_string_vec_u256(item: Vec<String>) -> Vec<U256> {
	item.iter().map(|x| parse_string_u256(&x)).collect()
}

/// Converts an `H256` hash into a `U256` value.
///
/// # Examples
///
/// ```
/// use primitive_types::{H256, U256};
/// use neo_rs::prelude::h256_to_u256;
/// let h256 = H256::repeat_byte(0x01);
/// let u256 = h256_to_u256(h256);
/// assert_eq!(u256, U256::from_big_endian(&[0x01; 32]));
/// ```
pub fn h256_to_u256(item: H256) -> U256 {
	U256::from_big_endian(item.as_bytes())
}

/// Converts a byte slice into a hexadecimal string prefixed with "0x".
///
/// # Examples
///
/// ```
/// use neo_rs::prelude::bytes_to_string;
/// let bytes = [0xde, 0xad, 0xbe, 0xef];
/// let hex_string = bytes_to_string(&bytes);
/// assert_eq!(hex_string, "0xdeadbeef");
/// ```
pub fn bytes_to_string(mybytes: &[u8]) -> String {
	format!("0x{}", hex::encode(mybytes))
}

/// Attempts to convert a hexadecimal string (optionally prefixed with "0x") into a byte vector.
///
/// # Examples
///
/// ```
/// use neo_rs::prelude::string_to_bytes;
/// let hex_string = "0xdeadbeef";
/// let bytes = string_to_bytes(hex_string).unwrap();
/// assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);
///
/// let invalid_hex = "deadbeefg";
/// assert!(string_to_bytes(invalid_hex).is_none());
/// ```
pub fn string_to_bytes(mystring: &str) -> Option<Vec<u8>> {
	if mystring.starts_with("0x") {
		let mystring = mystring.trim_start_matches("0x");
		let mybytes = match hex::decode(mystring) {
			Ok(mybytes) => Some(mybytes),
			Err(_) => None,
		};
		mybytes
	} else {
		None
	}
}

/// Calculates the square root of a `U256` value, returning another `U256`.
///
/// # Examples
///
/// ```
/// use primitive_types::U256;
/// use neo_rs::prelude::u256_sqrt;
/// let input = U256::from(16);
/// let sqrt = u256_sqrt(&input);
/// assert_eq!(sqrt, U256::from(4));
/// ```
pub fn u256_sqrt(input: &U256) -> U256 {
	if *input < 2.into() {
		return input.clone()
	}
	let mut x: U256 = (input + U256::one()) >> 1;
	let mut y = input.clone();
	while x < y {
		y = x;
		x = (input / x + x) >> 1;
	}
	y
}

/// Returns the minimum of two `U256` values.
///
/// # Examples
///
/// ```
/// use primitive_types::U256;
/// use neo_rs::prelude::u256_min;
/// let a = U256::from(1);
/// let b = U256::from(2);
/// assert_eq!(u256_min(a, b), U256::from(1));
pub fn u256_min(x: U256, y: U256) -> U256 {
	if x > y {
		y
	} else {
		x
	}
}

/// Converts a vector of bytes into an array of 32 bytes. Returns an error if the vector is not exactly 32 bytes long.
///
/// # Examples
///
/// ```
/// use neo_rs::prelude::vec_to_array32;
/// let vec = vec![0_u8; 32];
/// let array = vec_to_array32(vec).unwrap();
/// assert_eq!(array.len(), 32);
/// ```
pub fn vec_to_array32(vec: Vec<u8>) -> Result<[u8; 32], TypeError> {
	if vec.len() != 32 {
		return Err(TypeError::InvalidData(
			"Vector does not contain exactly 32 elements".to_string(),
		))
	}

	let mut array = [0u8; 32];
	let bytes = &vec[..array.len()]; // Take a slice of the vec
	array.copy_from_slice(bytes); // Copy the slice into the array
	Ok(array)
}

/// Calculates the size of a variable as the number of bytes required to represent it.
///
/// # Examples
///
/// ```
/// use neo_rs::prelude::var_size;
/// assert_eq!(var_size(256), 2); // 256 requires at least 2 bytes.
/// assert_eq!(var_size(1), 1); // Smallest non-zero values require at least 1 byte.
/// ```
pub fn var_size(value: usize) -> usize {
	let mut v = value;
	let mut bytes = 0;
	while v > 0 {
		v >>= 8;
		bytes += 1;
	}
	if bytes == 0 {
		1
	} else {
		bytes
	}
}

pub trait ToBase58 {
	/// Encodes a byte slice into a Base58 string.
	///
	/// # Examples
	///
	/// ```
	/// use neo_rs::prelude::ToBase58;
	/// let bytes = [1, 2, 3];
	/// assert_eq!(bytes.to_base58(), "Ldp");
	/// ```
	fn to_base58(&self) -> String;
}

impl ToBase58 for [u8] {
	fn to_base58(&self) -> String {
		bs58::encode(self).into_string()
	}
}

pub trait ToBase64 {
	/// Encodes a byte slice into a Base64 string.
	///
	/// # Examples
	///
	/// ```
	/// use neo_rs::prelude::ToBase64;
	/// let bytes = [1, 2, 3];
	/// assert_eq!(bytes.to_base64(), "AQID");
	/// ```
	fn to_base64(&self) -> String;
}

impl ToBase64 for [u8] {
	fn to_base64(&self) -> String {
		base64::encode(self)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// #[test]
	// pub fn test_blake2var_hash() {
	//     let mut data = [0_u8; 24];
	//     data[0..4].copy_from_slice(b"evm:");
	//     data[4..24].copy_from_slice(&hex::decode("7EF99B0E5bEb8ae42DbF126B40b87410a440a32a").unwrap());
	//     let hash = blake2_hash(&data);
	//     let actual = hex::decode("65f5fbd10250447019bb8b9e06f6918d033b2feb6478470137b1a552656e2911").unwrap();
	//     assert_eq!(&hash, actual.as_slice());
	// }

	#[test]
	pub fn test_bytes_to_string() {
		let mybytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let bytestring = bytes_to_string(&mybytes);
		let orig_bytestring = "0x0102030405060708090a";
		assert_eq!(&bytestring, orig_bytestring);
		let error_bytestring = "0102030405060708090a";
		let error_mybytes = string_to_bytes(error_bytestring);
		assert_eq!(None, error_mybytes);
		let ok_mybytes = string_to_bytes(orig_bytestring).unwrap();
		assert_eq!(&mybytes[..], &ok_mybytes[..]);
	}
}
