use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{AsRefStr, Display, EnumCount, EnumIter, EnumString, IntoStaticStr};

#[derive(
	EnumString,
	IntoStaticStr,
	AsRefStr,
	EnumCount,
	EnumIter,
	Display,
	Copy,
	Clone,
	Debug,
	PartialEq,
	Eq,
	TryFromPrimitive,
	IntoPrimitive,
)]
#[repr(u8)]
pub enum RecordType {
	#[strum(serialize = "A")]
	A = 1,
	#[strum(serialize = "CNAME")]
	CNAME = 5,
	#[strum(serialize = "TXT")]
	TXT = 16,
	#[strum(serialize = "AAAA")]
	AAAA = 28,
}

impl RecordType {
	pub fn byte_repr(self) -> u8 {
		self as u8
	}
}
