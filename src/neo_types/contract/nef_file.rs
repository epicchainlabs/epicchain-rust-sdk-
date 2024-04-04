use std::hash::Hasher;

use primitive_types::H160;
use tokio::io::AsyncReadExt;

use neo::prelude::{
	Bytes, CodecError, ContractParameter, Decoder, Encoder, HashableForVec, NeoSerializable,
	StackItem, TypeError,
};

/*
┌───────────────────────────────────────────────────────────────────────┐
│                    NEO Executable Format 3 (NEF3)                     │
├──────────┬───────────────┬────────────────────────────────────────────┤
│  Field   │     Type      │                  Comment                   │
├──────────┼───────────────┼────────────────────────────────────────────┤
│ Magic    │ uint32        │ Magic header                               │
│ Compiler │ byte[64]      │ Compiler name and version                  │
├──────────┼───────────────┼────────────────────────────────────────────┤
│ Source   │ byte[]        │ The url of the source files, max 255 bytes │
│ Reserve  │ byte[2]       │ Reserved for future extensions. Must be 0. │
│ Tokens   │ MethodToken[] │ Method tokens                              │
│ Reserve  │ byte[2]       │ Reserved for future extensions. Must be 0. │
│ Script   │ byte[]        │ Var bytes for the payload                  │
├──────────┼───────────────┼────────────────────────────────────────────┤
│ Checksum │ uint32        │ First four bytes of double SHA256 hash     │
└──────────┴───────────────┴────────────────────────────────────────────┘
 */

#[derive(Debug, Clone)]
pub struct NefFile {
	compiler: Option<String>,
	source_url: String,
	method_tokens: Vec<MethodToken>,
	script: Bytes,
	checksum: Bytes,
}

impl Into<ContractParameter> for NefFile {
	fn into(self) -> ContractParameter {
		ContractParameter::byte_array(self.to_array())
	}
}

impl NefFile {
	const MAGIC: u32 = 0x3346454E;
	const MAGIC_SIZE: usize = 4;
	const COMPILER_SIZE: usize = 64;
	const MAX_SOURCE_URL_SIZE: usize = 256;
	const MAX_SCRIPT_LENGTH: usize = 512 * 1024;
	const CHECKSUM_SIZE: usize = 4;
	pub const HEADER_SIZE: usize = Self::MAGIC_SIZE + Self::COMPILER_SIZE;

	fn get_checksum_as_integer(bytes: &Bytes) -> i32 {
		let mut bytes = bytes.clone();
		bytes.reverse();
		i32::from_be_bytes(bytes.try_into().unwrap())
	}

	fn compute_checksum(file: &NefFile) -> Bytes {
		Self::compute_checksum_from_bytes(file.to_array())
	}

	fn compute_checksum_from_bytes(bytes: Bytes) -> Bytes {
		let mut file_bytes = bytes.clone();
		file_bytes.truncate(bytes.len() - Self::CHECKSUM_SIZE);
		file_bytes.hash256()[..Self::CHECKSUM_SIZE].try_into().unwrap()
	}

	fn read_from_file(file: &str) -> Result<Self, TypeError> {
		let file_bytes = std::fs::read(file).unwrap();
		if file_bytes.len() > 0x100000 {
			return Err(TypeError::InvalidArgError("NEF file is too large".to_string()))
		}

		let mut reader = Decoder::new(&file_bytes);
		let nef = reader.read_serializable().unwrap();
		Ok(nef)
	}

	fn read_from_stack_item(item: StackItem) -> Result<Self, TypeError> {
		if let StackItem::ByteString { value: bytes } = item {
			let mut reader = Decoder::new(&bytes.as_bytes());
			let nef = reader.read_serializable().unwrap();
			Ok(nef)
		} else {
			Err(TypeError::UnexpectedReturnType(
				serde_json::to_string(&item).unwrap() + StackItem::BYTE_STRING_VALUE,
			))
		}
	}
}

impl NeoSerializable for NefFile {
	type Error = TypeError;

	fn size(&self) -> usize {
		let mut size = Self::HEADER_SIZE;
		size += self.source_url.len() + 1;
		size += self.method_tokens.len() + 2;
		size += self.script.len();
		size += Self::CHECKSUM_SIZE;

		size
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_u32(Self::MAGIC);
		writer
			.write_fixed_string(&self.compiler, Self::COMPILER_SIZE)
			.expect("Failed to serialize compiler");
		writer.write_var_string(&self.source_url);
		writer.write_u8(0);
		writer.write_serializable_variable_list(&self.method_tokens);
		writer.write_u16(0);
		writer.write_var_bytes(&self.script);
		writer.write_bytes(&self.checksum);
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		let magic = reader.read_u32();
		if magic != Self::MAGIC {
			return Err(TypeError::InvalidEncoding("Invalid magic".to_string()))
		}

		let compiler_bytes = reader.read_bytes(Self::COMPILER_SIZE)?;
		let compiler = String::from_utf8(compiler_bytes.to_vec())
			.map_err(|_| CodecError::InvalidEncoding("Invalid compiler".to_string()))?;

		let source_url = reader.read_var_string()?;
		if source_url.len() > Self::MAX_SOURCE_URL_SIZE {
			return Err(TypeError::InvalidEncoding("Invalid source url".to_string()))
		}

		if reader.read_u8() != 0 {
			return Err(TypeError::InvalidEncoding("Invalid reserve bytes".to_string()))
		}

		let method_tokens = reader.read_serializable_list()?;

		if reader.read_u16() != 0 {
			return Err(TypeError::InvalidEncoding("Invalid reserve bytes".to_string()))
		}

		let script = reader.read_var_bytes()?;
		if script.is_empty() {
			return Err(TypeError::InvalidEncoding("Invalid script".to_string()))
		}

		let file =
			Self { compiler: Some(compiler), source_url, method_tokens, script, checksum: vec![] };

		let checksum = reader.read_bytes(Self::CHECKSUM_SIZE)?;
		if checksum != Self::compute_checksum(&file) {
			return Err(TypeError::InvalidEncoding("Invalid checksum".to_string()))
		}

		Ok(file)
	}

	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}

#[derive(Debug, Clone)]
pub struct MethodToken {
	hash: H160,
	method: String,
	params_count: u16,
	has_return_value: bool,
	call_flags: u8,
}

impl MethodToken {
	const PARAMS_COUNT_SIZE: usize = 2;
	const HAS_RETURN_VALUE_SIZE: usize = 1;
	const CALL_FLAGS_SIZE: usize = 1;
}

impl NeoSerializable for MethodToken {
	type Error = TypeError;

	fn size(&self) -> usize {
		let mut size = H160::len_bytes();
		size += self.method.len();
		size += MethodToken::PARAMS_COUNT_SIZE;
		size += MethodToken::HAS_RETURN_VALUE_SIZE;
		size += MethodToken::CALL_FLAGS_SIZE;

		size
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_serializable_fixed(&self.hash);
		writer.write_var_string(&self.method);
		writer.write_u16(self.params_count);
		writer.write_bool(self.has_return_value);
		writer.write_u8(self.call_flags);
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		let hash = reader.read_serializable()?;
		let method = reader.read_var_string()?;
		let params_count = reader.read_u16();
		let has_return_value = reader.read_bool();
		let call_flags = reader.read_u8();

		Ok(Self { hash, method, params_count, has_return_value, call_flags })
	}

	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}
