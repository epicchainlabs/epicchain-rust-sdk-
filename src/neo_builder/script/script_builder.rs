use std::collections::HashMap;

use getset::{Getters, Setters};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use primitive_types::H160;
use rustc_serialize::hex::FromHex;
use tokio::io::AsyncWriteExt;

use neo::prelude::{
	BuilderError, Bytes, CallFlags, ContractParameter, Encoder, InteropService, OpCode,
	ScriptHashExtension, *,
};

#[derive(Debug, PartialEq, Eq, Hash, Getters, Setters)]
pub struct ScriptBuilder {
	#[getset(get = "pub")]
	pub script: Encoder,
}

impl ScriptBuilder {
	pub fn new() -> Self {
		Self { script: Encoder::new() }
	}
	pub fn op_code(&mut self, op_codes: &[OpCode]) -> &mut Self {
		for opcode in op_codes {
			self.script.write_u8(opcode.opcode());
		}
		self
	}

	pub fn op_code_with_arg(&mut self, opcode: OpCode, argument: Bytes) -> &mut Self {
		self.script.write_u8(opcode.opcode());
		let _ = self.script.write_bytes(&argument);
		self
	}

	pub fn contract_call(
		&mut self,
		hash160: &H160,
		method: &str,
		params: &[ContractParameter],
		call_flags: Option<CallFlags>,
	) -> Result<&mut Self, BuilderError> {
		if params.is_empty() {
			self.op_code(&[OpCode::NewArray]);
		} else {
			self.push_params(params);
		}

		Ok(self
			.push_integer(BigInt::from(match call_flags {
				Some(flags) => flags.value(),
				None => CallFlags::All.value(),
			}))
			.push_data(method.as_bytes().to_vec())
			.push_data(hash160.to_vec())
			.sys_call(InteropService::SystemContractCall))
	}

	pub fn sys_call(&mut self, operation: InteropService) -> &mut Self {
		self.push_opcode_bytes(OpCode::Syscall, operation.hash().from_hex().unwrap())
	}

	pub fn push_params(&mut self, params: &[ContractParameter]) -> &mut Self {
		for param in params {
			self.push_param(param).unwrap();
		}

		self.push_integer(BigInt::from(params.len())).op_code(&[OpCode::Pack])
	}

	pub fn push_param(&mut self, param: &ContractParameter) -> Result<&mut Self, BuilderError> {
		if param.get_type() == ContractParameterType::Any {
			self.op_code(&[OpCode::PushNull]);
		}
		match &param.value.clone().unwrap() {
			ParameterValue::Boolean(b) => self.push_bool(*b),
			ParameterValue::Integer(i) => self.push_integer(BigInt::from(i.clone())),
			ParameterValue::ByteArray(b)
			| ParameterValue::Signature(b)
			| ParameterValue::PublicKey(b) => self.push_data(b.as_bytes().to_vec()),
			ParameterValue::H160(h) => self.push_data(h.as_bytes().to_vec()),
			ParameterValue::H256(h) => self.push_data(h.as_bytes().to_vec()),
			ParameterValue::String(s) => self.push_data(s.as_bytes().to_vec()),
			ParameterValue::Array(arr) => self.push_array(arr).unwrap(),
			ParameterValue::Map(map) => self.push_map(&map.0).unwrap(),
			_ =>
				return Err(BuilderError::IllegalArgument("Unsupported parameter type".to_string())),
		};

		Ok(self)
	}

