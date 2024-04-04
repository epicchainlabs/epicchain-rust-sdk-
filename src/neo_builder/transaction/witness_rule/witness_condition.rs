use std::hash::{Hash, Hasher};

use primitive_types::H160;
use rustc_serialize::hex::{FromHex, ToHex};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use neo::prelude::{
	Decoder, Encoder, NeoSerializable, ScriptHashExtension, Secp256r1PublicKey, TransactionError,
};

/// Enum representing the different types of witness conditions that can be used in a smart contract.
#[derive(Clone, Debug, PartialEq)]
pub enum WitnessCondition {
	/// Boolean value.
	Boolean(bool),
	/// Not operator.
	Not(Box<WitnessCondition>),
	/// And operator.
	And(Vec<WitnessCondition>),
	/// Or operator.
	Or(Vec<WitnessCondition>),
	/// Script hash.
	ScriptHash(H160),
	/// Public key group.
	Group(Secp256r1PublicKey),
	/// Called by entry.
	CalledByEntry,
	/// Called by contract.
	CalledByContract(H160),
	/// Called by public key group.
	CalledByGroup(Secp256r1PublicKey),
}

impl Serialize for WitnessCondition {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("WitnessCondition", 2)?;
		match *self {
			WitnessCondition::Boolean(b) => {
				state.serialize_field("type", "Boolean")?;
				state.serialize_field("expression", &b)?;
			},
			WitnessCondition::Not(ref condition) => {
				state.serialize_field("type", "Not")?;
				state.serialize_field("expression", condition)?;
			},
			WitnessCondition::And(ref conditions) | WitnessCondition::Or(ref conditions) => {
				state.serialize_field(
					"type",
					if matches!(self, WitnessCondition::And(_)) { "And" } else { "Or" },
				)?;
				state.serialize_field("expressions", conditions)?;
			},
			WitnessCondition::ScriptHash(ref hash) => {
				state.serialize_field("type", "ScriptHash")?;
				state.serialize_field("hash", &hash.to_hex())?;
			},
			WitnessCondition::Group(ref key) | WitnessCondition::CalledByGroup(ref key) => {
				state.serialize_field(
					"type",
					if matches!(self, WitnessCondition::Group(_)) {
						"Group"
					} else {
						"CalledByGroup"
					},
				)?;
				state.serialize_field("group", &key.get_encoded(true).to_hex())?;
			},
			WitnessCondition::CalledByEntry => {
				state.serialize_field("type", "CalledByEntry")?;
			},
			WitnessCondition::CalledByContract(ref hash) => {
				state.serialize_field("type", "CalledByContract")?;
				state.serialize_field("hash", &hash.to_string())?;
			},
			WitnessCondition::CalledByGroup(ref key) => {
				state.serialize_field("type", "CalledByGroup")?;
				state.serialize_field("group", &key.to_string())?;
			},
		}
		state.end()
	}
}

