#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
	collections::{HashMap, HashSet},
	convert::TryInto,
};

use elliptic_curve::sec1::ToEncodedPoint;
use hex;
use primitive_types::{H160, H256, U256};
use reqwest::Url;
use serde::{
	ser::{SerializeMap, SerializeSeq},
	Deserialize, Deserializer, Serialize, Serializer,
};

use neo::prelude::{
	encode_string_h160, encode_string_h256, encode_string_u256, parse_address, parse_string_h256,
	parse_string_u256, parse_string_u64, Address, AddressOrScriptHash, ContractParameter,
	ScriptHash, Secp256r1PrivateKey, Secp256r1PublicKey,
};
#[cfg(feature = "substrate")]
use serde_big_array_substrate::big_array;

#[cfg(feature = "substrate")]
use serde_substrate as serde;

pub fn serialize_boolean_expression<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(if *value { "true" } else { "false" })
}

pub fn deserialize_boolean_expression<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	let value = s == "true";
	Ok(value)
}

pub fn serialize_bytes<S>(item: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = format!("0x{}", hex::encode(item));
	serializer.serialize_str(&item_str)
}

pub fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let bytes = hex::decode(s.trim_start_matches("0x")).unwrap();
	Ok(bytes)
}

pub fn serialize_url<S>(item: Url, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	// deserialize_script_hash
	let item_str = format!("{}", item);
	serializer.serialize_str(&item_str)
}

pub fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Secp256r1PublicKey, D::Error>
where
	D: Deserializer<'de>,
{
	let a: &[u8] = Deserialize::deserialize(deserializer)?;
	Secp256r1PublicKey::from_bytes(a).map_err(serde::de::Error::custom)
}

pub fn serialize_pubkey<S>(item: Secp256r1PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = format!("{:?}", item.get_encoded(true));
	serializer.serialize_str(&item_str)
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let url = Url::parse(&s).unwrap();
	Ok(url)
}

pub fn serialize_url_option<S>(item: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match item {
		Some(url) => {
			let url_str = format!("{}", url);
			serializer.serialize_str(&url_str)
		},
		None => serializer.serialize_none(),
	}
}

pub fn deserialize_url_option<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Deserialize::deserialize(deserializer)?;
	match s {
		Some(s) => {
			let url = Url::parse(&s).unwrap();
			Ok(Some(url))
		},
		None => Ok(None),
	}
}

pub fn serialize_wildcard<S>(value: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	if value == &vec!["*".to_string()] {
		serializer.serialize_str("*")
	} else {
		value.serialize(serializer)
	}
}

pub fn deserialize_wildcard<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	if s == "*" {
		Ok(vec!["*".to_string()])
	} else {
		Ok(vec![s])
	}
}

pub fn serialize_u256<S>(item: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = format!("{}", item);
	serializer.serialize_str(&item_str)
}

pub fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	Ok(parse_string_u256(&s))
}

pub fn serialize_u256_option<S>(item: &Option<U256>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match item {
		Some(u256) => {
			let u256_str = encode_string_u256(&u256);
			serializer.serialize_str(&u256_str)
		},
		None => serializer.serialize_none(),
	}
}

pub fn deserialize_u256_option<'de, D>(deserializer: D) -> Result<Option<U256>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Deserialize::deserialize(deserializer)?;
	match s {
		Some(s) => {
			let u256 = parse_string_u256(&s);
			Ok(Some(u256))
		},
		None => Ok(None),
	}
}

pub fn serialize_u32<S>(item: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = format!("0x{:x}", item);
	serializer.serialize_str(&item_str)
}

pub fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let v = if s.starts_with("0x") {
		let s = s.trim_start_matches("0x");
		u32::from_str_radix(&s, 16).unwrap()
	} else {
		u32::from_str_radix(&s, 10).unwrap()
	};
	Ok(v)
}

pub fn serialize_u64<S>(item: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = format!("{}", item);
	serializer.serialize_str(&item_str)
}

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	Ok(parse_string_u64(&s))
}

pub fn deserialize_script_hash<'de, D>(deserializer: D) -> Result<ScriptHash, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let addr = parse_address(&s);
	Ok(addr)
}

pub fn serialize_script_hash<S>(item: &ScriptHash, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = encode_string_h160(item);
	serializer.serialize_str(&item_str)
}

pub fn deserialize_address_or_script_hash<'de, D>(
	deserializer: D,
) -> Result<AddressOrScriptHash, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let addr = parse_address(&s);
	Ok(AddressOrScriptHash::ScriptHash(addr))
}

