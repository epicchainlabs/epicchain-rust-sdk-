use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnclaimedGas {
	pub unclaimed: String,
	pub address: String,
}
