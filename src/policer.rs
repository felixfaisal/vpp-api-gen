/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
use crate::policer_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerBind { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub worker_index : u32, 
	pub bind_enable : bool, 
} 
impl PolicerBind { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_bind_dcf516f9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerBindReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl PolicerBindReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_bind_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerInput { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub sw_if_index : InterfaceIndex, 
	pub apply : bool, 
} 
impl PolicerInput { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_input_233f0ef5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerInputReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl PolicerInputReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_input_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub name : FixedSizeString<U64>, 
	pub cir : u32, 
	pub eir : u32, 
	pub cb : u64, 
	pub eb : u64, 
	pub rate_type : Sse2QosRateType, 
	pub round_type : Sse2QosRoundType, 
	pub typ : Sse2QosPolicerType, 
	pub color_aware : bool, 
	pub conform_action : Sse2QosAction, 
	pub exceed_action : Sse2QosAction, 
	pub violate_action : Sse2QosAction, 
} 
impl PolicerAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_add_del_cb948f6e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub policer_index : u32, 
} 
impl PolicerAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_add_del_reply_a177cef2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub match_name_valid : bool, 
	pub match_name : FixedSizeString<U64>, 
} 
impl PolicerDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_dump_35f1ae0f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerDetails { 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub cir : u32, 
	pub eir : u32, 
	pub cb : u64, 
	pub eb : u64, 
	pub rate_type : Sse2QosRateType, 
	pub round_type : Sse2QosRoundType, 
	pub typ : Sse2QosPolicerType, 
	pub conform_action : Sse2QosAction, 
	pub exceed_action : Sse2QosAction, 
	pub violate_action : Sse2QosAction, 
	pub single_rate : bool, 
	pub color_aware : bool, 
	pub scale : u32, 
	pub cir_tokens_per_period : u32, 
	pub pir_tokens_per_period : u32, 
	pub current_limit : u32, 
	pub current_bucket : u32, 
	pub extended_limit : u32, 
	pub extended_bucket : u32, 
	pub last_update_time : u64, 
} 
impl PolicerDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("policer_details_a43f781a") 
	 } 
} 