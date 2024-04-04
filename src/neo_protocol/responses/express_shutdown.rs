use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExpressShutdown {
	#[serde(rename = "process-id")]
	process_id: i32,
}

impl ExpressShutdown {
	pub fn new(process_id: i32) -> Self {
		Self { process_id }
	}
}
