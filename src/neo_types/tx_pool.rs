use std::{collections::BTreeMap, fmt, str::FromStr};

use ethereum_types::U64;
use primitive_types::U256;
use serde::{
	de::{self, Deserializer, Visitor},
	Deserialize, Serialize,
};

use neo::prelude::Address;

/// Transaction summary as found in the Txpool Inspection property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxPoolInspectSummary {
	/// Recipient (None when contract creation)
	pub to: Option<Address>,
	/// Transferred value
	pub value: U256,
	/// Gas amount
	pub gas: U256,
}

/// Visitor struct for TxpoolInspectSummary.
struct TxPoolInspectSummaryVisitor;

/// Walk through the deserializer to parse a txpool inspection summary into the
/// `TxpoolInspectSummary` struct.
impl<'de> Visitor<'de> for TxPoolInspectSummaryVisitor {
	type Value = TxPoolInspectSummary;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("to: value wei + gasLimit gas × gas_price wei")
	}

	fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		self.visit_str(&value)
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		let addr_split: Vec<&str> = value.split(": ").collect();
		if addr_split.len() != 2 {
			return Err(de::Error::custom("invalid format for TxpoolInspectSummary: to"))
		}
		let value_split: Vec<&str> = addr_split[1].split(" wei + ").collect();
		if value_split.len() != 2 {
			return Err(de::Error::custom("invalid format for TxpoolInspectSummary: gasLimit"))
		}
		let gas_split: Vec<&str> = value_split[1].split(" gas × ").collect();
		if gas_split.len() != 2 {
			return Err(de::Error::custom("invalid format for TxpoolInspectSummary: gas"))
		}

		let addr = match addr_split[0] {
			"" => None,
			"0x" => None,
			"contract creation" => None,
			addr =>
				Some(Address::from_str(addr.trim_start_matches("0x")).map_err(de::Error::custom)?),
		};
		let value = U256::from_dec_str(value_split[0]).map_err(de::Error::custom)?;
		let gas = U256::from_dec_str(gas_split[0]).map_err(de::Error::custom)?;

		Ok(TxPoolInspectSummary { to: addr, value, gas })
	}
}

/// Implement the `Deserialize` trait for `TxpoolInspectSummary` struct.
impl<'de> Deserialize<'de> for TxPoolInspectSummary {
	fn deserialize<D>(deserializer: D) -> Result<TxPoolInspectSummary, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(TxPoolInspectSummaryVisitor)
	}
}

/// Implement the `Serialize` trait for `TxpoolInspectSummary` struct so that the
/// format matches the one from geth.
impl Serialize for TxPoolInspectSummary {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let formatted_to = if let Some(to) = self.to.clone() {
			format!("{to:?}")
		} else {
			"contract creation".to_string()
		};
		let formatted = format!("{}: {} wei + {} gas", formatted_to, self.value, self.gas);
		serializer.serialize_str(&formatted)
	}
}

/// Transaction Pool Content
///
/// The content inspection property can be queried to list the exact details of all
/// the transactions currently pending for inclusion in the next block(s), as well
/// as the ones that are being scheduled for future execution only.
///
/// See [here](https://geth.neo.org/docs/rpc/ns-txpool#txpool_content) for more details
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxpoolContent<TX> {
	/// pending tx
	pub pending: BTreeMap<Address, BTreeMap<String, TX>>,
	/// queued tx
	pub queued: BTreeMap<Address, BTreeMap<String, TX>>,
}

/// Transaction Pool Inspect
///
/// The inspect inspection property can be queried to list a textual summary
/// of all the transactions currently pending for inclusion in the next block(s),
/// as well as the ones that are being scheduled for future execution only.
/// This is a method specifically tailored to developers to quickly see the
/// transactions in the pool and find any potential issues.
///
/// See [here](https://geth.neo.org/docs/rpc/ns-txpool#txpool_inspect) for more details
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxpoolInspect {
	/// pending tx
	pub pending: BTreeMap<Address, BTreeMap<String, TxPoolInspectSummary>>,
	/// queued tx
	pub queued: BTreeMap<Address, BTreeMap<String, TxPoolInspectSummary>>,
}

/// Transaction Pool Status
///
/// The status inspection property can be queried for the number of transactions
/// currently pending for inclusion in the next block(s), as well as the ones that
/// are being scheduled for future execution only.
///
/// See [here](https://geth.neo.org/docs/rpc/ns-txpool#txpool_status) for more details
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TxpoolStatus {
	/// number of pending tx
	pub pending: U64,
	/// number of queued tx
	pub queued: U64,
}