fn deserialize_witness_condition<'de, D>(deserializer: D) -> Result<WitnessCondition, D::Error>
where
	D: Deserializer<'de>,
{
	let v = Value::deserialize(deserializer)?;
	match v["type"].as_str() {
		Some("Boolean") => {
			let expression = v["expression"]
				.as_str()
				.ok_or(serde::de::Error::custom("Expected a boolean for Boolean type"))?;
			Ok(WitnessCondition::Boolean(if expression == "true" { true } else { false }))
		},
		Some("Not") => {
			let expression = serde_json::from_value(v["expression"].clone())
				.map_err(|e| serde::de::Error::custom(format!("Not parsing failed: {}", e)))?;
			Ok(WitnessCondition::Not(Box::new(expression)))
		},
		Some("And") | Some("Or") => {
			let expressions = serde_json::from_value(v["expressions"].clone())
				.map_err(|e| serde::de::Error::custom(format!("And/Or parsing failed: {}", e)))?;
			if v["type"] == "And" {
				Ok(WitnessCondition::And(expressions))
			} else {
				Ok(WitnessCondition::Or(expressions))
			}
		},
		Some("ScriptHash") => {
			let hash = v["hash"]
				.as_str()
				.ok_or(serde::de::Error::custom("Expected a string for ScriptHash"))?;
			Ok(WitnessCondition::ScriptHash(H160::from_slice(&hash.from_hex().unwrap())))
		},
		Some("Group") | Some("CalledByGroup") => {
			let group = v["group"]
				.as_str()
				.ok_or(serde::de::Error::custom("Expected a string for Group/CalledByGroup"))?;
			let condition = if v["type"] == "Group" {
				WitnessCondition::Group(
					Secp256r1PublicKey::from_bytes(&group.from_hex().unwrap()).unwrap(),
				)
			} else {
				WitnessCondition::CalledByGroup(
					Secp256r1PublicKey::from_bytes(&group.from_hex().unwrap()).unwrap(),
				)
			};
			Ok(condition)
		},
		Some("CalledByEntry") => Ok(WitnessCondition::CalledByEntry),
		Some("CalledByContract") => {
			let hash = v["hash"]
				.as_str()
				.ok_or(serde::de::Error::custom("Expected a string for CalledByContract"))?;
			Ok(WitnessCondition::CalledByContract(H160::from_slice(&hash.from_hex().unwrap())))
		},
		_ => Err(serde::de::Error::custom("Unknown WitnessCondition type")),
	}
}

impl<'de> Deserialize<'de> for WitnessCondition {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserialize_witness_condition(deserializer)
	}
}

impl Hash for WitnessCondition {
	/// Hashes the witness condition.
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			WitnessCondition::Boolean(b) => b.hash(state),
			WitnessCondition::Not(exp) => exp.hash(state),
			WitnessCondition::And(exp) => exp.hash(state),
			WitnessCondition::Or(exp) => exp.hash(state),
			WitnessCondition::ScriptHash(hash) => hash.hash(state),
			WitnessCondition::Group(group) => group.get_encoded(true).hash(state),
			WitnessCondition::CalledByEntry => WitnessCondition::CalledByEntry.hash(state),
			WitnessCondition::CalledByContract(hash) => hash.hash(state),
			WitnessCondition::CalledByGroup(group) => group.get_encoded(true).hash(state),
		}
	}
}

impl WitnessCondition {
	/// Maximum number of subitems.
	const MAX_SUBITEMS: usize = 16;
	/// Maximum nesting depth.
	pub(crate) const MAX_NESTING_DEPTH: usize = 2;

	/// Boolean value string.
	const BOOLEAN_VALUE: &'static str = "Boolean";
	/// Not operator string.
	const NOT_VALUE: &'static str = "Not";
	/// And operator string.
	const AND_VALUE: &'static str = "And";
	/// Or operator string.
	const OR_VALUE: &'static str = "Or";
	/// Script hash string.
	const SCRIPT_HASH_VALUE: &'static str = "ScriptHash";
	/// Public key group string.
	const GROUP_VALUE: &'static str = "Group";
	/// Called by entry string.
	const CALLED_BY_ENTRY_VALUE: &'static str = "CalledByEntry";
	/// Called by contract string.
	const CALLED_BY_CONTRACT_VALUE: &'static str = "CalledByContract";
	/// Called by public key group string.
	const CALLED_BY_GROUP_VALUE: &'static str = "CalledByGroup";

	/// Boolean byte value.
	const BOOLEAN_BYTE: u8 = 0x00;
	/// Not operator byte value.
	const NOT_BYTE: u8 = 0x01;
	/// And operator byte value.
	const AND_BYTE: u8 = 0x02;
	/// Or operator byte value.
	const OR_BYTE: u8 = 0x03;
	/// Script hash byte value.
	const SCRIPT_HASH_BYTE: u8 = 0x18;
	/// Public key group byte value.
	const GROUP_BYTE: u8 = 0x19;
	/// Called by entry byte value.
	const CALLED_BY_ENTRY_BYTE: u8 = 0x20;
	/// Called by contract byte value.
	const CALLED_BY_CONTRACT_BYTE: u8 = 0x28;
	/// Called by public key group byte value.
	const CALLED_BY_GROUP_BYTE: u8 = 0x29;

