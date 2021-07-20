/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6RaPrefixInfo { 
	pub prefix : Prefix, 
	pub flags : u8, 
	pub valid_time : u32, 
	pub preferred_time : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceIp6ndRaConfig { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub suppress : u8, 
	pub managed : u8, 
	pub other : u8, 
	pub ll_option : u8, 
	pub send_unicast : u8, 
	pub cease : u8, 
	pub is_no : bool, 
	pub default_router : u8, 
	pub max_interval : u32, 
	pub min_interval : u32, 
	pub lifetime : u32, 
	pub initial_count : u32, 
	pub initial_interval : u32, 
} 
impl SwInterfaceIp6ndRaConfig { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sw_interface_ip6nd_ra_config_3eb00b1c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceIp6ndRaConfigReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SwInterfaceIp6ndRaConfigReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sw_interface_ip6nd_ra_config_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceIp6ndRaPrefix { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub prefix : Prefix, 
	pub use_default : bool, 
	pub no_advertise : bool, 
	pub off_link : bool, 
	pub no_autoconfig : bool, 
	pub no_onlink : bool, 
	pub is_no : bool, 
	pub val_lifetime : u32, 
	pub pref_lifetime : u32, 
} 
impl SwInterfaceIp6ndRaPrefix { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sw_interface_ip6nd_ra_prefix_e098785f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceIp6ndRaPrefixReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SwInterfaceIp6ndRaPrefixReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sw_interface_ip6nd_ra_prefix_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6ndProxyAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_add : bool, 
	pub ip : Ip6Address, 
} 
impl Ip6ndProxyAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6nd_proxy_add_del_3fdf6659") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6ndProxyAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ip6ndProxyAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6nd_proxy_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6ndProxyDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip : Ip6Address, 
} 
impl Ip6ndProxyDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6nd_proxy_details_d35be8ff") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6ndProxyDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Ip6ndProxyDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6nd_proxy_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6ndSendRouterSolicitation { 
	pub client_index : u32, 
	pub context : u32, 
	pub irt : u32, 
	pub mrt : u32, 
	pub mrc : u32, 
	pub mrd : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub stop : bool, 
} 
impl Ip6ndSendRouterSolicitation { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6nd_send_router_solicitation_e5de609c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6ndSendRouterSolicitationReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ip6ndSendRouterSolicitationReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6nd_send_router_solicitation_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIp6RaEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub pid : u32, 
} 
impl WantIp6RaEvents { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_ip6_ra_events_3ec6d6c2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIp6RaEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl WantIp6RaEventsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_ip6_ra_events_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6RaEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub router_addr : Ip6Address, 
	pub current_hop_limit : u8, 
	pub flags : u8, 
	pub router_lifetime_in_sec : u16, 
	pub neighbor_reachable_time_in_msec : u32, 
	pub time_in_msec_between_retransmitted_neighbor_solicitations : u32, 
	pub n_prefixes : u32, 
	pub prefixes : Ip6RaPrefixInfo, 
} 
impl Ip6RaEvent { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ip6_ra_event_47e8cfbe") 
	 } 
} 
