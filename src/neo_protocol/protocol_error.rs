use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
	#[error("RPC responses error: {error}")]
	RpcResponse { error: String },
	#[error("Invocation fault state: {error}")]
	InvocationFaultState { error: String },
	#[error("Client connection error: {message}")]
	ClientConnection { message: String },
	#[error("Cannot cast {item} to {target}")]
	StackItemCast { item: String, target: String },
	#[error("Illegal state: {message}")]
	IllegalState { message: String },
	#[error("HTTP error: {0}")]
	HttpError(#[from] reqwest::Error),
}
