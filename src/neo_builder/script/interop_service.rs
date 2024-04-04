use std::{
	collections::HashMap,
	hash::Hash,
	sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use neo::prelude::HashableForVec;

lazy_static! {
	static ref INTEROP_SERVICE_HASHES: Arc<Mutex<HashMap<String, String>>> =
		Arc::new(Mutex::new(HashMap::new()));
}

#[derive(EnumString, EnumIter, Display, Copy, Clone, PartialEq, Eq)]
pub enum InteropService {
	#[strum(serialize = "System.Crypto.CheckSig")]
	SystemCryptoCheckSig,
	#[strum(serialize = "System.Crypto.CheckMultiSig")]
	SystemCryptoCheckMultiSig,
	#[strum(serialize = "System.Contract.Call")]
	SystemContractCall,
	#[strum(serialize = "System.Contract.CallNative")]
	SystemContractCallNative,
	#[strum(serialize = "System.Contract.GetCallFlags")]
	SystemContractGetCallFlags,
	#[strum(serialize = "System.Contract.CreateStandardAccount")]
	SystemContractCreateStandardAccount,
	#[strum(serialize = "System.Contract.CreateMultiSigAccount")]
	SystemContractCreateMultiSigAccount,
	#[strum(serialize = "System.Contract.NativeOnPersist")]
	SystemContractNativeOnPersist,
	#[strum(serialize = "System.Contract.NativePostPersist")]
	SystemContractNativePostPersist,
	#[strum(serialize = "System.Iterator.Next")]
	SystemIteratorNext,
	#[strum(serialize = "System.Iterator.Value")]
	SystemIteratorValue,
	#[strum(serialize = "System.Runtime.Platform")]
	SystemRuntimePlatform,
	#[strum(serialize = "System.Runtime.GetTrigger")]
	SystemRuntimeGetTrigger,
	#[strum(serialize = "System.Runtime.GetTime")]
	SystemRuntimeGetTime,
	#[strum(serialize = "System.Runtime.GetScriptContainer")]
	SystemRuntimeGetScriptContainer,
	#[strum(serialize = "System.Runtime.GetExecutingScriptHash")]
	SystemRuntimeGetExecutingScriptHash,
	#[strum(serialize = "System.Runtime.GetCallingScriptHash")]
	SystemRuntimeGetCallingScriptHash,
	#[strum(serialize = "System.Runtime.GetEntryScriptHash")]
	SystemRuntimeGetEntryScriptHash,
	#[strum(serialize = "System.Runtime.CheckWitness")]
	SystemRuntimeCheckWitness,
	#[strum(serialize = "System.Runtime.GetInvocationCounter")]
	SystemRuntimeGetInvocationCounter,
	#[strum(serialize = "System.Runtime.Log")]
	SystemRuntimeLog,
	#[strum(serialize = "System.Runtime.Notify")]
	SystemRuntimeNotify,
	#[strum(serialize = "System.Runtime.GetNotifications")]
	SystemRuntimeGetNotifications,
	#[strum(serialize = "System.Runtime.GasLeft")]
	SystemRuntimeGasLeft,
	#[strum(serialize = "System.Runtime.BurnGas")]
	SystemRuntimeBurnGas,
	#[strum(serialize = "System.Runtime.GetNetwork")]
	SystemRuntimeGetNetwork,
	#[strum(serialize = "System.Runtime.GetRandom")]
	SystemRuntimeGetRandom,
	#[strum(serialize = "System.Storage.GetContext")]
	SystemStorageGetContext,
	#[strum(serialize = "System.Storage.GetReadOnlyContext")]
	SystemStorageGetReadOnlyContext,
	#[strum(serialize = "System.Storage.AsReadOnly")]
	SystemStorageAsReadOnly,
	#[strum(serialize = "System.Storage.Get")]
	SystemStorageGet,
	#[strum(serialize = "System.Storage.Find")]
	SystemStorageFind,
	#[strum(serialize = "System.Storage.Put")]
	SystemStoragePut,
	#[strum(serialize = "System.Storage.Delete")]
	SystemStorageDelete,
}

impl InteropService {
	pub fn hash(&self) -> String {
		let mut hashes = INTEROP_SERVICE_HASHES.lock().unwrap();
		return if let Some(hash) = hashes.get(self.to_string().as_str()) {
			hash.clone()
		} else {
			let sha = self.to_string().as_bytes().hash256()[..4].to_vec();
			let hash = hex::encode(sha);
			hashes.insert(self.to_string(), hash.clone());
			hash
		}
	}

	pub fn from_hash(hash: String) -> Option<InteropService> {
		InteropService::iter().find(|service| service.hash() == hash)
	}
	pub fn price(&self) -> u64 {
		match self {
			InteropService::SystemRuntimePlatform
			| InteropService::SystemRuntimeGetTrigger
			| InteropService::SystemRuntimeGetTime
			| InteropService::SystemRuntimeGetScriptContainer
			| InteropService::SystemRuntimeGetNetwork => 1 << 3,

			InteropService::SystemIteratorValue
			| InteropService::SystemRuntimeGetExecutingScriptHash
			| InteropService::SystemRuntimeGetCallingScriptHash
			| InteropService::SystemRuntimeGetEntryScriptHash
			| InteropService::SystemRuntimeGetInvocationCounter
			| InteropService::SystemRuntimeGasLeft
			| InteropService::SystemRuntimeBurnGas
			| InteropService::SystemRuntimeGetRandom
			| InteropService::SystemStorageGetContext
			| InteropService::SystemStorageGetReadOnlyContext
			| InteropService::SystemStorageAsReadOnly => 1 << 4,

			InteropService::SystemContractGetCallFlags
			| InteropService::SystemRuntimeCheckWitness => 1 << 10,

			InteropService::SystemRuntimeGetNotifications => 1 << 12,

			InteropService::SystemCryptoCheckSig
			| InteropService::SystemContractCall
			| InteropService::SystemContractCreateStandardAccount
			| InteropService::SystemIteratorNext
			| InteropService::SystemRuntimeLog
			| InteropService::SystemRuntimeNotify
			| InteropService::SystemStorageGet
			| InteropService::SystemStorageFind
			| InteropService::SystemStoragePut
			| InteropService::SystemStorageDelete => 1 << 15,
			_ => 0,
		}
	}
}
