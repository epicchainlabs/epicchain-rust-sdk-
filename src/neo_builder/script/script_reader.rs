use rustc_serialize::hex::ToHex;
use tokio::io::AsyncReadExt;

use neo::prelude::{BuilderError, Bytes, Decoder, InteropService, OpCode, OperandSize};

pub struct ScriptReader;

impl ScriptReader {
	pub fn get_interop_service_code(_hash: String) -> Option<InteropService> {
		InteropService::from_hash(_hash)
	}
	pub fn convert_to_op_code_string(script: &Bytes) -> String {
		let mut reader = Decoder::new(script);
		let mut result = String::new();
		while reader.pointer().clone() < script.len() {
			if let Ok(op_code) = OpCode::try_from(reader.read_u8()) {
				result.push_str(&format!("{:?}", op_code).to_uppercase());
				if let Some(size) = op_code.operand_size() {
					if size.size().clone() > 0 {
						result.push_str(&format!(
							" {}",
							reader.read_bytes(size.size().clone() as usize).unwrap().to_hex()
						));
					} else if size.prefix_size().clone() > 0 {
						let prefix_size = Self::get_prefix_size(&mut reader, size).unwrap();
						result.push_str(&format!(
							" {} {}",
							prefix_size,
							reader.read_bytes(prefix_size).unwrap().to_hex()
						));
					}
				}
				result.push('\n');
			}
		}
		result
	}

	fn get_prefix_size(reader: &mut Decoder, size: OperandSize) -> Result<usize, BuilderError> {
		match size.prefix_size() {
			1 => Ok(reader.read_u8() as usize),
			2 => Ok(reader.read_i16() as usize),
			4 => Ok(reader.read_i32() as usize),
			_ => Err(BuilderError::UnsupportedOperation(
				"Only operand prefix sizes 1, 2, and 4 are supported".to_string(),
			)),
		}
	}
}

#[cfg(test)]
mod tests {
	use rustc_serialize::hex::FromHex;

	use super::*;

	// Adjust this to import your ScriptReader and other necessary items.

	#[test]
	fn test_convert_to_op_code_string() {
		let script = "0c0548656c6c6f0c05576f726c642150419bf667ce41e63f18841140".from_hex().unwrap();
		let expected_op_code_string = "PUSHDATA1 5 48656c6c6f\nPUSHDATA1 5 576f726c64\nNOP\nSWAP\nSYSCALL 9bf667ce\nSYSCALL e63f1884\nPUSH1\nRET\n";

		// Assuming ScriptReader::convert_to_op_code_string exists and performs as expected
		let op_code_string = ScriptReader::convert_to_op_code_string(&script);

		assert_eq!(op_code_string.as_str(), expected_op_code_string);
	}
}
