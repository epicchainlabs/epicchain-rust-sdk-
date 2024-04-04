use async_trait::async_trait;
use num_traits::{real::Real, ToPrimitive};
use primitive_types::H160;
use rust_decimal::Decimal;

use neo::prelude::*;

#[async_trait]
pub trait TokenTrait<'a, P: JsonRpcClient>: SmartContractTrait<'a, P = P> {
	const TOTAL_SUPPLY: &'static str = "totalSupply";
	const SYMBOL: &'static str = "symbol";
	const DECIMALS: &'static str = "decimals";

	fn total_supply(&self) -> Option<u64>;

	fn set_total_supply(&mut self, total_supply: u64);

	fn decimals(&self) -> Option<u8>;

	fn set_decimals(&mut self, decimals: u8);

	fn symbol(&self) -> Option<String>;

	fn set_symbol(&mut self, symbol: String);

	async fn get_total_supply(&mut self) -> Result<u64, ContractError> {
		if let Some(supply) = &self.total_supply() {
			return Ok(supply.clone().into())
		}

		let supply =
			self.call_function_returning_int(Self::TOTAL_SUPPLY, vec![]).await.unwrap() as u64;

		self.set_total_supply(supply);
		Ok(supply)
	}

	async fn get_decimals(&mut self) -> Result<u8, ContractError> {
		if let Some(decimals) = &self.decimals() {
			return Ok(decimals.clone().into())
		}

		let decimals =
			self.call_function_returning_int(Self::DECIMALS, vec![]).await.unwrap() as u8;

		self.set_decimals(decimals);
		Ok(decimals)
	}

	// Other methods

	async fn get_symbol(&mut self) -> Result<String, ContractError> {
		if let Some(symbol) = &self.symbol() {
			return Ok(symbol.clone())
		}

		let symbol = self.call_function_returning_string(Self::SYMBOL, vec![]).await.unwrap();

		self.set_symbol(symbol.clone());
		Ok(symbol)
	}

	fn to_fractions(&self, amount: u64, decimals: u32) -> Result<i32, ContractError> {
		let scale = (amount as f64).log10().floor() as u32 + 1;
		if scale > decimals {
			return Err(ContractError::RuntimeError(
				"Amount has too many decimal points".to_string(),
			))
		}

		let scaled = Decimal::from(amount) * Decimal::from(10i32.pow(decimals));
		Ok(scaled.trunc().to_i32().unwrap())
	}

	fn to_fractions_decimal(&self, amount: Decimal, decimals: u32) -> Result<u64, ContractError> {
		if amount.scale() > decimals {
			return Err(ContractError::RuntimeError(
				"Amount has too many decimal places".to_string(),
			))
		}

		let mut scaled = amount;
		scaled *= Decimal::from(10_u32.pow(decimals));

		let fractions = scaled.trunc().to_u64().unwrap();
		Ok(fractions)
	}

	fn to_decimals_u64(&self, fractions: u64, decimals: u32) -> Decimal {
		let divisor = Decimal::from(10_u32.pow(decimals));
		let amount = Decimal::from(fractions);

		amount / divisor
	}

	fn to_decimals(&self, amount: i64, decimals: u32) -> Decimal {
		let divisor = Decimal::from(10_u32.pow(decimals));
		let decimal_amount = Decimal::from(amount);

		if decimals >= 0 {
			decimal_amount / divisor
		} else {
			decimal_amount * divisor
		}
	}

	async fn resolve_nns_text_record(&self, name: &NNSName) -> Result<H160, ContractError>;
	// {
	// 	let req = {
	// 		self.provider()
	// 			.unwrap()
	// 			.invoke_function(
	// 				&NeoNameService::new().script_hash(),
	// 				"resolve".to_string(),
	// 				vec![
	// 					ContractParameter::from(name.name()),
	// 					ContractParameter::from(RecordType::TXT.byte_repr()),
	// 				],
	// 				(),
	// 			)
	// 			.clone()
	// 	};
	//
	// 	let address = req.await.unwrap().stack.first().unwrap().clone();
	//
	//
	// 	Ok(H160::from_slice(&address.as_bytes().unwrap()))
	// }
}
