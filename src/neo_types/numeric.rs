use num_bigint::BigInt;

/// Trait to convert types to padded byte vectors.
pub trait ToBytesPadded {
	/// Converts the type to a byte vector padded to the given length.
	///
	/// # Arguments
	///
	/// * `length` - The desired length of the resulting byte vector.
	fn to_bytes_padded(&self, length: usize) -> Vec<u8>;
}

impl ToBytesPadded for BigInt {
	fn to_bytes_padded(&self, length: usize) -> Vec<u8> {
		let bytes = self.to_signed_bytes_be();
		if bytes.len() < length {
			let mut padded = vec![0u8; length];
			padded[length - bytes.len()..].copy_from_slice(&bytes);
			padded
		} else {
			bytes
		}
	}
}

/// Returns the result of raising the base to the power of the exponent.
///
/// # Arguments
///
/// * `base` - The base number.
/// * `exp` - The exponent to which the base is raised.
fn power_of(base: i32, exp: i32) -> i32 {
	base.pow(exp as u32)
}

/// Determines the size of a variable based on its value.
///
/// # Arguments
///
/// * `n` - The value to determine the size for.
fn var_size(n: i128) -> usize {
	match n {
		n if n < 0xfd => 1,
		n if n <= 0xffff => 3,
		n if n <= 0xffffffff => 5,
		_ => 9,
	}
}

/// Converts an i32 to its unsigned counterpart.
///
/// # Arguments
///
/// * `n` - The signed integer to convert.
fn to_unsigned(n: i32) -> u32 {
	n as u32
}

/// Trait to convert types to byte vectors.
pub trait ToBytes {
	/// Converts the type to a byte vector.
	fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for i32 {
	fn to_bytes(&self) -> Vec<u8> {
		self.to_be_bytes().to_vec()
	}
}

impl ToBytes for i64 {
	fn to_bytes(&self) -> Vec<u8> {
		self.to_be_bytes().to_vec()
	}
}

impl ToBytes for f32 {
	fn to_bytes(&self) -> Vec<u8> {
		self.to_be_bytes().to_vec()
	}
}

impl ToBytes for f64 {
	fn to_bytes(&self) -> Vec<u8> {
		self.to_be_bytes().to_vec()
	}
}

/// Converts a DateTime object to milliseconds since the Unix epoch.
///
/// # Arguments
///
/// * `datetime` - The DateTime object to convert.
fn to_milliseconds(datetime: chrono::DateTime<chrono::Utc>) -> i64 {
	datetime.timestamp_millis()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_bytes_padded() {
		let n = BigInt::from(1234);
		let bytes = n.to_bytes_padded(8);

		assert_eq!(bytes, vec![0, 0, 0, 0, 4, 210, 0, 0]);
	}

	#[test]
	fn test_power() {
		assert_eq!(power_of(2, 3), 8);
		assert_eq!(power_of(5, 2), 25);
	}

	#[test]
	fn test_var_size() {
		assert_eq!(var_size(100), 1);
		assert_eq!(var_size(1000), 3);
		assert_eq!(var_size(1000000), 5);
		assert_eq!(var_size(10000000000), 9);
	}

	#[test]
	fn test_to_unsigned() {
		assert_eq!(to_unsigned(-1), 4294967295);
		assert_eq!(to_unsigned(10), 10);
	}

	#[test]
	fn test_i32_to_bytes() {
		let n = 256;
		assert_eq!(n.to_bytes(), vec![0, 1, 0, 0]);
	}

	#[test]
	fn test_i64_to_bytes() {
		let n = 123456;
		assert_eq!(n.to_bytes(), vec![0, 0, 1, 214, 210, 96, 0, 0]);
	}

	#[test]
	fn test_f32_to_bytes() {
		let n = 1.5f32;
		assert_eq!(n.to_bytes(), vec![0, 0, 120, 63]);
	}

	#[test]
	fn test_datetime_to_ms() {
		let dt = chrono::Utc::now();
		let ms = to_milliseconds(dt);

		assert!(ms > 0);
	}
}
