use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Peers {
	pub connected: Vec<AddressEntry>,
	pub bad: Vec<AddressEntry>,
	pub unconnected: Vec<AddressEntry>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressEntry {
	pub address: String,
	pub port: u16,
}
