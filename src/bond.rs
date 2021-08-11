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
use crate::ethernet_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum BondMode { 
	 BOND_API_MODE_ROUND_ROBIN=1, 
	 BOND_API_MODE_ACTIVE_BACKUP=2, 
	 BOND_API_MODE_XOR=3, 
	 BOND_API_MODE_BROADCAST=4, 
	 BOND_API_MODE_LACP=5, 
	 #[serde(other)] 
	 Invalid 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum BondLbAlgo { 
	 BOND_API_LB_ALGO_L2=0, 
	 BOND_API_LB_ALGO_L34=1, 
	 BOND_API_LB_ALGO_L23=2, 
	 BOND_API_LB_ALGO_RR=3, 
	 BOND_API_LB_ALGO_BC=4, 
	 BOND_API_LB_ALGO_AB=5, 
	 #[serde(other)] 
	 Invalid 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_create_48883c7e)] 
pub struct BondCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub id : u32, 
	pub use_custom_mac : bool, 
	pub mac_address : MacAddress, 
	pub mode : BondMode, 
	pub lb : BondLbAlgo, 
	pub numa_only : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_create_reply_5383d31f)] 
pub struct BondCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_create2_912fda76)] 
pub struct BondCreate2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub mode : BondMode, 
	pub lb : BondLbAlgo, 
	pub numa_only : bool, 
	pub enable_gso : bool, 
	pub use_custom_mac : bool, 
	pub mac_address : MacAddress, 
	pub id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_create2_reply_5383d31f)] 
pub struct BondCreate2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_delete_f9e6675e)] 
pub struct BondDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_delete_reply_e8d4e804)] 
pub struct BondDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_enslave_076ecfa7)] 
pub struct BondEnslave { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub bond_sw_if_index : InterfaceIndex, 
	pub is_passive : bool, 
	pub is_long_timeout : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_enslave_reply_e8d4e804)] 
pub struct BondEnslaveReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_add_member_e7d14948)] 
pub struct BondAddMember { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub bond_sw_if_index : InterfaceIndex, 
	pub is_passive : bool, 
	pub is_long_timeout : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_add_member_reply_e8d4e804)] 
pub struct BondAddMemberReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_detach_slave_f9e6675e)] 
pub struct BondDetachSlave { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_detach_slave_reply_e8d4e804)] 
pub struct BondDetachSlaveReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_detach_member_f9e6675e)] 
pub struct BondDetachMember { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(bond_detach_member_reply_e8d4e804)] 
pub struct BondDetachMemberReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_bond_dump_51077d14)] 
pub struct SwInterfaceBondDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_bond_details_f5ef2106)] 
pub struct SwInterfaceBondDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub id : u32, 
	pub mode : BondMode, 
	pub lb : BondLbAlgo, 
	pub numa_only : bool, 
	pub active_slaves : u32, 
	pub slaves : u32, 
	pub interface_name : FixedSizeString<U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_bond_interface_dump_f9e6675e)] 
pub struct SwBondInterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_bond_interface_details_9428a69c)] 
pub struct SwBondInterfaceDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub id : u32, 
	pub mode : BondMode, 
	pub lb : BondLbAlgo, 
	pub numa_only : bool, 
	pub active_members : u32, 
	pub members : u32, 
	pub interface_name : FixedSizeString<U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_slave_dump_f9e6675e)] 
pub struct SwInterfaceSlaveDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_slave_details_3c4a0e23)] 
pub struct SwInterfaceSlaveDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub interface_name : FixedSizeString<U64>, 
	pub is_passive : bool, 
	pub is_long_timeout : bool, 
	pub is_local_numa : bool, 
	pub weight : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_member_interface_dump_f9e6675e)] 
pub struct SwMemberInterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_member_interface_details_3c4a0e23)] 
pub struct SwMemberInterfaceDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub interface_name : FixedSizeString<U64>, 
	pub is_passive : bool, 
	pub is_long_timeout : bool, 
	pub is_local_numa : bool, 
	pub weight : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_set_bond_weight_deb510a0)] 
pub struct SwInterfaceSetBondWeight { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub weight : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_set_bond_weight_reply_e8d4e804)] 
pub struct SwInterfaceSetBondWeightReply { 
	pub context : u32, 
	pub retval : i32, 
} 
