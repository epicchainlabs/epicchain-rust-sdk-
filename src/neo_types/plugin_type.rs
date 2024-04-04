use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum NodePluginType {
	#[strum(serialize = "ApplicationLogs")]
	ApplicationLogs,
	#[strum(serialize = "CoreMetrics")]
	CoreMetrics,
	#[strum(serialize = "ImportBlocks")]
	ImportBlocks,
	#[strum(serialize = "LevelDBStore")]
	LevelDbStore,
	#[strum(serialize = "RocksDBStore")]
	RocksDbStore,
	#[strum(serialize = "RpcNep17Tracker")]
	RpcNep17Tracker,
	#[strum(serialize = "RpcSecurity")]
	RpcSecurity,
	#[strum(serialize = "RpcServerPlugin")]
	RpcServerPlugin,
	#[strum(serialize = "RpcSystemAssetTrackerPlugin")]
	RpcSystemAssetTracker,
	#[strum(serialize = "SimplePolicyPlugin")]
	SimplePolicy,
	#[strum(serialize = "StatesDumper")]
	StatesDumper,
	#[strum(serialize = "SystemLog")]
	SystemLog,
}
