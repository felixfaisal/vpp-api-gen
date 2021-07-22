/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
use crate::ethernet_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArp { 
	pub table_id : u32, 
	pub low : Ip4Address, 
	pub hi : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub proxy : ProxyArp, 
} 
impl ProxyArpAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_add_del_85486cbd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl ProxyArpAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ProxyArpDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpDetails { 
	pub context : u32, 
	pub proxy : ProxyArp, 
} 
impl ProxyArpDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_details_9228c150") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpIntfcEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
impl ProxyArpIntfcEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_intfc_enable_disable_ae6cfcfb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpIntfcEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl ProxyArpIntfcEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_intfc_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpIntfcDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ProxyArpIntfcDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_intfc_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ProxyArpIntfcDetails { 
	pub context : u32, 
	pub sw_if_index : u32, 
} 
impl ProxyArpIntfcDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("proxy_arp_intfc_details_f6458e5f") 
	 } 
} 