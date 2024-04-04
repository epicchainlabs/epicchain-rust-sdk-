use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CryptoError {
	#[error("Invalid passphrase: {0}")]
	InvalidPassphrase(String),
	#[error("Invalid format: {0}")]
	InvalidFormat(String),
	#[error("invalid signature length, got {0}, expected 65")]
	HeaderOutOfRange(u8),
	#[error("Could not recover public key from signature")]
	RecoverFailed,
	#[error("Invalid public key")]
	InvalidPublicKey,
	#[error("Invalid private key")]
	InvalidPrivateKey,
	#[error("Invalid private key")]
	P256Error(#[from] p256::elliptic_curve::Error),
	#[error("Signing error")]
	SigningError,
	#[error("Signature verification error")]
	SignatureVerificationError,
	#[error(transparent)]
	FromHexError(#[from] hex::FromHexError),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Nep2Error {
	#[error("Invalid passphrase: {0}")]
	InvalidPassphrase(String),
	#[error("Invalid format: {0}")]
	InvalidFormat(String),
}

#[derive(Error, Debug, PartialEq, Eq, Hash, Clone)]
pub enum SignError {
	#[error("Header byte out of range: {0}")]
	HeaderOutOfRange(u8),
	#[error("Could not recover public key from signature")]
	RecoverFailed,
}
