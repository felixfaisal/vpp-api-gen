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
#[message_name_and_crc(l2_emulation_ae6cfcfb)] 
pub struct L2Emulation { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(l2_emulation_reply_e8d4e804)] 
pub struct L2EmulationReply { 
	pub context : u32, 
	pub retval : i32, 
} 
