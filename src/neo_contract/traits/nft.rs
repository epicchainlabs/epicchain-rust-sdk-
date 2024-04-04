use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use primitive_types::H160;

use neo::prelude::{
	Account, AccountSigner, AccountTrait, Address, Bytes, ContractError, ContractParameter,
	JsonRpcClient, NNSName, NeoIterator, NftContract, ScriptHash, ScriptHashExtension, StackItem,
	TokenTrait, TransactionBuilder,
};

#[async_trait]
pub trait NonFungibleTokenTrait<'a, P: JsonRpcClient>: TokenTrait<'a, P> + Send {
	const OWNER_OF: &'static str = "ownerOf";
	const TOKENS_OF: &'static str = "tokensOf";
	const BALANCE_OF: &'static str = "balanceOf";
	const TRANSFER: &'static str = "transfer";
	const TOKENS: &'static str = "tokens";
	const PROPERTIES: &'static str = "properties";

	// Token methods

	async fn balance_of(&mut self, owner: H160) -> Result<i32, ContractError> {
		self.call_function_returning_int(
			<NftContract<P> as NonFungibleTokenTrait<P>>::BALANCE_OF,
			vec![owner.into()],
		)
		.await
	}

	// NFT methods

	async fn tokens_of(&mut self, owner: H160) -> Result<NeoIterator<Bytes, P>, ContractError> {
		let mapper_fn = Arc::new(|item: StackItem| item.as_bytes().unwrap());
		Ok(self
			.call_function_returning_iterator(
				<NftContract<P> as NonFungibleTokenTrait<P>>::TOKENS_OF,
				vec![owner.into()],
				mapper_fn,
			)
			.await)
	}

	// Non-divisible NFT methods

	async fn transfer(
		&mut self,
		from: &Account,
		to: ScriptHash,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		let mut builder = self.transfer_inner(to, token_id, data).await.unwrap();
		&builder.set_signers(vec![AccountSigner::called_by_entry(from).unwrap().into()]);

		Ok(builder)
	}

