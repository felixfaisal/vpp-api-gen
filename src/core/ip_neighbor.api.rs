/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighbor { 
	pub sw_if_index : InterfaceIndex, 
	pub flags : IpNeighborFlags, 
	pub mac_address : MacAddress, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum AddressFamily { 
	 ADDRESS_IP4=0, 
	 ADDRESS_IP6=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpFeatureLocation { 
	 IP_API_FEATURE_INPUT=0, 
	 IP_API_FEATURE_OUTPUT=1, 
	 IP_API_FEATURE_LOCAL=2, 
	 IP_API_FEATURE_PUNT=3, 
	 IP_API_FEATURE_DROP=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpEcn { 
	 IP_API_ECN_NONE=0, 
	 IP_API_ECN_ECT0=1, 
	 IP_API_ECN_ECT1=2, 
	 IP_API_ECN_CE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpDscp { 
	 IP_API_DSCP_CS0=0, 
	 IP_API_DSCP_CS1=8, 
	 IP_API_DSCP_AF11=10, 
	 IP_API_DSCP_AF12=12, 
	 IP_API_DSCP_AF13=14, 
	 IP_API_DSCP_CS2=16, 
	 IP_API_DSCP_AF21=18, 
	 IP_API_DSCP_AF22=20, 
	 IP_API_DSCP_AF23=22, 
	 IP_API_DSCP_CS3=24, 
	 IP_API_DSCP_AF31=26, 
	 IP_API_DSCP_AF32=28, 
	 IP_API_DSCP_AF33=30, 
	 IP_API_DSCP_CS4=32, 
	 IP_API_DSCP_AF41=34, 
	 IP_API_DSCP_AF42=36, 
	 IP_API_DSCP_AF43=38, 
	 IP_API_DSCP_CS5=40, 
	 IP_API_DSCP_EF=46, 
	 IP_API_DSCP_CS6=48, 
	 IP_API_DSCP_CS7=50, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpProto { 
	 IP_API_PROTO_HOPOPT=0, 
	 IP_API_PROTO_ICMP=1, 
	 IP_API_PROTO_IGMP=2, 
	 IP_API_PROTO_TCP=6, 
	 IP_API_PROTO_UDP=17, 
	 IP_API_PROTO_GRE=47, 
	 IP_API_PROTO_ESP=50, 
	 IP_API_PROTO_AH=51, 
	 IP_API_PROTO_ICMP6=58, 
	 IP_API_PROTO_EIGRP=88, 
	 IP_API_PROTO_OSPF=89, 
	 IP_API_PROTO_SCTP=132, 
	 IP_API_PROTO_RESERVED=255, 
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
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpNeighborFlags { 
	 IP_API_NEIGHBOR_FLAG_NONE=0, 
	 IP_API_NEIGHBOR_FLAG_STATIC=1, 
	 IP_API_NEIGHBOR_FLAG_NO_FIB_ENTRY=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IpNeighborEventFlags { 
	 IP_NEIGHBOR_API_EVENT_FLAG_ADDED=1, 
	 IP_NEIGHBOR_API_EVENT_FLAG_REMOVED=2, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub neighbor : IpNeighbor, 
} 
impl IpNeighborAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_add_del_105518b6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stats_index : u32, 
} 
impl IpNeighborAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_add_del_reply_1992deab") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub af : AddressFamily, 
} 
impl IpNeighborDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_dump_cd831298") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborDetails { 
	pub context : u32, 
	pub age : f64, 
	pub neighbor : IpNeighbor, 
} 
impl IpNeighborDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_details_870e80b9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborConfig { 
	pub client_index : u32, 
	pub context : u32, 
	pub af : AddressFamily, 
	pub max_number : u32, 
	pub max_age : u32, 
	pub recycle : bool, 
} 
impl IpNeighborConfig { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_config_f4a5cf44") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborConfigReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpNeighborConfigReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_config_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborReplaceBegin { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpNeighborReplaceBegin { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_replace_begin_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborReplaceBeginReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpNeighborReplaceBeginReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_replace_begin_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborReplaceEnd { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpNeighborReplaceEnd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_replace_end_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborReplaceEndReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpNeighborReplaceEndReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_replace_end_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborFlush { 
	pub client_index : u32, 
	pub context : u32, 
	pub af : AddressFamily, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpNeighborFlush { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_flush_16aa35d2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpNeighborFlushReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_flush_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIpNeighborEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub pid : u32, 
	pub ip : Address, 
	pub sw_if_index : InterfaceIndex, 
} 
impl WantIpNeighborEvents { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_ip_neighbor_events_1a312870") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIpNeighborEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl WantIpNeighborEventsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_ip_neighbor_events_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub neighbor : IpNeighbor, 
} 
impl IpNeighborEvent { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_event_83933131") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIpNeighborEventsV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub pid : u32, 
	pub ip : Address, 
	pub sw_if_index : InterfaceIndex, 
} 
impl WantIpNeighborEventsV2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_ip_neighbor_events_v2_73e70a86") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIpNeighborEventsV2Reply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl WantIpNeighborEventsV2Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_ip_neighbor_events_v2_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpNeighborEventV2 { 
	pub client_index : u32, 
	pub pid : u32, 
	pub flags : IpNeighborEventFlags, 
	pub neighbor : IpNeighbor, 
} 
impl IpNeighborEventV2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip_neighbor_event_v2_c1d53dc0") 
	 } 
} 