	/// Returns the JSON value of the witness condition.
	pub fn json_value(&self) -> &'static str {
		match self {
			WitnessCondition::Boolean(_) => WitnessCondition::BOOLEAN_VALUE,
			WitnessCondition::Not(_) => WitnessCondition::NOT_VALUE,
			WitnessCondition::And(_) => WitnessCondition::AND_VALUE,
			WitnessCondition::Or(_) => WitnessCondition::OR_VALUE,
			WitnessCondition::ScriptHash(_) => WitnessCondition::SCRIPT_HASH_VALUE,
			WitnessCondition::Group(_) => WitnessCondition::GROUP_VALUE,
			WitnessCondition::CalledByEntry => WitnessCondition::CALLED_BY_ENTRY_VALUE,
			WitnessCondition::CalledByContract(_) => WitnessCondition::CALLED_BY_CONTRACT_VALUE,
			WitnessCondition::CalledByGroup(_) => WitnessCondition::CALLED_BY_GROUP_VALUE,
		}
	}

	/// Returns the byte value of the witness condition.
	pub fn byte(&self) -> u8 {
		match self {
			WitnessCondition::Boolean(_) => WitnessCondition::BOOLEAN_BYTE,
			WitnessCondition::Not(_) => WitnessCondition::NOT_BYTE,
			WitnessCondition::And(_) => WitnessCondition::AND_BYTE,
			WitnessCondition::Or(_) => WitnessCondition::OR_BYTE,
			WitnessCondition::ScriptHash(_) => WitnessCondition::SCRIPT_HASH_BYTE,
			WitnessCondition::Group(_) => WitnessCondition::GROUP_BYTE,
			WitnessCondition::CalledByEntry => WitnessCondition::CALLED_BY_ENTRY_BYTE,
			WitnessCondition::CalledByContract(_) => WitnessCondition::CALLED_BY_CONTRACT_BYTE,
			WitnessCondition::CalledByGroup(_) => WitnessCondition::CALLED_BY_GROUP_BYTE,
		}
	}

	/// Returns the boolean expression of the witness condition.
	pub fn boolean_expression(&self) -> Option<bool> {
		match self {
			WitnessCondition::Boolean(b) => Some(*b),
			_ => None,
		}
	}

	/// Returns the expression of the witness condition.
	pub fn expression(&self) -> Option<&WitnessCondition> {
		match self {
			WitnessCondition::Not(exp) => Some(&exp),
			_ => None,
		}
	}

	/// Returns the expression list of the witness condition.
	pub fn expression_list(&self) -> Option<&[WitnessCondition]> {
		match self {
			WitnessCondition::And(exp) | WitnessCondition::Or(exp) => Some(&exp),
			_ => None,
		}
	}

	/// Returns the script hash of the witness condition.
	pub fn script_hash(&self) -> Option<&H160> {
		match self {
			WitnessCondition::ScriptHash(hash) | WitnessCondition::CalledByContract(hash) =>
				Some(hash),
			_ => None,
		}
	}

	/// Returns the public key group of the witness condition.
	pub fn group(&self) -> Option<&Secp256r1PublicKey> {
		match self {
			WitnessCondition::Group(group) | WitnessCondition::CalledByGroup(group) => Some(group),
			_ => None,
		}
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<WitnessCondition, TransactionError> {
		let mut reader = Decoder::new(bytes);
		WitnessCondition::decode(&mut reader)
	}
}

impl NeoSerializable for WitnessCondition {
	type Error = TransactionError;

