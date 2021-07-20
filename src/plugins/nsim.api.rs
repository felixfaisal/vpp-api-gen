/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfStatusFlags { 
	 IF_STATUS_API_FLAG_ADMIN_UP=1, 
	 IF_STATUS_API_FLAG_LINK_UP=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum MtuProto { 
	 MTU_PROTO_API_L3=0, 
	 MTU_PROTO_API_IP4=1, 
	 MTU_PROTO_API_IP6=2, 
	 MTU_PROTO_API_MPLS=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LinkDuplex { 
	 LINK_DUPLEX_API_UNKNOWN=0, 
	 LINK_DUPLEX_API_HALF=1, 
	 LINK_DUPLEX_API_FULL=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SubIfFlags { 
	 SUB_IF_API_FLAG_NO_TAGS=1, 
	 SUB_IF_API_FLAG_ONE_TAG=2, 
	 SUB_IF_API_FLAG_TWO_TAGS=4, 
	 SUB_IF_API_FLAG_DOT1AD=8, 
	 SUB_IF_API_FLAG_EXACT_MATCH=16, 
	 SUB_IF_API_FLAG_DEFAULT=32, 
	 SUB_IF_API_FLAG_OUTER_VLAN_ID_ANY=64, 
	 SUB_IF_API_FLAG_INNER_VLAN_ID_ANY=128, 
	 SUB_IF_API_FLAG_MASK_VNET=254, 
	 SUB_IF_API_FLAG_DOT1AH=256, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum RxMode { 
	 RX_MODE_API_UNKNOWN=0, 
	 RX_MODE_API_POLLING=1, 
	 RX_MODE_API_INTERRUPT=2, 
	 RX_MODE_API_ADAPTIVE=3, 
	 RX_MODE_API_DEFAULT=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfType { 
	 IF_API_TYPE_HARDWARE=0, 
	 IF_API_TYPE_SUB=1, 
	 IF_API_TYPE_P2P=2, 
	 IF_API_TYPE_PIPE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum Direction { 
	 RX=0, 
	 TX=1, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimCrossConnectEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub sw_if_index0 : InterfaceIndex, 
	pub sw_if_index1 : InterfaceIndex, 
} 
impl NsimCrossConnectEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_cross_connect_enable_disable_16f70bdf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimCrossConnectEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NsimCrossConnectEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_cross_connect_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimOutputFeatureEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
impl NsimOutputFeatureEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_output_feature_enable_disable_3865946c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimOutputFeatureEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NsimOutputFeatureEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_output_feature_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimConfigure { 
	pub client_index : u32, 
	pub context : u32, 
	pub delay_in_usec : u32, 
	pub average_packet_size : u32, 
	pub bandwidth_in_bits_per_second : u64, 
	pub packets_per_drop : u32, 
} 
impl NsimConfigure { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_configure_16ed400f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimConfigureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NsimConfigureReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_configure_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimConfigure2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub delay_in_usec : u32, 
	pub average_packet_size : u32, 
	pub bandwidth_in_bits_per_second : u64, 
	pub packets_per_drop : u32, 
	pub packets_per_reorder : u32, 
} 
impl NsimConfigure2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_configure2_64de8ed3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NsimConfigure2Reply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NsimConfigure2Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nsim_configure2_reply_e8d4e804") 
	 } 
} 