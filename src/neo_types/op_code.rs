// op_code
use getset::Getters;
use num_enum::TryFromPrimitive;
use strum_macros::{Display, EnumCount, EnumString};

#[derive(
	Display, EnumString, EnumCount, TryFromPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash,
)]
#[repr(u8)]
pub enum OpCode {
	#[strum(serialize = "PushInt8")]
	PushInt8 = 0x00,
	#[strum(serialize = "PushInt16")]
	PushInt16 = 0x01,
	#[strum(serialize = "PushInt32")]
	PushInt32 = 0x02,
	#[strum(serialize = "PushInt64")]
	PushInt64 = 0x03,
	#[strum(serialize = "PushInt128")]
	PushInt128 = 0x04,
	#[strum(serialize = "PushInt256")]
	PushInt256 = 0x05,
	#[strum(serialize = "PushTrue")]
	PushTrue = 0x08,
	#[strum(serialize = "PushFalse")]
	PushFalse = 0x09,
	#[strum(serialize = "PushA")]
	PushA = 0x0A,
	#[strum(serialize = "PushNull")]
	PushNull = 0x0B,
	#[strum(serialize = "PushData1")]
	PushData1 = 0x0C,
	#[strum(serialize = "PushData2")]
	PushData2 = 0x0D,
	#[strum(serialize = "PushData4")]
	PushData4 = 0x0E,
	#[strum(serialize = "PushM1")]
	PushM1 = 0x0F,
	#[strum(serialize = "Push0")]
	Push0 = 0x10,
	#[strum(serialize = "Push1")]
	Push1 = 0x11,
	#[strum(serialize = "Push2")]
	Push2 = 0x12,
	#[strum(serialize = "Push3")]
	Push3 = 0x13,
	#[strum(serialize = "Push4")]
	Push4 = 0x14,
	#[strum(serialize = "Push5")]
	Push5 = 0x15,
	#[strum(serialize = "Push6")]
	Push6 = 0x16,
	#[strum(serialize = "Push7")]
	Push7 = 0x17,
	#[strum(serialize = "Push8")]
	Push8 = 0x18,
	#[strum(serialize = "Push9")]
	Push9 = 0x19,
	#[strum(serialize = "Push10")]
	Push10 = 0x1A,
	#[strum(serialize = "Push11")]
	Push11 = 0x1B,
	#[strum(serialize = "Push12")]
	Push12 = 0x1C,
	#[strum(serialize = "Push13")]
	Push13 = 0x1D,
	#[strum(serialize = "Push14")]
	Push14 = 0x1E,
	#[strum(serialize = "Push15")]
	Push15 = 0x1F,
	#[strum(serialize = "Push16")]
	Push16 = 0x20,

	#[strum(serialize = "Nop")]
	Nop = 0x21,
	#[strum(serialize = "Jmp")]
	Jmp = 0x22,
	#[strum(serialize = "JmpL")]
	JmpL = 0x23,
	#[strum(serialize = "JmpIf")]
	JmpIf = 0x24,
	#[strum(serialize = "JmpIfL")]
	JmpIfL = 0x25,
	#[strum(serialize = "JmpIfNot")]
	JmpIfNot = 0x26,
	#[strum(serialize = "JmpIfNotL")]
	JmpIfNotL = 0x27,
	#[strum(serialize = "JmpEq")]
	JmpEq = 0x28,
	#[strum(serialize = "JmpEqL")]
	JmpEqL = 0x29,
	#[strum(serialize = "JmpNe")]
	JmpNe = 0x2A,
	#[strum(serialize = "JmpNeL")]
	JmpNeL = 0x2B,
	#[strum(serialize = "JmpGt")]
	JmpGt = 0x2C,
	#[strum(serialize = "JmpGtL")]
	JmpGtL = 0x2D,
	#[strum(serialize = "JmpGe")]
	JmpGe = 0x2E,
	#[strum(serialize = "JmpGeL")]
	JmpGeL = 0x2F,
	#[strum(serialize = "JmpLt")]
	JmpLt = 0x30,
	#[strum(serialize = "JmpLtL")]
	JmpLtL = 0x31,
	#[strum(serialize = "JmpLe")]
	JmpLe = 0x32,
	#[strum(serialize = "JmpLeL")]
	JmpLeL = 0x33,
	#[strum(serialize = "Call")]
	Call = 0x34,
	#[strum(serialize = "CallL")]
	CallL = 0x35,
	#[strum(serialize = "CallA")]
	CallA = 0x36,
	#[strum(serialize = "CallT")]
	CallT = 0x37,
	#[strum(serialize = "Abort")]
	Abort = 0x38,
	#[strum(serialize = "Assert")]
	Assert = 0x39,
	#[strum(serialize = "Throw")]
	Throw = 0x3A,
	#[strum(serialize = "Try")]
	Try = 0x3B,
	#[strum(serialize = "TryL")]
	TryL = 0x3C,
	#[strum(serialize = "EndTry")]
	EndTry = 0x3D,
	#[strum(serialize = "EndTryL")]
	EndTryL = 0x3E,
	#[strum(serialize = "EndFinally")]
	EndFinally = 0x3F,
	#[strum(serialize = "Ret")]
	Ret = 0x40,
	#[strum(serialize = "Syscall")]
	Syscall = 0x41,

