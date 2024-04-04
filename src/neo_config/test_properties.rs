pub struct TestConstants {}
impl TestConstants {
	// pub const TEST_SCRYPT_PARAMS: Params = Params::new(7, 8, 9, 32).unwrap();

	pub const TEST_RESOURCE_PATH: &'static str = "../../../test_resources/";
	// Default Account
	pub const DEFAULT_ACCOUNT_ADDRESS: &'static str = "NM7Aky765FG8NhhwtxjXRx7jEL1cnw7PBP";
	pub const DEFAULT_ACCOUNT_SCRIPT_HASH: &'static str =
		"69ecca587293047be4c59159bf8bc399985c160d";
	pub const DEFAULT_ACCOUNT_VERIFICATION_SCRIPT: &'static str =
		"0c21033a4d051b04b7fc0230d2b1aaedfd5a84be279a5361a7358db665ad7857787f1b4156e7b327";
	pub const DEFAULT_ACCOUNT_PUBLIC_KEY: &'static str =
		"033a4d051b04b7fc0230d2b1aaedfd5a84be279a5361a7358db665ad7857787f1b";
	pub const DEFAULT_ACCOUNT_PRIVATE_KEY: &'static str =
		"84180ac9d6eb6fba207ea4ef9d2200102d1ebeb4b9c07e2c6a738a42742e27a5";
	pub const DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY: &'static str =
		"6PYM7jHL4GmS8Aw2iEFpuaHTCUKjhT4mwVqdoozGU6sUE25BjV4ePXDdLz";
	pub const DEFAULT_ACCOUNT_WIF: &'static str =
		"L1eV34wPoj9weqhGijdDLtVQzUpWGHszXXpdU9dPuh2nRFFzFa7E";
	pub const DEFAULT_ACCOUNT_PASSWORD: &'static str = "neo";

	// Committee Account
	pub const COMMITTEE_ACCOUNT_ADDRESS: &'static str = "NXXazKH39yNFWWZF5MJ8tEN98VYHwzn7g3";
	pub const COMMITTEE_ACCOUNT_SCRIPT_HASH: &'static str =
		"05859de95ccbbd5668e0f055b208273634d4657f";
	pub const COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT: &'static str =
		"110c21033a4d051b04b7fc0230d2b1aaedfd5a84be279a5361a7358db665ad7857787f1b11419ed0dc3a";

	// Native Contracts
	pub const CONTRACT_MANAGEMENT_HASH: &'static str = "fffdc93764dbaddd97c48f252a53ea4643faa3fd";
	pub const STD_LIB_HASH: &'static str = "acce6fd80d44e1796aa0c2c625e9e4e0ce39efc0";
	pub const CRYPTO_LIB_HASH: &'static str = "726cb6e0cd8628a1350a611384688911ab75f51b";
	pub const LEDGER_CONTRACT_HASH: &'static str = "da65b600f7124ce6c79950c1772a36403104f2be";
	pub const NEO_TOKEN_HASH: &'static str = "ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5";
	pub const GAS_TOKEN_HASH: &'static str = "d2a4cff31913016155e38e474a2c06d08be276cf";
	pub const GAS_TOKEN_NAME: &'static str = "GasToken";
	pub const POLICY_CONTRACT_HASH: &'static str = "cc5e4edd9f5f8dba8bb65734541df7a1c081c67b";
	pub const ROLE_MANAGEMENT_HASH: &'static str = "49cf4e5378ffcd4dec034fd98a174c5491e395e2";
	pub const ORACLE_CONTRACT_HASH: &'static str = "fe924b7cfe89ddd271abaf7210a80a7e11178758";
	pub const NAME_SERVICE_HASH: &'static str = "7a8fcf0392cd625647907afa8e45cc66872b596b";

	// Client 1 Account
	pub const CLIENT1_ACCOUNT_WIF: &'static str =
		"L3cNMQUSrvUrHx1MzacwHiUeCWzqK2MLt5fPvJj9mz6L2rzYZpok";
}
