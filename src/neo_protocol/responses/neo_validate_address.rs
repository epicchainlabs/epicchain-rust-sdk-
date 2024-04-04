use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidateAddress {
	pub address: String,
	pub is_valid: bool,
}

impl ValidateAddress {
	pub fn new(address: String, is_valid: bool) -> Self {
		Self { address, is_valid }
	}
}
