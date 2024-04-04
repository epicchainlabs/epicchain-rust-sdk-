#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum CallFlags {
	None,
	ReadStates,
	WriteStates,
	AllowCall,
	AllowNotify,
	States,
	ReadOnly,
	All,
}

impl CallFlags {
	pub fn value(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::ReadStates => 0b00000001,
			Self::WriteStates => 0b00000010,
			Self::AllowCall => 0b00000100,
			Self::AllowNotify => 0b00001000,
			Self::States => Self::ReadStates.value() | Self::WriteStates.value(),
			Self::ReadOnly => Self::ReadStates.value() | Self::AllowCall.value(),
			Self::All => Self::States.value() | Self::AllowCall.value() | Self::AllowNotify.value(),
		}
	}

	pub fn from_value(value: u8) -> Result<Self, &'static str> {
		match value {
			0 => Ok(Self::None),
			0b00000001 => Ok(Self::ReadStates),
			0b00000010 => Ok(Self::WriteStates),
			0b00000100 => Ok(Self::AllowCall),
			0b00001000 => Ok(Self::AllowNotify),
			0b00000011 => Ok(Self::States),
			0b00000101 => Ok(Self::ReadOnly),
			0b00001111 => Ok(Self::All),
			_ => Err("Invalid value"),
		}
	}
}