	#[strum(serialize = "Depth")]
	Depth = 0x43,
	#[strum(serialize = "Drop")]
	Drop = 0x45,
	#[strum(serialize = "Nip")]
	Nip = 0x46,
	#[strum(serialize = "Xdrop")]
	Xdrop = 0x48,
	#[strum(serialize = "Clear")]
	Clear = 0x49,
	#[strum(serialize = "Dup")]
	Dup = 0x4A,
	#[strum(serialize = "Over")]
	Over = 0x4B,
	#[strum(serialize = "Pick")]
	Pick = 0x4D,
	#[strum(serialize = "Tuck")]
	Tuck = 0x4E,
	#[strum(serialize = "Swap")]
	Swap = 0x50,
	#[strum(serialize = "Rot")]
	Rot = 0x51,
	#[strum(serialize = "Roll")]
	Roll = 0x52,
	#[strum(serialize = "Reverse3")]
	Reverse3 = 0x53,
	#[strum(serialize = "Reverse4")]
	Reverse4 = 0x54,
	#[strum(serialize = "Reverse5")]
	ReverseN = 0x55,

	#[strum(serialize = "InitSSlot")]
	InitSSLot = 0x56,
	#[strum(serialize = "InitSlot")]
	InitSlot = 0x57,

	#[strum(serialize = "LdSFLd0")]
	LdSFLd0 = 0x58,
	#[strum(serialize = "LdSFLd1")]
	LdSFLd1 = 0x59,
	#[strum(serialize = "LdSFLd2")]
	LdSFLd2 = 0x5A,
	#[strum(serialize = "LdSFLd3")]
	LdSFLd3 = 0x5B,
	#[strum(serialize = "LdSFLd4")]
	LdSFLd4 = 0x5C,
	#[strum(serialize = "LdSFLd5")]
	LdSFLd5 = 0x5D,
	#[strum(serialize = "LdSFLd6")]
	LdSFLd6 = 0x5E,
	#[strum(serialize = "LdSFLd")]
	LdSFLd = 0x5F,

	#[strum(serialize = "StSFLd0")]
	StSFLd0 = 0x60,
	#[strum(serialize = "StSFLd1")]
	StSFLd1 = 0x61,
	#[strum(serialize = "StSFLd2")]
	StSFLd2 = 0x62,
	#[strum(serialize = "StSFLd3")]
	StSFLd3 = 0x63,
	#[strum(serialize = "StSFLd4")]
	StSFLd4 = 0x64,
	#[strum(serialize = "StSFLd5")]
	StSFLd5 = 0x65,
	#[strum(serialize = "StSFLd6")]
	StSFLd6 = 0x66,
	#[strum(serialize = "StSFLd")]
	StSFLd = 0x67,

	#[strum(serialize = "LdLoc0")]
	LdLoc0 = 0x68,
	#[strum(serialize = "LdLoc1")]
	LdLoc1 = 0x69,
	#[strum(serialize = "LdLoc2")]
	LdLoc2 = 0x6A,
	#[strum(serialize = "LdLoc3")]
	LdLoc3 = 0x6B,
	#[strum(serialize = "LdLoc4")]
	LdLoc4 = 0x6C,
	#[strum(serialize = "LdLoc5")]
	LdLoc5 = 0x6D,
	#[strum(serialize = "LdLoc6")]
	LdLoc6 = 0x6E,
	#[strum(serialize = "LdLoc")]
	LdLoc = 0x6F,

