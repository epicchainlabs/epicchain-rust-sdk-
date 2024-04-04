use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NameState {
	pub(crate) name: String,
	pub(crate) expiration: Option<i64>,
	pub(crate) admin: Option<[u8; 20]>,
}

impl NameState {
	pub(crate) fn new(name: String, expiration: Option<i64>, admin: Option<[u8; 20]>) -> Self {
		Self { name, expiration, admin }
	}
}