	/// Adds a push operation with the given integer to the script.
	///
	/// The integer is encoded in its two's complement representation and little-endian byte order.
	///
	/// The integer can be up to 32 bytes in length. Values larger than 32 bytes will return an error.
	///
	/// # Arguments
	///
	/// * `i` - The integer to push to the script
	///
	/// # Errors
	///
	/// Returns an error if the integer is larger than 32 bytes when encoded.
	///
	/// # Examples
	///
	/// ```
	/// use num_bigint::BigInt;
	/// use neo_rs::prelude::ScriptBuilder;
	///
	/// let mut builder = ScriptBuilder::new();
	/// builder.push_int(&BigInt::from(15))?;
	/// ```
	pub fn push_integer(&mut self, i: BigInt) -> &mut Self {
		if i >= BigInt::from(-1) && i <= BigInt::from(16) {
			self.op_code(
				vec![OpCode::try_from(i.to_i32().unwrap() as u8 + OpCode::Push0 as u8).unwrap()]
					.as_slice(),
			);
		} else {
			let bytes = i.to_signed_bytes_le();
			let len = bytes.len();

			// bytes.reverse();

			match len {
				1 => self.push_opcode_bytes(OpCode::PushInt8, bytes),
				2 => self.push_opcode_bytes(OpCode::PushInt16, bytes),
				len if len <= 4 => self.push_opcode_bytes(
					OpCode::PushInt32,
					Self::pad_right(&bytes, 4, i.is_negative()),
				),
				len if len <= 8 => self.push_opcode_bytes(
					OpCode::PushInt64,
					Self::pad_right(&bytes, 8, i.is_negative()),
				),
				len if len <= 16 => self.push_opcode_bytes(
					OpCode::PushInt128,
					Self::pad_right(&bytes, 16, i.is_negative()),
				),
				len if len <= 32 => self.push_opcode_bytes(
					OpCode::PushInt256,
					Self::pad_right(&bytes, 32, i.is_negative()),
				),
				_ => panic!("Integer too large"),
			};
		}

		self
	}

	/// Append opcodes to the script in the provided order.
	///
	/// # Arguments
	///
	/// * `opcode` - The opcode to append
	/// * `argument` - The data argument for the opcode
	///
	/// # Examples
	///
	/// ```
	/// use neo_rs::prelude::{OpCode, ScriptBuilder};
	/// let mut builder = ScriptBuilder::new();
	/// builder.push_opcode_bytes(OpCode::PushData1, vec![0x01]);
	/// ```
	pub fn push_opcode_bytes(&mut self, opcode: OpCode, argument: Vec<u8>) -> &mut ScriptBuilder {
		self.script.write_u8(opcode as u8);
		self.script.write_bytes(&argument);

		self
	}

	fn pad_right(bytes: &[u8], size: usize, negative: bool) -> Vec<u8> {
		let pad_value = if negative { 0xFF } else { 0 };

		let mut padded = vec![0; size];
		padded[0..bytes.len()].copy_from_slice(bytes);
		padded[bytes.len()..].fill(pad_value);
		padded
	}

	// Push data handling

	pub fn push_data(&mut self, data: Vec<u8>) -> &mut Self {
		match data.len() {
			0..=0xff => {
				self.op_code(&[OpCode::PushData1]);
				self.script.write_u8(data.len() as u8);
				let _ = self.script.write_bytes(&data);
			},
			0x100..=0xffff => {
				self.op_code(&[OpCode::PushData2]);
				self.script.write_u16(data.len() as u16);
				let _ = self.script.write_bytes(&data);
			},
			_ => {
				self.op_code(&[OpCode::PushData4]);
				self.script.write_u32(data.len() as u32);
				let _ = self.script.write_bytes(&data);
			},
			// _ => return Err(BuilderError::IllegalArgument("Data too long".to_string())),
		}
		self
	}

	pub fn push_bool(&mut self, b: bool) -> &mut Self {
		if b {
			self.op_code(&[OpCode::PushTrue])
		} else {
			self.op_code(&[OpCode::PushFalse])
		};
		self
	}

	pub fn push_array(&mut self, arr: &[ContractParameter]) -> Result<&mut Self, BuilderError> {
		if arr.is_empty() {
			self.op_code(&[OpCode::NewArray0]);
		} else {
			self.push_params(arr);
		};
		Ok(self)
	}

	pub fn push_map(
		&mut self,
		map: &HashMap<ContractParameter, ContractParameter>,
	) -> Result<&mut Self, BuilderError> {
		for (k, v) in map {
			let kk: ContractParameter = k.clone().into();
			let vv: ContractParameter = v.clone().into();
			self.push_param(&vv).unwrap();
			self.push_param(&kk).unwrap();
		}

		Ok(self.push_integer(BigInt::from(map.len())).op_code(&[OpCode::PackMap]))
	}

