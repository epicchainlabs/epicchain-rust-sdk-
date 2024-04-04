use sha2::{Digest, Sha256};

/// Encodes a byte slice into a base58check string.
///
/// # Arguments
///
/// * `bytes` - A byte slice to be encoded.
///
/// # Example
///
/// ```
///
/// use neo_rs::prelude::base58check_encode;
/// let bytes = [0x01, 0x02, 0x03];
/// let encoded = base58check_encode(&bytes);
/// ```
pub fn base58check_encode(bytes: &[u8]) -> String {
	if bytes.len() == 0 {
		return "".to_string()
	}

	let checksum = &calculate_checksum(bytes)[..4];
	let bytes_with_checksum = [bytes, checksum].concat();
	bs58::encode(bytes_with_checksum).into_string()
}

/// Decodes a base58check string into a byte vector.
///
/// # Arguments
///
/// * `input` - A base58check string to be decoded.
///
/// # Example
///
/// ```
///
/// use neo_rs::prelude::base58check_decode;
/// let input = "Abc123";
/// let decoded = base58check_decode(input);
/// ```
pub fn base58check_decode(input: &str) -> Option<Vec<u8>> {
	let bytes_with_checksum = match bs58::decode(input).into_vec() {
		Ok(bytes) => bytes,
		Err(_) => return None,
	};

	if bytes_with_checksum.len() < 4 {
		return None
	}

	let (bytes, checksum) = bytes_with_checksum.split_at(bytes_with_checksum.len() - 4);
	let expected_checksum = calculate_checksum(bytes);

	if checksum != &expected_checksum[..4] {
		return None
	}

	Some(bytes.to_vec())
}

/// Calculates the checksum of a byte slice.
///
/// # Arguments
///
/// * `input` - A byte slice to calculate the checksum for.
///
/// # Example
///
/// ```
///
/// use neo_rs::prelude::calculate_checksum;
/// let bytes = [0x01, 0x02, 0x03];
/// let checksum = calculate_checksum(&bytes);
/// ```
pub fn calculate_checksum(input: &[u8]) -> [u8; 4] {
	let mut hasher = Sha256::new();
	hasher.update(input);
	let hash = hasher.finalize();
	let hash256 = Sha256::digest(&hash);
	hash256[..4].try_into().unwrap()
}

#[cfg(test)]
mod base58_tests {
	use super::*;

	// Define tuples of arbitrary strings that are mapped to valid Base58 encodings
	static VALID_STRING_DECODED_TO_ENCODED: &[(&str, &str)] = &[
		("", ""),
		(" ", "Z"),
		("-", "n"),
		("0", "q"),
		("1", "r"),
		("-1", "4SU"),
		("11", "4k8"),
		("abc", "ZiCa"),
		("1234598760", "3mJr7AoUXx2Wqd"),
		("abcdefghijklmnopqrstuvwxyz", "3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f"),
		(
			"00000000000000000000000000000000000000000000000000000000000000",
			"3sN2THZeE9Eh9eYrwkvZqNstbHGvrxSAM7gXUXvyFQP8XvQLUqNCS27icwUeDT7ckHm4FUHM2mTVh1vbLmk7y",
		),
	];

	// Define invalid strings
	static INVALID_STRINGS: &[&str] =
		&["0", "O", "I", "l", "3mJr0", "O3yxU", "3sNI", "4kl8", "0OIl", "!@#$%^&*()-_=+~`"];

	#[test]
	fn test_base58_encoding_for_valid_strings() {
		for (decoded, encoded) in VALID_STRING_DECODED_TO_ENCODED {
			let bytes = decoded.as_bytes();
			let result = bs58::encode(bytes).into_string();
			assert_eq!(&result, *encoded);
		}
	}

	#[test]
	fn test_base58_decoding_for_valid_strings() {
		for (decoded, encoded) in VALID_STRING_DECODED_TO_ENCODED {
			let result = bs58::decode(encoded).into_vec().unwrap();
			assert_eq!(result, Vec::from(*decoded));
		}
	}

	#[test]
	fn test_base58_decoding_for_invalid_strings() {
		for invalid_string in INVALID_STRINGS {
			let result = base58check_decode(invalid_string);
			assert!(result.is_none());
		}
	}

	#[test]
	fn test_base58check_encoding() {
		let input_data: Vec<u8> = vec![
			6, 161, 159, 136, 34, 110, 33, 238, 14, 79, 14, 218, 133, 13, 109, 40, 194, 236, 153,
			44, 61, 157, 254,
		];
		let expected_output = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtRw";
		let actual_output = base58check_encode(&input_data);
		assert_eq!(actual_output, expected_output);
	}

	#[test]
	fn test_base58check_decoding() {
		let input_string = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtRw";
		let expected_output_data: Vec<u8> = vec![
			6, 161, 159, 136, 34, 110, 33, 238, 14, 79, 14, 218, 133, 13, 109, 40, 194, 236, 153,
			44, 61, 157, 254,
		];
		let actual_output = base58check_decode(input_string);
		assert_eq!(actual_output, Some(expected_output_data));
	}

	#[test]
	fn test_base58check_decoding_with_invalid_characters() {
		assert!(base58check_decode("0oO1lL").is_none());
	}

	#[test]
	fn test_base58check_decoding_with_invalid_checksum() {
		assert!(base58check_decode("tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtrW").is_none());
	}
}