	fn size(&self) -> usize {
		match self {
			WitnessCondition::Boolean(_) => 2,
			WitnessCondition::Not(_) => 1 + self.expression().unwrap().size(),
			WitnessCondition::And(_) | WitnessCondition::Or(_) => {
				let exp = self.expression_list().unwrap();
				1 + 1 + exp.len() + exp.iter().map(|e| e.size()).sum::<usize>()
			},
			WitnessCondition::ScriptHash(_) | WitnessCondition::CalledByContract(_) => 1 + 20,
			WitnessCondition::Group(_) | WitnessCondition::CalledByGroup(_) => 1 + 33,
			WitnessCondition::CalledByEntry => 1,
		}
	}

	fn encode(&self, writer: &mut Encoder) {
		match self {
			WitnessCondition::Boolean(b) => {
				writer.write_u8(WitnessCondition::BOOLEAN_BYTE);
				writer.write_bool(*b);
			},
			WitnessCondition::Not(exp) => {
				writer.write_u8(WitnessCondition::NOT_BYTE);
				writer.write_serializable_fixed(exp.expression().unwrap());
			},
			WitnessCondition::And(exp) => {
				writer.write_u8(WitnessCondition::AND_BYTE);
				writer.write_serializable_variable_list(exp);
			},
			WitnessCondition::Or(exp) => {
				writer.write_u8(WitnessCondition::OR_BYTE);
				writer.write_serializable_variable_list(exp)
			},
			WitnessCondition::ScriptHash(hash) => {
				writer.write_u8(WitnessCondition::SCRIPT_HASH_BYTE);
				writer.write_serializable_fixed(hash);
			},
			WitnessCondition::Group(group) => {
				writer.write_u8(WitnessCondition::GROUP_BYTE);
				writer.write_serializable_fixed(group);
			},
			WitnessCondition::CalledByEntry => {
				writer.write_u8(WitnessCondition::CALLED_BY_ENTRY_BYTE);
			},
			WitnessCondition::CalledByContract(hash) => {
				writer.write_u8(WitnessCondition::CALLED_BY_CONTRACT_BYTE);
				writer.write_serializable_fixed(hash);
			},
			WitnessCondition::CalledByGroup(group) => {
				writer.write_u8(WitnessCondition::CALLED_BY_GROUP_BYTE);
				writer.write_serializable_fixed(group);
			},
		}
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		let byte = reader.read_u8();
		match byte {
			WitnessCondition::BOOLEAN_BYTE => {
				let b = reader.read_bool();
				Ok(WitnessCondition::Boolean(b))
			},
			WitnessCondition::NOT_BYTE => {
				let exp = WitnessCondition::decode(reader)?;
				Ok(WitnessCondition::Not(Box::from(exp)))
			},
			WitnessCondition::OR_BYTE | WitnessCondition::AND_BYTE => {
				let len = reader.read_var_int()? as usize;
				if len > Self::MAX_SUBITEMS {
					return Err(TransactionError::InvalidWitnessCondition)
				}
				let mut expressions = Vec::with_capacity(len);
				for _ in 0..len {
					expressions.push(WitnessCondition::decode(reader)?);
				}
				if byte == Self::OR_BYTE {
					Ok(WitnessCondition::Or(expressions))
				} else {
					Ok(WitnessCondition::And(expressions))
				}
			},
			WitnessCondition::SCRIPT_HASH_BYTE | WitnessCondition::CALLED_BY_CONTRACT_BYTE => {
				let hash = H160::decode(reader)?;
				if byte == WitnessCondition::SCRIPT_HASH_BYTE {
					Ok(WitnessCondition::ScriptHash(hash))
				} else {
					Ok(WitnessCondition::CalledByContract(hash))
				}
			},
			WitnessCondition::GROUP_BYTE | WitnessCondition::CALLED_BY_GROUP_BYTE => {
				let group = Secp256r1PublicKey::decode(reader)?;
				if byte == WitnessCondition::GROUP_BYTE {
					Ok(WitnessCondition::Group(group))
				} else {
					Ok(WitnessCondition::CalledByGroup(group))
				}
			},
			WitnessCondition::CALLED_BY_ENTRY_BYTE => Ok(WitnessCondition::CalledByEntry),
			_ => Err(TransactionError::InvalidTransaction),
		}
	}

	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}