	pub fn pack(&mut self) -> &mut Self {
		self.op_code(&[OpCode::Pack])
	}

	pub fn to_bytes(&self) -> Bytes {
		self.script.to_bytes()
	}

	pub fn build_verification_script(pub_key: &Secp256r1PublicKey) -> Bytes {
		let mut sb = ScriptBuilder::new();
		sb.push_data(pub_key.get_encoded(true))
			.sys_call(InteropService::SystemCryptoCheckSig);
		sb.to_bytes()
	}

	pub fn build_multi_sig_script(
		pubkeys: &mut [Secp256r1PublicKey],
		threshold: u8,
	) -> Result<Bytes, BuilderError> {
		let mut sb = ScriptBuilder::new();
		sb.push_integer(BigInt::from(threshold));
		pubkeys.sort_by(|a, b| a.get_encoded(true).cmp(&b.get_encoded(true)));
		for pk in pubkeys.iter() {
			sb.push_data(pk.get_encoded(true));
		}
		sb.push_integer(BigInt::from(pubkeys.len()));
		sb.sys_call(InteropService::SystemCryptoCheckMultiSig);
		Ok(sb.to_bytes())
	}

	pub fn build_contract_script(
		sender: &H160,
		nef_checksum: u32,
		name: &str,
	) -> Result<Bytes, BuilderError> {
		let mut sb = ScriptBuilder::new();
		sb.op_code(&[OpCode::Abort])
			.push_data(sender.to_vec())
			.push_integer(BigInt::from(nef_checksum))
			.push_data(name.as_bytes().to_vec());
		Ok(sb.to_bytes())
	}

	pub fn build_contract_call_and_unwrap_iterator(
		contract_hash: &H160,
		method: &str,
		params: &[ContractParameter],
		max_items: u32,
		call_flags: Option<CallFlags>,
	) -> Result<Bytes, BuilderError> {
		let mut sb = Self::new();
		sb.push_integer(BigInt::from(max_items));

		sb.contract_call(contract_hash, method, params, call_flags).unwrap();

		sb.op_code(&[OpCode::NewArray]);

		let cycle_start = sb.len();
		sb.op_code(&[OpCode::Over]);
		sb.sys_call(InteropService::SystemIteratorNext);

		let jmp_if_not = sb.len();
		sb.op_code_with_arg(OpCode::JmpIf, vec![0]);

		sb.op_code(&[OpCode::Dup, OpCode::Push2, OpCode::Pick])
			.sys_call(InteropService::SystemIteratorValue)
			.op_code(&[
				OpCode::Append,
				OpCode::Dup,
				OpCode::Size,
				OpCode::Push3,
				OpCode::Pick,
				OpCode::Ge,
			]);

		let jmp_if_max = sb.len();
		sb.op_code_with_arg(OpCode::JmpIf, vec![0]);

		let jmp_offset = sb.len();
		let jmp_bytes = (cycle_start - jmp_offset) as u8;
		sb.op_code_with_arg(OpCode::Jmp, vec![jmp_bytes]);

		let load_result = sb.len();
		sb.op_code(&[OpCode::Nip, OpCode::Nip]);

		let mut script = sb.to_bytes();
		let jmp_not_bytes = (load_result - jmp_if_not) as i8;
		script[jmp_if_not + 1] = jmp_not_bytes as u8;

		let jmp_max_bytes = (load_result - jmp_if_max) as i8;
		script[jmp_if_max + 1] = jmp_max_bytes as u8;

		Ok(script)
	}

	pub fn len(&self) -> usize {
		self.script().size()
	}
	// Other static helper methods
}

#[cfg(test)]
mod tests {
	use std::vec;

	use hex_literal::hex;
	use num_bigint::BigInt;
	use num_traits::FromPrimitive;
	use rustc_serialize::hex::{FromHex, ToHex};

	use super::*;

