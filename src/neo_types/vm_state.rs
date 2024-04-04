use num_enum::TryFromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Represents the state of a virtual machine.
#[derive(
	Display,
	EnumString,
	Debug,
	Clone,
	Copy,
	Hash,
	PartialEq,
	Eq,
	TryFromPrimitive,
	Serialize,
	Deserialize,
)]
#[repr(u8)]
pub enum VMState {
	/// The virtual machine is in the "NONE" state.
	#[strum(serialize = "NONE")]
	None = 0,
	/// The virtual machine is in the "HALT" state.
	#[strum(serialize = "HALT")]
	Halt = 1,
	/// The virtual machine is in the "FAULT" state.
	#[strum(serialize = "FAULT")]
	Fault = 2,
	/// The virtual machine is in the "BREAK" state.
	#[strum(serialize = "BREAK")]
	Break = 4,
}
