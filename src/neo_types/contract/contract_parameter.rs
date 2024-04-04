use std::{
	collections::HashMap,
	hash::{Hash, Hasher},
};

use primitive_types::{H160, H256};
use rustc_serialize::{
	base64::FromBase64,
	hex::{FromHex, ToHex},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha3::Digest;
use strum_macros::{Display, EnumString};

use neo::prelude::{
	deserialize_map, serialize_map, Base64Encode, ContractParameterType, NNSName, NefFile,
	NeoSerializable, Role, ScriptHashExtension, Secp256r1PublicKey, ValueExtension,
};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct ContractParameter {
	#[serde(skip_serializing_if = "Option::is_none")]
	name: Option<String>,
	#[serde(rename = "type")]
	typ: ContractParameterType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<ParameterValue>,
}

impl From<&H160> for ContractParameter {
	fn from(value: &H160) -> Self {
		Self::H160(value)
	}
}

impl From<H160> for ContractParameter {
	fn from(value: H160) -> Self {
		Self::H160(&value)
	}
}

impl From<u8> for ContractParameter {
	fn from(value: u8) -> Self {
		Self::integer(value as i64)
	}
}

impl From<i32> for ContractParameter {
	fn from(value: i32) -> Self {
		Self::integer(value as i64)
	}
}

impl From<u32> for ContractParameter {
	fn from(value: u32) -> Self {
		Self::integer(value as i64)
	}
}

impl From<u64> for ContractParameter {
	fn from(value: u64) -> Self {
		Self::integer(value as i64)
	}
}

impl From<&Role> for ContractParameter {
	fn from(value: &Role) -> Self {
		Self::integer(value.clone() as i64)
	}
}

impl From<&str> for ContractParameter {
	fn from(value: &str) -> Self {
		Self::string(value.to_string())
	}
}

impl From<usize> for ContractParameter {
	fn from(value: usize) -> Self {
		Self::integer(value as i64)
	}
}

impl From<&[u8]> for ContractParameter {
	fn from(value: &[u8]) -> Self {
		Self::byte_array(value.to_vec())
	}
}

impl From<Vec<u8>> for ContractParameter {
	fn from(value: Vec<u8>) -> Self {
		Self::byte_array(value)
	}
}

impl Into<Vec<u8>> for ContractParameter {
	fn into(self) -> Vec<u8> {
		match self.clone().value.unwrap() {
			ParameterValue::ByteArray(b) => b.into_bytes(),
			_ => panic!("Cannot convert {:?} to Vec<u8>", self.clone()),
		}
	}
}

impl Into<String> for ContractParameter {
	fn into(self) -> String {
		match self.clone().value.unwrap() {
			ParameterValue::String(s) => s,
			_ => panic!("Cannot convert {:?} to String", self.clone()),
		}
	}
}

// impl Into<Vec<u8>> for Vec<ContractParameter> {
// 	fn into(self) -> Vec<u8> {
// 		self.into_iter().map(|x| x.into()).collect()
// 	}
// }

impl From<&Secp256r1PublicKey> for ContractParameter {
	fn from(value: &Secp256r1PublicKey) -> Self {
		Self::public_key(value)
	}
}

impl From<&H256> for ContractParameter {
	fn from(value: &H256) -> Self {
		Self::H256(value)
	}
}

impl From<&Vec<ContractParameter>> for ContractParameter {
	fn from(value: &Vec<ContractParameter>) -> Self {
		Self::array(value.clone())
	}
}

// impl From<&[(ContractParameter, ContractParameter)]> for ContractParameter {
// 	fn from(value: &[(ContractParameter, ContractParameter)]) -> Self {
// 		Self::map(value.to_vec())
// 	}
// }

impl From<&NefFile> for ContractParameter {
	fn from(value: &NefFile) -> Self {
		Self::byte_array(value.to_array())
	}
}

impl From<String> for ContractParameter {
	fn from(value: String) -> Self {
		Self::string(value)
	}
}

impl From<bool> for ContractParameter {
	fn from(value: bool) -> Self {
		Self::bool(value)
	}
}
impl From<&String> for ContractParameter {
	fn from(value: &String) -> Self {
		Self::string(value.to_string())
	}
}

impl From<NNSName> for ContractParameter {
	fn from(value: NNSName) -> Self {
		Self::string(value.to_string())
	}
}

impl From<Value> for ContractParameter {
	fn from(value: Value) -> Self {
		match value {
			Value::Null => Self::new(ContractParameterType::Any),
			Value::Bool(b) => Self::bool(b),
			Value::Number(n) => Self::integer(n.as_i64().unwrap()),
			Value::String(s) => Self::string(s),
			Value::Array(a) =>
				Self::array(a.into_iter().map(|v| ContractParameter::from(v)).collect()),
			Value::Object(o) => Self::map(ContractParameterMap::from_map(
				o.into_iter()
					.map(|(k, v)| (ContractParameter::from(k), ContractParameter::from(v)))
					.collect(),
			)),
		}
	}
}

impl Into<Value> for ContractParameter {
	fn into(self) -> Value {
		match self.value.unwrap() {
			ParameterValue::Boolean(b) => Value::Bool(b),
			ParameterValue::Integer(i) => Value::Number(serde_json::Number::from(i)),
			ParameterValue::ByteArray(b) => Value::String(b),
			ParameterValue::String(s) => Value::String(s),
			ParameterValue::H160(h) => Value::String(h),
			ParameterValue::H256(h) => Value::String(h),
			ParameterValue::PublicKey(p) => Value::String(p),
			ParameterValue::Signature(s) => Value::String(s),
			ParameterValue::Array(a) => Value::Array(a.into_iter().map(|v| v.into()).collect()),
			ParameterValue::Map(m) => Value::Array(
				m.0.iter()
					.flat_map(|(key, value)| vec![key.clone().into(), value.clone().into()])
					.collect(),
			),
			ParameterValue::Any => Value::Null,
		}
	}
}

impl From<Vec<Value>> for ContractParameter {
	fn from(value: Vec<Value>) -> Self {
		Self::array(value.into_iter().map(|v| ContractParameter::from(v)).collect())
	}
}

// impl Into<Vec<Value>> for ContractParameter{
// 	fn into(self) -> Vec<Value> {
// 		match self.value.clone().unwrap() {
// 			ParameterValue::Array(a) => a.into_iter().map(|v| v.into()).collect(),
// 			ParameterValue::Map(m) => m.into_iter().map(|v| v.into()).collect(),
// 			_ => panic!("Cannot convert {:?} to Vec<Value>", self.clone()),
// 		}
// 	}
// }

impl ValueExtension for ContractParameter {
	fn to_value(&self) -> Value {
		Value::String(serde_json::to_string(self).unwrap())
	}
}

impl ValueExtension for Vec<ContractParameter> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

#[derive(Display, EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ParameterValue {
	Boolean(bool),
	Integer(i64),
	ByteArray(String),
	String(String),
	H160(String),
	H256(String),
	PublicKey(String),
	Signature(String),
	Array(Vec<ContractParameter>),
	Map(ContractParameterMap),
	Any,
}