	#[test]
	fn test_push_empty_array() {
		let mut builder = ScriptBuilder::new();
		builder.push_array(&[]).unwrap();
		assert_eq!(builder.to_bytes(), vec![OpCode::NewArray0 as u8]);
	}

	#[test]
	fn test_push_byte_array() {
		let mut builder = ScriptBuilder::new();

		builder.push_data(vec![0xAAu8; 1]);
		assert_eq!(builder.to_bytes()[..2], hex!("0c01"));

		let mut builder = ScriptBuilder::new();
		builder.push_data(vec![0xAAu8; 75]);
		assert_eq!(builder.to_bytes()[..2], hex!("0c4b"));

		let mut builder = ScriptBuilder::new();
		builder.push_data(vec![0xAAu8; 256]);
		assert_eq!(builder.to_bytes()[..3], hex!("0d0001"));

		let mut builder = ScriptBuilder::new();
		builder.push_data(vec![0xAAu8; 65536]);
		assert_eq!(builder.to_bytes()[..5], hex!("0e00000100"));
	}

	#[test]
	fn test_push_string() {
		let mut builder = ScriptBuilder::new();

		builder.push_data("".as_bytes().to_vec());
		assert_eq!(builder.to_bytes()[..2], hex!("0c00"));

		builder.push_data("a".as_bytes().to_vec());
		assert_eq!(builder.to_bytes()[2..], hex!("0c0161"));

		builder.push_data("a".repeat(10000).as_bytes().to_vec());
		assert_eq!(builder.to_bytes()[5..8], hex!("0d1027"));
	}

