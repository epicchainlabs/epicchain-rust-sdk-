use hex::FromHexError;
use thiserror::Error;

use neo::prelude::{BuilderError, CryptoError, TypeError, WalletError};

/// Represents errors that can occur within the signing process.
///
/// This enum encapsulates a variety of errors that may arise when performing
/// operations related to signing transactions, messages, or performing cryptographic
/// operations within a blockchain context. It includes errors related to invalid input
/// data (like passphrases or addresses), failures in underlying cryptographic operations,
/// and errors from external libraries used for tasks such as hex encoding/decoding or
/// type conversion.
///
/// Variants of this enum are designed to be converted from errors thrown by these
/// operations, allowing for a unified error handling interface throughout the signing
/// process.
///
/// # Variants
///
/// - `InvalidPassphrase`: Occurs when a passphrase does not meet validation criteria.
/// - `InvalidAddress`: Triggered by malformed blockchain addresses.
/// - `BuilderError`: Wraps errors from the construction of cryptographic objects.
/// - `WalletError`: Encompasses errors from wallet operations.
/// - `FromHexError`: Pertains to failures in hexadecimal to binary data conversion.
/// - `CryptoError`: Covers general cryptographic failures.
/// - `RustcFromHexError`: Specific to hex decoding issues via `rustc_serialize`.
/// - `TypeError`: Indicates failures in type conversion or coercion.
///
/// # Examples
///
/// Handling a `SignerError` in a function that performs signing operations might look
/// like this:
///
/// ```
/// use neo_rs::prelude::SignerError;
///  async fn sign_data(data: &[u8]) -> Result<(), SignerError> {
///     // Example function body
///     Ok(())
/// }
///
/// async fn example_usage() {
///     let data = b"example data";
///
///     match sign_data(data).await {
///         Ok(_) => println!("Data signed successfully"),
///         Err(e) => match e {
///             SignerError::InvalidPassphrase(_) => println!("Invalid passphrase provided"),
///             SignerError::InvalidAddress => println!("Invalid address"),
///             // Handle other errors accordingly
///             _ => println!("An error occurred: {:?}", e),
///         },
///     }
/// }
/// ```
#[derive(Debug, Error)]
pub enum SignerError {
	/// Represents an error when an invalid passphrase is provided.
	/// This could happen during the decryption of a private key or any operation
	/// that requires passphrase verification.
	#[error("Invalid passphrase: {0}")]
	InvalidPassphrase(String),

	/// Indicates that the provided address is not valid.
	/// This error might occur if an address does not comply with expected formats or checksums.
	#[error("Invalid address")]
	InvalidAddress,

	/// Wraps errors related to building or configuring objects, possibly during
	/// the setup of cryptographic operations or when constructing complex objects
	/// that have specific requirements.
	#[error(transparent)]
	BuilderError(#[from] BuilderError),

	/// Encapsulates errors that originate from wallet operations.
	/// This can include issues with creating, loading, or performing transactions with wallets.
	#[error(transparent)]
	WalletError(#[from] WalletError),

	/// Represents errors that occur when converting from hexadecimal strings to binary data.
	/// This could be used when parsing keys, addresses, or any other data represented in hex format.
	#[error(transparent)]
	FromHexError(#[from] FromHexError),

	/// Covers general cryptographic errors such as failures in hashing, signature generation,
	/// or encryption/decryption processes.
	#[error(transparent)]
	CryptoError(#[from] CryptoError),

	/// Error that occurs when decoding from hexadecimal representation fails using
	/// `rustc_serialize` library. It specifically indicates a problem with hex decoding,
	/// likely due to an invalid character or incorrect string length.
	#[error(transparent)]
	RustcFromHexError(#[from] rustc_serialize::hex::FromHexError),

	/// Indicates a failure related to type conversion or coercion.
	/// This variant is useful for signaling issues when trying to convert between incompatible types,
	/// such as when deserializing data into a specific structure.
	#[error(transparent)]
	TypeError(#[from] TypeError),
}
