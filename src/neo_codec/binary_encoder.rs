use std::hash::Hasher;

/// A binary encoder that can write various primitive types and serializable objects to a byte vector.
///
/// # Examples
///
/// ```
///
/// use neo_rs::prelude::Encoder;
/// let mut encoder = Encoder::new();
/// encoder.write_u8(0x12);
/// encoder.write_i32(-123456);
/// encoder.write_string("hello");
/// let bytes = encoder.to_bytes();
/// assert_eq!(bytes, vec![0x12, 0x30, 0x71, 0xfe, 0xff, 0xff, 0xff, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
/// ```
use serde::Serialize;
use serde_derive::Deserialize;

use neo::prelude::NeoSerializable;

use crate::prelude::CodecError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Encoder {
	data: Vec<u8>,
}

impl Encoder {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}

	pub fn size(&self) -> usize {
		self.data.len()
	}

	pub fn write_bool(&mut self, value: bool) {
		self.write_u8(if value { 1 } else { 0 });
	}

	pub fn write_u8(&mut self, value: u8) {
		self.data.push(value);
	}

	pub fn write_i16(&mut self, v: i16) {
		self.write_u16(v as u16);
	}

	pub fn write_i32(&mut self, v: i32) {
		self.write_u32(v as u32);
	}

	pub fn write_i64(&mut self, v: i64) {
		self.data.extend_from_slice(&v.to_le_bytes());
	}

	pub fn write_u16(&mut self, v: u16) {
		self.data.extend_from_slice(&v.to_le_bytes());
	}

	pub fn write_u32(&mut self, v: u32) {
		self.data.extend_from_slice(&v.to_le_bytes());
	}

	pub fn write_bytes(&mut self, bytes: &[u8]) {
		self.data.extend_from_slice(bytes);
	}

	fn write_var_int(&mut self, v: i64) {
		if v < 0 {
			panic!("Negative value not allowed")
		}
		if v < 0xfd {
			self.write_u8(v as u8)
		} else if v <= u16::MAX as i64 {
			self.write_u8(0xfd);
			self.write_u16(v as u16);
		} else if v <= u32::MAX as i64 {
			self.write_u8(0xfe);
			self.write_u32(v as u32);
		} else {
			self.write_u8(0xff);
			self.write_i64(v);
		}
	}

	pub fn write_var_string(&mut self, v: &str) {
		self.write_var_bytes(v.as_bytes());
	}

	pub fn write_fixed_string(
		&mut self,
		v: &Option<String>,
		length: usize,
	) -> Result<(), CodecError> {
		let bytes = v.as_deref().unwrap_or_default().as_bytes();
		if bytes.len() > length {
			return Err(CodecError::InvalidEncoding("String too long".to_string()))
		}
		let mut padded = vec![0; length];
		padded[0..bytes.len()].copy_from_slice(bytes);
		Ok(self.write_bytes(&padded))
	}

	pub fn write_var_bytes(&mut self, bytes: &[u8]) {
		self.write_var_int(bytes.len() as i64);
		self.write_bytes(bytes);
	}

	pub fn write_serializable_fixed<S: NeoSerializable>(&mut self, value: &S) {
		value.encode(self);
	}
	pub fn write_serializable_list_fixed<S: NeoSerializable>(&mut self, value: &[S]) {
		value.iter().for_each(|v| v.encode(self));
	}

	pub fn write_serializable_variable_bytes<S: NeoSerializable>(&mut self, values: &S) {
		self.write_var_int(values.to_array().len() as i64);
		values.encode(self);
	}

	pub fn write_serializable_variable_list<S: NeoSerializable>(&mut self, values: &[S]) {
		self.write_var_int(values.len() as i64);
		self.write_serializable_list_fixed(values);
	}

	pub fn write_serializable_variable_list_bytes<S: NeoSerializable>(&mut self, values: &[S]) {
		let total_size: usize = values.iter().map(|item| item.to_array().len()).sum();
		self.write_var_int(total_size as i64);
		self.write_serializable_list_fixed(values);
	}

	pub fn reset(&mut self) {
		self.data.clear();
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		self.data.clone()
	}
}

impl Hasher for Encoder {
	fn finish(&self) -> u64 {
		unimplemented!()
	}

	fn write(&mut self, bytes: &[u8]) {
		self.write_bytes(bytes);
	}
}

#[cfg(test)]
mod tests {
	use neo::prelude::Encoder;

