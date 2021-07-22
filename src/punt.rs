/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntL4 { 
	pub af : AddressFamily, 
	pub protocol : IpProto, 
	pub port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntIpProto { 
	pub af : AddressFamily, 
	pub protocol : IpProto, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntException { 
	pub id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Punt { 
	pub typ : PuntType, 
	pub punt : PuntUnion, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntReason { 
	pub id : u32, 
	pub name : String, 
} 
type PuntUnion = [u8;4]; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum PuntType { 
	 PUNT_API_TYPE_L4=1, 
	 PUNT_API_TYPE_IP_PROTO=2, 
	 PUNT_API_TYPE_EXCEPTION=3, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetPunt { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub punt : Punt, 
} 
impl SetPunt { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_punt_83799618") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetPuntReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SetPuntReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_punt_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketRegister { 
	pub client_index : u32, 
	pub context : u32, 
	pub header_version : u32, 
	pub punt : Punt, 
	pub pathname : FixedSizeString<U108>, 
} 
impl PuntSocketRegister { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_register_c8cd10fa") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketRegisterReply { 
	pub context : u32, 
	pub retval : i32, 
	pub pathname : FixedSizeString<U108>, 
} 
impl PuntSocketRegisterReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_register_reply_bd30ae90") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub typ : PuntType, 
} 
impl PuntSocketDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_dump_52974935") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDetails { 
	pub context : u32, 
	pub punt : Punt, 
	pub pathname : FixedSizeString<U108>, 
} 
impl PuntSocketDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_details_1de0ce75") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDeregister { 
	pub client_index : u32, 
	pub context : u32, 
	pub punt : Punt, 
} 
impl PuntSocketDeregister { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_deregister_98a444f4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDeregisterReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl PuntSocketDeregisterReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_deregister_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntReasonDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub reason : PuntReason, 
} 
impl PuntReasonDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_reason_dump_5c0dd4fe") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntReasonDetails { 
	pub context : u32, 
	pub reason : PuntReason, 
} 
impl PuntReasonDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_reason_details_2c9d4a40") 
	 } 
} 