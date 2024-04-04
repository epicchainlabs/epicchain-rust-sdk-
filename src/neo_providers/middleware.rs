use std::{collections::HashMap, error::Error, fmt::Debug};

use async_trait::async_trait;
use auto_impl::auto_impl;
use primitive_types::{H160, H256};

use neo::prelude::{JsonRpcError, *};

/// [`MiddlewareError`] is a companion trait to [`crate::Middleware`]. It
/// describes error behavior that is common to all Middleware errors.
///
/// Like [`crate::Middleware`], it allows moving down through layered errors.
///
/// Like [`RpcError`] it exposes convenient accessors to useful underlying
/// error information.
///
///
/// ## Not to Devs:
/// While this trait includes the same methods as [`RpcError`], it is not a
/// supertrait. This is so that 3rd party developers do not need to learn and
/// implement both traits. We provide default methods that delegate to inner
/// middleware errors on the assumption that it will eventually reach a
/// [`ProviderError`], which has correct behavior. This allows Middleware devs
/// to ignore the methods' presence if they want. Middleware are already plenty
/// complicated and we don't need to make it worse :)
pub(crate) trait MiddlewareError: Error + Sized + Send + Sync {
	/// The `Inner` type is the next lower middleware layer's error type.
	type Inner: MiddlewareError;

	/// Convert the next lower middleware layer's error to this layer's error
	fn from_err(e: Self::Inner) -> Self;

	/// Attempt to convert this error to the next lower middleware's error.
	/// Conversion fails if the error is not from an inner layer (i.e. the
	/// error originates at this middleware layer)
	fn as_inner(&self) -> Option<&Self::Inner>;

	/// Returns `true` if the underlying error stems from a lower middleware
	/// layer
	fn is_inner(&self) -> bool {
		self.as_inner().is_some()
	}

	/// Access an underlying `serde_json` error (if any)
	///
	/// Attempts to access an underlying [`serde_json::Error`]. If the
	/// underlying error is not a serde_json error, this function will return
	/// `None`.
	///
	/// ### Implementor's Note:
	///
	/// When writing a custom middleware, if your middleware uses `serde_json`
	/// we recommend a custom implementation of this method. It should first
	/// check your Middleware's error for local `serde_json` errors, and then
	/// delegate to inner if none is found. Failing to implement this method may
	/// result in missed `serde_json` errors.
	fn as_serde_error(&self) -> Option<&serde_json::Error> {
		self.as_inner()?.as_serde_error()
	}

	/// Returns `true` if the underlying error is a serde_json (de)serialization
	/// error. This method can be used to identify
	fn is_serde_error(&self) -> bool {
		self.as_serde_error().is_some()
	}

	/// Attempts to access an underlying [`ProviderError`], usually by
	/// traversing the entire middleware stack. Access fails if the underlying
	/// error is not a [`ProviderError`]
	fn as_provider_error(&self) -> Option<&ProviderError> {
		self.as_inner()?.as_provider_error()
	}

	/// Convert a [`ProviderError`] to this type, by successively wrapping it
	/// in the error types of all lower middleware
	fn from_provider_err(p: ProviderError) -> Self {
		Self::from_err(Self::Inner::from_provider_err(p))
	}

	/// Access an underlying JSON-RPC error (if any)
	///
	/// Attempts to access an underlying [`JsonRpcError`]. If the underlying
	/// error is not a JSON-RPC error response, this function will return
	/// `None`.
	fn as_error_response(&self) -> Option<&JsonRpcError> {
		self.as_inner()?.as_error_response()
	}

	/// Returns `true` if the underlying error is a JSON-RPC error response
	fn is_error_response(&self) -> bool {
		self.as_error_response().is_some()
	}
}

