use neo::prelude::ProviderError;
use thiserror::Error;

/// Custom error type for contract-related errors
#[derive(Error, Debug)]
pub enum ContractError {
	/// Error indicating an invalid Neo name
	#[error("Invalid NNS name {0}")]
	InvalidNeoName(String),
	/// Error indicating an invalid Neo Name Service root
	#[error("Invalid NNS root {0}")]
	InvalidNeoNameServiceRoot(String),
	/// Error indicating an unexpected return type
	#[error("Unexpected return type {0}")]
	UnexpectedReturnType(String),
	/// Error indicating an unresolvable domain name
	#[error("Unresolvable domain name {0}")]
	UnresolvableDomainName(String),
	/// Error indicating that a domain name is not available
	#[error("Domain name {0} is not available")]
	DomainNameNotAvailable(String),
	/// Error indicating that a domain name is not registered
	#[error("Domain name {0} is not registered")]
	DomainNameNotRegistered(String),
	/// Error indicating a runtime error
	#[error("Runtime error: {0}")]
	RuntimeError(String),
	/// Error indicating an invalid state error
	#[error("Invalid state error: {0}")]
	InvalidStateError(String),
	/// Error indicating an invalid argument error
	#[error("Invalid argument error: {0}")]
	InvalidArgError(String),
	/// Error indicating a provider error, transparently wrapped
	#[error(transparent)]
	ProviderError(#[from] ProviderError),
}
