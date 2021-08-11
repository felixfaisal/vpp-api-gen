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
use crate::tunnel_types::*; 
use crate::interface_types::*; 
use crate::ip_types::*; 
use crate::ip_types::*; 
// Implementation for gre_tunnel 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GreTunnel { 
	pub typ : GreTunnelType, 
	pub mode : TunnelMode, 
	pub flags : TunnelEncapDecapFlags, 
	pub session_id : u16, 
	pub instance : u32, 
	pub outer_table_id : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub src : Address, 
	pub dst : Address, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum GreTunnelType { 
	 GRE_API_TUNNEL_TYPE_L3=0, 
	 GRE_API_TUNNEL_TYPE_TEB=1, 
	 GRE_API_TUNNEL_TYPE_ERSPAN=2, 
	 #[serde(other)] 
	 Invalid 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(gre_tunnel_add_del_6efc9c22)] 
pub struct GreTunnelAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub tunnel : GreTunnel, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(gre_tunnel_add_del_reply_5383d31f)] 
pub struct GreTunnelAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(gre_tunnel_dump_f9e6675e)] 
pub struct GreTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(gre_tunnel_details_003bfbf1)] 
pub struct GreTunnelDetails { 
	pub context : u32, 
	pub tunnel : GreTunnel, 
} 
