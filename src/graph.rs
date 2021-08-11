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
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum NodeFlag { 
	 NODE_FLAG_FRAME_NO_FREE_AFTER_DISPATCH=1, 
	 NODE_FLAG_IS_OUTPUT=2, 
	 NODE_FLAG_IS_DROP=4, 
	 NODE_FLAG_IS_PUNT=8, 
	 NODE_FLAG_IS_HANDOFF=16, 
	 NODE_FLAG_TRACE=32, 
	 NODE_FLAG_SWITCH_FROM_INTERRUPT_TO_POLLING_MODE=64, 
	 NODE_FLAG_SWITCH_FROM_POLLING_TO_INTERRUPT_MODE=128, 
	 NODE_FLAG_TRACE_SUPPORTED=256, 
	 #[serde(other)] 
	 Invalid 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(graph_node_get_39c8792e)] 
pub struct GraphNodeGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub cursor : u32, 
	pub index : u32, 
	pub name : FixedSizeString<U64>, 
	pub flags : NodeFlag, 
	pub want_arcs : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(graph_node_get_reply_53b48f5d)] 
pub struct GraphNodeGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(graph_node_details_ac762018)] 
pub struct GraphNodeDetails { 
	pub context : u32, 
	pub index : u32, 
	pub name : FixedSizeString<U64>, 
	pub flags : NodeFlag, 
	pub n_arcs : u32, 
	pub arcs_out : u32, 
} 
