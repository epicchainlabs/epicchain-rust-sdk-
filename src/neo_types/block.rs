use std::{
	fmt::{Formatter, Write},
	str::FromStr,
};

use ethereum_types::U64;
use primitive_types::H256;
use serde::{
	de::{MapAccess, Visitor},
	ser::SerializeStruct,
	Deserialize, Deserializer, Serialize, Serializer,
};

use neo::prelude::*;

pub trait TXTrait {
	fn hash(&self) -> H256;
}

/// A struct representing a block in the NEO blockchain.
#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct Block<TX, W> {
	/// The hash of the block.
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub hash: H256,
	/// The size of the block.
	pub size: u32,
	/// The version of the block.
	pub version: u32,
	/// The hash of the previous block in the blockchain.
	#[serde(rename = "previousblockhash")]
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub prev_block_hash: H256,
	/// The hash of the Merkle root of all transactions in the block.
	#[serde(rename = "merkleroot")]
	#[serde(serialize_with = "serialize_h256")]
	#[serde(deserialize_with = "deserialize_h256")]
	pub merkle_root_hash: H256,
	/// The timestamp of the block.
	pub time: u32,
	/// The index of the block.
	pub index: u32,
	/// The index of the primary node that produced the block.
	pub primary: Option<u32>,
	/// The address of the next consensus node.
	#[serde(rename = "nextconsensus")]
	pub next_consensus: String,
	/// The list of witnesses for the block.
	pub witnesses: Option<Vec<W>>,
	/// The list of transactions in the block.
	#[serde(rename = "tx")]
	pub transactions: Option<Vec<TX>>,
	/// The number of confirmations for the block.
	pub confirmations: u32,
	/// The hash of the next block in the blockchain.
	#[serde(rename = "nextblockhash")]
	#[serde(serialize_with = "serialize_h256_option")]
	#[serde(deserialize_with = "deserialize_h256_option")]
	pub next_block_hash: Option<H256>,
}

impl<TX, W> PartialEq for Block<TX, W>
where
	TX: TXTrait,
{
	fn eq(&self, other: &Self) -> bool {
		// loop every tranactions and compare the hash of transactions
		if let Some(transactions) = &self.transactions {
			if let Some(other_transactions) = &other.transactions {
				if transactions.len() != other_transactions.len() {
					return false
				}
				for i in 0..transactions.len() {
					if transactions[i].hash() != other_transactions[i].hash() {
						return false
					}
				}
			}
		}
		self.hash == other.hash
	}
}

/// A [block hash](H256) or [block number](BlockNumber).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockId {
	/// A block hash
	Hash(H256),
	/// A block number
	Number(u64),
}

impl From<u64> for BlockId {
	fn from(num: u64) -> Self {
		num.into()
	}
}

impl From<U64> for BlockId {
	fn from(num: U64) -> Self {
		BlockId::Number(num.as_u64())
	}
}

impl From<H256> for BlockId {
	fn from(hash: H256) -> Self {
		BlockId::Hash(hash)
	}
}

impl Serialize for BlockId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match *self {
			BlockId::Hash(ref x) => {
				let mut s = serializer.serialize_struct("BlockIdEip1898", 1)?;
				s.serialize_field("blockHash", &format!("{x:?}"))?;
				s.end()
			},
			BlockId::Number(ref num) => num.serialize(serializer),
		}
	}
}

impl<'de> Deserialize<'de> for BlockId {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct BlockIdVisitor;

		impl<'de> Visitor<'de> for BlockIdVisitor {
			type Value = BlockId;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("Block identifier following EIP-1898")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				Ok(BlockId::Number(v.parse().map_err(serde::de::Error::custom)?))
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: MapAccess<'de>,
			{
				let mut number = None;
				let mut hash = None;

				while let Some(key) = map.next_key::<String>()? {
					match key.as_str() {
						"blockNumber" => {
							if number.is_some() || hash.is_some() {
								return Err(serde::de::Error::duplicate_field("blockNumber"))
							}
							number = Some(BlockId::Number(map.next_value::<u64>()?))
						},
						"blockHash" => {
							if number.is_some() || hash.is_some() {
								return Err(serde::de::Error::duplicate_field("blockHash"))
							}
							hash = Some(BlockId::Hash(map.next_value::<H256>()?))
						},
						key =>
							return Err(serde::de::Error::unknown_field(
								key,
								&["blockNumber", "blockHash"],
							)),
					}
				}

				number.or(hash).ok_or_else(|| {
					serde::de::Error::custom("Expected `blockNumber` or `blockHash`")
				})
			}
		}

		deserializer.deserialize_any(BlockIdVisitor)
	}
}

impl FromStr for BlockId {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.starts_with("0x") && s.len() == 66 {
			let hash = s.parse::<H256>().map_err(|e| e.to_string());
			hash.map(Self::Hash)
		} else {
			s.parse().map(Self::Number).map_err(|e| e.to_string())
		}
	}
}