	#[strum(serialize = "StLoc0")]
	StLoc0 = 0x70,
	#[strum(serialize = "StLoc1")]
	StLoc1 = 0x71,
	#[strum(serialize = "StLoc2")]
	StLoc2 = 0x72,
	#[strum(serialize = "StLoc3")]
	StLoc3 = 0x73,
	#[strum(serialize = "StLoc4")]
	StLoc4 = 0x74,
	#[strum(serialize = "StLoc5")]
	StLoc5 = 0x75,
	#[strum(serialize = "StLoc6")]
	StLoc6 = 0x76,
	#[strum(serialize = "StLoc")]
	StLoc = 0x77,

	#[strum(serialize = "LdArg0")]
	LdArg0 = 0x78,
	#[strum(serialize = "LdArg1")]
	LdArg1 = 0x79,
	#[strum(serialize = "LdArg2")]
	LdArg2 = 0x7A,
	#[strum(serialize = "LdArg3")]
	LdArg3 = 0x7B,
	#[strum(serialize = "LdArg4")]
	LdArg4 = 0x7C,
	#[strum(serialize = "LdArg5")]
	LdArg5 = 0x7D,
	#[strum(serialize = "LdArg6")]
	LdArg6 = 0x7E,
	#[strum(serialize = "LdArg")]
	LdArg = 0x7F,

	#[strum(serialize = "StArg0")]
	StArg0 = 0x80,
	#[strum(serialize = "StArg1")]
	StArg1 = 0x81,
	#[strum(serialize = "StArg2")]
	StArg2 = 0x82,
	#[strum(serialize = "StArg3")]
	StArg3 = 0x83,
	#[strum(serialize = "StArg4")]
	StArg4 = 0x84,
	#[strum(serialize = "StArg5")]
	StArg5 = 0x85,
	#[strum(serialize = "StArg6")]
	StArg6 = 0x86,
	#[strum(serialize = "StArg")]
	StArg = 0x87,

	#[strum(serialize = "NewBuffer")]
	NewBuffer = 0x88,
	#[strum(serialize = "MemCpy")]
	MemCpy = 0x89,
	#[strum(serialize = "Cat")]
	Cat = 0x8B,
	#[strum(serialize = "Substr")]
	Substr = 0x8C,
	#[strum(serialize = "Left")]
	Left = 0x8D,
	#[strum(serialize = "Right")]
	Right = 0x8E,

	#[strum(serialize = "Invert")]
	Invert = 0x90,
	#[strum(serialize = "And")]
	And = 0x91,
	#[strum(serialize = "Or")]
	Or = 0x92,
	#[strum(serialize = "Xor")]
	Xor = 0x93,
	#[strum(serialize = "Equal")]
	Equal = 0x97,
	#[strum(serialize = "NotEqual")]
	NotEqual = 0x98,

	#[strum(serialize = "Sign")]
	Sign = 0x99,
	#[strum(serialize = "Abs")]
	Abs = 0x9A,
	#[strum(serialize = "Negate")]
	Negate = 0x9B,
	#[strum(serialize = "Inc")]
	Inc = 0x9C,
	#[strum(serialize = "Dec")]
	Dec = 0x9D,
	#[strum(serialize = "Add")]
	Add = 0x9E,
	#[strum(serialize = "Sub")]
	Sub = 0x9F,
	#[strum(serialize = "Mul")]
	Mul = 0xA0,
	#[strum(serialize = "Div")]
	Div = 0xA1,
	#[strum(serialize = "Mod")]
	Mod = 0xA2,
	#[strum(serialize = "Pow")]
	Pow = 0xA3,
	#[strum(serialize = "Sqrt")]
	Sqrt = 0xA4,
	#[strum(serialize = "ModMul")]
	ModMul = 0xA5,
	#[strum(serialize = "ModPow")]
	ModPow = 0xA6,
	#[strum(serialize = "Shl")]
	Shl = 0xA8,
	#[strum(serialize = "Shr")]
	Shr = 0xA9,
	#[strum(serialize = "Not")]
	Not = 0xAA,
	#[strum(serialize = "BoolAnd")]
	BoolAnd = 0xAB,
	#[strum(serialize = "BoolOr")]
	BoolOr = 0xAC,
	#[strum(serialize = "Nz")]
	Nz = 0xB1,
	#[strum(serialize = "NumEqual")]
	NumEqual = 0xB3,
	#[strum(serialize = "NumNotEqual")]
	NumNotEqual = 0xB4,
	#[strum(serialize = "Lt")]
	Lt = 0xB5,
	#[strum(serialize = "Le")]
	Le = 0xB6,
	#[strum(serialize = "Gt")]
	Gt = 0xB7,
	#[strum(serialize = "Ge")]
	Ge = 0xB8,
	#[strum(serialize = "Min")]
	Min = 0xB9,
	#[strum(serialize = "Max")]
	Max = 0xBA,
	#[strum(serialize = "Within")]
	Within = 0xBB,

