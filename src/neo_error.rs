use neo::prelude::{ContractError, SignError, TransactionError, WalletError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NeoError {
	#[error("Illegal argument: {0}")]
	IllegalArgument(String),
	#[error("Illegal state: {0}")]
	Deserialization(String),
	#[error("Illegal state: {0}")]
	IllegalState(String),
	#[error("Index out of bounds: {0}")]
	IndexOutOfBounds(String),
	#[error("Invalid configuration: {0}")]
	InvalidConfiguration(String),
	#[error("Runtime error: {0}")]
	Runtime(String),
	#[error("Invalid data: {0}")]
	InvalidData(String),
	#[error("Unsupported operation: {0}")]
	UnsupportedOperation(String),
	#[error("Transaction error: {0}")]
	Transaction(String),
	#[error("Invalid script: {0}")]
	InvalidScript(String),
	#[error("Invalid format")]
	InvalidFormat,
	#[error("neo-rs not initialized")]
	NeoNotInitialized,
	#[error("Contract error: {0}")]
	ContractError(#[from] ContractError),
	#[error("Wallet error: {0}")]
	WalletError(#[from] WalletError),
	#[error("Sign error: {0}")]
	SignError(#[from] SignError),
	#[error("Transaction error: {0}")]
	TransactionError(#[from] TransactionError),
	#[error("Unexpected returned type")]
	UnexpectedReturnType,
	#[error("Invalid private key")]
	InvalidPrivateKey,
	#[error("Invalid public key")]
	InvalidPublicKey,
	#[error("Invalid address")]
	InvalidAddress,
	#[error("Invalid signature")]
	InvalidSignature,
	#[error("Invalid encoding {0}")]
	InvalidEncoding(String),
	#[error("Invalid op code")]
	InvalidOpCode,
	#[error("Numeric overflow")]
	NumericOverflow,
	#[error("Wif error {0}")]
	WifError(String),
}

impl Into<TransactionError> for NeoError {
	fn into(self) -> TransactionError {
		TransactionError::TransactionConfiguration(self.to_string())
	}
}
