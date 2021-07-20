/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TeibEntry { 
	pub sw_if_index : InterfaceIndex, 
	pub peer : Address, 
	pub nh : Address, 
	pub nh_table_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TeibEntryAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub entry : TeibEntry, 
} 
impl TeibEntryAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("teib_entry_add_del_5aa0a538") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TeibEntryAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl TeibEntryAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("teib_entry_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TeibDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl TeibDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("teib_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TeibDetails { 
	pub context : u32, 
	pub entry : TeibEntry, 
} 
impl TeibDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("teib_details_e3b6a503") 
	 } 
} 
