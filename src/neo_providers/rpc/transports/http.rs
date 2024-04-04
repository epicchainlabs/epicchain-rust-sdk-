// Code adapted from: https://github.com/althea-net/guac_rs/tree/master/web3/src/jsonrpc

use std::{
	str::FromStr,
	sync::atomic::{AtomicU64, Ordering},
};

use async_trait::async_trait;
use neo::config::NeoConstants;
use reqwest::{header, Client, Error as ReqwestError};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use url::Url;

use neo::prelude::{JsonRpcClient, ProviderError, RpcError};

use super::common::{JsonRpcError, Request, Response};

/// A low-level JSON-RPC Client over HTTP.
///
/// # Example
///
/// ```no_run
/// use std::str::FromStr;
/// use primitive_types::H256;
/// use neo_rs::prelude::{Http, JsonRpcClient, Middleware, NeoConstants, Provider};
///
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = Provider::<Http>::try_from(
///     NeoConstants::SEED_1
/// ).expect("could not instantiate HTTP Provider");
/// let block_number = provider.get_block(H256::zero(), false).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct HttpProvider {
	id: AtomicU64,
	client: Client,
	url: Url,
}

#[derive(Error, Debug)]
/// Error thrown when sending an HTTP request
pub enum ClientError {
	/// Thrown if the request failed
	#[error(transparent)]
	ReqwestError(#[from] ReqwestError),
	#[error(transparent)]
	/// Thrown if the response could not be parsed
	JsonRpcError(#[from] JsonRpcError),

	#[error("Deserialization Error: {err}. Response: {text}")]
	/// Serde JSON Error
	SerdeJson {
		/// Underlying error
		err: serde_json::Error,
		/// The contents of the HTTP response that could not be deserialized
		text: String,
	},
}

impl From<ClientError> for ProviderError {
	fn from(src: ClientError) -> Self {
		match src {
			ClientError::ReqwestError(err) => ProviderError::HTTPError(err),
			_ => ProviderError::JsonRpcClientError(Box::new(src)),
		}
	}
}

impl RpcError for ClientError {
	fn as_error_response(&self) -> Option<&JsonRpcError> {
		if let ClientError::JsonRpcError(err) = self {
			Some(err)
		} else {
			None
		}
	}

	fn as_serde_error(&self) -> Option<&serde_json::Error> {
		match self {
			ClientError::SerdeJson { err, .. } => Some(err),
			_ => None,
		}
	}
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl JsonRpcClient for HttpProvider {
	type Error = ClientError;

	async fn fetch<T: Serialize + Send + Sync, R: DeserializeOwned>(
		&self,
		method: &str,
		params: T,
	) -> Result<R, ClientError> {
		let next_id = self.id.fetch_add(1, Ordering::SeqCst);
		let payload = Request::new(next_id, method, params);

		let res = self.client.post(self.url.as_ref()).json(&payload).send().await?;
		let body = res.bytes().await?;

		let raw = match serde_json::from_slice(&body) {
			Ok(Response::Success { result, .. }) => result.to_owned(),
			Ok(Response::Error { error, .. }) => return Err(error.into()),
			Ok(_) => {
				let err = ClientError::SerdeJson {
					err: serde::de::Error::custom("unexpected notification over HTTP transport"),
					text: String::from_utf8_lossy(&body).to_string(),
				};
				return Err(err)
			},
			Err(err) =>
				return Err(ClientError::SerdeJson {
					err,
					text: String::from_utf8_lossy(&body).to_string(),
				}),
		};

		let res = serde_json::from_str(raw.get())
			.map_err(|err| ClientError::SerdeJson { err, text: raw.to_string() })?;

		Ok(res)
	}
}

impl Default for HttpProvider {
	/// Default HTTP Provider from SEED_1
	fn default() -> Self {
		Self::new(Url::parse(NeoConstants::SEED_1).unwrap())
	}
}

impl HttpProvider {
	/// Initializes a new HTTP Client
	///
	/// # Example
	///
	/// ```
	/// use url::Url;
	/// use neo_rs::prelude::Http;
	///
	/// let url = Url::parse("http://localhost:8545").unwrap();
	/// let provider = Http::new(url);
	/// ```
	pub fn new(url: impl Into<Url>) -> Self {
		Self::new_with_client(url, Client::new())
	}

	/// The Url to which requests are made
	pub fn url(&self) -> &Url {
		&self.url
	}

	/// Mutable access to the Url to which requests are made
	pub fn url_mut(&mut self) -> &mut Url {
		&mut self.url
	}

	/// Initializes a new HTTP Client with authentication
	///
	/// # Example
	///
	/// ```
	/// use url::Url;
	/// use neo_rs::prelude::Http;
	///
	/// let url = Url::parse("http://localhost:8545").unwrap();
	/// let provider = Http::new(url);
	/// ```
	// pub fn new_with_auth(
	// 	url: impl Into<Url>,
	// 	auth: Authorization,
	// ) -> Result<Self, HttpClientError> {
	// 	let mut auth_value = HeaderValue::from_str(&auth.to_string())?;
	// 	auth_value.set_sensitive(true);
	//
	// 	let mut headers = reqwest::header::HeaderMap::new();
	// 	headers.insert(reqwest::header::AUTHORIZATION, auth_value);
	//
	// 	let client = Client::builder().default_headers(headers).build()?;
	//
	// 	Ok(Self::new_with_client(url, client))
	// }

	/// Allows to customize the provider by providing your own http client
	///
	/// # Example
	///
	/// ```
	/// use url::Url;
	/// use neo_rs::prelude::Http;
	///
	/// let url = Url::parse("http://localhost:8545").unwrap();
	/// let client = reqwest::Client::builder().build().unwrap();
	/// let provider = Http::new_with_client(url, client);
	/// ```
	pub fn new_with_client(url: impl Into<Url>, client: reqwest::Client) -> Self {
		Self { id: AtomicU64::new(1), client, url: url.into() }
	}
}

impl FromStr for HttpProvider {
	type Err = url::ParseError;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		let url = Url::parse(src)?;
		Ok(HttpProvider::new(url))
	}
}

impl Clone for HttpProvider {
	fn clone(&self) -> Self {
		Self { id: AtomicU64::new(1), client: self.client.clone(), url: self.url.clone() }
	}
}

#[derive(Error, Debug)]
/// Error thrown when dealing with Http clients
pub(crate) enum HttpClientError {
	/// Thrown if unable to build headers for client
	#[error(transparent)]
	InvalidHeader(#[from] header::InvalidHeaderValue),

	/// Thrown if unable to build client
	#[error(transparent)]
	ClientBuild(#[from] reqwest::Error),
}