impl Hash for ParameterValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			ParameterValue::Boolean(b) => b.hash(state),
			ParameterValue::Integer(i) => i.hash(state),
			ParameterValue::ByteArray(b) => b.hash(state),
			ParameterValue::String(s) => s.hash(state),
			ParameterValue::H160(h) => h.hash(state),
			ParameterValue::H256(h) => h.hash(state),
			ParameterValue::PublicKey(p) => p.hash(state),
			ParameterValue::Signature(s) => s.hash(state),
			ParameterValue::Array(a) => a.hash(state),
			// ParameterValue::Map(m) =>
			// 	for (k, v) in m.0 {
			// 		k.hash(state);
			// 		v.hash(state);
			// 	},
			ParameterValue::Any => "Any".hash(state),
			_ => panic!("Invalid Hash Key"),
		}
	}
}

impl ContractParameter {
	pub fn new(typ: ContractParameterType) -> Self {
		Self { name: None, typ, value: None }
	}

	pub fn get_type(&self) -> ContractParameterType {
		self.typ.clone()
	}

	pub fn with_value(typ: ContractParameterType, value: ParameterValue) -> Self {
		Self { name: None, typ, value: Some(value) }
	}

	pub fn bool(value: bool) -> Self {
		Self::with_value(ContractParameterType::Boolean, ParameterValue::Boolean(value))
	}

