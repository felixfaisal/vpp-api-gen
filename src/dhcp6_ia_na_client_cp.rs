/*
   Autogenerated Data, Do not Edit! 
   Author: @felixfaisal 
*/
#![allow(non_camel_case_types)]
use vpp_macros::Message; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(dhcp6_client_enable_disable_ae6cfcfb)] 
pub struct Dhcp6ClientEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(dhcp6_client_enable_disable_reply_e8d4e804)] 
pub struct Dhcp6ClientEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
