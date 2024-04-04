use thiserror::Error;

use neo::prelude::{CodecError, CryptoError, ProviderError};

#[derive(Error, Debug, PartialEq)]
pub enum TransactionError {
	#[error("Script format error: {0}")]
	ScriptFormat(String),
	#[error("Signer configuration error: {0}")]
	SignerConfiguration(String),
	#[error("Invalid nonce")]
	InvalidNonce,
	#[error("Invalid block")]
	InvalidBlock,
	#[error("Invalid transaction")]
	InvalidTransaction,
	#[error("Invalid witness condition")]
	InvalidWitnessCondition,
	#[error("Too many signers")]
	TooManySigners,
	#[error("Duplicate signer")]
	DuplicateSigner,
	#[error("No signers")]
	NoSigners,
	#[error("No script")]
	NoScript,
	#[error("Empty script")]
	EmptyScript,
	#[error("Invalid sender")]
	InvalidSender,
	#[error("Invalid state:{0}")]
	IllegalState(String),
	#[error("Transaction too large")]
	TxTooLarge,
	#[error("Transaction configuration error: {0}")]
	TransactionConfiguration(String),
	#[error("Codec error: {0}")]
	CodecError(#[from] CodecError),
	#[error("Crypto error: {0}")]
	CryptoError(#[from] CryptoError),
	#[error(transparent)]
	ProviderError(#[from] ProviderError),
}
