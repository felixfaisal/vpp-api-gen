/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclPluginGetVersion { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl AclPluginGetVersion { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_plugin_get_version_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclPluginGetVersionReply { 
	pub context : u32, 
	pub major : u32, 
	pub minor : u32, 
} 
impl AclPluginGetVersionReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_plugin_get_version_reply_9b32cf86") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclPluginControlPing { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl AclPluginControlPing { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_plugin_control_ping_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclPluginControlPingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub client_index : u32, 
	pub vpe_pid : u32, 
} 
impl AclPluginControlPingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_plugin_control_ping_reply_f6b0b8ca") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclPluginGetConnTableMaxEntries { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl AclPluginGetConnTableMaxEntries { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_plugin_get_conn_table_max_entries_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclPluginGetConnTableMaxEntriesReply { 
	pub context : u32, 
	pub conn_table_max_entries : u64, 
} 
impl AclPluginGetConnTableMaxEntriesReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_plugin_get_conn_table_max_entries_reply_7a096d3d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclAddReplace { 
	pub client_index : u32, 
	pub context : u32, 
	pub acl_index : u32, 
	pub tag : FixedSizeString<U64>, 
	pub count : u32, 
	pub r : AclRule, 
} 
impl AclAddReplace { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_add_replace_1cabdeab") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclAddReplaceReply { 
	pub context : u32, 
	pub acl_index : u32, 
	pub retval : i32, 
} 
impl AclAddReplaceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_add_replace_reply_ac407b0c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub acl_index : u32, 
} 
impl AclDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_del_ef34fea4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AclDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_input : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub acl_index : u32, 
} 
impl AclInterfaceAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_add_del_4b54bebd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AclInterfaceAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceSetAclList { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub count : u8, 
	pub n_input : u8, 
	pub acls : u32, 
} 
impl AclInterfaceSetAclList { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_set_acl_list_473982bd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceSetAclListReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AclInterfaceSetAclListReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_set_acl_list_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub acl_index : u32, 
} 
impl AclDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_dump_ef34fea4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclDetails { 
	pub context : u32, 
	pub acl_index : u32, 
	pub tag : FixedSizeString<U64>, 
	pub count : u32, 
	pub r : AclRule, 
} 
impl AclDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_details_7a97f21c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceListDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl AclInterfaceListDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_list_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceListDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub count : u8, 
	pub n_input : u8, 
	pub acls : u32, 
} 
impl AclInterfaceListDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_list_details_e695d256") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub tag : FixedSizeString<U64>, 
	pub count : u32, 
	pub r : MacipAclRule, 
} 
impl MacipAclAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_add_d648fd0a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclAddReply { 
	pub context : u32, 
	pub acl_index : u32, 
	pub retval : i32, 
} 
impl MacipAclAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_add_reply_ac407b0c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclAddReplace { 
	pub client_index : u32, 
	pub context : u32, 
	pub acl_index : u32, 
	pub tag : FixedSizeString<U64>, 
	pub count : u32, 
	pub r : MacipAclRule, 
} 
impl MacipAclAddReplace { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_add_replace_e34402a7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclAddReplaceReply { 
	pub context : u32, 
	pub acl_index : u32, 
	pub retval : i32, 
} 
impl MacipAclAddReplaceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_add_replace_reply_ac407b0c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub acl_index : u32, 
} 
impl MacipAclDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_del_ef34fea4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MacipAclDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclInterfaceAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub acl_index : u32, 
} 
impl MacipAclInterfaceAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_interface_add_del_4b8690b1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclInterfaceAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MacipAclInterfaceAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_interface_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub acl_index : u32, 
} 
impl MacipAclDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_dump_ef34fea4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclDetails { 
	pub context : u32, 
	pub acl_index : u32, 
	pub tag : FixedSizeString<U64>, 
	pub count : u32, 
	pub r : MacipAclRule, 
} 
impl MacipAclDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_details_57c7482f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclInterfaceGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl MacipAclInterfaceGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_interface_get_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclInterfaceGetReply { 
	pub context : u32, 
	pub count : u32, 
	pub acls : u32, 
} 
impl MacipAclInterfaceGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_interface_get_reply_accf9b05") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclInterfaceListDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl MacipAclInterfaceListDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_interface_list_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MacipAclInterfaceListDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub count : u8, 
	pub acls : u32, 
} 
impl MacipAclInterfaceListDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("macip_acl_interface_list_details_a0c5d56d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceSetEtypeWhitelist { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub count : u8, 
	pub n_input : u8, 
	pub whitelist : u16, 
} 
impl AclInterfaceSetEtypeWhitelist { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_set_etype_whitelist_3f5c2d2d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceSetEtypeWhitelistReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AclInterfaceSetEtypeWhitelistReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_set_etype_whitelist_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceEtypeWhitelistDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl AclInterfaceEtypeWhitelistDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_etype_whitelist_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclInterfaceEtypeWhitelistDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub count : u8, 
	pub n_input : u8, 
	pub whitelist : u16, 
} 
impl AclInterfaceEtypeWhitelistDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_interface_etype_whitelist_details_cc2bfded") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclStatsIntfCountersEnable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
} 
impl AclStatsIntfCountersEnable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_stats_intf_counters_enable_b3e225d2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AclStatsIntfCountersEnableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AclStatsIntfCountersEnableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("acl_stats_intf_counters_enable_reply_e8d4e804") 
	 } 
} 