	pub fn to_bool(&self) -> bool {
		match self.value.as_ref().unwrap() {
			ParameterValue::Boolean(b) => *b,
			_ => panic!("Cannot convert {:?} to bool", self.clone()),
		}
	}

	pub fn integer(value: i64) -> Self {
		Self::with_value(ContractParameterType::Integer, ParameterValue::Integer(value))
	}

	pub fn to_integer(&self) -> i64 {
		match self.value.as_ref().unwrap() {
			ParameterValue::Integer(i) => *i,
			_ => panic!("Cannot convert {:?} to i64", self.clone()),
		}
	}

	pub fn byte_array(value: Vec<u8>) -> Self {
		let encoded = value.to_base64();
		Self::with_value(ContractParameterType::ByteArray, ParameterValue::ByteArray(encoded))
	}

	pub fn to_byte_array(&self) -> Vec<u8> {
		match self.value.as_ref().unwrap() {
			ParameterValue::ByteArray(b) => b.from_base64().unwrap().to_vec(),
			_ => panic!("Cannot convert {:?} to Vec<u8>", self.clone()),
		}
	}

	pub fn string(value: String) -> Self {
		Self::with_value(ContractParameterType::String, ParameterValue::String(value))
	}

	pub fn to_string(&self) -> String {
		match self.value.as_ref().unwrap() {
			ParameterValue::String(s) => s.clone(),
			// ParameterValue::ByteArray(b) => b.clone(),
			// ParameterValue::H160(h) => h.clone(),
			// ParameterValue::H256(h) => h.clone(),
			// ParameterValue::PublicKey(p) => p.clone(),
			// ParameterValue::Signature(s) => s.clone(),
			// ParameterValue::Integer(i) => i.to_string(),
			// ParameterValue::Boolean(b) => b.to_string(),
			_ => panic!("Cannot convert {:?} to String", self.clone()),
		}
	}
	pub fn H160(value: &H160) -> Self {
		Self::with_value(ContractParameterType::H160, ParameterValue::H160(value.to_hex()))
	}

	pub fn to_h160(&self) -> H160 {
		match self.value.as_ref().unwrap() {
			ParameterValue::H160(h) => H160::from_slice(&h.from_hex().unwrap()),
			_ => panic!("Cannot convert {:?} to H160", self.clone()),
		}
	}

	pub fn H256(value: &H256) -> Self {
		Self::with_value(ContractParameterType::H256, ParameterValue::H256(value.0.to_hex()))
	}

	pub fn to_h256(&self) -> H256 {
		match self.value.as_ref().unwrap() {
			ParameterValue::H256(h) => H256::from_slice(&h.from_hex().unwrap()),
			_ => panic!("Cannot convert {:?} to H256", self.clone()),
		}
	}

	pub fn public_key(value: &Secp256r1PublicKey) -> Self {
		Self::with_value(
			ContractParameterType::PublicKey,
			ParameterValue::PublicKey(hex::encode(value.get_encoded(true))),
		)
	}

	pub fn to_public_key(&self) -> Secp256r1PublicKey {
		match self.value.as_ref().unwrap() {
			ParameterValue::PublicKey(p) => {
				let bytes = hex::decode(p).unwrap();
				Secp256r1PublicKey::from_bytes(&bytes).unwrap()
			},
			_ => panic!("Cannot convert {:?} to PublicKey", self.clone()),
		}
	}