	#[test]
	fn test_write_u32() {
		let mut writer = Encoder::new();

		let max = u32::MAX;
		writer.write_u32(max);
		assert_eq!(writer.to_bytes(), vec![0xff; 4]);
		writer.reset();
		writer.write_u32(0);
		assert_eq!(writer.to_bytes(), vec![0; 4]);
		writer.reset();
		writer.write_u32(12345);
		assert_eq!(writer.to_bytes(), vec![0x39, 0x30, 0, 0]);
	}

	#[test]
	fn test_write_i64() {
		let mut writer = Encoder::new();

		writer.write_i64(0x1234567890123456i64);
		assert_eq!(writer.to_bytes(), [0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);

		writer.reset();
		writer.write_i64(i64::MAX);
		assert_eq!(writer.to_bytes(), [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);

		writer.reset();
		writer.write_i64(i64::MIN);
		assert_eq!(writer.to_bytes(), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]);

		writer.reset();
		writer.write_i64(0);
		assert_eq!(writer.to_bytes(), vec![0u8; 8]);

		writer.reset();
		writer.write_i64(1234567890);
		assert_eq!(writer.to_bytes(), vec![0xd2, 0x02, 0x96, 0x49, 0, 0, 0, 0]);
	}

	#[test]
	fn test_write_u16() {
		let mut writer = Encoder::new();

		let max = u16::MAX;
		writer.write_u16(max);
		assert_eq!(writer.to_bytes(), vec![0xff; 2]);

		writer.reset();
		writer.write_u16(0);
		assert_eq!(writer.to_bytes(), vec![0; 2]);

		writer.reset();
		writer.write_u16(12345);
		assert_eq!(writer.to_bytes(), vec![0x39, 0x30]);
	}

	#[test]
	fn test_write_var_int() {
		let mut writer = Encoder::new();

		writer.write_var_int(0);
		assert_eq!(writer.to_bytes(), vec![0]);

		writer.reset();
		writer.write_var_int(252);
		assert_eq!(writer.to_bytes(), vec![0xfc]);

		writer.reset();
		writer.write_var_int(253);
		assert_eq!(writer.to_bytes(), vec![0xfd, 0xfd, 0]);

		writer.reset();
		writer.write_var_int(65_534);
		assert_eq!(writer.to_bytes(), vec![0xfd, 0xfe, 0xff]);

		writer.reset();
		writer.write_var_int(65_536);
		assert_eq!(writer.to_bytes(), vec![0xfe, 0, 0, 1, 0]);

		writer.reset();
		writer.write_var_int(4_294_967_295);
		assert_eq!(writer.to_bytes(), vec![0xfe, 0xff, 0xff, 0xff, 0xff]);

		writer.reset();
		writer.write_var_int(4_294_967_296);
		assert_eq!(writer.to_bytes(), vec![0xff, 0, 0, 0, 0, 1, 0, 0, 0]);
	}

	#[test]
	fn test_write_var_bytes() {
		let mut writer = Encoder::new();

		let bytes = hex::decode("010203").unwrap();
		writer.write_var_bytes(&bytes);
		assert_eq!(writer.to_bytes(), hex::decode("03010203").unwrap());

		writer.reset();
		let bytes = "00102030102030102030102030102030102030102030102030102030102030102031020301020301020301020301020301020301020301020301020301020301020310203010203010203010203010203010203010203010203010203010203010203102030102030102030102030102030102030102030102030102030102030102030010203010203010203010203010203010203010203010203010203010203010203102030102030102030102030102030102030102030102030102030102030102031020301020301020301020301020301020301020301020301020301020301020310203010203010203010203010203010203010203010203010203010203010203";
		writer.write_var_bytes(&hex::decode(bytes.clone()).unwrap());
		assert_eq!(writer.to_bytes(), hex::decode(format!("fd0601{}", bytes)).unwrap());
	}

	#[test]
	fn test_write_var_string() {
		let mut writer = Encoder::new();

		let s = "hello, world!";
		writer.write_var_string(s);
		assert_eq!(writer.to_bytes(), hex::decode("0d68656c6c6f2c20776f726c6421").unwrap());
		writer.reset();
		let s = "hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!hello, world!";
		writer.write_var_string(&s);
		assert_eq!(
			writer.to_bytes(),
			[hex::decode("fd1502").unwrap(), s.as_bytes().to_vec()].concat()
		);
	}
}
