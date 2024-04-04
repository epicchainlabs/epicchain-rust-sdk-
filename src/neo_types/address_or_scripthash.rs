// This module demonstrates the flexibility in handling blockchain addresses and script hashes, leveraging Rust's type system
// and trait implementations to provide a seamless interface for converting and working with these two fundamental types.

use std::hash::{Hash, Hasher};

use primitive_types::H160;
use serde_derive::{Deserialize, Serialize};

use neo::prelude::{Address, AddressExtension, Bytes, ScriptHashExtension};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// An enum that can represent either a blockchain `Address` or a `ScriptHash`,
/// offering flexibility for APIs that can work with either.
pub enum AddressOrScriptHash {
	/// An address type
	Address(Address),
	/// A bytes type
	ScriptHash(H160),
}

impl Hash for AddressOrScriptHash {
	/// Implements the `Hash` trait to allow `AddressOrScriptHash`
	/// instances to be used as keys in hash maps or elements in hash sets.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashSet;
	/// use neo_rs::prelude::AddressOrScriptHash;
	/// let mut set = HashSet::new();
	/// set.insert(AddressOrScriptHash::Address("myAddress".into()));
	/// ```
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			AddressOrScriptHash::Address(a) => a.hash(state),
			AddressOrScriptHash::ScriptHash(s) => s.hash(state),
		}
	}
}

impl Default for AddressOrScriptHash {
	fn default() -> Self {
		AddressOrScriptHash::Address(Default::default())
	}
}

impl From<Address> for AddressOrScriptHash {
	/// Allows creating an `AddressOrScriptHash` directly from an `Address`.
	///
	/// # Examples
	///
	/// ```
	/// use neo_rs::prelude::AddressOrScriptHash;
	/// let from_address = AddressOrScriptHash::from("myAddress".into());
	/// assert!(matches!(from_address, AddressOrScriptHash::Address(_)));
	/// ```
	fn from(s: Address) -> Self {
		Self::Address(s)
	}
}

impl From<Bytes> for AddressOrScriptHash {
	/// Allows creating an `AddressOrScriptHash` from a `Bytes` array, automatically converting it into a `ScriptHash`.
	///
	/// # Examples
	///
	/// ```
	/// use neo_rs::prelude::{AddressOrScriptHash, Bytes};
	/// let bytes: Bytes = vec![0xdeu8, 0xadu8, 0xbeu8, 0xefu8];
	/// let from_bytes = AddressOrScriptHash::from(bytes);
	/// assert!(matches!(from_bytes, AddressOrScriptHash::ScriptHash(_)));
	/// ```
	fn from(s: Bytes) -> Self {
		Self::ScriptHash(H160::from_slice(&s))
	}
}

impl AddressOrScriptHash {
	/// Retrieves the `Address` representation. If the instance is a `ScriptHash`, converts it to an `Address`.
	///
	/// # Examples
	///
	/// ```
	/// use primitive_types::H160;
	/// use neo_rs::prelude::AddressOrScriptHash;
	/// let script_hash = AddressOrScriptHash::ScriptHash(H160::repeat_byte(0x01));
	/// let address = script_hash.address();
	/// assert_eq!(address, "convertedAddressFromScriptHash");
	/// ```
	pub fn address(&self) -> Address {
		match self {
			AddressOrScriptHash::Address(a) => a.clone(),
			AddressOrScriptHash::ScriptHash(s) => s.to_address(),
		}
	}

	/// Retrieves the `ScriptHash` representation. If the instance is an `Address`, converts it to a `ScriptHash`.
	///
	/// # Examples
	///
	/// ```
	/// use primitive_types::H160;
	/// use neo_rs::prelude::AddressOrScriptHash;
	/// let address = AddressOrScriptHash::Address("myAddress".into());
	/// let script_hash = address.script_hash();
	/// assert_eq!(script_hash, H160::repeat_byte(0x02)); // Assuming `to_address` converts an address into a specific script hash
	/// ```
	pub fn script_hash(&self) -> H160 {
		match self {
			AddressOrScriptHash::Address(a) => a.address_to_script_hash().unwrap(),
			AddressOrScriptHash::ScriptHash(s) => s.clone(),
		}
	}
}
