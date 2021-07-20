/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct StnAddDelRule { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Address, 
	pub sw_if_index : InterfaceIndex, 
	pub is_add : bool, 
} 
impl StnAddDelRule { 
	 pub fn get_message_id() -> String { 
	 	 String::from("stn_add_del_rule_53f751e6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct StnAddDelRuleReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl StnAddDelRuleReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("stn_add_del_rule_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct StnRulesDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl StnRulesDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("stn_rules_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct StnRulesDetails { 
	pub context : u32, 
	pub ip_address : Address, 
	pub sw_if_index : InterfaceIndex, 
} 
impl StnRulesDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("stn_rules_details_b0f6606c") 
	 } 
} 
