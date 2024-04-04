use std::vec;

use getset::{Getters, Setters};
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use p256::pkcs8::der::Encode;
use primitive_types::H160;
use rustc_serialize::hex::{FromHex, ToHex};
use serde::{Deserialize, Serialize};

use neo::prelude::{
	var_size, BuilderError, Bytes, Decoder, Encoder, InteropService, NeoSerializable, OpCode,
	ScriptBuilder, Secp256r1PublicKey, Secp256r1Signature,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters, Serialize, Deserialize)]
pub struct VerificationScript {
	#[getset(get = "pub", set = "pub")]
	script: Bytes,
}

impl VerificationScript {
	pub fn new() -> Self {
		Self { script: Bytes::new() }
	}

	pub fn from(script: Bytes) -> Self {
		Self { script: script.to_vec() }
	}

	pub fn from_public_key(public_key: &Secp256r1PublicKey) -> Self {
		let mut builder = ScriptBuilder::new();
		builder
			.push_data(public_key.get_encoded(true))
			.sys_call(InteropService::SystemCryptoCheckSig);
		Self::from(builder.to_bytes())
	}

	pub fn from_multi_sig(public_keys: &mut [Secp256r1PublicKey], threshold: u8) -> Self {
		// Build multi-sig script
		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from(threshold));
		public_keys.sort();
		for key in public_keys.iter() {
			builder.push_data(key.get_encoded(true));
		}
		builder
			.push_integer(BigInt::from(public_keys.len()))
			.sys_call(InteropService::SystemCryptoCheckMultiSig);
		Self::from(builder.to_bytes())
	}

	/// Checks if this verification script is from a single signature account.
	///
	/// Returns `true` if this script is from a single signature account, otherwise `false`.
	pub fn is_single_sig(&self) -> bool {
		if self.script.len() != 40 {
			return false
		}

		let interop_service = &self.script[self.script.len() - 4..]; // Get the last 4 bytes
		let interop_service_hex = interop_service.to_hex();

		self.script[0] == OpCode::PushData1.opcode()
			&& self.script[1] == 33
			&& self.script[35] == OpCode::Syscall.opcode()
			&& interop_service_hex == InteropService::SystemCryptoCheckSig.hash() // Assuming `hash` returns a hex string
	}

	/// Checks if this verification script is from a multi-signature account.
	///
	/// Returns `true` if this script is from a multi-signature account.
	/// Otherwise returns `false`.
	#[doc(hidden)]
	pub fn is_multi_sig(&self) -> bool {
		if self.script.len() < 42 {
			return false
		}

		let mut reader = Decoder::new(&self.script);

		let n = match reader.by_ref().read_push_int() {
			Ok(n) => n,
			Err(_) => return false,
		};
		if !(1..=16).contains(&(n.to_i32().unwrap())) {
			return false
		}

		let mut m: BigInt = BigInt::zero();
		while reader.by_ref().read_u8() == OpCode::PushData1.opcode() {
			let len = reader.by_ref().read_u8();
			if len != 33 {
				return false
			}
			reader.by_ref().read_encoded_ec_point();
			m += 1;
			reader.mark();
		}

		if !(m >= n && m <= BigInt::from(16)) {
			return false
		}

		reader.reset();

		if BigInt::from(reader.read_push_int().unwrap()) != m
			|| reader.read_u8() != OpCode::Syscall.opcode()
		{
			return false
		}

		let service_bytes = &reader.read_bytes(4).unwrap();
		let hash = &InteropService::SystemCryptoCheckMultiSig.hash().from_hex().unwrap();
		if service_bytes != hash {
			return false
		}

		match reader.by_ref().read_var_int() {
			Ok(v) =>
				if BigInt::from(v) != m {
					return false
				},
			Err(_) => return false,
		}

		if reader.by_ref().read_u8() != OpCode::Syscall as u8 {
			return false
		}

		true
	}

	// other methods
	pub fn hash(&self) -> H160 {
		H160::from_slice(&self.script)
	}

	pub fn get_signatures(&self) -> Vec<Secp256r1Signature> {
		let mut reader = Decoder::new(&self.script);
		let mut signatures = vec![];

		while reader.by_ref().read_u8() == OpCode::PushData1 as u8 {
			let len = reader.by_ref().read_u8();
			let sig =
				Secp256r1Signature::from_bytes(&reader.by_ref().read_bytes(len as usize).unwrap())
					.unwrap();
			signatures.push(sig);
		}

		signatures
	}

	pub fn get_public_keys(&self) -> Result<Vec<Secp256r1PublicKey>, BuilderError> {
		if self.is_single_sig() {
			let mut reader = Decoder::new(&self.script);
			reader.by_ref().read_u8(); // skip pushdata1
			reader.by_ref().read_u8(); // skip length

			let mut point = [0; 33];
			point.copy_from_slice(&reader.by_ref().read_bytes(33).unwrap());

			let key = Secp256r1PublicKey::from_bytes(&point).unwrap();
			return Ok(vec![key])
		}

		if self.is_multi_sig() {
			let mut reader = Decoder::new(&self.script);
			reader.by_ref().read_var_int().unwrap(); // skip threshold

			let mut keys = vec![];
			while reader.by_ref().read_u8() == OpCode::PushData1 as u8 {
				reader.by_ref().read_u8(); // skip length
				let mut point = [0; 33];
				point.copy_from_slice(&reader.by_ref().read_bytes(33).unwrap());
				keys.push(Secp256r1PublicKey::from_bytes(&point).unwrap());
			}

			return Ok(keys)
		}

		Err(BuilderError::InvalidScript("Invalid verification script".to_string()))
	}

	pub fn get_signing_threshold(&self) -> Result<usize, BuilderError> {
		if self.is_single_sig() {
			Ok(1)
		} else if self.is_multi_sig() {
			let reader = &mut Decoder::new(&self.script);
			Ok(reader.by_ref().read_var_int()? as usize)
		} else {
			Err(BuilderError::InvalidScript("Invalid verification script".to_string()))
		}
	}

	pub fn get_nr_of_accounts(&self) -> Result<usize, BuilderError> {
		match self.get_public_keys() {
			Ok(keys) => Ok(keys.len()),
			Err(e) => Err(e),
		}
	}
}

