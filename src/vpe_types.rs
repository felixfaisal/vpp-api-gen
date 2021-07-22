/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Version { 
	pub major : u32, 
	pub minor : u32, 
	pub patch : u32, 
	pub pre_release : u8, 
	pub build_metadata : u8, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LogLevel { 
	 VPE_API_LOG_LEVEL_EMERG=0, 
	 VPE_API_LOG_LEVEL_ALERT=1, 
	 VPE_API_LOG_LEVEL_CRIT=2, 
	 VPE_API_LOG_LEVEL_ERR=3, 
	 VPE_API_LOG_LEVEL_WARNING=4, 
	 VPE_API_LOG_LEVEL_NOTICE=5, 
	 VPE_API_LOG_LEVEL_INFO=6, 
	 VPE_API_LOG_LEVEL_DEBUG=7, 
	 VPE_API_LOG_LEVEL_DISABLED=8, 
} 
pub type Timestamp=f64; 
pub type Timedelta=f64; 