	#[strum(serialize = "PackMap")]
	PackMap = 0xBE,
	#[strum(serialize = "PackStruct")]
	PackStruct = 0xBF,
	#[strum(serialize = "Pack")]
	Pack = 0xC0,
	#[strum(serialize = "Unpack")]
	Unpack = 0xC1,
	#[strum(serialize = "NewArray0")]
	NewArray0 = 0xC2,
	#[strum(serialize = "NewArray")]
	NewArray = 0xC3,
	#[strum(serialize = "NewArrayT")]
	NewArrayT = 0xC4,
	#[strum(serialize = "NewStruct0")]
	NewStruct0 = 0xC5,
	#[strum(serialize = "NewStruct")]
	NewStruct = 0xC6,
	#[strum(serialize = "NewMap")]
	NewMap = 0xC8,
	#[strum(serialize = "Size")]
	Size = 0xCA,
	#[strum(serialize = "HasKey")]
	HasKey = 0xCB,
	#[strum(serialize = "Keys")]
	Keys = 0xCC,
	#[strum(serialize = "Values")]
	Values = 0xCD,
	#[strum(serialize = "PickItem")]
	PickItem = 0xCE,
	#[strum(serialize = "Append")]
	Append = 0xCF,
	#[strum(serialize = "SetItem")]
	SetItem = 0xD0,
	#[strum(serialize = "ReverseItems")]
	ReverseItems = 0xD1,
	#[strum(serialize = "Remove")]
	Remove = 0xD2,
	#[strum(serialize = "ClearItems")]
	ClearItems = 0xD3,
	#[strum(serialize = "PopItem")]
	PopItem = 0xD4,

	#[strum(serialize = "IsNull")]
	IsNull = 0xD8,
	#[strum(serialize = "IsType")]
	IsType = 0xD9,
	#[strum(serialize = "Convert")]
	Convert = 0xDB,

	#[strum(serialize = "AbortMsg")]
	AbortMsg = 0xE0,
	#[strum(serialize = "AssertMsg")]
	AssertMsg = 0xE1,
}

