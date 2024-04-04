use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
	Display,
	EnumString,
	TryFromPrimitive,
	Copy,
	Clone,
	Debug,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
)]
#[repr(u8)]
pub enum WitnessAction {
	#[strum(serialize = "Deny")]
	Deny = 0,
	#[strum(serialize = "Allow")]
	Allow = 1,
}
