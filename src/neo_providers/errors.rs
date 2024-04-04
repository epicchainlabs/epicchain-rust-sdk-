use std::{error::Error, fmt::Debug};

use thiserror::Error;

use neo::{
	prelude::{CryptoError, JsonRpcError, TypeError},
	providers::middleware::MiddlewareError,
};

use crate::prelude::Middleware;

/// An `RpcError` is an abstraction over error types returned by a
/// [`crate::JsonRpcClient`].
///
/// All clients can return [`JsonRpcError`] responses, as
/// well as serde deserialization errors. However, because client errors are
/// typically type-erased via the [`ProviderError`], the error info can be
/// difficult to access. This trait provides convenient access to the
/// underlying error types.
///
/// This trait deals only with behavior that is common to all clients.
/// Client-specific errorvariants cannot be accessed via this trait.
pub trait RpcError: Error + Debug + Send + Sync {
	/// Access an underlying JSON-RPC error (if any)
	///
	/// Attempts to access an underlying [`JsonRpcError`]. If the underlying
	/// error is not a JSON-RPC error response, this function will return
	/// `None`.
	fn as_error_response(&self) -> Option<&JsonRpcError>;

	/// Returns `true` if the underlying error is a JSON-RPC error response
	fn is_error_response(&self) -> bool {
		self.as_error_response().is_some()
	}

	/// Access an underlying `serde_json` error (if any)
	///
	/// Attempts to access an underlying [`serde_json::Error`]. If the
	/// underlying error is not a serde_json error, this function will return
	/// `None`.
	///
	/// ### Implementor's Note
	///
	/// When writing a stacked [`crate::JsonRpcClient`] abstraction (e.g. a quorum
	/// provider or retrying provider), be sure to account for `serde_json`
	/// errors at your layer, as well as at lower layers.
	fn as_serde_error(&self) -> Option<&serde_json::Error>;

	/// Returns `true` if the underlying error is a serde_json (de)serialization
	/// error. This method can be used to identify
	fn is_serde_error(&self) -> bool {
		self.as_serde_error().is_some()
	}
}

#[derive(Debug, Error)]
/// An error thrown when making a call to the provider
pub enum ProviderError {
	/// An internal error in the JSON RPC Client
	#[error("{0}")]
	JsonRpcClientError(Box<dyn RpcError + Send + Sync>),
	/// An error during NNS name resolution
	#[error("nns name not found: {0}")]
	NnsError(String),
	/// Invalid reverse NNS name
	#[error("reverse nns name not pointing to itself: {0}")]
	NnsNotOwned(String),
	/// Error in underlying lib `serde_json`
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),
	/// Error in underlying lib `hex`
	#[error(transparent)]
	HexError(#[from] hex::FromHexError),
	/// Error in underlying lib `reqwest`
	#[error(transparent)]
	HTTPError(#[from] reqwest::Error),
	/// Custom error from unknown source
	#[error("custom error: {0}")]
	CustomError(String),
	/// RPC method is not supported by this provider
	#[error("unsupported RPC")]
	UnsupportedRPC,
	/// Node is not supported by this provider
	#[error("unsupported node client")]
	UnsupportedNodeClient,
	/// Signer is not available to this provider.
	#[error("Attempted to sign a transaction with no available signer. Hint: did you mean to use a SignerMiddleware?")]
	SignerUnavailable,
	#[error("Illegal state: {0}")]
	IllegalState(String),
	#[error("Invalid address")]
	InvalidAddress,
	#[error(transparent)]
	CryptoError(#[from] CryptoError),
	#[error(transparent)]
	TypeError(#[from] TypeError),
	#[error("Invalid password")]
	InvalidPassword,
}

impl PartialEq for ProviderError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(ProviderError::JsonRpcClientError(a), ProviderError::JsonRpcClientError(b)) =>
				a.as_error_response() == b.as_error_response(),
			(ProviderError::SerdeJson(a), ProviderError::SerdeJson(b)) =>
				a.to_string() == b.to_string(),
			(ProviderError::HTTPError(a), ProviderError::HTTPError(b)) => a.status() == b.status(),
			(ProviderError::CustomError(a), ProviderError::CustomError(b)) => a == b,
			(ProviderError::UnsupportedRPC, ProviderError::UnsupportedRPC) => true,
			(ProviderError::UnsupportedNodeClient, ProviderError::UnsupportedNodeClient) => true,
			(ProviderError::SignerUnavailable, ProviderError::SignerUnavailable) => true,
			(ProviderError::IllegalState(a), ProviderError::IllegalState(b)) => a == b,
			(ProviderError::InvalidAddress, ProviderError::InvalidAddress) => true,
			(ProviderError::CryptoError(a), ProviderError::CryptoError(b)) => a == b,
			(ProviderError::TypeError(a), ProviderError::TypeError(b)) => a == b,
			(ProviderError::InvalidPassword, ProviderError::InvalidPassword) => true,
			_ => false,
		}
	}
}

impl RpcError for ProviderError {
	fn as_error_response(&self) -> Option<&super::JsonRpcError> {
		if let ProviderError::JsonRpcClientError(err) = self {
			err.as_error_response()
		} else {
			None
		}
	}

	fn as_serde_error(&self) -> Option<&serde_json::Error> {
		match self {
			ProviderError::JsonRpcClientError(e) => e.as_serde_error(),
			ProviderError::SerdeJson(e) => Some(e),
			_ => None,
		}
	}
}

impl MiddlewareError for ProviderError {
	type Inner = Self;

	fn from_err(e: Self::Inner) -> Self {
		e
	}

	fn as_inner(&self) -> Option<&Self::Inner> {
		// prevents infinite loops
		None
	}

	fn as_serde_error(&self) -> Option<&serde_json::Error> {
		RpcError::as_serde_error(self)
	}

	fn as_error_response(&self) -> Option<&super::JsonRpcError> {
		RpcError::as_error_response(self)
	}
}
