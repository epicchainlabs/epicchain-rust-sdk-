pub struct NeoConstants {}
impl NeoConstants {
	// Accounts, Addresses, Keys
	pub const MAX_PUBLIC_KEYS_PER_MULTI_SIG: u32 = 1024;
	pub const HASH160_SIZE: u32 = 20;
	pub const HASH256_SIZE: u32 = 32;
	pub const PRIVATE_KEY_SIZE: u32 = 32;
	pub const PUBLIC_KEY_SIZE_COMPRESSED: u32 = 33;
	pub const SIGNATURE_SIZE: u32 = 64;
	pub const VERIFICATION_SCRIPT_SIZE: u32 = 40;
	pub const MAX_ITERATOR_ITEMS_DEFAULT: u32 = 100;

	pub const MAX_SUBITEMS: u32 = 16;
	pub const MAX_NESTING_DEPTH: u8 = 2;

	// Transactions & Contracts
	pub const CURRENT_TX_VERSION: u8 = 0;
	pub const MAX_TRANSACTION_SIZE: u32 = 102400;
	pub const MAX_TRANSACTION_ATTRIBUTES: u32 = 16;
	pub const MAX_SIGNER_SUBITEMS: u32 = 16;
	pub const MAX_MANIFEST_SIZE: u32 = 0xFFFF;

	// pub const DEFAULT_SCRYPT_PARAMS: Params = Params::new(14, 8, 8, 32).unwrap();

	pub const SEED_1: &'static str = "seed1.neo.org:10333";
	pub const SEED_2: &'static str = "seed2.neo.org:10333";
	pub const SEED_3: &'static str = "seed3.neo.org:10333";
	pub const SEED_4: &'static str = "seed4.neo.org:10333";
	pub const SEED_5: &'static str = "seed5.neo.org:10333";

	pub const SCRYPT_N: usize = 16384;
	pub const SCRYPT_R: u32 = 8;
	pub const SCRYPT_P: u32 = 8;
	pub const SCRYPT_LOG_N: u8 = 14;
	pub const SCRYPT_DK_LEN: usize = 64;

	pub const NEP_HEADER_1: u8 = 0x01;
	pub const NEP_HEADER_2: u8 = 0x42;
	pub const NEP_FLAG: u8 = 0xe0;

	pub fn new() -> Self {
		Self {}
	}
}