	pub fn signature(value: &str) -> Self {
		Self::with_value(
			ContractParameterType::Signature,
			ParameterValue::Signature(value.to_string()),
		)
	}

	pub fn to_signature(&self) -> String {
		match self.value.as_ref().unwrap() {
			ParameterValue::Signature(s) => s.clone(),
			_ => panic!("Cannot convert {:?} to String", self.clone()),
		}
	}

	pub fn array(values: Vec<Self>) -> Self {
		Self::with_value(ContractParameterType::Array, ParameterValue::Array(values))
	}

	pub fn to_array(&self) -> Vec<ContractParameter> {
		match self.value.as_ref().unwrap() {
			ParameterValue::Array(a) => a.clone(),
			_ => panic!("Cannot convert {:?} to Vec<ContractParameter>", self.clone()),
		}
	}

	pub fn map(values: ContractParameterMap) -> Self {
		Self::with_value(ContractParameterType::Map, ParameterValue::Map(values))
	}

	pub fn to_map(&self) -> ContractParameterMap {
		match self.value.as_ref().unwrap() {
			ParameterValue::Map(m) => m.clone(),
			_ => panic!(
				"Cannot convert {:?} to HashMap<ContractParameter, ContractParameter>",
				self.clone()
			),
		}
	}

	pub fn hash(self) -> Vec<u8> {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		Hash::hash(&self, &mut hasher);
		hasher.finish().to_be_bytes().to_vec()
	}
}

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ContractParameterMap(
	#[serde(serialize_with = "serialize_map", deserialize_with = "deserialize_map")]
	pub  HashMap<ContractParameter, ContractParameter>,
);

impl ContractParameterMap {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn from_map(map: HashMap<ContractParameter, ContractParameter>) -> Self {
		Self(map)
	}

	pub fn to_map(&mut self) -> &HashMap<ContractParameter, ContractParameter> {
		&mut self.0
	}
}

impl ParameterValue {
	pub fn to_bool(&self) -> bool {
		match self {
			ParameterValue::Boolean(b) => *b,
			_ => panic!("Cannot convert {:?} to bool", self.clone()),
		}
	}

	pub fn to_integer(&self) -> i64 {
		match self {
			ParameterValue::Integer(i) => *i,
			_ => panic!("Cannot convert {:?} to i64", self.clone()),
		}
	}

	pub fn to_byte_array(&self) -> Vec<u8> {
		match self {
			ParameterValue::ByteArray(b) => b.from_base64().unwrap().to_vec(),
			_ => panic!("Cannot convert {:?} to Vec<u8>", self.clone()),
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			ParameterValue::String(s) => s.clone(),
			_ => panic!("Cannot convert {:?} to String", self.clone()),
		}
	}

	pub fn to_h160(&self) -> H160 {
		match self {
			ParameterValue::H160(h) => H160::from_slice(&h.from_hex().unwrap()),
			_ => panic!("Cannot convert {:?} to H160", self.clone()),
		}
	}

	pub fn to_h256(&self) -> H256 {
		match self {
			ParameterValue::H256(h) => H256::from_slice(&h.from_hex().unwrap()),
			_ => panic!("Cannot convert {:?} to H256", self.clone()),
		}
	}

	pub fn to_public_key(&self) -> Secp256r1PublicKey {
		match self {
			ParameterValue::PublicKey(p) => {
				let bytes = hex::decode(p).unwrap();
				Secp256r1PublicKey::from_bytes(&bytes).unwrap()
			},
			_ => panic!("Cannot convert {:?} to PublicKey", self.clone()),
		}
	}

	pub fn to_signature(&self) -> String {
		match self {
			ParameterValue::Signature(s) => s.clone(),
			_ => panic!("Cannot convert {:?} to String", self.clone()),
		}
	}