impl NeoSerializable for VerificationScript {
	type Error = BuilderError;

	fn size(&self) -> usize {
		var_size(self.script.len()) + self.script.len()
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_var_bytes(&self.script);
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		let script = reader.read_var_bytes()?;
		Ok(Self { script })
	}
	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hex_literal::hex;
	use rustc_serialize::hex::FromHex;

	#[test]
	fn test_from_public_key() {
		let key = "035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50";
		let pubkey = Secp256r1PublicKey::from_encoded(key.clone()).unwrap();
		let script = VerificationScript::from_public_key(&pubkey);
		let expected = format!(
			"{}21{}{}{}",
			OpCode::PushData1.to_string(),
			key,
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckSig.hash()
		)
		.from_hex()
		.unwrap();

		assert_eq!(script.script(), &expected);
	}

	#[test]
	fn test_from_public_keys() {
		let key1 =
			hex!("035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50").to_vec();
		let key2 =
			hex!("03eda286d19f7ee0b472afd1163d803d620a961e1581a8f2704b52c0285f6e022d").to_vec();
		let key3 =
			hex!("03ac81ec17f2f15fd6d193182f927c5971559c2a32b9408a06fec9e711fb7ca02e").to_vec();

		let mut pubkeys = vec![
			Secp256r1PublicKey::from(key1.clone()),
			Secp256r1PublicKey::from(key2.clone()),
			Secp256r1PublicKey::from(key3.clone()),
		];

		let script = VerificationScript::from_multi_sig(&mut pubkeys, 2);

		let expected = format!(
			"{}{}21{}{}21{}{}21{}{}{}{}",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			key1.to_hex(),
			OpCode::PushData1.to_string(),
			key3.to_hex(),
			OpCode::PushData1.to_string(),
			key2.to_hex(),
			OpCode::Push3.to_string(),
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckMultiSig.hash()
		)
		.from_hex()
		.unwrap();

		assert_eq!(script.script(), &expected);
	}

	#[test]
	fn test_serialize_deserialize() {
		let key = "035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50";
		let pubkey = Secp256r1PublicKey::from_encoded(key.clone()).unwrap();

		let script = VerificationScript::from_public_key(&pubkey);

		let expected = format!(
			"{}21{}{}{}",
			OpCode::PushData1.to_string(),
			key,
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckSig.hash()
		);

		let serialized = script.to_array();

		// Manually deserialize
		let deserialized = VerificationScript::from(serialized[1..].to_vec());

		// Check deserialized script matches
		assert_eq!(deserialized.script(), &expected.from_hex().unwrap());
	}

	#[test]
	fn test_get_signing_threshold() {
		// let key = hex!("...").to_vec();
		//
		// let script = VerificationScript::from(key);
		// assert_eq!(script.get_signing_threshold(), 2);
		//
		// let script = VerificationScript::from(long_script);
		// assert_eq!(script.get_signing_threshold(), 127);
	}

	#[test]
	fn test_invalid_script() {
		let script = VerificationScript::from(hex!("0123456789abcdef").to_vec());

		assert!(script.get_signing_threshold().is_err());
		assert!(script.get_public_keys().is_err());
		assert!(script.get_nr_of_accounts().is_err());
	}

