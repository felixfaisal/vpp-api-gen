/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdEntry { 
	pub spd_id : u32, 
	pub priority : i32, 
	pub is_outbound : bool, 
	pub sa_id : u32, 
	pub policy : IpsecSpdAction, 
	pub protocol : u8, 
	pub remote_address_start : Address, 
	pub remote_address_stop : Address, 
	pub local_address_start : Address, 
	pub local_address_stop : Address, 
	pub remote_port_start : u16, 
	pub remote_port_stop : u16, 
	pub local_port_start : u16, 
	pub local_port_stop : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtect { 
	pub sw_if_index : InterfaceIndex, 
	pub nh : Address, 
	pub sa_out : u32, 
	pub n_sa_in : u8, 
	pub sa_in : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItf { 
	pub user_instance : u32, 
	pub mode : TunnelMode, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IpsecSpdAction { 
	 IPSEC_API_SPD_ACTION_BYPASS=0, 
	 IPSEC_API_SPD_ACTION_DISCARD=1, 
	 IPSEC_API_SPD_ACTION_RESOLVE=2, 
	 IPSEC_API_SPD_ACTION_PROTECT=3, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub spd_id : u32, 
} 
impl IpsecSpdAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_add_del_20e89a95") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecSpdAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecInterfaceAddDelSpd { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub spd_id : u32, 
} 
impl IpsecInterfaceAddDelSpd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_interface_add_del_spd_80f80cbb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecInterfaceAddDelSpdReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecInterfaceAddDelSpdReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_interface_add_del_spd_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdEntryAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSpdEntry, 
} 
impl IpsecSpdEntryAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_entry_add_del_9f384b8d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdEntryAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
impl IpsecSpdEntryAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_entry_add_del_reply_9ffac24b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdsDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpsecSpdsDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spds_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdsDetails { 
	pub context : u32, 
	pub spd_id : u32, 
	pub npolicies : u32, 
} 
impl IpsecSpdsDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spds_details_a04bb254") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub spd_id : u32, 
	pub sa_id : u32, 
} 
impl IpsecSpdDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_dump_afefbf7d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdDetails { 
	pub context : u32, 
	pub entry : IpsecSpdEntry, 
} 
impl IpsecSpdDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_details_f2222790") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSadEntryAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSadEntry, 
} 
impl IpsecSadEntryAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sad_entry_add_del_b8def364") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSadEntryAddDelV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSadEntryV2, 
} 
impl IpsecSadEntryAddDelV2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sad_entry_add_del_v2_aca78b27") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSadEntryAddDelV3 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSadEntryV3, 
} 
impl IpsecSadEntryAddDelV3 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sad_entry_add_del_v3_c77ebd92") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSadEntryAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
impl IpsecSadEntryAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sad_entry_add_del_reply_9ffac24b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSadEntryAddDelV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
impl IpsecSadEntryAddDelV2Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sad_entry_add_del_v2_reply_9ffac24b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSadEntryAddDelV3Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
impl IpsecSadEntryAddDelV3Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sad_entry_add_del_v3_reply_9ffac24b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtectUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub tunnel : IpsecTunnelProtect, 
} 
impl IpsecTunnelProtectUpdate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_tunnel_protect_update_143f155d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtectUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecTunnelProtectUpdateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_tunnel_protect_update_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtectDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub nh : Address, 
} 
impl IpsecTunnelProtectDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_tunnel_protect_del_ddd2ba36") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtectDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecTunnelProtectDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_tunnel_protect_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtectDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpsecTunnelProtectDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_tunnel_protect_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtectDetails { 
	pub context : u32, 
	pub tun : IpsecTunnelProtect, 
} 
impl IpsecTunnelProtectDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_tunnel_protect_details_ac6c823b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdInterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub spd_index : u32, 
	pub spd_index_valid : u8, 
} 
impl IpsecSpdInterfaceDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_interface_dump_8971de19") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSpdInterfaceDetails { 
	pub context : u32, 
	pub spd_index : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpsecSpdInterfaceDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_spd_interface_details_7a0bcf3e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItfCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub itf : IpsecItf, 
} 
impl IpsecItfCreate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_itf_create_6f50b3bc") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItfCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpsecItfCreateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_itf_create_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItfDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpsecItfDelete { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_itf_delete_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItfDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecItfDeleteReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_itf_delete_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItfDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpsecItfDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_itf_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecItfDetails { 
	pub context : u32, 
	pub itf : IpsecItf, 
} 
impl IpsecItfDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_itf_details_548a73b8") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSaDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_id : u32, 
} 
impl IpsecSaDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sa_dump_2076c2f4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSaV2Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_id : u32, 
} 
impl IpsecSaV2Dump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sa_v2_dump_2076c2f4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSaV3Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_id : u32, 
} 
impl IpsecSaV3Dump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sa_v3_dump_2076c2f4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSaDetails { 
	pub context : u32, 
	pub entry : IpsecSadEntry, 
	pub sw_if_index : InterfaceIndex, 
	pub salt : u32, 
	pub seq_outbound : u64, 
	pub last_seq_inbound : u64, 
	pub replay_window : u64, 
	pub stat_index : u32, 
} 
impl IpsecSaDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sa_details_b30c7f41") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSaV2Details { 
	pub context : u32, 
	pub entry : IpsecSadEntryV2, 
	pub sw_if_index : InterfaceIndex, 
	pub salt : u32, 
	pub seq_outbound : u64, 
	pub last_seq_inbound : u64, 
	pub replay_window : u64, 
	pub stat_index : u32, 
} 
impl IpsecSaV2Details { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sa_v2_details_e2130051") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSaV3Details { 
	pub context : u32, 
	pub entry : IpsecSadEntryV3, 
	pub sw_if_index : InterfaceIndex, 
	pub seq_outbound : u64, 
	pub last_seq_inbound : u64, 
	pub replay_window : u64, 
	pub stat_index : u32, 
} 
impl IpsecSaV3Details { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_sa_v3_details_2fc991ee") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecBackendDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpsecBackendDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_backend_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecBackendDetails { 
	pub context : u32, 
	pub name : FixedSizeString<U128>, 
	pub protocol : IpsecProto, 
	pub index : u8, 
	pub active : bool, 
} 
impl IpsecBackendDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_backend_details_ee601c29") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSelectBackend { 
	pub client_index : u32, 
	pub context : u32, 
	pub protocol : IpsecProto, 
	pub index : u8, 
} 
impl IpsecSelectBackend { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_select_backend_5bcfd3b7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSelectBackendReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecSelectBackendReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_select_backend_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSetAsyncMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub async_enable : bool, 
} 
impl IpsecSetAsyncMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_set_async_mode_a6465f7c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecSetAsyncModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpsecSetAsyncModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipsec_set_async_mode_reply_e8d4e804") 
	 } 
} 
