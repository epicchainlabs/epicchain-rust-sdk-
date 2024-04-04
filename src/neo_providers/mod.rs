#![feature(inherent_associated_types)]
#![doc = include_str!("../../README.md")]
#![allow(clippy::type_complexity)]
#![warn(missing_docs)]
#![deny(unsafe_code, rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use lazy_static::lazy_static;

pub use errors::{ProviderError, RpcError};
pub use ext::*;
use neo::prelude::NeoConstants;
pub use rpc::*;
#[allow(deprecated)]
pub use test_provider::{MAINNET, TESTNET};
pub use utils::*;

/// Errors
mod errors;
mod ext;
mod middleware;
pub use middleware::*;
mod rpc;
/// Crate utilities and type aliases
mod utils;

lazy_static! {
	pub static ref HTTP_PROVIDER: Provider<Http> = Provider::<Http>::try_from(
		std::env::var("ENDPOINT").unwrap_or_else(|_| NeoConstants::SEED_1.to_string())
	)
	.unwrap();
}

#[allow(missing_docs)]
/// Pre-instantiated Infura HTTP clients which rotate through multiple API keys
/// to prevent rate limits
mod test_provider {
	use std::{convert::TryFrom, iter::Cycle, slice::Iter, sync::Mutex};

	use once_cell::sync::Lazy;

	use super::*;

	// List of infura keys to rotate through so we don't get rate limited
	const INFURA_KEYS: &[&str] = &["15e8aaed6f894d63a0f6a0206c006cdd"];

	pub static MAINNET: Lazy<TestProvider> =
		Lazy::new(|| TestProvider::new(INFURA_KEYS, "mainnet"));

	pub static TESTNET: Lazy<TestProvider> =
		Lazy::new(|| TestProvider::new(INFURA_KEYS, "testnet"));

	#[derive(Debug)]
	pub struct TestProvider {
		network: String,
		keys: Mutex<Cycle<Iter<'static, &'static str>>>,
	}

	impl TestProvider {
		pub fn new(keys: &'static [&'static str], network: impl Into<String>) -> Self {
			Self { keys: keys.iter().cycle().into(), network: network.into() }
		}

		pub fn url(&self) -> String {
			let Self { network, keys } = self;
			let key = keys.lock().unwrap().next().unwrap();
			format!("https://{network}.infura.io/v3/{key}")
		}

		pub fn provider(&self) -> Provider<Http> {
			Provider::try_from(self.url().as_str()).unwrap()
		}

		#[cfg(feature = "ws")]
		pub async fn ws(&self) -> Provider<crate::Ws> {
			let url = format!(
				"wss://{}.infura.neo.io/ws/v3/{}",
				self.network,
				self.keys.lock().unwrap().next().unwrap()
			);
			Provider::connect(url.as_str()).await.unwrap()
		}
	}
}
