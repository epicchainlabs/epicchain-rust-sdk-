use num_enum::TryFromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
	Display,
	EnumString,
	Debug,
	Clone,
	Hash,
	Copy,
	PartialEq,
	Eq,
	TryFromPrimitive,
	Serialize,
	Deserialize,
)]
#[repr(u8)]
pub enum ContractParameterType {
	#[strum(serialize = "Any")]
	Any = 0x00,
	#[strum(serialize = "Boolean")]
	Boolean = 0x10,
	#[strum(serialize = "Integer")]
	Integer = 0x11,
	#[strum(serialize = "ByteArray")]
	ByteArray = 0x12,
	#[strum(serialize = "String")]
	String = 0x13,
	#[strum(serialize = "H160")]
	H160 = 0x14,
	#[strum(serialize = "H256")]
	H256 = 0x15,
	#[strum(serialize = "PublicKey")]
	PublicKey = 0x16,
	#[strum(serialize = "Signature")]
	Signature = 0x17,
	#[strum(serialize = "Array")]
	Array = 0x20,
	#[strum(serialize = "Map")]
	Map = 0x22,
	#[strum(serialize = "InteropInterface")]
	InteropInterface = 0x30,
	#[strum(serialize = "Void")]
	Void = 0xff,
}