	async fn transfer_inner(
		&mut self,
		to: ScriptHash,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<Self::P>, ContractError> {
		self.throw_if_divisible_nft().await.unwrap();
		self.invoke_function(
			<NftContract<P> as NonFungibleTokenTrait<P>>::TRANSFER,
			vec![to.into(), token_id.into(), data.unwrap()],
		)
		.await
	}

	async fn transfer_from_name(
		&mut self,
		from: &Account,
		to: &str,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.throw_if_sender_is_not_owner(&from.get_script_hash(), &token_id)
			.await
			.unwrap();

		let mut build = self
			.transfer_inner(ScriptHash::from_address(to).unwrap(), token_id, data)
			.await
			.unwrap();
		build.set_signers(vec![AccountSigner::called_by_entry(from).unwrap().into()]);

		Ok(build)
	}

	async fn transfer_to_name(
		&mut self,
		to: &str,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.throw_if_divisible_nft().await.unwrap();

		self.transfer_inner(
			self.resolve_nns_text_record(&NNSName::new(to).unwrap()).await.unwrap(),
			token_id,
			data,
		)
		.await
	}

	async fn build_non_divisible_transfer_script(
		&mut self,
		to: Address,
		token_id: Bytes,
		data: ContractParameter,
	) -> Result<Bytes, ContractError> {
		self.throw_if_divisible_nft().await.unwrap();

		self.build_invoke_function_script(
			<NftContract<P> as NonFungibleTokenTrait<P>>::TRANSFER,
			vec![to.into(), token_id.into(), data],
		)
		.await
	}

	async fn owner_of(&mut self, token_id: Bytes) -> Result<H160, ContractError> {
		self.throw_if_divisible_nft().await.unwrap();

		self.call_function_returning_script_hash(
			<NftContract<P> as NonFungibleTokenTrait<P>>::OWNER_OF,
			vec![token_id.into()],
		)
		.await
	}

	async fn throw_if_divisible_nft(&mut self) -> Result<(), ContractError> {
		if self.get_decimals().await.unwrap() != 0 {
			return Err(ContractError::InvalidStateError(
				"This method is only intended for non-divisible NFTs.".to_string(),
			))
		}

		Ok(())
	}

	async fn throw_if_sender_is_not_owner(
		&mut self,
		from: &ScriptHash,
		token_id: &Bytes,
	) -> Result<(), ContractError> {
		let token_owner = &self.owner_of(token_id.clone()).await.unwrap();
		if token_owner != from {
			return Err(ContractError::InvalidArgError(
				"The provided from account is not the owner of this token.".to_string(),
			))
		}

		Ok(())
	}

	// Divisible NFT methods

	async fn transfer_divisible(
		&mut self,
		from: &Account,
		to: &ScriptHash,
		amount: i32,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		let mut builder = self
			.transfer_divisible_from_hashes(&from.get_script_hash(), to, amount, token_id, data)
			.await
			.unwrap();
		builder.set_signers(vec![AccountSigner::called_by_entry(from).unwrap().into()]);
		Ok(builder)
	}

	async fn transfer_divisible_from_hashes(
		&mut self,
		from: &ScriptHash,
		to: &ScriptHash,
		amount: i32,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.throw_if_non_divisible_nft().await.unwrap();

		self.invoke_function(
			<NftContract<P> as NonFungibleTokenTrait<P>>::TRANSFER,
			vec![from.into(), to.into(), amount.into(), token_id.into(), data.unwrap()],
		)
		.await
	}

	async fn transfer_divisible_from_name(
		&mut self,
		from: &Account,
		to: &str,
		amount: i32,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		let mut builder = self
			.transfer_divisible_from_hashes(
				&from.get_script_hash(),
				&self.resolve_nns_text_record(&NNSName::new(to).unwrap()).await.unwrap(),
				amount,
				token_id,
				data,
			)
			.await
			.unwrap();
		builder.set_signers(vec![AccountSigner::called_by_entry(from).unwrap().into()]);
		Ok(builder)
	}

	async fn transfer_divisible_to_name(
		&mut self,
		from: &ScriptHash,
		to: &str,
		amount: i32,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<P>, ContractError> {
		self.throw_if_non_divisible_nft().await.unwrap();

		self.transfer_divisible_from_hashes(
			from,
			&self.resolve_nns_text_record(&NNSName::new(to).unwrap()).await.unwrap(),
			amount,
			token_id,
			data,
		)
		.await
	}

	async fn build_divisible_transfer_script(
		&self,
		from: Address,
		to: Address,
		amount: i32,
		token_id: Bytes,
		data: Option<ContractParameter>,
	) -> Result<Bytes, ContractError> {
		self.build_invoke_function_script(
			<NftContract<P> as NonFungibleTokenTrait<P>>::TRANSFER,
			vec![from.into(), to.into(), amount.into(), token_id.into(), data.unwrap()],
		)
		.await
	}

	async fn owners_of(
		&mut self,
		token_id: Bytes,
	) -> Result<NeoIterator<Address, P>, ContractError> {
		self.throw_if_non_divisible_nft().await.unwrap();

		Ok(self
			.call_function_returning_iterator(
				<NftContract<P> as NonFungibleTokenTrait<P>>::OWNER_OF,
				vec![token_id.into()],
				Arc::new(|item: StackItem| item.as_address().unwrap()),
			)
			.await)
	}

	async fn throw_if_non_divisible_nft(&mut self) -> Result<(), ContractError> {
		if self.get_decimals().await.unwrap() == 0 {
			return Err(ContractError::InvalidStateError(
				"This method is only intended for divisible NFTs.".to_string(),
			))
		}

		Ok(())
	}

	async fn balance_of_divisible(
		&mut self,
		owner: H160,
		token_id: Bytes,
	) -> Result<i32, ContractError> {
		self.throw_if_non_divisible_nft().await.unwrap();

		self.call_function_returning_int(
			<NftContract<P> as NonFungibleTokenTrait<P>>::BALANCE_OF,
			vec![owner.into(), token_id.into()],
		)
		.await
	}

	// Optional methods

	async fn tokens(&mut self) -> Result<NeoIterator<Bytes, P>, ContractError> {
		Ok(self
			.call_function_returning_iterator(
				<NftContract<P> as NonFungibleTokenTrait<P>>::TOKENS,
				vec![],
				Arc::new(|item: StackItem| item.as_bytes().unwrap()),
			)
			.await)
	}

	async fn properties(
		&mut self,
		token_id: Bytes,
	) -> Result<HashMap<String, String>, ContractError> {
		let invocation_result = self
			.call_invoke_function(
				<NftContract<P> as NonFungibleTokenTrait<P>>::PROPERTIES,
				vec![token_id.into()],
				vec![],
			)
			.await
			.unwrap();

		let stack_item = invocation_result.get_first_stack_item().unwrap();
		let map = stack_item
			.as_map()
			.ok_or(ContractError::UnexpectedReturnType(
				stack_item.to_string() + &StackItem::MAP_VALUE.to_string(),
			))
			.unwrap();

		map.iter()
			.map(|(k, v)| {
				let key = k.as_string().unwrap();
				let value = v.as_string().unwrap();
				Ok((key, value))
			})
			.collect()
	}

	async fn custom_properties(
		&mut self,
		token_id: Bytes,
	) -> Result<HashMap<String, StackItem>, ContractError> {
		let invocation_result = self
			.call_invoke_function(
				<NftContract<P> as NonFungibleTokenTrait<P>>::PROPERTIES,
				vec![token_id.into()],
				vec![],
			)
			.await
			.unwrap();

		let stack_item = invocation_result.get_first_stack_item().unwrap();
		let map = stack_item
			.as_map()
			.ok_or(ContractError::UnexpectedReturnType(
				stack_item.to_string() + &StackItem::MAP_VALUE.to_string(),
			))
			.unwrap();

		map.into_iter()
			.map(|(k, v)| {
				let key = k.as_string().unwrap();
				Ok((key, v.clone()))
			})
			.collect()
	}
}