	// #[test]
	// fn test_size() {
	// 	let data = hex!("147e5f3c929dd830d961626551dbea6b70e4b2837ed2fe9089eed2072ab3a655523ae0fa8711eee4769f1913b180b9b3410bbb2cf770f529c85f6886f22cbaaf").to_vec();
	// 	let script = VerificationScript::from(data);
	// 	assert_eq!(script.size(), 65);
	// }

	#[test]
	fn test_is_single_sig_script() {
		let script = format!(
			"{}2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef{}{}",
			OpCode::PushData1.to_string(),
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckSig.hash()
		);

		let verification = VerificationScript::from(script.from_hex().unwrap());
		assert!(verification.is_single_sig());
	}

	// #[test]
	// fn test_from_public_key() {
	// 	let key = "035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50";
	// 	let ec_key = Secp256r1PublicKey::from(key);
	// 	let script = VerificationScript::from_public_key(&ec_key).unwrap();
	//
	// 	let expected = format!(
	// 		"{}21{}{}{}",
	// 		OpCode::PushData1.to_string(),
	// 		key,
	// 		OpCode::Syscall.to_string(),
	// 		InteropService::SystemCryptoCheckSig.hash()
	// 	).from_hex().unwrap();
	//
	// 	assert_eq!(script.script(), &expected);
	// }

	// #[test]
	// fn test_from_public_keys() {
	// 	let key1 = hex!("035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50").to_vec();
	// 	let key2 = hex!("03eda286d19f7ee0b472afd1163d803d620a961e1581a8f2704b52c0285f6e022d").to_vec();
	// 	let key3 = hex!("03ac81ec17f2f15fd6d193182f927c5971559c2a32b9408a06fec9e711fb7ca02e").to_vec();
	//
	// 	let mut pubkeys = vec![
	// 		Secp256r1PublicKey::from(key1.clone()),
	// 		Secp256r1PublicKey::from(key2.clone()),
	// 		Secp256r1PublicKey::from(key3.clone())
	// 	];
	//
	// 	let script = VerificationScript::from_multi_sig(&mut pubkeys, 2).unwrap();
	//
	// 	let expected = format!(
	// 		"{}{}{}21{}{}21{}{}21{}{}{}",
	// 		OpCode::Push2,
	// 		OpCode::PushData1,
	// 		hex::encode(key1),
	// 		OpCode::PushData1,
	// 		hex::encode(key3),
	// 		OpCode::PushData1,
	// 		hex::encode(key2),
	// 		OpCode::Push3,
	// 		OpCode::Syscall,
	// 		InteropService::SystemCryptoCheckMultiSig.hash()
	// 	).from_hex().unwrap();
	//
	// 	assert_eq!(script.script(), &expected);
	// }

	// #[test]
	// fn test_serialize_and_deserialize() {
	// 	let key = hex!("035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50").to_vec();
	// 	let ec_pub_key = Secp256r1PublicKey::from(key.clone());
	//
	// 	let script = VerificationScript::from_public_key(&ec_pub_key);
	//
	// 	let size = vec![NeoConstants::VERIFICATION_SCRIPT_SIZE];
	// 	let expected = format!(
	// 		"{}21{}{}{}",
	// 		OpCode::PushData1,
	// 		hex::encode(key),
	// 		OpCode::Syscall,
	// 		InteropService::SystemCryptoCheckSig.hash(),
	// 	);
	//
	// 	let serialized = format!("{}{}", size, expected).from_hex().unwrap();
	//
	// 	assert_eq!(script.script, serialized);
	//
	// 	let deserialized = VerificationScript::deserialize(&serialized).unwrap();
	//
	// 	assert_eq!(deserialized.script(), &expected);
	// }

	// #[test]
	// fn test_get_signing_threshold() {
	// 	let key = format!(
	// 		"{}{}{}",
	// 		OpCode::PushData1,
	// 		"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
	// 		OpCode::PushData1
	// 	).from_hex().unwrap();
	//
	// 	let mut s = String::new();
	// 	for _ in 1..=3 {
	// 		s.push_str(&hex::encode(key.clone()));
	// 	}
	// 	s.push_str(&format!(
	// 		"{}{}{}",
	// 		OpCode::Push3,
	// 		OpCode::Syscall,
	// 		InteropService::SystemCryptoCheckMultiSig.hash()
	// 	));
	//
	// 	let script = VerificationScript::from(s.from_hex().unwrap()).unwrap();
	// 	assert_eq!(script.get_signing_threshold(), Ok(2));
	// }

	#[test]
	fn test_throw_on_invalid_script() {
		let script = VerificationScript::from("0123456789abcdef".from_hex().unwrap());

		let err = script.get_signing_threshold().unwrap_err();
		assert_eq!(err.to_string(), "Invalid operation");

		let err = script.get_public_keys().unwrap_err();
		assert_eq!(err.to_string(), "Invalid operation");
		let err = script.get_nr_of_accounts().unwrap_err();
		assert_eq!(err.to_string(), "Invalid operation");
	}

