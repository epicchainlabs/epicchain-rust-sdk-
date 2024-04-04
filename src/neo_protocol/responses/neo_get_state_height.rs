use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StateHeight {
	#[serde(rename = "localrootindex")]
	pub local_root_index: u32,
	#[serde(rename = "validatedrootindex")]
	pub validated_root_index: u32,
}
