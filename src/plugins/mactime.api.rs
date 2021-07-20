/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TimeRange { 
	pub start : f64, 
	pub end : f64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeTimeRange { 
	pub start : f64, 
	pub end : f64, 
} 
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
pub struct MactimeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
impl MactimeEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_enable_disable_3865946c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MactimeEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeAddDelRange { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub drop : bool, 
	pub allow : bool, 
	pub allow_quota : u8, 
	pub no_udp_10001 : bool, 
	pub data_quota : u64, 
	pub mac_address : MacAddress, 
	pub device_name : FixedSizeString<U64>, 
	pub count : u32, 
	pub ranges : TimeRange, 
} 
impl MactimeAddDelRange { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_add_del_range_101858ef") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeAddDelRangeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MactimeAddDelRangeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_add_del_range_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub my_table_epoch : u32, 
} 
impl MactimeDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_dump_8f454e23") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeDetails { 
	pub context : u32, 
	pub pool_index : u32, 
	pub mac_address : MacAddress, 
	pub data_quota : u64, 
	pub data_used_in_range : u64, 
	pub flags : u32, 
	pub device_name : FixedSizeString<U64>, 
	pub nranges : u32, 
	pub ranges : MactimeTimeRange, 
} 
impl MactimeDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_details_44921c06") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MactimeDumpReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_epoch : u32, 
} 
impl MactimeDumpReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("mactime_dump_reply_49bcc753") 
	 } 
} 