impl OpCode {
	pub fn price(self) -> u32 {
		match self {
			OpCode::PushInt8
			| OpCode::PushInt16
			| OpCode::PushInt32
			| OpCode::PushInt64
			| OpCode::PushNull
			| OpCode::PushM1
			| OpCode::Push0
			| OpCode::Push1
			| OpCode::Push2
			| OpCode::Push3
			| OpCode::Push4
			| OpCode::Push5
			| OpCode::Push6
			| OpCode::Push7
			| OpCode::Push8
			| OpCode::Push9
			| OpCode::Push10
			| OpCode::Push11
			| OpCode::Push12
			| OpCode::Push13
			| OpCode::Push14
			| OpCode::Push15
			| OpCode::Push16
			| OpCode::Nop
			| OpCode::Assert => 1,
			OpCode::PushInt128
			| OpCode::PushInt256
			| OpCode::PushA
			| OpCode::Try
			| OpCode::Sign
			| OpCode::Abs
			| OpCode::Negate
			| OpCode::Inc
			| OpCode::Dec
			| OpCode::Not
			| OpCode::Nz
			| OpCode::Size => 1 << 2,
			OpCode::PushData1
			| OpCode::And
			| OpCode::Or
			| OpCode::Xor
			| OpCode::Add
			| OpCode::Sub
			| OpCode::Mul
			| OpCode::Div
			| OpCode::Mod
			| OpCode::Shl
			| OpCode::Shr
			| OpCode::BoolAnd
			| OpCode::BoolOr
			| OpCode::NumEqual
			| OpCode::NumNotEqual
			| OpCode::Lt
			| OpCode::Le
			| OpCode::Gt
			| OpCode::Ge
			| OpCode::Min
			| OpCode::Max
			| OpCode::Within
			| OpCode::NewMap => 1 << 3,
			OpCode::Xdrop
			| OpCode::Clear
			| OpCode::Roll
			| OpCode::ReverseN
			| OpCode::InitSSLot
			| OpCode::NewArray0
			| OpCode::NewStruct0
			| OpCode::Keys
			| OpCode::Remove
			| OpCode::ClearItems => 1 << 4,
			OpCode::Equal | OpCode::NotEqual | OpCode::ModMul => 1 << 5,
			OpCode::InitSlot | OpCode::Pow | OpCode::HasKey | OpCode::PickItem => 1 << 6,
			OpCode::NewBuffer => 1 << 8,
			OpCode::PushData2
			| OpCode::Call
			| OpCode::CallL
			| OpCode::CallA
			| OpCode::Throw
			| OpCode::NewArray
			| OpCode::NewArrayT
			| OpCode::NewStruct => 1 << 9,
			OpCode::MemCpy
			| OpCode::Cat
			| OpCode::Substr
			| OpCode::Left
			| OpCode::Right
			| OpCode::Sqrt
			| OpCode::ModPow
			| OpCode::PackMap
			| OpCode::PackStruct
			| OpCode::Pack
			| OpCode::Unpack => 1 << 11,
			OpCode::PushData4 => 1 << 12,
			OpCode::Values
			| OpCode::Append
			| OpCode::SetItem
			| OpCode::ReverseItems
			| OpCode::Convert => 1 << 13,
			OpCode::CallT => 1 << 15,
			OpCode::Abort | OpCode::Ret | OpCode::Syscall => 0,
			_ => 1 << 1,
		}
	}
	pub fn opcode(self) -> u8 {
		self as u8
	}

	pub fn to_string(self) -> String {
		format!("{:02X}", self as u8)
	}

	pub fn operand_size(self) -> Option<OperandSize> {
		match self {
			Self::PushInt8
			| Self::Jmp
			| Self::JmpIf
			| Self::JmpIfNot
			| Self::JmpEq
			| Self::JmpNe
			| Self::JmpGt
			| Self::JmpGe
			| Self::JmpLt
			| Self::JmpLe
			| Self::Call
			| Self::EndTry
			| Self::InitSSLot
			| Self::LdSFLd
			| Self::StSFLd
			| Self::LdLoc
			| Self::StLoc
			| Self::LdArg
			| Self::StArg
			| Self::NewArrayT
			| Self::IsType
			| Self::Convert => Some(OperandSize::with_size(1)),

			Self::PushInt16 | Self::CallT | Self::Try | Self::InitSlot =>
				Some(OperandSize::with_size(2)),

			Self::PushInt32
			| Self::PushA
			| Self::JmpL
			| Self::JmpIfL
			| Self::JmpIfNotL
			| Self::JmpEqL
			| Self::JmpNeL
			| Self::JmpGtL
			| Self::JmpGeL
			| Self::JmpLtL
			| Self::JmpLeL
			| Self::CallL
			| Self::EndTryL
			| Self::Syscall => Some(OperandSize::with_size(4)),

			Self::PushInt64 | Self::TryL => Some(OperandSize::with_size(8)),

			Self::PushInt128 => Some(OperandSize::with_size(16)),

			Self::PushInt256 => Some(OperandSize::with_size(32)),

			Self::PushData1 => Some(OperandSize::with_prefix_size(1)),
			Self::PushData2 => Some(OperandSize::with_prefix_size(2)),
			Self::PushData4 => Some(OperandSize::with_prefix_size(4)),

			_ => None,
		}
	}
}

// impl TryFrom<u8> for OpCode {
// 	type Error = ();
//
// 	fn try_from(value: u8) -> Result<Self, Self::Error> {
// 		match value {
// 			0x00 => Ok(OpCode::PushInt8),
// 			_ => Err(()),
// 		}
// 	}
// }

#[derive(Clone, Debug, Getters)]
pub struct OperandSize {
	#[get = "pub"]
	prefix_size: u8,
	#[get = "pub"]
	size: u8,
}

impl OperandSize {
	pub fn with_size(size: u8) -> Self {
		Self { prefix_size: 0, size }
	}

	pub fn with_prefix_size(prefix_size: u8) -> Self {
		Self { prefix_size, size: 0 }
	}
}