pub fn serialize_address_or_script_hash<S>(
	item: &AddressOrScriptHash,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = encode_string_h160(&item.script_hash());
	serializer.serialize_str(&item_str)
}

pub fn deserialize_vec_script_hash<'de, D>(deserializer: D) -> Result<Vec<ScriptHash>, D::Error>
where
	D: Deserializer<'de>,
{
	let string_seq = <Vec<ScriptHash>>::deserialize(deserializer)?;
	// let mut vec: Vec<Address> = Vec::new();
	// for v_str in string_seq {
	// 	let v = parse_address(&v_str);
	// 	vec.push(v);
	// }
	Ok(string_seq)
}

pub fn serialize_vec_script_hash<S>(
	item: &Vec<ScriptHash>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut seq = serializer.serialize_seq(Some(item.len()))?;
	for i in item {
		seq.serialize_element(&i)?;
	}
	seq.end()
}

pub fn deserialize_vec_script_hash_option<'de, D>(
	deserializer: D,
) -> Result<Option<Vec<ScriptHash>>, D::Error>
where
	D: Deserializer<'de>,
{
	let string_seq = <Option<Vec<ScriptHash>>>::deserialize(deserializer)?;
	// let mut vec: Vec<Address> = Vec::new();
	// for v_str in string_seq {
	// 	let v = parse_address(&v_str);
	// 	vec.push(v);
	// }
	match string_seq {
		Some(s) => Ok(Some(s)),
		None => Ok(None),
	}
}

pub fn serialize_vec_script_hash_option<S>(
	item: &Option<Vec<ScriptHash>>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match item {
		Some(addr) => {
			let mut seq = serializer.serialize_seq(Some(addr.len()))?;
			for i in item {
				seq.serialize_element(&i)?;
			}
			seq.end()
		},
		None => serializer.serialize_none(),
	}
}

pub fn serialize_script_hash_option<S>(
	item: &Option<ScriptHash>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match item {
		Some(addr) => {
			let addr_str = encode_string_h160(&addr);
			serializer.serialize_str(&addr_str)
		},
		None => serializer.serialize_none(),
	}
}

pub fn deserialize_script_hash_option<'de, D>(
	deserializer: D,
) -> Result<Option<ScriptHash>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Deserialize::deserialize(deserializer)?;
	match s {
		Some(s) => {
			let addr = parse_address(&s);
			Ok(Some(addr))
		},
		None => Ok(None),
	}
}

pub fn serialize_hash_map_h160_account<S, Account>(
	item: &HashMap<H160, Account>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	Account: Serialize,
{
	let mut map = serializer.serialize_map(Some(item.len()))?;
	for (k, v) in item {
		map.serialize_entry(&encode_string_h160(k), &v)?;
	}
	map.end()
}

pub fn deserialize_hash_map_h160_account<'de, D, Account>(
	deserializer: D,
) -> Result<HashMap<H160, Account>, D::Error>
where
	D: Deserializer<'de>,
	Account: Deserialize<'de>,
{
	let map = <HashMap<String, Account>>::deserialize(deserializer)?;
	let mut hashmap: HashMap<H160, Account> = HashMap::new();

	for (k, v) in map {
		let k_h160 = parse_address(&k);
		hashmap.insert(k_h160, v);
	}
	Ok(hashmap)
}

// Secp256r1PrivateKey

pub fn deserialize_private_key<'de, D>(deserializer: D) -> Result<Secp256r1PrivateKey, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let key = Secp256r1PrivateKey::from_bytes(parse_string_h256(&s).as_bytes()).unwrap();
	Ok(key)
}

pub fn serialize_private_key<S>(
	item: &Secp256r1PrivateKey,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = encode_string_h256(&H256::from_slice(&item.to_raw_bytes().to_vec()));
	serializer.serialize_str(&item_str)
}

// Secp256r1PublicKey
pub fn deserialize_public_key<'de, D>(deserializer: D) -> Result<Secp256r1PublicKey, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	let key = Secp256r1PublicKey::from_bytes(parse_string_h256(&s).as_bytes()).unwrap();
	Ok(key)
}

pub fn serialize_public_key<S>(item: &Secp256r1PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let item_str = encode_string_h256(&H256::from_slice(&item.get_encoded(true)));
	serializer.serialize_str(&item_str)
}