#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait Middleware: Sync + Send + Debug {
	/// Error type returned by most operations
	type Error: MiddlewareError<Inner = <<Self as Middleware>::Inner as Middleware>::Error>;
	/// The JSON-RPC client type at the bottom of the stack
	type Provider: JsonRpcClient;
	/// The next-lower middleware in the middleware stack
	type Inner: Middleware<Provider = Self::Provider>;

	/// Get a reference to the next-lower middleware in the middleware stack
	fn inner(&self) -> &Self::Inner;

	/// Convert a provider error into the associated error type by successively
	/// converting it to every intermediate middleware error
	fn convert_err(p: ProviderError) -> Self::Error {
		Self::Error::from_provider_err(p)
	}

	/// The HTTP or Websocket provider.
	fn provider(&self) -> &Provider<Self::Provider> {
		self.inner().provider()
	}

	fn config(&self) -> &NeoConfig {
		&self.inner().config()
	}

	async fn network(&self) -> u32;

	fn nns_resolver(&self) -> H160 {
		H160::from(self.config().nns_resolver.clone())
	}

	fn block_interval(&self) -> u32 {
		self.config().block_interval
	}

	fn polling_interval(&self) -> u32 {
		self.config().polling_interval
	}

	fn max_valid_until_block_increment(&self) -> u32 {
		self.config().max_valid_until_block_increment
	}

	// Blockchain methods
	async fn get_best_block_hash(&self) -> Result<H256, Self::Error> {
		self.inner().get_best_block_hash().await.map_err(MiddlewareError::from_err)
	}

	async fn get_block_hash(&self, block_index: u32) -> Result<H256, Self::Error> {
		self.inner()
			.get_block_hash(block_index)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_block(&self, block_hash: H256, full_tx: bool) -> Result<NeoBlock, Self::Error> {
		self.inner()
			.get_block(block_hash, full_tx)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_raw_block(&self, block_hash: H256) -> Result<String, Self::Error> {
		self.inner().get_raw_block(block_hash).await.map_err(MiddlewareError::from_err)
	}

	// Node methods
	async fn get_block_header_count(&self) -> Result<u32, Self::Error> {
		self.inner().get_block_count().await.map_err(MiddlewareError::from_err)
	}

	async fn get_block_count(&self) -> Result<u32, Self::Error> {
		self.inner().get_block_count().await.map_err(MiddlewareError::from_err)
	}

	async fn get_block_header(&self, block_hash: H256) -> Result<NeoBlock, Self::Error> {
		self.inner()
			.get_block_header(block_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_block_header_by_index(&self, index: u32) -> Result<NeoBlock, Self::Error> {
		self.inner()
			.get_block_header_by_index(index)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// Smart contract methods

	async fn get_raw_block_header(&self, block_hash: H256) -> Result<String, Self::Error> {
		self.inner()
			.get_raw_block_header(block_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_raw_block_header_by_index(&self, index: u32) -> Result<String, Self::Error> {
		self.inner()
			.get_raw_block_header_by_index(index)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// Utility methods

	async fn get_native_contracts(&self) -> Result<Vec<NativeContractState>, Self::Error> {
		self.inner().get_native_contracts().await.map_err(MiddlewareError::from_err)
	}

	// Wallet methods

	async fn get_contract_state(&self, hash: H160) -> Result<ContractState, Self::Error> {
		self.inner().get_contract_state(hash).await.map_err(MiddlewareError::from_err)
	}

	async fn get_native_contract_state(&self, name: &str) -> Result<ContractState, Self::Error> {
		self.inner()
			.get_native_contract_state(name)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_mem_pool(&self) -> Result<MemPoolDetails, Self::Error> {
		self.inner().get_mem_pool().await.map_err(MiddlewareError::from_err)
	}

	async fn get_raw_mem_pool(&self) -> Result<Vec<H256>, Self::Error> {
		self.inner().get_raw_mem_pool().await.map_err(MiddlewareError::from_err)
	}

	// Application logs

	async fn get_transaction(&self, hash: H256) -> Result<Option<TransactionResult>, Self::Error> {
		self.inner().get_transaction(hash).await.map_err(MiddlewareError::from_err)
	}

	// State service

	async fn get_raw_transaction(&self, tx_hash: H256) -> Result<RawTransaction, Self::Error> {
		self.inner()
			.get_raw_transaction(tx_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_storage(&self, contract_hash: H160, key: &str) -> Result<String, Self::Error> {
		self.inner()
			.get_storage(contract_hash, key)
			.await
			.map_err(MiddlewareError::from_err)
	}
	// Blockchain methods

	async fn get_transaction_height(&self, tx_hash: H256) -> Result<u32, Self::Error> {
		self.inner()
			.get_transaction_height(tx_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_next_block_validators(&self) -> Result<Vec<Validator>, Self::Error> {
		self.inner()
			.get_next_block_validators()
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_committee(&self) -> Result<Vec<String>, Self::Error> {
		self.inner().get_committee().await.map_err(MiddlewareError::from_err)
	}

	async fn get_connection_count(&self) -> Result<u32, Self::Error> {
		self.inner().get_connection_count().await.map_err(MiddlewareError::from_err)
	}

	async fn get_peers(&self) -> Result<Peers, Self::Error> {
		self.inner().get_peers().await.map_err(MiddlewareError::from_err)
	}

	// Smart contract method
	async fn get_version(&self) -> Result<NeoVersion, Self::Error> {
		self.inner().get_version().await.map_err(MiddlewareError::from_err)
	}

	async fn send_raw_transaction(&self, hex: String) -> Result<RawTransaction, Self::Error> {
		self.inner().send_raw_transaction(hex).await.map_err(MiddlewareError::from_err)
	}

	async fn submit_block(&self, hex: String) -> Result<bool, Self::Error> {
		self.inner().submit_block(hex).await.map_err(MiddlewareError::from_err)
	}

	// Blockchain methods
	async fn invoke_function(
		&self,
		contract_hash: &H160,
		method: String,
		params: Vec<ContractParameter>,
		signers: Option<Vec<Signer>>,
	) -> Result<InvocationResult, Self::Error> {
		self.inner()
			.invoke_function(contract_hash, method, params, signers)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn invoke_script(
		&self,
		hex: String,
		signers: Vec<Signer>,
	) -> Result<InvocationResult, Self::Error> {
		self.inner()
			.invoke_script(hex, signers)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// More smart contract methods

	async fn get_unclaimed_gas(&self, hash: H160) -> Result<UnclaimedGas, Self::Error> {
		self.inner().get_unclaimed_gas(hash).await.map_err(MiddlewareError::from_err)
	}

	async fn list_plugins(&self) -> Result<Vec<Plugin>, Self::Error> {
		self.inner().list_plugins().await.map_err(MiddlewareError::from_err)
	}

	async fn validate_address(&self, address: &str) -> Result<ValidateAddress, Self::Error> {
		self.inner().validate_address(address).await.map_err(MiddlewareError::from_err)
	}

	// Wallet methods
	async fn close_wallet(&self) -> Result<bool, Self::Error> {
		self.inner().close_wallet().await.map_err(MiddlewareError::from_err)
	}

	async fn dump_priv_key(&self, script_hash: H160) -> Result<String, Self::Error> {
		self.inner().dump_priv_key(script_hash).await.map_err(MiddlewareError::from_err)
	}

	async fn get_wallet_balance(&self, token_hash: H160) -> Result<Balance, Self::Error> {
		self.inner()
			.get_wallet_balance(token_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_new_address(&self) -> Result<String, Self::Error> {
		self.inner().get_new_address().await.map_err(MiddlewareError::from_err)
	}

	async fn get_wallet_unclaimed_gas(&self) -> Result<String, Self::Error> {
		self.inner().get_wallet_unclaimed_gas().await.map_err(MiddlewareError::from_err)
	}

	async fn import_priv_key(&self, priv_key: String) -> Result<NeoAddress, Self::Error> {
		self.inner().import_priv_key(priv_key).await.map_err(MiddlewareError::from_err)
	}

	async fn calculate_network_fee(&self, hex: String) -> Result<u64, Self::Error> {
		self.inner().calculate_network_fee(hex).await.map_err(MiddlewareError::from_err)
	}

	async fn list_address(&self) -> Result<Vec<NeoAddress>, Self::Error> {
		self.inner().list_address().await.map_err(MiddlewareError::from_err)
	}
	async fn open_wallet(&self, path: String, password: String) -> Result<bool, Self::Error> {
		self.inner()
			.open_wallet(path, password)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn send_from(
		&self,
		token_hash: H160,
		from: Address,
		to: Address,
		amount: u32,
	) -> Result<Transaction<Self::Provider>, Self::Error> {
		self.inner()
			.send_from(token_hash, from, to, amount)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// Transaction methods

	async fn send_many(
		&self,
		from: Option<H160>,
		send_tokens: Vec<TransactionSendToken>,
	) -> Result<Transaction<Self::Provider>, Self::Error> {
		self.inner()
			.send_many(from, send_tokens)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn send_to_address(
		&self,
		token_hash: H160,
		to: Address,
		amount: u32,
	) -> Result<Transaction<Self::Provider>, Self::Error> {
		self.inner()
			.send_to_address(token_hash, to, amount)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_application_log(&self, tx_hash: H256) -> Result<ApplicationLog, Self::Error> {
		self.inner()
			.get_application_log(tx_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep17_balances(&self, script_hash: H160) -> Result<Nep17Balances, Self::Error> {
		self.inner()
			.get_nep17_balances(script_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep17_transfers(&self, script_hash: H160) -> Result<Nep17Transfers, Self::Error> {
		self.inner()
			.get_nep17_transfers(script_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// NEP-17 methods

	async fn get_nep17_transfers_from(
		&self,
		script_hash: H160,
		from: u64,
	) -> Result<Nep17Transfers, Self::Error> {
		self.inner()
			.get_nep17_transfers_from(script_hash, from)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep17_transfers_range(
		&self,
		script_hash: H160,
		from: u64,
		to: u64,
	) -> Result<Nep17Transfers, Self::Error> {
		self.inner()
			.get_nep17_transfers_range(script_hash, from, to)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep11_balances(&self, script_hash: H160) -> Result<Nep11Balances, Self::Error> {
		self.inner()
			.get_nep11_balances(script_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// NEP-11 methods

	async fn get_nep11_transfers(&self, script_hash: H160) -> Result<Nep11Transfers, Self::Error> {
		self.inner()
			.get_nep11_transfers(script_hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep11_transfers_from(
		&self,
		script_hash: H160,
		from: u64,
	) -> Result<Nep11Transfers, Self::Error> {
		self.inner()
			.get_nep11_transfers_from(script_hash, from)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep11_transfers_range(
		&self,
		script_hash: H160,
		from: u64,
		to: u64,
	) -> Result<Nep11Transfers, Self::Error> {
		self.inner()
			.get_nep11_transfers_range(script_hash, from, to)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_nep11_properties(
		&self,
		script_hash: H160,
		token_id: &str,
	) -> Result<HashMap<String, String>, Self::Error> {
		self.inner()
			.get_nep11_properties(script_hash, token_id)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_state_root(&self, block_index: u32) -> Result<StateRoot, Self::Error> {
		self.inner()
			.get_state_root(block_index)
			.await
			.map_err(MiddlewareError::from_err)
	}

	// State service methods
	async fn get_proof(
		&self,
		root_hash: H256,
		contract_hash: H160,
		key: &str,
	) -> Result<String, Self::Error> {
		self.inner()
			.get_proof(root_hash, contract_hash, key)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn verify_proof(&self, root_hash: H256, proof: &str) -> Result<bool, Self::Error> {
		self.inner()
			.verify_proof(root_hash, proof)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_state_height(&self) -> Result<StateHeight, Self::Error> {
		self.inner().get_state_height().await.map_err(MiddlewareError::from_err)
	}

	async fn get_state(
		&self,
		root_hash: H256,
		contract_hash: H160,
		key: &str,
	) -> Result<String, Self::Error> {
		self.inner()
			.get_state(root_hash, contract_hash, key)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn find_states(
		&self,
		root_hash: H256,
		contract_hash: H160,
		key_prefix: &str,
		start_key: Option<&str>,
		count: Option<u32>,
	) -> Result<States, Self::Error> {
		self.inner()
			.find_states(root_hash, contract_hash, key_prefix, start_key, count)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_block_by_index(&self, index: u32, full_tx: bool) -> Result<NeoBlock, Self::Error> {
		self.inner()
			.get_block_by_index(index, full_tx)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_raw_block_by_index(&self, index: u32) -> Result<String, Self::Error> {
		self.inner()
			.get_raw_block_by_index(index)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn invoke_function_diagnostics(
		&self,
		contract_hash: H160,
		name: String,
		params: Vec<ContractParameter>,
		signers: Vec<Signer>,
	) -> Result<InvocationResult, Self::Error> {
		self.inner()
			.invoke_function_diagnostics(contract_hash, name, params, signers)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn invoke_script_diagnostics(
		&self,
		hex: String,
		signers: Vec<Signer>,
	) -> Result<InvocationResult, Self::Error> {
		self.inner()
			.invoke_script_diagnostics(hex, signers)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn traverse_iterator(
		&self,
		session_id: String,
		iterator_id: String,
		count: u32,
	) -> Result<Vec<StackItem>, Self::Error> {
		self.inner()
			.traverse_iterator(session_id, iterator_id, count)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn terminate_session(&self, session_id: &str) -> Result<bool, Self::Error> {
		self.inner()
			.terminate_session(session_id)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn invoke_contract_verify(
		&self,
		hash: H160,
		params: Vec<ContractParameter>,
		signers: Vec<Signer>,
	) -> Result<InvocationResult, Self::Error> {
		self.inner()
			.invoke_contract_verify(hash, params, signers)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn get_raw_mempool(&self) -> Result<MemPoolDetails, Self::Error> {
		self.inner().get_raw_mempool().await.map_err(MiddlewareError::from_err)
	}

	async fn import_private_key(&self, wif: String) -> Result<NeoAddress, Self::Error> {
		self.inner().import_private_key(wif).await.map_err(MiddlewareError::from_err)
	}

	async fn get_block_header_hash(&self, hash: H256) -> Result<NeoBlock, Self::Error> {
		self.inner()
			.get_block_header_hash(hash)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn send_to_address_send_token(
		&self,
		send_token: &TransactionSendToken,
	) -> Result<Transaction<Self::Provider>, Self::Error> {
		self.inner()
			.send_to_address_send_token(send_token)
			.await
			.map_err(MiddlewareError::from_err)
	}

	async fn send_from_send_token(
		&self,
		send_token: &TransactionSendToken,
		from: Address,
	) -> Result<Transaction<Self::Provider>, Self::Error> {
		self.inner()
			.send_from_send_token(send_token, from)
			.await
			.map_err(MiddlewareError::from_err)
	}
}
