/// This module provides a binary decoder that can read various types of data from a byte slice.
///
/// # Examples
///
/// ```
///
/// use neo_rs::prelude::Decoder;
/// let data = [0x01, 0x02, 0x03, 0x04];
/// let mut decoder = Decoder::new(&data);
///
/// assert_eq!(decoder.read_bool(), true);
/// assert_eq!(decoder.read_u8(), 2);
/// assert_eq!(decoder.read_u16(), 0x0403);
/// assert_eq!(decoder.read_i16(), 0x0403);
/// assert_eq!(decoder.read_u32(), 0x04030201);
/// assert_eq!(decoder.read_i32(), 0x04030201);
/// assert_eq!(decoder.read_u64(), 0x0807060504030201);
/// assert_eq!(decoder.read_i64(), 0x0807060504030201);
/// ```
use getset::{Getters, Setters};
use num_bigint::{BigInt, Sign};
use serde::Deserialize;
use serde_derive::Serialize;

use neo::prelude::{CodecError, NeoSerializable, OpCode};

/// A binary decoder that can read various types of data from a byte slice.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Getters, Setters)]
pub struct Decoder<'a> {
	data: &'a [u8],
	#[getset(get = "pub")]
	pointer: usize,
	marker: usize,
}

impl<'a> Iterator for Decoder<'a> {
	type Item = u8;

	/// Returns the next byte in the byte slice, or None if the end of the slice has been reached.
	fn next(&mut self) -> Option<Self::Item> {
		if self.pointer < self.data.len() {
			let val = self.data[self.pointer];
			self.pointer += 1;
			Some(val)
		} else {
			None
		}
	}
}

impl<'a> Decoder<'a> {
	/// Creates a new binary decoder that reads from the given byte slice.
	pub fn new(data: &'a [u8]) -> Self {
		Self { data, pointer: 0, marker: 0 }
	}

	/// Reads a boolean value from the byte slice.
	pub fn read_bool(&mut self) -> bool {
		let val = self.data[self.pointer] == 1;
		self.pointer += 1;
		val
	}

	/// Reads an unsigned 8-bit integer from the byte slice.
	pub fn read_u8(&mut self) -> u8 {
		let val = self.data[self.pointer];
		self.pointer += 1;
		val
	}

	/// Reads an unsigned 16-bit integer from the byte slice.
	pub fn read_u16(&mut self) -> u16 {
		let bytes = self.read_bytes(2).unwrap();
		u16::from_ne_bytes(bytes.try_into().unwrap())
	}

	/// Reads a signed 16-bit integer from the byte slice.
	pub fn read_i16(&mut self) -> i16 {
		let bytes = self.read_bytes(2).unwrap();
		i16::from_ne_bytes(bytes.try_into().unwrap())
	}

	/// Reads an unsigned 32-bit integer from the byte slice.
	pub fn read_u32(&mut self) -> u32 {
		let bytes = self.read_bytes(4).unwrap();
		u32::from_ne_bytes(bytes.try_into().unwrap())
	}

	/// Reads a signed 32-bit integer from the byte slice.
	pub fn read_i32(&mut self) -> i32 {
		let bytes = self.read_bytes(4).unwrap();
		i32::from_ne_bytes(bytes.try_into().unwrap())
	}

	/// Reads an unsigned 64-bit integer from the byte slice.
	pub fn read_u64(&mut self) -> u64 {
		let bytes = self.read_bytes(8).unwrap();
		u64::from_ne_bytes(bytes.try_into().unwrap())
	}
	pub fn read_i64(&mut self) -> i64 {
		let bytes = self.read_bytes(8).unwrap();
		i64::from_ne_bytes(bytes.try_into().unwrap())
	}

	pub fn read_bigint(&mut self) -> Result<BigInt, CodecError> {
		let byte = self.read_u8();

		let negative = byte & 0x80 != 0;
		let len = match byte {
			0..=0x4b => 1,
			0x4c => self.read_u8() as usize,
			0x4d => self.read_u16() as usize,
			0x4e => self.read_u32() as usize,
			_ => return Err(CodecError::InvalidFormat),
		};

		let bytes = self.read_bytes(len).unwrap();
		if negative {
			// Flip sign bit
			if let Some(byte) = bytes.to_owned().get_mut(len - 1) {
				*byte ^= 0x80;
			} else {
				return Err(CodecError::InvalidFormat)
			}
			// bytes.get_mut()[len - 1] ^= 0x80;
		}
		//TODO:: need to check be or le and sign
		Ok(BigInt::from_bytes_be(Sign::Minus, &bytes))
	}

