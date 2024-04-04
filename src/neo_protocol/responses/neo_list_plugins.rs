use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Plugin {
	pub name: String,
	pub version: String,
	pub interfaces: Vec<String>,
}