	#[test]
	fn test_push_integer() {
		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from(0));
		assert_eq!(builder.to_bytes()[..1], vec![OpCode::Push0 as u8]);
		//
		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from(1));
		assert_eq!(builder.to_bytes()[..1], vec![OpCode::Push1 as u8]);

		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from(16));
		assert_eq!(builder.to_bytes()[..1], vec![OpCode::Push16 as u8]);

		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from(17));
		assert_eq!(builder.to_bytes()[..2], hex!("0011"));

		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from(-800000));
		assert_eq!(builder.to_bytes()[1..], hex!("00cbf3ff")); // vec![ 0xff, 0xf3, 0xcb, 0x00].reverse());

		let mut builder = ScriptBuilder::new();
		builder.push_integer(BigInt::from_i64(100000000000).unwrap());
		assert_eq!(builder.to_bytes()[builder.len() - 8..], hex!("00e8764817000000"));

		builder.push_integer(BigInt::from(-100000000000_i64));
		assert_eq!(
			builder.to_bytes()[builder.len() - 8..],
			[0x00, 0x18, 0x89, 0xb7, 0xe8, 0xff, 0xff, 0xff]
		);

		builder.push_integer(BigInt::from(100000000000_i64));
		assert_eq!(
			builder.to_bytes()[builder.len() - 8..],
			[0x00, 0xe8, 0x76, 0x48, 0x17, 0x00, 0x00, 0x00]
		);

		builder.push_integer(BigInt::from(-10i128.pow(23)));
		assert_eq!(
			builder.to_bytes()[builder.len() - 16..],
			"ffffffffffffead2fd381eb509800000".from_hex().unwrap().reverse()
		);

		builder.push_integer(BigInt::from(10i128.pow(23)));
		assert_eq!(
			builder.to_bytes()[builder.len() - 16..],
			"000000000000152d02c7e14af6800000".from_hex().unwrap().reverse()
		);

		builder.push_integer(BigInt::from(10).pow(40));
		assert_eq!(
			builder.to_bytes()[builder.len() - 32..],
			"0000000000000000000000000000001d6329f1c35ca4bfabb9f5610000000000"
				.from_hex()
				.unwrap()
				.reverse()
		);

		builder.push_integer(-BigInt::from(10).pow(40));
		assert_eq!(
			builder.to_bytes()[builder.len() - 32..],
			"ffffffffffffffffffffffffffffffe29cd60e3ca35b4054460a9f0000000000"
				.from_hex()
				.unwrap()
				.reverse()
		);
	}

	#[test]
	fn test_verification_script() {
		let pubkey1 = "035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50"
			.from_hex()
			.unwrap();
		let pubkey2 = "03eda286d19f7ee0b472afd1163d803d620a961e1581a8f2704b52c0285f6e022d"
			.from_hex()
			.unwrap();
		let pubkey3 = "03ac81ec17f2f15fd6d193182f927c5971559c2a32b9408a06fec9e711fb7ca02e"
			.from_hex()
			.unwrap();

		let script = ScriptBuilder::build_multi_sig_script(
			&mut [pubkey1.into(), pubkey2.into(), pubkey3.into()],
			2,
		)
		.unwrap();

		// let expected = hex!("5221035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50210"
		//     "03ac81ec17f2f15fd6d193182f927c5971559c2a32b9408a06fec9e711fb7ca02e210"
		//     "03eda286d19f7ee0b472afd1163d803d620a961e1581a8f2704b52c0285f6e022d53ae");
		//
		// assert_eq!(script, expected);
	}

	#[test]
	fn test_map() {
		let mut map: HashMap<ContractParameter, ContractParameter> = HashMap::new();
		map.insert(ContractParameter::from(1), ContractParameter::from("first".to_string()));
		map.insert(ContractParameter::from("second"), ContractParameter::from(true));

		let expected_one = ScriptBuilder::new()
			.push_data("first".as_bytes().to_vec())
			.push_integer(BigInt::from(1))
			.push_bool(true)
			.push_data("7365636f6e64".from_hex().unwrap())
			.push_integer(BigInt::from(2))
			.op_code(&[OpCode::PackMap])
			.to_bytes()
			.to_hex();

		let expected_two = ScriptBuilder::new()
			.push_bool(true)
			.push_data("7365636f6e64".from_hex().unwrap())
			.push_data("first".as_bytes().to_vec())
			.push_integer(BigInt::from(1))
			.push_integer(BigInt::from(2))
			.op_code(&[OpCode::PackMap])
			.to_bytes()
			.to_hex();

		let mut builder3 = ScriptBuilder::new().push_map(&map).unwrap().to_bytes().to_hex();

		assert!(builder3 == expected_one || builder3 == expected_two)
	}

	#[test]
	fn test_map_nested() {
		let mut inner: ContractParameterMap = ContractParameterMap::new();
		inner
			.0
			.insert(ContractParameter::from(10), ContractParameter::from("nestedFirst"));

		let mut outer: ContractParameterMap = ContractParameterMap::new();
		outer.0.insert(ContractParameter::from(1), ContractParameter::from("first"));
		outer.0.insert(ContractParameter::from("nested"), ContractParameter::map(inner));

		let expected = ScriptBuilder::new().push_map(&outer.to_map()).unwrap().to_bytes().to_hex();

		let expected_one = ScriptBuilder::new()
			.push_data("first".as_bytes().to_vec())
			.push_integer(BigInt::from(1))
			.push_data("nestedFirst".as_bytes().to_vec())
			.push_integer(BigInt::from(10))
			.push_integer(BigInt::from(1))
			.op_code(&[OpCode::PackMap])
			.push_data("nested".as_bytes().to_vec())
			.push_integer(BigInt::from(2))
			.op_code(&[OpCode::PackMap])
			.to_bytes()
			.to_hex();

		let expected_two = ScriptBuilder::new()
			.push_data("nestedFirst".as_bytes().to_vec())
			.push_integer(BigInt::from(10))
			.push_integer(BigInt::from(1))
			.op_code(&[OpCode::PackMap])
			.push_data("nested".as_bytes().to_vec())
			.push_data("first".as_bytes().to_vec())
			.push_integer(BigInt::from(1))
			.push_integer(BigInt::from(2))
			.op_code(&[OpCode::PackMap])
			.to_bytes()
			.to_hex();

		assert!(expected == expected_one || expected == expected_two);
	}

	fn assert_builder(builder: &ScriptBuilder, expected: &[u8]) {
		assert_eq!(builder.to_bytes(), expected);
	}

	fn byte_array(size: usize) -> Vec<u8> {
		vec![0xAA; size]
	}
}
