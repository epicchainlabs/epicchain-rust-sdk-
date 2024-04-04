use std::{
	hash::{Hash, Hasher},
	sync::{Arc, Mutex},
};

use primitive_types::H160;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NeoNetwork {
	MainNet = 0x00746e41,
	TestNet = 0x74746e41,
	PrivateNet = 0x4e454e,
}

impl NeoNetwork {
	pub fn to_magic(&self) -> u32 {
		match self {
			NeoNetwork::MainNet => 0x00746e41,
			NeoNetwork::TestNet => 0x74746e41,
			NeoNetwork::PrivateNet => 0x4e454e,
		}
	}
	pub fn from_magic(magic: u32) -> Option<NeoNetwork> {
		match magic {
			0x00746e41 => Some(NeoNetwork::MainNet),
			0x74746e41 => Some(NeoNetwork::TestNet),
			0x4e454e => Some(NeoNetwork::PrivateNet),
			_ => None,
		}
	}
}

pub const DEFAULT_BLOCK_TIME: u64 = 15_000;
pub const DEFAULT_ADDRESS_VERSION: u8 = 0x35;
pub const MAX_VALID_UNTIL_BLOCK_INCREMENT_BASE: u64 = 86_400_000;

#[derive(Clone, Debug)]
pub struct NeoConfig {
	pub network: Option<u32>,
	pub block_interval: u32,
	pub max_valid_until_block_increment: u32,
	pub polling_interval: u32,
	pub executor: Arc<Mutex<Handle>>,
	pub allows_transmission_on_fault: bool,
	pub nns_resolver: H160,
}

impl Hash for NeoConfig {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.network.hash(state);
		self.block_interval.hash(state);
		self.max_valid_until_block_increment.hash(state);
		self.polling_interval.hash(state);
		self.allows_transmission_on_fault.hash(state);
		self.nns_resolver.hash(state);
	}
}

impl Default for NeoConfig {
	fn default() -> Self {
		NeoConfig {
			network: None,
			block_interval: DEFAULT_BLOCK_TIME as u32,
			max_valid_until_block_increment: (MAX_VALID_UNTIL_BLOCK_INCREMENT_BASE
				/ DEFAULT_BLOCK_TIME) as u32,
			polling_interval: DEFAULT_BLOCK_TIME as u32,
			executor: Arc::new(Mutex::new(tokio::runtime::Handle::current())),
			allows_transmission_on_fault: false,
			nns_resolver: H160::from_slice(
				[
					0x50, 0xac, 0x1c, 0x37, 0x69, 0x0c, 0xc2, 0xc5, 0x8f, 0xc5, 0x94, 0x47, 0x28,
					0x33, 0xcf, 0x57, 0x50, 0x5d, 0x5f, 0x46,
				]
				.as_slice(),
			),
		}
	}
}

impl NeoConfig {
	// constructor
	pub fn new(
		network: Option<u32>,
		block_interval: u32,
		max_valid_until_block_increment: u32,
		polling_interval: u32,
		scheduled_executor_service: Arc<Mutex<Handle>>,
		allows_transmission_on_fault: bool,
		nns_resolver: [u8; 20],
	) -> Self {
		NeoConfig {
			network,
			block_interval,
			max_valid_until_block_increment,
			polling_interval,
			executor: scheduled_executor_service,
			allows_transmission_on_fault,
			nns_resolver: H160::from_slice(nns_resolver.as_slice()),
		}
	}

	// setters
	pub fn set_polling_interval(&mut self, interval: u32) {
		self.polling_interval = interval;
	}

	pub fn set_executor(&mut self, executor: Arc<Mutex<Handle>>) {
		self.executor = executor;
	}

	pub fn set_network(&mut self, magic: u32) -> Result<(), &'static str> {
		if &magic > &0xFFFFFFFFu32 {
			return Err("Network magic must fit in 32 bits")
		}

		self.network = Some(magic);
		Ok(())
	}

	// other methods
}

#[derive(Clone, Debug)]
pub struct Counter {
	count: Arc<Mutex<u32>>,
}

impl Hash for Counter {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.count.lock().unwrap().hash(state);
	}
}

impl PartialEq for Counter {
	fn eq(&self, other: &Self) -> bool {
		*self.count.lock().unwrap() == *other.count.lock().unwrap()
	}
}

impl Counter {
	pub fn new() -> Self {
		Counter { count: Arc::new(Mutex::new(1)) }
	}

	pub fn get_and_increment(&self) -> u32 {
		let mut count = self.count.lock().unwrap();
		let v: u32 = *count;
		*count += 1;
		v
	}
}
