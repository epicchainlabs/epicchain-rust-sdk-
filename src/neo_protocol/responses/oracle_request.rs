use crate::neo::prelude::{
	deserialize_h256, deserialize_script_hash, serialize_h256, serialize_script_hash,
};
use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct OracleRequest {
	#[serde(rename = "requestid")]
	pub request_id: i32,

	#[serde(rename = "originaltxid")]
	#[serde(deserialize_with = "deserialize_h256")]
	#[serde(serialize_with = "serialize_h256")]
	pub original_transaction_hash: H256,

	#[serde(rename = "gasforresponse")]
	pub gas_for_response: i32,

	pub url: String,

	pub filter: String,

	#[serde(rename = "callbackcontract")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub callback_contract: H160,

	#[serde(rename = "callbackmethod")]
	pub callback_method: String,

	#[serde(rename = "userdata")]
	pub user_data: String,
}

impl OracleRequest {
	pub fn new(
		request_id: i32,
		original_transaction_hash: H256,
		gas_for_response: i32,
		url: String,
		filter: String,
		callback_contract: H160,
		callback_method: String,
		user_data: String,
	) -> Self {
		Self {
			request_id,
			original_transaction_hash,
			gas_for_response,
			url,
			filter,
			callback_contract,
			callback_method,
			user_data,
		}
	}
}