	/// Reads an encoded EC point from the byte slice.
	pub fn read_encoded_ec_point(&mut self) -> Result<Vec<u8>, &'static str> {
		let byte = self.read_u8();
		match byte {
			0x02 | 0x03 => Ok(self.read_bytes(32).unwrap()),
			_ => Err("Invalid encoded EC point"),
		}
	}

	/// Reads a byte slice of the given length from the byte slice.
	pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, CodecError> {
		if self.pointer + length > self.data.len() {
			return Err(CodecError::IndexOutOfBounds("Read beyond end of buffer".to_string()))
		}
		let result = self.data[self.pointer..self.pointer + length].to_vec();
		self.pointer += length;
		Ok(result)
	}

	/// Reads a variable-length byte slice from the byte slice.
	pub fn read_var_bytes(&mut self) -> Result<Vec<u8>, CodecError> {
		let len = self.read_var_int().unwrap() as usize;
		self.read_bytes(len)
	}

	/// Reads a variable-length integer from the byte slice.
	pub fn read_var_int(&mut self) -> Result<i64, CodecError> {
		let first = self.read_u8();
		match first {
			0xfd => Ok(self.read_i16() as i64),
			0xfe => Ok(self.read_i32() as i64),
			0xff => Ok(self.read_i64() as i64),
			_ => Ok(first as i64),
		}
	}

	pub fn read_var_string(&mut self) -> Result<String, CodecError> {
		let bytes = self.read_var_bytes().unwrap();

		let string = match String::from_utf8(bytes.to_vec()) {
			Ok(s) => s,
			Err(e) => {
				// Handle invalid UTF-8
				return Err(CodecError::InvalidEncoding(e.to_string()))
			},
		};

		// Trim null bytes from end
		let string = string.trim_end_matches(char::from(0));

		Ok(string.to_string())
	}

	/// Reads a push byte slice from the byte slice.
	pub fn read_push_bytes(&mut self) -> Result<Vec<u8>, CodecError> {
		let opcode = self.read_u8();
		let len = match OpCode::try_from(opcode)? {
			OpCode::PushData1 => self.read_u8() as usize,
			OpCode::PushData2 => self.read_i16() as usize,
			OpCode::PushData4 => self.read_i32() as usize,
			_ => return Err(CodecError::InvalidOpCode),
		};

		self.read_bytes(len)
	}

	/// Reads a push integer from the byte slice.
	pub fn read_push_int(&mut self) -> Result<BigInt, CodecError> {
		let byte = self.read_u8();

		if (OpCode::PushM1 as u8..=OpCode::Push16 as u8).contains(&byte) {
			return Ok(BigInt::from(byte as i8 - OpCode::Push0 as i8))
		}

		let count = match OpCode::try_from(byte)? {
			OpCode::PushInt8 => 1,
			OpCode::PushInt16 => 2,
			OpCode::PushInt32 => 4,
			OpCode::PushInt64 => 8,
			OpCode::PushInt128 => 16,
			OpCode::PushInt256 => 32,
			_ =>
				return Err(CodecError::InvalidEncoding("Couldn't parse PUSHINT OpCode".to_string())),
		};

		let bytes = self.read_bytes(count)?;
		Ok(BigInt::from_signed_bytes_be(&bytes))
	}

	/// Reads a push string from the byte slice.
	pub fn read_push_string(&mut self) -> Result<String, CodecError> {
		let bytes = self.read_push_bytes().unwrap();
		String::from_utf8(Vec::from(bytes))
			.map_err(|_| CodecError::InvalidEncoding("Invalid UTF-8".to_string()))
	}

	/// Reads a deserializable value from the byte slice.
	pub fn read_serializable<T: NeoSerializable>(&mut self) -> Result<T, CodecError> {
		T::decode(self).map_err(|_e| CodecError::InvalidFormat)
	}

	/// Reads a list of deserializable values from the byte slice.
	pub fn read_serializable_list<T: NeoSerializable>(&mut self) -> Result<Vec<T>, CodecError> {
		let len = self.read_var_int().unwrap();
		let mut list = Vec::with_capacity(len as usize);
		for _ in 0..len {
			T::decode(self)
				.and_then(|item| Ok(list.push(item)))
				.expect("TODO: panic message");
		}
		Ok(list)
	}

	pub fn read_serializable_list_var_bytes<T: NeoSerializable>(
		&mut self,
	) -> Result<Vec<T>, CodecError> {
		let len = self.read_var_int().unwrap();
		let mut bytes_read = 0;
		let offset = self.pointer;
		let mut list = Vec::with_capacity(len as usize);
		while bytes_read < len {
			T::decode(self)
				.and_then(|item| Ok(list.push(item)))
				.expect("TODO: panic message");
			bytes_read = (self.pointer - offset) as i64;
		}
		Ok(list)
	}

	pub fn mark(&mut self) {
		self.marker = self.pointer;
	}

	pub fn reset(&mut self) {
		self.pointer = self.marker;
	}

	// pub fn read_ec_point(&mut self) -> Result<ProjectivePoint, &'static str> {
	// 	let tag = self.read_u8();
	// 	let bytes = match tag {
	// 		0x00 => return Ok(ProjectivePoint::IDENTITY),
	// 		0x02 | 0x03 => self.read_bytes(32),
	// 		0x04 => self.read_bytes(64),
	// 		_ => return Err("Invalid EC point tag"),
	// 	};
	//
	// 	let point = EncodedPoint::from_bytes(bytes).unwrap();
	// 	match ProjectivePoint::from_encoded_point(&point) {
	// 		Some(point) => Ok(point),
	// 		None => Err("Invalid EC point"),
	// 	}
	// }

	pub fn available(&self) -> usize {
		self.data.len() - self.pointer
	}
}