pub fn deserialize_vec_public_key<'de, D>(
	deserializer: D,
) -> Result<Vec<Secp256r1PublicKey>, D::Error>
where
	D: Deserializer<'de>,
{
	let string_seq = <Vec<String>>::deserialize(deserializer)?;
	let mut vec: Vec<Secp256r1PublicKey> = Vec::new();
	for v_str in string_seq {
		let v = parse_string_h256(&v_str);
		let key = Secp256r1PublicKey::from_bytes(v.as_bytes()).unwrap();
		vec.push(key);
	}
	Ok(vec)
}

pub fn serialize_vec_public_key<S>(
	item: &Vec<Secp256r1PublicKey>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut seq = serializer.serialize_seq(Some(item.len()))?;
	for i in item {
		seq.serialize_element(&encode_string_h256(&H256::from_slice(&i.get_encoded(true))))?;
	}
	seq.end()
}

// impl serialize_public_key_option
pub fn serialize_public_key_option<S>(
	item: &Option<Secp256r1PublicKey>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match item {
		Some(key) => {
			let key_str = encode_string_h256(&H256::from_slice(&key.get_encoded(true)));
			serializer.serialize_str(&key_str)
		},
		None => serializer.serialize_none(),
	}
}

// impl deserialize_public_key_option
pub fn deserialize_public_key_option<'de, D>(
	deserializer: D,
) -> Result<Option<Secp256r1PublicKey>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Deserialize::deserialize(deserializer)?;
	match s {
		Some(s) => {
			let pubkey_bytes = parse_string_h256(&s);
			let key = Secp256r1PublicKey::from_bytes(pubkey_bytes.as_bytes()).unwrap();
			Ok(Some(key))
		},
		None => Ok(None),
	}
}

// pub fn serialize_vec_methodtoken<S>(
// 	item: &Vec<MethodToken>,
// 	serializer: S,
// ) -> Result<S::Ok, S::Error>
// where
// 	S: Serializer,
// {
// 	let mut seq = serializer.serialize_seq(Some(item.len()))?;
// 	for i in item {
// 		seq.serialize_element(&i)?;
// 	}
// 	seq.end()
// }
//
// pub fn deserialize_vec_methodtoken<'de, D>(deserializer: D) -> Result<Vec<MethodToken>, D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let string_seq = <Vec<MethodToken>>::deserialize(deserializer)?;
// 	let mut vec: Vec<MethodToken> = Vec::new();
// 	for v_str in string_seq {
// 		let v = v_str;
// 		vec.push(v);
// 	}
// 	Ok(vec)
// }

pub fn serialize_h256<S>(item: &H256, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&encode_string_h256(item))
}

pub fn deserialize_h256<'de, D>(deserializer: D) -> Result<H256, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	Ok(parse_string_h256(&s))
}

pub fn serialize_hashset_u256<S>(item: &HashSet<U256>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut seq = serializer.serialize_seq(Some(item.len()))?;
	for i in item {
		seq.serialize_element(&encode_string_u256(i))?;
	}
	seq.end()
}

pub fn deserialize_hashset_u256<'de, D>(deserializer: D) -> Result<HashSet<U256>, D::Error>
where
	D: Deserializer<'de>,
{
	let string_seq = <HashSet<String>>::deserialize(deserializer)?;
	let mut hashset: HashSet<U256> = HashSet::new();
	for v_str in string_seq {
		let v = parse_string_u256(&v_str);
		hashset.insert(v);
	}
	Ok(hashset)
}

pub fn serialize_vec_h256<S>(item: &Vec<H256>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut seq = serializer.serialize_seq(Some(item.len()))?;
	for i in item {
		seq.serialize_element(&encode_string_h256(i))?;
	}
	seq.end()
}

pub fn deserialize_vec_h256<'de, D>(deserializer: D) -> Result<Vec<H256>, D::Error>
where
	D: Deserializer<'de>,
{
	let string_seq = <Vec<String>>::deserialize(deserializer)?;
	let mut vec: Vec<H256> = Vec::new();
	for v_str in string_seq {
		let v = parse_string_h256(&v_str);
		vec.push(v);
	}
	Ok(vec)
}

pub fn serialize_vec_u256<S>(item: &Vec<U256>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut seq = serializer.serialize_seq(Some(item.len()))?;
	for i in item {
		seq.serialize_element(&encode_string_u256(i))?;
	}
	seq.end()
}

pub fn deserialize_vec_u256<'de, D>(deserializer: D) -> Result<Vec<U256>, D::Error>
where
	D: Deserializer<'de>,
{
	let string_seq = <Vec<String>>::deserialize(deserializer)?;
	let mut vec: Vec<U256> = Vec::new();
	for v_str in string_seq {
		let v = parse_string_u256(&v_str);
		vec.push(v);
	}
	Ok(vec)
}