	pub fn to_array(&self) -> Vec<ContractParameter> {
		match self {
			ParameterValue::Array(a) => a.clone(),
			_ => panic!("Cannot convert {:?} to Vec<ContractParameter>", self.clone()),
		}
	}

	pub fn to_map(&self) -> ContractParameterMap {
		match self {
			ParameterValue::Map(m) => m.clone(),
			_ => panic!(
				"Cannot convert {:?} to HashMap<ContractParameter, ContractParameter>",
				self.clone()
			),
		}
	}

	pub fn hash(&self) -> Vec<u8> {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		Hash::hash(&self, &mut hasher);
		hasher.finish().to_be_bytes().to_vec()
	}
}

#[cfg(test)]
mod tests {
	use primitive_types::{H160, H256};
	use rustc_serialize::hex::FromHex;

	use neo::prelude::{
		ContractParameter, ContractParameterMap, ContractParameterType, Secp256r1PublicKey,
	};

	#[test]
	fn test_string_from_string() {
		let param = ContractParameter::string("value".to_string());
		// assert_param(&param, "value", ContractParameterType::String);
		assert_eq!(param.typ, ContractParameterType::String);
		assert_eq!(param.value.unwrap().to_string(), "value");
	}

	#[test]
	fn test_bytes_from_bytes() {
		let bytes = vec![0x01, 0x01];
		let param = ContractParameter::byte_array(bytes.clone());
		// assert_param(&param, bytes, ContractParameterType::ByteArray);
		assert_eq!(param.typ, ContractParameterType::ByteArray);
		assert_eq!(param.value.unwrap().to_byte_array(), bytes);
	}

	#[test]
	fn test_bytes_from_hex_string() {
		let param = ContractParameter::byte_array("a602".from_hex().unwrap());
		// assert_param(&param, vec![0xa6, 0x02], ContractParameterType::ByteArray);
		assert_eq!(param.typ, ContractParameterType::ByteArray);
		assert_eq!(param.value.unwrap().to_byte_array(), vec![0xa6, 0x02]);
	}

	#[test]
	fn test_array_from_array() {
		let params = vec![
			ContractParameter::string("value".to_string()),
			ContractParameter::byte_array("0101".from_hex().unwrap()),
		];

		let param = ContractParameter::array(params.clone());
		// assert_param(&param, params, ContractParameterType::Array);
		assert_eq!(param.typ, ContractParameterType::Array);
		assert_eq!(param.value.unwrap().to_array(), params);
	}

	#[test]
	fn test_array_from_empty() {
		let param = ContractParameter::array(Vec::new());

		// assert!(matches!(param.value, Some([])));
	}

	#[test]
	fn test_nested_array() {
		let nested_params = vec![
			ContractParameter::integer(420),
			ContractParameter::integer(1024),
			ContractParameter::string("neow3j:)".to_string()),
			ContractParameter::integer(10),
		];

		let params = vec![
			ContractParameter::string("value".to_string()),
			ContractParameter::byte_array("0101".from_hex().unwrap()),
			ContractParameter::array(nested_params),
			ContractParameter::integer(55),
		];

		let param = ContractParameter::array(params);

		assert_eq!(param.typ, ContractParameterType::Array);

		// let nested_vec = nested.value.as_ref().unwrap();
		// assert_eq!(nested_vec.len(), 4);
		//
		// let nested_nested = &nested_vec[3];
		// assert_eq!(nested_nested.typ, ContractParameterType::Array);
	}

	#[test]
	fn test_map() {
		let mut map = ContractParameterMap::new();
		map.0
			.insert(ContractParameter::integer(1), ContractParameter::string("first".to_string()));

		let param = ContractParameter::map(map);

		assert_eq!(param.typ, ContractParameterType::Map);
		let map = param.value.as_ref().unwrap();

		let map = map.to_map();
		let (key, val) = map.0.iter().next().unwrap();
		assert_eq!(*key, ContractParameter::integer(1));
		assert_eq!(*val, ContractParameter::string("first".to_string()));
	}

