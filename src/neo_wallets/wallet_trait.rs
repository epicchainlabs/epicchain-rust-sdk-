use primitive_types::H160;

use neo::prelude::{AccountTrait, ScryptParamsDef};

/// Represents the core functionalities of a cryptocurrency wallet.
///
/// This trait defines the common operations that a cryptocurrency wallet should support,
/// including access to account details, wallet metadata (like name and version), and
/// scrypt parameters for key derivation. It also provides methods for account management,
/// such as adding, removing, and setting a default account.
///
/// # Type Parameters
///
/// - `Account`: The specific type of account managed by the wallet, constrained by
///   the `AccountTrait` to ensure it adheres to the expected interface for accounts.
///
/// # Required Methods
///
/// - `name`, `version`: Accessors for the wallet's metadata.
/// - `scrypt_params`: Access to the wallet's key derivation parameters.
/// - `accounts`: Lists all accounts stored within the wallet.
/// - `default_account`: Retrieves the wallet's default account.
/// - `set_name`, `set_version`, `set_scrypt_params`, `set_default_account`: Mutators for
///   the wallet's properties.
/// - `add_account`, `remove_account`: Methods for account management within the wallet.
///
/// # Example
///
/// Implementing the `WalletTrait` for a simple wallet:
///
/// ```ignore
/// struct SimpleWallet {
///     name: String,
///     version: String,
///     scrypt_params: ScryptParamsDef,
///     accounts: Vec<Account>,
///     default_account: H160,
/// }
///
/// impl WalletTrait for SimpleWallet {
///     type Account = Account;
///
///     fn name(&self) -> &String {
///         &self.name
///     }
///
///     // Implementations for other methods follow...
/// }
/// ```
///
/// This trait allows for the abstraction over different wallet implementations,
/// facilitating the use of different key storage mechanisms, account management strategies,
/// and cryptographic algorithms.
pub trait WalletTrait {
	/// The type of account managed by the wallet.
	type Account: AccountTrait;

	/// Returns the name of the wallet.
	fn name(&self) -> &String;

	/// Returns the version of the wallet.
	fn version(&self) -> &String;

	/// Returns the scrypt parameters used for key derivation.
	fn scrypt_params(&self) -> &ScryptParamsDef;

	/// Returns a list of accounts stored in the wallet.
	fn accounts(&self) -> Vec<Self::Account>;

	/// Returns a reference to the default account of the wallet.
	fn default_account(&self) -> &Self::Account;

	/// Sets the name of the wallet.
	fn set_name(&mut self, name: String);

	/// Sets the version of the wallet.
	fn set_version(&mut self, version: String);

	/// Sets the scrypt parameters for the wallet.
	fn set_scrypt_params(&mut self, params: ScryptParamsDef);

	/// Sets the default account of the wallet.
	fn set_default_account(&mut self, default_account: H160);

	/// Adds a new account to the wallet.
	fn add_account(&mut self, account: Self::Account);

	/// Removes an account from the wallet by its hash.
	///
	/// Returns the removed account if it existed, or `None` otherwise.
	fn remove_account(&mut self, hash: &H160) -> Option<Self::Account>;
}