#[cfg(test)]
mod tests {
	use num_bigint::BigInt;

	use neo::prelude::Decoder;

	#[test]
	fn test_read_push_data_bytes() {
		let prefix_count_map = [
			(hex::decode("0c01").unwrap(), 1),
			(hex::decode("0cff").unwrap(), 255),
			(hex::decode("0d0001").unwrap(), 256),
			(hex::decode("0d0010").unwrap(), 4096),
			(hex::decode("0e00000100").unwrap(), 65536),
		];

		for (prefix, count) in prefix_count_map {
			let bytes = vec![1u8; count];
			let data = [prefix.as_slice(), bytes.as_slice()].concat();
			assert_eq!(Decoder::new(&data).read_push_bytes().unwrap(), bytes);
		}
	}

	#[test]
	fn test_fail_read_push_data() {
		let data = hex::decode("4b010000").unwrap();
		let err = Decoder::new(&data).read_push_bytes().unwrap_err();
		assert_eq!(err.to_string(), "Invalid op code")
	}

	#[test]
	fn test_read_push_data_string() {
		let empty = hex::decode("0c00").unwrap();
		assert_eq!(Decoder::new(&empty).read_push_string().unwrap(), "");

		let a = hex::decode("0c0161").unwrap();
		assert_eq!(Decoder::new(&a).read_push_string().unwrap(), "a");

		let bytes = vec![0u8; 10000];
		let input = [hex::decode("0e10270000").unwrap(), bytes.as_slice().to_vec()].concat();
		let expected = String::from_utf8(bytes).unwrap();

		assert_eq!(Decoder::new(&input).read_push_string().unwrap(), expected);
	}

	#[test]
	fn test_read_push_data_big_integer() {
		let zero = hex::decode("10").unwrap();
		assert_eq!(Decoder::new(&zero).read_push_int().unwrap(), BigInt::from(0));

		let one = hex::decode("11").unwrap();
		assert_eq!(Decoder::new(&one).read_push_int().unwrap(), BigInt::from(1));

		let minus_one = hex::decode("0f").unwrap();
		assert_eq!(Decoder::new(&minus_one).read_push_int().unwrap(), BigInt::from(-1));

		let sixteen = hex::decode("20").unwrap();
		assert_eq!(Decoder::new(&sixteen).read_push_int().unwrap(), BigInt::from(16));
	}

	#[test]
	fn test_read_u32() {
		let max = [0xffu8; 4];
		assert_eq!(Decoder::new(&max).read_u32(), 4_294_967_295);

		let one = hex::decode("01000000").unwrap();
		assert_eq!(Decoder::new(&one).read_u32(), 1);

		let zero = [0u8; 4];
		assert_eq!(Decoder::new(&zero).read_u32(), 0);

		let custom = hex::decode("8cae0000ff").unwrap();
		assert_eq!(Decoder::new(&custom).read_u32(), 44_684);
	}

	#[test]
	fn test_read_i64() {
		let min = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
		assert_eq!(Decoder::new(&min).read_i64(), i64::MIN);

		let max = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
		assert_eq!(Decoder::new(&max).read_i64(), i64::MAX);

		let zero = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
		assert_eq!(Decoder::new(&zero).read_i64(), 0);

		let custom = [0x11, 0x33, 0x22, 0x8c, 0xae, 0x00, 0x00, 0x00, 0xff];
		assert_eq!(Decoder::new(&custom).read_i64(), 749_675_361_041);
	}
}