	#[test]
	fn test_nested_map() {
		let inner_map = {
			let mut map = ContractParameterMap::new();
			map.0.insert(
				ContractParameter::string("halo".to_string()),
				ContractParameter::integer(1234),
			);
			ContractParameter::map(map)
		};

		let mut map = ContractParameterMap::new();
		map.0.insert(ContractParameter::integer(16), inner_map);

		let param = ContractParameter::map(map);

		let outer_map = param.value.as_ref().unwrap();
		assert_eq!(outer_map.to_map().0.len(), 1);

		let outer_map = outer_map.to_map();
		let inner_param = outer_map.0.get(&ContractParameter::integer(16)).unwrap();
		let inner_map = inner_param.value.as_ref().unwrap();

		assert_eq!(inner_map.to_map().0.len(), 1);
		let inner_map = inner_map.to_map();
		let (key, val) = inner_map.0.iter().next().unwrap();
		assert_eq!(*key, ContractParameter::string("halo".to_string()));
		assert_eq!(*val, ContractParameter::integer(1234));
	}

	#[test]
	fn test_serialize_deserialize() {
		let array_param_1 = ContractParameter::integer(1000);
		let array_param_2 = ContractParameter::integer(2000);

		let mut inner_map = ContractParameterMap::new();
		inner_map
			.0
			.insert(ContractParameter::integer(5), ContractParameter::string("value".to_string()));
		inner_map.0.insert(
			ContractParameter::byte_array(vec![0x01, 0x02, 0x03]),
			ContractParameter::integer(5),
		);
		let inner_map_param = ContractParameter::map(inner_map);

		let array_params = vec![array_param_1, array_param_2, inner_map_param];

		let param = ContractParameter::array(array_params);

		// Serialize
		let json = serde_json::to_string(&param).unwrap();

		// Deserialize
		let deserialized: ContractParameter = serde_json::from_str(&json).unwrap();

		// Assert
		assert_eq!(deserialized, param);

		// Round trip
		let roundtrip_json = serde_json::to_string(&deserialized).unwrap();
		let roundtrip = serde_json::from_str::<ContractParameter>(&roundtrip_json).unwrap();

		assert_eq!(roundtrip, param);
	}
	#[test]
	fn test_bytes_equals() {
		let param1 = ContractParameter::byte_array("796573".from_hex().unwrap());
		let param2 = ContractParameter::byte_array(vec![0x79, 0x65, 0x73]);
		assert_eq!(param1, param2);
	}

	#[test]
	fn test_bytes_from_string() {
		let param = ContractParameter::byte_array("Neo".as_bytes().to_vec());
		// assert_param(&param, b"Neo", ContractParameterType::ByteArray);
		assert_eq!(param.typ, ContractParameterType::ByteArray);
		assert_eq!(param.value.unwrap().to_byte_array(), b"Neo");
	}

	#[test]
	fn test_bool() {
		let param = ContractParameter::bool(false);
		// assert_param(&param, false, ContractParameterType::Boolean);
		assert_eq!(param.typ, ContractParameterType::Boolean);
		assert_eq!(param.value.unwrap().to_bool(), false);
	}

	#[test]
	fn test_int() {
		let param = ContractParameter::integer(10);
		// assert_param(&param, 10, ContractParameterType::Integer);
		assert_eq!(param.typ, ContractParameterType::Integer);
		assert_eq!(param.value.unwrap().to_integer(), 10);
	}

	#[test]
	fn test_H160() {
		let hash = H160::from([0u8; 20]);
		let param = ContractParameter::H160(&hash);
		// assert_param(&param, hash.into(), ContractParameterType::H160);
		assert_eq!(param.typ, ContractParameterType::H160);
		assert_eq!(param.value.unwrap().to_h160(), hash);
	}

