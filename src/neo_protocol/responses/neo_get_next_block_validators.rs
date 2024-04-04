use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Validator {
	#[serde(rename = "publickey")]
	pub public_key: String,
	pub votes: String,
	pub active: bool,
}
