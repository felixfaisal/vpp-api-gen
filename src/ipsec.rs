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
use crate::ipsec_types::*; 
use crate::ip_types::*; 
use crate::tunnel_types::*; 
use crate::interface_types::*; 
use crate::ip_types::*; 
use crate::interface_types::*; 
use crate::ip_types::*; 
use crate::interface_types::*; 
use crate::tunnel_types::*; 
use crate::interface_types::*; 
use crate::ip_types::*; 
// Implementation for ipsec_spd_entry 
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
// Implementation for ipsec_tunnel_protect 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpsecTunnelProtect { 
	pub sw_if_index : InterfaceIndex, 
	pub nh : Address, 
	pub sa_out : u32, 
	pub n_sa_in : u8, 
	pub sa_in : u32, 
} 
// Implementation for ipsec_itf 
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
	 #[serde(other)] 
	 Invalid 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_add_del_20e89a95)] 
pub struct IpsecSpdAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub spd_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_add_del_reply_e8d4e804)] 
pub struct IpsecSpdAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_interface_add_del_spd_80f80cbb)] 
pub struct IpsecInterfaceAddDelSpd { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub spd_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_interface_add_del_spd_reply_e8d4e804)] 
pub struct IpsecInterfaceAddDelSpdReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_entry_add_del_9f384b8d)] 
pub struct IpsecSpdEntryAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSpdEntry, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_entry_add_del_reply_9ffac24b)] 
pub struct IpsecSpdEntryAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spds_dump_51077d14)] 
pub struct IpsecSpdsDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spds_details_a04bb254)] 
pub struct IpsecSpdsDetails { 
	pub context : u32, 
	pub spd_id : u32, 
	pub npolicies : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_dump_afefbf7d)] 
pub struct IpsecSpdDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub spd_id : u32, 
	pub sa_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_details_f2222790)] 
pub struct IpsecSpdDetails { 
	pub context : u32, 
	pub entry : IpsecSpdEntry, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sad_entry_add_del_b8def364)] 
pub struct IpsecSadEntryAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSadEntry, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sad_entry_add_del_v2_aca78b27)] 
pub struct IpsecSadEntryAddDelV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSadEntryV2, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sad_entry_add_del_v3_c77ebd92)] 
pub struct IpsecSadEntryAddDelV3 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub entry : IpsecSadEntryV3, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sad_entry_add_del_reply_9ffac24b)] 
pub struct IpsecSadEntryAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sad_entry_add_del_v2_reply_9ffac24b)] 
pub struct IpsecSadEntryAddDelV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sad_entry_add_del_v3_reply_9ffac24b)] 
pub struct IpsecSadEntryAddDelV3Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub stat_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_tunnel_protect_update_143f155d)] 
pub struct IpsecTunnelProtectUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub tunnel : IpsecTunnelProtect, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_tunnel_protect_update_reply_e8d4e804)] 
pub struct IpsecTunnelProtectUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_tunnel_protect_del_ddd2ba36)] 
pub struct IpsecTunnelProtectDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub nh : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_tunnel_protect_del_reply_e8d4e804)] 
pub struct IpsecTunnelProtectDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_tunnel_protect_dump_f9e6675e)] 
pub struct IpsecTunnelProtectDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_tunnel_protect_details_ac6c823b)] 
pub struct IpsecTunnelProtectDetails { 
	pub context : u32, 
	pub tun : IpsecTunnelProtect, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_interface_dump_8971de19)] 
pub struct IpsecSpdInterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub spd_index : u32, 
	pub spd_index_valid : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_spd_interface_details_7a0bcf3e)] 
pub struct IpsecSpdInterfaceDetails { 
	pub context : u32, 
	pub spd_index : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_itf_create_6f50b3bc)] 
pub struct IpsecItfCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub itf : IpsecItf, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_itf_create_reply_5383d31f)] 
pub struct IpsecItfCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_itf_delete_f9e6675e)] 
pub struct IpsecItfDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_itf_delete_reply_e8d4e804)] 
pub struct IpsecItfDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_itf_dump_f9e6675e)] 
pub struct IpsecItfDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_itf_details_548a73b8)] 
pub struct IpsecItfDetails { 
	pub context : u32, 
	pub itf : IpsecItf, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sa_dump_2076c2f4)] 
pub struct IpsecSaDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sa_v2_dump_2076c2f4)] 
pub struct IpsecSaV2Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sa_v3_dump_2076c2f4)] 
pub struct IpsecSaV3Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sa_details_b30c7f41)] 
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
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sa_v2_details_e2130051)] 
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
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_sa_v3_details_2fc991ee)] 
pub struct IpsecSaV3Details { 
	pub context : u32, 
	pub entry : IpsecSadEntryV3, 
	pub sw_if_index : InterfaceIndex, 
	pub seq_outbound : u64, 
	pub last_seq_inbound : u64, 
	pub replay_window : u64, 
	pub stat_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_backend_dump_51077d14)] 
pub struct IpsecBackendDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_backend_details_ee601c29)] 
pub struct IpsecBackendDetails { 
	pub context : u32, 
	pub name : FixedSizeString<U128>, 
	pub protocol : IpsecProto, 
	pub index : u8, 
	pub active : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_select_backend_5bcfd3b7)] 
pub struct IpsecSelectBackend { 
	pub client_index : u32, 
	pub context : u32, 
	pub protocol : IpsecProto, 
	pub index : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_select_backend_reply_e8d4e804)] 
pub struct IpsecSelectBackendReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_set_async_mode_a6465f7c)] 
pub struct IpsecSetAsyncMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub async_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipsec_set_async_mode_reply_e8d4e804)] 
pub struct IpsecSetAsyncModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