	#[test]
	fn test_H256() {
		let hash = H256::from([0u8; 32]);
		let param = ContractParameter::H256(&hash);
		// assert_param(&param, hash.into(), ContractParameterType::H256);
		assert_eq!(param.typ, ContractParameterType::H256);
		assert_eq!(param.value.unwrap().to_h256(), hash);
	}

	#[test]
	fn test_public_key() {
		let key = "03b4af8efe55d98b44eedfcfaa39642fd5d53ad543d18d3cc2db5880970a4654f6"
			.from_hex()
			.unwrap()
			.to_vec();
		let key = Secp256r1PublicKey::from_bytes(&key).unwrap();
		let param = ContractParameter::public_key(&key);
		// assert_param(&param, key, ContractParameterType::PublicKey);
		assert_eq!(param.typ, ContractParameterType::PublicKey);
		assert_eq!(param.value.unwrap().to_public_key(), key);
	}

	#[test]
	fn test_signature() {
		let sig = "010203..."; // 64 byte signature
		let param = ContractParameter::signature(sig);
		// assert_param(&param, sig, ContractParameterType::Signature);
		assert_eq!(param.typ, ContractParameterType::Signature);
		assert_eq!(param.value.unwrap().to_signature(), sig);
	}

	#[test]
	fn create_from_various_types() {
		let string_param = ContractParameter::from("hello");
		// assert_param(&string_param, "hello".as_bytes(), ContractParameterType::String);

		assert_eq!(string_param.typ, ContractParameterType::String);
		assert_eq!(string_param.value.unwrap().to_string(), "hello");

		let bool_param = ContractParameter::from(true);
		// assert_param(&bool_param, true, ContractParameterType::Boolean);
		assert_eq!(bool_param.typ, ContractParameterType::Boolean);
		assert_eq!(bool_param.value.unwrap().to_bool(), true);

		let int_param = ContractParameter::from(10);
		// assert_param(&int_param, 10, ContractParameterType::Integer);
		assert_eq!(int_param.typ, ContractParameterType::Integer);
		assert_eq!(int_param.value.unwrap().to_integer(), 10);
	}

	#[test]
	fn create_array_from_vec() {
		let vec = vec![ContractParameter::from(1), ContractParameter::from("test")];

		let param = ContractParameter::from(&vec);

		assert_eq!(param.typ, ContractParameterType::Array);

		let array = param.value.unwrap().to_array();
		assert_eq!(array.len(), 2);
		// assert_param(&array[0], 1, ContractParameterType::Integer);
		assert_eq!(&array[0].typ, &ContractParameterType::Integer);
		assert_eq!(&array[0].value.clone().unwrap().to_integer(), &1);
		// assert_param(&array[1], "test".as_bytes(), ContractParameterType::String);
		assert_eq!(&array[1].typ, &ContractParameterType::String);
		assert_eq!(&array[1].value.clone().unwrap().to_string(), "test");
	}

	#[test]
	fn create_map_from_hashmap() {
		let mut map = ContractParameterMap::new();
		map.0.insert("key".to_owned().into(), ContractParameter::from(1));

		let param = ContractParameter::map(map);

		assert_eq!(param.typ, ContractParameterType::Map);

		let map = param.value.as_ref().unwrap();

		let map = map.to_map();
		let (key, val) = map.0.iter().next().unwrap();
		assert_eq!(key.typ, ContractParameterType::String);
		assert_eq!(key.value.clone().unwrap().to_string(), "key");
	}

	#[test]
	fn equality_operator() {
		let p1 = ContractParameter::from(1);
		let p2 = ContractParameter::from(1);

		assert_eq!(p1, p2);

		let p3 = ContractParameter::from("test");
		assert_ne!(p1, p3);
	}

	// #[test]
	// fn invalid_type_errors() {
	// 	let result = ContractParameter::from(MyStruct);
	//
	// 	assert!(result.is_err());
	// 	assert_eq!(result.err(), Some(InvalidTypeError));
	// }
}
