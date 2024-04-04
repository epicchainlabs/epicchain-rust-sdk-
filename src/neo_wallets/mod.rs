#![doc = include_str!("../../README.md")]
#![deny(unsafe_code, rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "yubihsm", not(target_arch = "wasm32")))]
pub use yubihsm;

pub use error::*;
#[cfg(all(feature = "ledger", not(target_arch = "wasm32")))]
pub use ledger::{
	app::LedgerNeo as Ledger,
	types::{DerivationType as HDPath, LedgerError},
};
use neo::prelude::Account;
pub use wallet::*;
pub use wallet_signer::WalletSigner;
pub use wallet_trait::WalletTrait;

mod wallet;
mod wallet_trait;

/// A wallet instantiated with a locally stored private key
pub type LocalSigner = WalletSigner<Account>;

#[cfg(all(feature = "yubihsm", not(target_arch = "wasm32")))]
/// A wallet instantiated with a YubiHSM
pub type YubiWallet = WalletSigner<yubihsm::ecdsa::Signer<NistP256>>;

#[cfg(all(feature = "ledger", not(target_arch = "wasm32")))]
mod ledger;
#[cfg(all(feature = "yubihsm", not(target_arch = "wasm32")))]
mod yubi;

mod error;
mod wallet_signer;