	#[test]
	fn test_size() {
		let script = "147e5f3c929dd830d961626551dbea6b70e4b2837ed2fe9089eed2072ab3a655523ae0fa8711eee4769f1913b180b9b3410bbb2cf770f529c85f6886f22cbaaf".from_hex().unwrap();

		let verification = VerificationScript::from(script);
		assert_eq!(verification.size(), 65);
	}

	// #[test]
	// fn test_is_single_sig_script() {
	// 	let script = format!(
	// 		"{}{}{}{}",
	// 		OpCode::PushData1,
	// 		"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
	// 		OpCode::Syscall,
	// 		InteropService::SystemCryptoCheckSig.hash()
	// 	).from_hex().unwrap();
	//
	// 	let verification = VerificationScript::from(script);
	// 	assert!(verification.is_single_sig_script());
	// }

	#[test]
	fn test_is_multi_sig() {
		let script = format!(
			"{}{}{}{}{}{}{}{}{}{}",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::PushData1.to_string(),
			"2103f0f9b358dfed564e74ffe242713f8bc866414226649f59859b140a130818898b",
			OpCode::Push3.to_string(),
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckMultiSig.hash()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_too_short() {
		let script = "a89429c3be9f".from_hex().unwrap();
		let verification = VerificationScript::from(script);

		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_n_less_than_one() {
		let script = format!(
			"{}{}{}{}{}{}3073b3bb",
			OpCode::Push0.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::Push1.to_string(),
			OpCode::PushNull.to_string(),
			OpCode::Syscall.to_string(),
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_abrupt_end() {
		let script = format!(
			"{}{}2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_wrong_push_data() {
		let script = format!(
			"{}{}{}{}{}{}{}{}3073b3bb",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"43031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::Push2.to_string(),
			OpCode::PushNull.to_string(),
			OpCode::Syscall.to_string(),
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_n_greater_than_m() {
		let script = format!(
			"{}{}{}{}{}{}{}{}3073b3bb",
			OpCode::Push3.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::Push2.to_string(),
			OpCode::PushNull.to_string(),
			OpCode::Syscall.to_string()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_m_incorrect() {
		let script = format!(
			"{}{}{}{}{}{}{}{}3073b3bb",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::Push3.to_string(),
			OpCode::PushNull.to_string(),
			OpCode::Syscall.to_string()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_missing_push_null() {
		let script = format!(
			"{}{}{}{}{}{}{}3073b3bb",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::Push2.to_string(),
			OpCode::Syscall.to_string()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_missing_syscall() {
		let script = format!(
			"{}{}{}{}{}{}{}3073b3bb",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::Push2.to_string(),
			OpCode::PushNull.to_string()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_fail_is_multi_sig_wrong_interop_service() {
		let script = format!(
			"{}{}{}{}{}{}{}{}103ab300",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::Push3.to_string(),
			OpCode::PushNull.to_string(),
			OpCode::Syscall.to_string()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);
		assert!(!verification.is_multi_sig());
	}

	#[test]
	fn test_public_keys_from_single_sig() {
		let script = format!(
			"{}{}{}{}",
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckSig.hash()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);

		let keys = verification.get_public_keys().unwrap();

		assert_eq!(keys.len(), 1);

		let encoded = keys[0].get_encoded(true);

		assert_eq!(
			encoded.to_hex(),
			"02028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef"
		);
	}

	#[test]
	fn test_public_keys_from_multi_sig() {
		let script = format!(
			"{}{}{}{}{}{}{}{}{}{}",
			OpCode::Push2.to_string(),
			OpCode::PushData1.to_string(),
			"2102028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
			OpCode::PushData1.to_string(),
			"21031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
			OpCode::PushData1.to_string(),
			"2103f0f9b358dfed564e74ffe242713f8bc866414226649f59859b140a130818898b",
			OpCode::Push3.to_string(),
			OpCode::Syscall.to_string(),
			InteropService::SystemCryptoCheckMultiSig.hash()
		)
		.from_hex()
		.unwrap();

		let verification = VerificationScript::from(script);

		let keys = verification.get_public_keys().unwrap();

		assert_eq!(keys.len(), 3);

		let key1 = keys[0].get_encoded(true);
		let key2 = keys[1].get_encoded(true);
		let key3 = keys[2].get_encoded(true);

		assert_eq!(
			key1.to_hex(),
			"02028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef"
		);
		assert_eq!(
			key2.to_hex(),
			"031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9"
		);
		assert_eq!(
			key3.to_hex(),
			"03f0f9b358dfed564e74ffe242713f8bc866414226649f59859b140a130818898b"
		);
	}
}
