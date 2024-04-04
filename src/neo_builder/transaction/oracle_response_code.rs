use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
	Display,
	EnumString,
	TryFromPrimitive,
	IntoPrimitive,
	Serialize,
	Deserialize,
	PartialEq,
	Eq,
	Copy,
	Clone,
	Hash,
	Debug,
)]
#[repr(u8)]
pub enum OracleResponseCode {
	#[strum(serialize = "Success")]
	Success = 0x00,
	#[strum(serialize = "ProtocolNotSupported")]
	ProtocolNotSupported = 0x10,
	#[strum(serialize = "ConsensusUnreachable")]
	ConsensusUnreachable = 0x12,
	#[strum(serialize = "NotFound")]
	NotFound = 0x14,
	#[strum(serialize = "Timeout")]
	Timeout = 0x16,
	#[strum(serialize = "Forbidden")]
	Forbidden = 0x18,
	#[strum(serialize = "ResponseTooLarge")]
	ResponseTooLarge = 0x1A,
	#[strum(serialize = "InsufficientFunds")]
	InsufficientFunds = 0x1C,
	#[strum(serialize = "ContentTypeNotSupported")]
	ContentTypeNotSupported = 0x1F,
	#[strum(serialize = "Error")]
	Error = 0xFF,
}
