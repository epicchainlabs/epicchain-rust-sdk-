use derive_more::Display;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use neo::prelude::TypeError;

#[derive(Debug, Clone, Display, PartialEq, Eq, Serialize, Deserialize, Getters, Setters)]
pub struct NNSName {
	#[getset(get = "pub")]
	name: String,
}

impl NNSName {
	pub fn new(name: &str) -> Result<Self, TypeError> {
		Self::validate(name, true).unwrap();
		Ok(Self { name: name.to_owned() })
	}

	pub fn is_valid(name: &str, allow_multi_fragments: bool) -> Result<(), TypeError> {
		if name.len() < 3 || name.len() > 255 {
			return Err(TypeError::InvalidNeoName("Invalid name length".to_string()))
		}

		let fragments: Vec<&str> = name.split('.').collect();
		if fragments.len() < 2 || fragments.len() > 8 {
			return Err(TypeError::InvalidNeoName("Invalid fragment count".to_string()))
		}

		if fragments.len() > 2 && !allow_multi_fragments {
			return Err(TypeError::InvalidNeoName("Multiple fragments not allowed".to_string()))
		}

		for fragment in &fragments {
			Self::validate_fragment(fragment, fragment == fragments.last().unwrap()).unwrap();
		}

		Ok(())
	}

	fn validate_fragment(fragment: &str, is_root: bool) -> Result<(), TypeError> {
		let max_len = if is_root { 16 } else { 63 };
		if fragment.is_empty() || fragment.len() > max_len {
			return Err(TypeError::InvalidNeoName("Invalid fragment length".to_string()))
		}

		let first = fragment.chars().next().unwrap();
		if is_root && !first.is_ascii_alphabetic() {
			return Err(TypeError::InvalidNeoName("Root must start with letter".to_string()))
		} else if !is_root && !(first.is_ascii_alphanumeric() || first == '-'.into()) {
			return Err(TypeError::InvalidNeoName("Invalid start character".to_string()))
		}

		if fragment.len() == 1 {
			return Ok(())
		}

		if fragment[1..].chars().any(|c| !(c.is_ascii_alphanumeric() || c == '-'.into())) {
			return Err(TypeError::InvalidNeoName("Invalid character in fragment".to_string()))
		}

		let last = fragment.chars().last().unwrap();
		if !(last.is_ascii_alphanumeric()) {
			return Err(TypeError::InvalidNeoName("Must end with alphanumeric".to_string()))
		}

		Ok(())
	}

	pub fn validate(name: &str, allow_multi_fragments: bool) -> Result<(), TypeError> {
		Self::is_valid(name, allow_multi_fragments).unwrap();
		Ok(())
	}

	pub fn bytes(&self) -> Vec<u8> {
		self.name.as_bytes().to_vec()
	}

	pub fn is_second_level_domain(&self) -> bool {
		Self::is_valid(&self.name, false).is_ok()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NNSRoot {
	root: String,
}

impl NNSRoot {
	pub fn new(root: &str) -> Result<Self, TypeError> {
		Self::validate(root).unwrap();
		Ok(Self { root: root.to_owned() })
	}

	fn validate(root: &str) -> Result<(), TypeError> {
		NNSName::validate_fragment(root, true)
	}
}
