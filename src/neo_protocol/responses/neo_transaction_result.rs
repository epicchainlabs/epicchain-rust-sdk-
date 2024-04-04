use neo::prelude::{
	NeoVMStateType, NeoWitness, TransactionAttribute, TransactionSigner, WitnessRule, WitnessScope,
};
use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct TransactionResult {
	pub hash: H256,
	pub size: i32,
	pub version: i32,
	pub nonce: i32,
	pub sender: String,
	#[serde(rename = "sysfee")]
	pub sys_fee: String,
	#[serde(rename = "netfee")]
	pub net_fee: String,
	#[serde(rename = "validuntilblock")]
	pub valid_until_block: i32,
	pub signers: Vec<TransactionSigner>,
	pub attributes: Vec<TransactionAttribute>,
	pub script: String,
	pub witnesses: Vec<NeoWitness>,
	#[serde(rename = "blockhash")]
	pub block_hash: Option<H256>,
	pub confirmations: Option<i32>,
	#[serde(rename = "blocktime")]
	pub block_time: Option<i32>,
	#[serde(rename = "vmstate")]
	pub vm_state: Option<NeoVMStateType>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct NeoTransactionSigner {
	account: H160,
	scopes: Vec<WitnessScope>,
	allowed_contracts: Option<Vec<String>>,
	allowed_groups: Option<Vec<String>>,
	rules: Option<Vec<WitnessRule>>,
}
