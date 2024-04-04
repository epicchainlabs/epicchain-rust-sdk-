use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Balance {
	#[serde(alias = "Balance")]
	pub balance: String,
}
