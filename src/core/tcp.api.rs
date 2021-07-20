/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TcpConfigureSrcAddresses { 
	pub client_index : u32, 
	pub context : u32, 
	pub vrf_id : u32, 
	pub first_address : Address, 
	pub last_address : Address, 
} 
impl TcpConfigureSrcAddresses { 
	 pub fn get_message_id() -> String { 
	 	 String::from("tcp_configure_src_addresses_4b02b946") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TcpConfigureSrcAddressesReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl TcpConfigureSrcAddressesReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("tcp_configure_src_addresses_reply_e8d4e804") 
	 } 
} 