pub fn serialize_h256_option<S>(item: &Option<H256>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match item {
		Some(h256) => {
			let h256_str = encode_string_h256(&h256);
			serializer.serialize_str(&h256_str)
		},
		None => serializer.serialize_none(),
	}
}

pub fn deserialize_h256_option<'de, D>(deserializer: D) -> Result<Option<H256>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Deserialize::deserialize(deserializer)?;
	match s {
		Some(s) => {
			let h256 = parse_string_h256(&s);
			Ok(Some(h256))
		},
		None => Ok(None),
	}
}

pub fn serialize_hashmap_u256_hashset_u256<S>(
	item: &HashMap<U256, HashSet<U256>>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut map = serializer.serialize_map(Some(item.len()))?;
	for (k, v) in item {
		let value: HashSet<String> = v.iter().map(|x| encode_string_u256(&x)).collect();
		map.serialize_entry(&encode_string_u256(k), &value)?;
	}
	map.end()
}

pub fn deserialize_hashmap_u256_hashset_u256<'de, D>(
	deserializer: D,
) -> Result<HashMap<U256, HashSet<U256>>, D::Error>
where
	D: Deserializer<'de>,
{
	let map = <HashMap<String, HashSet<String>>>::deserialize(deserializer)?;
	let mut hashmap: HashMap<U256, HashSet<U256>> = HashMap::new();

	for (k, v) in map {
		let k_u256 = parse_string_u256(&k);
		let v_hashset_u256: HashSet<U256> = v.iter().map(|x| parse_string_u256(&x)).collect();
		hashmap.insert(k_u256, v_hashset_u256);
	}
	Ok(hashmap)
}

pub fn serialize_hashmap_address_u256<S>(
	item: &HashMap<Address, U256>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut map = serializer.serialize_map(Some(item.len()))?;
	for (k, v) in item {
		map.serialize_entry(k, &encode_string_u256(v))?;
	}
	map.end()
}

pub fn deserialize_hashmap_address_u256<'de, D>(
	deserializer: D,
) -> Result<HashMap<Address, U256>, D::Error>
where
	D: Deserializer<'de>,
{
	let map = <HashMap<String, String>>::deserialize(deserializer)?;
	let mut hashmap: HashMap<Address, U256> = HashMap::new();

	for (k, v) in map {
		// let k_h160 = parse_address(&k);
		let v_u256 = parse_string_u256(&v);
		hashmap.insert(k, v_u256);
	}
	Ok(hashmap)
}

pub fn serialize_hashmap_u256_hashset_h256<S>(
	item: &HashMap<U256, HashSet<H256>>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut map = serializer.serialize_map(Some(item.len()))?;
	for (k, v) in item {
		let value: HashSet<String> = v.iter().map(|x| encode_string_h256(&x)).collect();
		map.serialize_entry(&encode_string_u256(k), &value)?;
	}
	map.end()
}

pub fn deserialize_hashmap_u256_hashset_h256<'de, D>(
	deserializer: D,
) -> Result<HashMap<U256, HashSet<H256>>, D::Error>
where
	D: Deserializer<'de>,
{
	let map = <HashMap<String, HashSet<String>>>::deserialize(deserializer)?;
	let mut hashmap: HashMap<U256, HashSet<H256>> = HashMap::new();

	for (k, v) in map {
		let k_u256 = parse_string_u256(&k);
		let v_hashset_h256: HashSet<H256> = v.iter().map(|x| parse_string_h256(&x)).collect();
		hashmap.insert(k_u256, v_hashset_h256);
	}
	Ok(hashmap)
}

pub fn serialize_hashmap_u256_vec_u256<S>(
	item: &HashMap<U256, Vec<U256>>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut map = serializer.serialize_map(Some(item.len()))?;
	for (k, v) in item {
		let value: Vec<String> = v.iter().map(|x| encode_string_u256(&x)).collect();
		map.serialize_entry(&encode_string_u256(k), &value)?;
	}
	map.end()
}

pub fn deserialize_hashmap_u256_vec_u256<'de, D>(
	deserializer: D,
) -> Result<HashMap<U256, Vec<U256>>, D::Error>
where
	D: Deserializer<'de>,
{
	let map = <HashMap<String, Vec<String>>>::deserialize(deserializer)?;
	let mut hashmap: HashMap<U256, Vec<U256>> = HashMap::new();

	for (k, v) in map {
		let k_u256 = parse_string_u256(&k);
		let v_vec_u256: Vec<U256> = v.iter().map(|x| parse_string_u256(&x)).collect();
		hashmap.insert(k_u256, v_vec_u256);
	}
	Ok(hashmap)
}

