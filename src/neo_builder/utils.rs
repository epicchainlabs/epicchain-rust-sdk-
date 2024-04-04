use serde_json::Value;

use neo::prelude::*;

// pub type ScriptHash = H160;

/// Converts a list of public keys to a script hash using a given threshold.
///
/// # Arguments
///
/// * `public_keys` - A mutable slice of `Secp256r1PublicKey` instances.
/// * `threshold` - The minimum number of signatures required to validate the transaction.
///
/// # Returns
///
/// A `ScriptHash` instance representing the script hash of the MultiSig script.
pub fn public_keys_to_scripthash(
	public_keys: &mut [Secp256r1PublicKey],
	threshold: usize,
) -> ScriptHash {
	let script = ScriptBuilder::build_multi_sig_script(public_keys, threshold as u8).unwrap();
	// Self::from_script(&script)
	ScriptHash::from_slice(&script)
}

/// Converts a public key to a script hash.
///
/// # Arguments
///
/// * `public_key` - A `Secp256r1PublicKey` instance.
///
/// # Returns
///
/// A `ScriptHash` instance representing the script hash of the verification script.
pub fn pubkey_to_scripthash(public_key: &Secp256r1PublicKey) -> ScriptHash {
	let script = ScriptBuilder::build_verification_script(public_key);
	ScriptHash::from_script(&script)
}

pub trait VecValueExtension {
	fn to_value(&self) -> Value;
}

impl ValueExtension for TransactionAttribute {
	fn to_value(&self) -> Value {
		Value::String(self.to_json())
	}
}

impl ValueExtension for TransactionSendToken {
	fn to_value(&self) -> Value {
		Value::String(serde_json::to_string(self).unwrap())
	}
}

impl VecValueExtension for Vec<TransactionSendToken> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

impl VecValueExtension for Vec<TransactionAttribute> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}
impl ValueExtension for Signer {
	fn to_value(&self) -> Value {
		Value::String(serde_json::to_string(self).unwrap())
	}
}

impl VecValueExtension for Vec<Signer> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

impl ValueExtension for TransactionSigner {
	fn to_value(&self) -> Value {
		Value::String(serde_json::to_string(self).unwrap())
	}
}

impl VecValueExtension for Vec<TransactionSigner> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}