pub fn serialize_map<S>(
	map: &HashMap<ContractParameter, ContractParameter>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let serializable_map: Vec<(_, _)> =
		map.iter().map(|(k, v)| (serde_json::to_string(k).unwrap(), v)).collect();
	serializable_map.serialize(serializer)
}

pub fn deserialize_map<'de, D>(
	deserializer: D,
) -> Result<HashMap<ContractParameter, ContractParameter>, D::Error>
where
	D: Deserializer<'de>,
{
	let deserialized_vector: Vec<(String, ContractParameter)> = Vec::deserialize(deserializer)?;
	let map: HashMap<ContractParameter, ContractParameter> = deserialized_vector
		.into_iter()
		.map(|(k, v)| (serde_json::from_str(&k).unwrap(), v))
		.collect();
	Ok(map)
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Clone, Default, Debug, Serialize, Deserialize)]
	struct TestStruct {
		#[serde(serialize_with = "serialize_hashset_u256")]
		#[serde(deserialize_with = "deserialize_hashset_u256")]
		value: HashSet<U256>,
	}

	#[derive(Clone, Default, Debug, Serialize)]
	struct TestStruct2 {
		#[serde(serialize_with = "serialize_hashmap_u256_hashset_u256")]
		value2: HashMap<U256, HashSet<U256>>,
	}

	#[test]
	fn test_serialize_hashset_u256() {
		let mut v: HashSet<U256> = HashSet::new();
		v.insert(10.into());
		v.insert(0x10000.into());
		let _copy = v.clone();
		let test_struct = TestStruct { value: v };
		let json_string = serde_json::to_string_pretty(&test_struct).unwrap();
		println!("{}", json_string);
		let v_copy: TestStruct = serde_json::from_str(&json_string).unwrap();
		assert_eq!(test_struct.value, v_copy.value);
	}

	#[test]
	fn test_serialize_hashmap_u256_hashset_u256() {
		let mut v: HashMap<U256, HashSet<U256>> = HashMap::new();
		let mut v2: HashSet<U256> = HashSet::new();
		v2.insert(10.into());
		v2.insert(0x10000.into());
		v.insert(20.into(), v2);
		let test_struct = TestStruct2 { value2: v };
		let json_string = serde_json::to_string_pretty(&test_struct).unwrap();
		println!("{}", json_string);
	}

	#[test]
	fn test_serialize_bytes() {
		#[derive(Clone, Default, Debug, Serialize, Deserialize)]
		struct TestStruct {
			#[serde(serialize_with = "serialize_bytes")]
			#[serde(deserialize_with = "deserialize_bytes")]
			value: Vec<u8>,
		}

		let v = TestStruct { value: vec![23, 253, 255, 255, 0, 123] };
		let json_string = serde_json::to_string_pretty(&v).unwrap();
		println!("{}", json_string);
		let v_copy: TestStruct = serde_json::from_str(&json_string).unwrap();
		assert_eq!(v.value, v_copy.value);
	}

	#[test]
	fn test_serialize_u32() {
		#[derive(Clone, Default, Debug, Serialize, Deserialize)]
		struct TestStruct {
			#[serde(serialize_with = "serialize_u32")]
			#[serde(deserialize_with = "deserialize_u32")]
			value: u32,
		}

		let v = TestStruct { value: 20 };
		let json_string = serde_json::to_string_pretty(&v).unwrap();
		println!("{}", json_string);
		let v_copy: TestStruct = serde_json::from_str(&json_string).unwrap();
		assert_eq!(v.value, v_copy.value);
	}

	#[test]
	fn test_serialize_vec_h256() {
		#[derive(Clone, Default, Debug, Serialize, Deserialize)]
		struct TestStruct {
			#[serde(serialize_with = "serialize_vec_h256")]
			#[serde(deserialize_with = "deserialize_vec_h256")]
			value: Vec<H256>,
		}

		let v = TestStruct {
			value: vec![parse_string_h256(
				"0x95ff99bcdac06fad4a141f06c5f9f1c65e71b188ff5978116a110c4170fd7355",
			)],
		};
		let json_string = serde_json::to_string_pretty(&v).unwrap();
		println!("{}", json_string);
		let v_copy: TestStruct = serde_json::from_str(&json_string).unwrap();
		assert_eq!(v.value, v_copy.value);
	}
}
